use crate::models::{
    CreateMealTemplate, LocationType, MealTemplate, MealTemplateRow, SlotType, UpdateMealTemplate,
};
use sqlx::{Result, Row, SqlitePool};

pub struct MealTemplateRepository;

impl MealTemplateRepository {
    /// Helper to map a row to MealTemplate
    fn row_to_template(row: &sqlx::sqlite::SqliteRow) -> Result<MealTemplate> {
        let location_str: String = row.try_get("location_type")?;
        let location_type = LocationType::from_db_string(&location_str).map_err(|e| {
            sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e,
            )))
        })?;

        let compatible_slots_json: String = row.try_get("compatible_slots")?;
        let compatible_slots = MealTemplate::parse_compatible_slots(&compatible_slots_json).map_err(|e| {
            sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )))
        })?;

        Ok(MealTemplate {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            compatible_slots,
            location_type,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Create a new meal template
    pub async fn create(pool: &SqlitePool, template: CreateMealTemplate) -> Result<MealTemplate> {
        template.validate().map_err(sqlx::Error::Protocol)?;

        let location_str = template.location_type.to_db_string();
        let compatible_slots_json =
            MealTemplate::serialize_compatible_slots(&template.compatible_slots);

        let row = sqlx::query(
            r#"
            INSERT INTO meal_templates (name, description, compatible_slots, location_type)
            VALUES (?1, ?2, ?3, ?4)
            RETURNING id, name, description, compatible_slots, location_type, created_at, updated_at
            "#,
        )
        .bind(&template.name)
        .bind(&template.description)
        .bind(&compatible_slots_json)
        .bind(location_str)
        .fetch_one(pool)
        .await?;

        Self::row_to_template(&row)
    }

    /// Get a template by ID
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Option<MealTemplate>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, compatible_slots, location_type, created_at, updated_at
            FROM meal_templates
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_template(&r)?)),
            None => Ok(None),
        }
    }

    /// Get all templates
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<MealTemplate>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, compatible_slots, location_type, created_at, updated_at
            FROM meal_templates
            ORDER BY name
            "#,
        )
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_template).collect()
    }

    /// Get templates by location type
    pub async fn get_by_location(
        pool: &SqlitePool,
        location: LocationType,
    ) -> Result<Vec<MealTemplate>> {
        let location_str = location.to_db_string();

        let rows = sqlx::query(
            r#"
            SELECT id, name, description, compatible_slots, location_type, created_at, updated_at
            FROM meal_templates
            WHERE location_type = ?1 OR location_type = 'any'
            ORDER BY name
            "#,
        )
        .bind(location_str)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_template).collect()
    }

    /// Get templates compatible with a specific slot
    pub async fn get_by_slot(pool: &SqlitePool, slot: SlotType) -> Result<Vec<MealTemplate>> {
        // Fetch all templates and filter in Rust
        // This is simpler and more reliable than trying to match JSON in SQL
        let all_templates = Self::get_all(pool).await?;

        Ok(all_templates
            .into_iter()
            .filter(|t| t.compatible_slots.contains(&slot))
            .collect())
    }

    /// Search templates by name
    pub async fn search(pool: &SqlitePool, query: &str) -> Result<Vec<MealTemplate>> {
        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query(
            r#"
            SELECT id, name, description, compatible_slots, location_type, created_at, updated_at
            FROM meal_templates
            WHERE name LIKE ?1 OR description LIKE ?1
            ORDER BY name
            "#,
        )
        .bind(search_pattern)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_template).collect()
    }

    /// Update a template
    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        update: UpdateMealTemplate,
    ) -> Result<MealTemplate> {
        // Get existing template first
        let existing = Self::get_by_id(pool, id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        // Apply updates
        let name = update.name.unwrap_or(existing.name);
        let description = match update.description {
            Some(val) => val,
            None => existing.description,
        };
        let compatible_slots = update.compatible_slots.unwrap_or(existing.compatible_slots);
        let location_type = update.location_type.unwrap_or(existing.location_type);

        let location_str = location_type.to_db_string();
        let compatible_slots_json = MealTemplate::serialize_compatible_slots(&compatible_slots);

        let row = sqlx::query(
            r#"
            UPDATE meal_templates
            SET name = ?1, description = ?2, compatible_slots = ?3, location_type = ?4
            WHERE id = ?5
            RETURNING id, name, description, compatible_slots, location_type, created_at, updated_at
            "#,
        )
        .bind(&name)
        .bind(&description)
        .bind(&compatible_slots_json)
        .bind(location_str)
        .bind(id)
        .fetch_one(pool)
        .await?;

        Self::row_to_template(&row)
    }

    /// Delete a template
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM meal_templates WHERE id = ?1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use tempfile::tempdir;

    async fn setup_test_db() -> SqlitePool {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        db::initialize_database(db_path).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_template() {
        let pool = setup_test_db().await;

        let create = CreateMealTemplate {
            name: "Pane con marmellata".to_string(),
            description: Some("Bread with jam".to_string()),
            compatible_slots: vec![SlotType::Breakfast, SlotType::MorningSnack],
            location_type: LocationType::Home,
        };

        let template = MealTemplateRepository::create(&pool, create).await.unwrap();

        assert_eq!(template.name, "Pane con marmellata");
        assert_eq!(template.compatible_slots.len(), 2);
        assert_eq!(template.location_type, LocationType::Home);
        assert!(template.id > 0);
    }

    #[tokio::test]
    async fn test_get_template_by_id() {
        let pool = setup_test_db().await;

        let created = MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Yogurt".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Breakfast],
                location_type: LocationType::Any,
            },
        )
        .await
        .unwrap();

        let fetched = MealTemplateRepository::get_by_id(&pool, created.id)
            .await
            .unwrap();

        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().name, "Yogurt");
    }

    #[tokio::test]
    async fn test_get_by_location() {
        let pool = setup_test_db().await;

        // Create templates with different locations
        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Home Meal".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Home,
            },
        )
        .await
        .unwrap();

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Office Meal".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Office,
            },
        )
        .await
        .unwrap();

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Any Location".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Any,
            },
        )
        .await
        .unwrap();

        let home_templates = MealTemplateRepository::get_by_location(&pool, LocationType::Home)
            .await
            .unwrap();

        // Should get Home + Any
        assert_eq!(home_templates.len(), 2);
        assert!(home_templates.iter().any(|t| t.name == "Home Meal"));
        assert!(home_templates.iter().any(|t| t.name == "Any Location"));
    }

    #[tokio::test]
    async fn test_get_by_slot() {
        let pool = setup_test_db().await;

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Breakfast Only".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Breakfast],
                location_type: LocationType::Home,
            },
        )
        .await
        .unwrap();

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Lunch and Dinner".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Lunch, SlotType::Dinner],
                location_type: LocationType::Home,
            },
        )
        .await
        .unwrap();

        let breakfast_templates = MealTemplateRepository::get_by_slot(&pool, SlotType::Breakfast)
            .await
            .unwrap();
        assert_eq!(breakfast_templates.len(), 1);
        assert_eq!(breakfast_templates[0].name, "Breakfast Only");

        let lunch_templates = MealTemplateRepository::get_by_slot(&pool, SlotType::Lunch)
            .await
            .unwrap();
        assert_eq!(lunch_templates.len(), 1);
        assert_eq!(lunch_templates[0].name, "Lunch and Dinner");
    }

    #[tokio::test]
    async fn test_search_templates() {
        let pool = setup_test_db().await;

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Pasta carbonara".to_string(),
                description: Some("Classic pasta dish".to_string()),
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Home,
            },
        )
        .await
        .unwrap();

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Pasta aglio e olio".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Dinner],
                location_type: LocationType::Home,
            },
        )
        .await
        .unwrap();

        let results = MealTemplateRepository::search(&pool, "pasta")
            .await
            .unwrap();
        assert_eq!(results.len(), 2);

        let results = MealTemplateRepository::search(&pool, "carbonara")
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Pasta carbonara");
    }

    #[tokio::test]
    async fn test_update_template() {
        let pool = setup_test_db().await;

        let created = MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Original".to_string(),
                description: Some("Original description".to_string()),
                compatible_slots: vec![SlotType::Breakfast],
                location_type: LocationType::Home,
            },
        )
        .await
        .unwrap();

        let updated = MealTemplateRepository::update(
            &pool,
            created.id,
            UpdateMealTemplate {
                name: Some("Updated".to_string()),
                description: Some(None), // Clear description
                compatible_slots: Some(vec![SlotType::Lunch, SlotType::Dinner]),
                location_type: Some(LocationType::Office),
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.name, "Updated");
        assert!(updated.description.is_none());
        assert_eq!(updated.compatible_slots.len(), 2);
        assert_eq!(updated.location_type, LocationType::Office);
    }

    #[tokio::test]
    async fn test_delete_template() {
        let pool = setup_test_db().await;

        let created = MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "To Delete".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Breakfast],
                location_type: LocationType::Home,
            },
        )
        .await
        .unwrap();

        let deleted = MealTemplateRepository::delete(&pool, created.id)
            .await
            .unwrap();
        assert!(deleted);

        let fetched = MealTemplateRepository::get_by_id(&pool, created.id)
            .await
            .unwrap();
        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn test_validation_error() {
        let pool = setup_test_db().await;

        // Empty name should fail
        let result = MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Breakfast],
                location_type: LocationType::Home,
            },
        )
        .await;

        assert!(result.is_err());
    }
}
