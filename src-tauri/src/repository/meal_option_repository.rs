use crate::models::{CreateMealOption, MealOption, MealOptionWithTags, UpdateMealOption};
use sqlx::{Result, Row, SqlitePool};

pub struct MealOptionRepository;

impl MealOptionRepository {
    /// Helper to convert a database row to MealOption
    fn row_to_option(row: &sqlx::sqlite::SqliteRow) -> Result<MealOption> {
        let id = row.try_get("id")?;
        let template_id = row.try_get("template_id")?;
        let name = row.try_get("name")?;
        let description: Option<String> = row.try_get("description")?;
        let nutritional_notes: Option<String> = row.try_get("nutritional_notes")?;
        let created_at = row.try_get("created_at")?;
        let updated_at = row.try_get("updated_at")?;

        Ok(MealOption {
            id,
            template_id,
            name,
            description,
            nutritional_notes,
            created_at,
            updated_at,
        })
    }

    /// Create a new meal option
    pub async fn create(pool: &SqlitePool, option: CreateMealOption) -> Result<MealOption> {
        // Validate using the model's validation method
        option.validate().map_err(sqlx::Error::Protocol)?;

        // Check that template_id exists
        let template_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM meal_templates WHERE id = ?)")
                .bind(option.template_id)
                .fetch_one(pool)
                .await?;

        if !template_exists {
            return Err(sqlx::Error::Protocol(format!(
                "Template with id {} does not exist",
                option.template_id
            )));
        }

        let result = sqlx::query(
            "INSERT INTO meal_options (template_id, name, description, nutritional_notes) 
             VALUES (?, ?, ?, ?)",
        )
        .bind(option.template_id)
        .bind(&option.name)
        .bind(&option.description)
        .bind(&option.nutritional_notes)
        .execute(pool)
        .await?;

        let id = result.last_insert_rowid();
        Self::get_by_id(pool, id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)
    }

    /// Get a meal option by ID
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Option<MealOption>> {
        let row = sqlx::query(
            "SELECT id, template_id, name, description, nutritional_notes, 
                    created_at, updated_at
             FROM meal_options 
             WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_option(&row)?)),
            None => Ok(None),
        }
    }

    /// Get all meal options
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<MealOption>> {
        let rows = sqlx::query(
            "SELECT id, template_id, name, description, nutritional_notes, 
                    created_at, updated_at
             FROM meal_options 
             ORDER BY name",
        )
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_option).collect()
    }

    /// Get all meal options for a specific template
    pub async fn get_by_template_id(
        pool: &SqlitePool,
        template_id: i64,
    ) -> Result<Vec<MealOption>> {
        let rows = sqlx::query(
            "SELECT id, template_id, name, description, nutritional_notes, 
                    created_at, updated_at
             FROM meal_options 
             WHERE template_id = ?
             ORDER BY name",
        )
        .bind(template_id)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_option).collect()
    }

    /// Get a meal option with its associated tags
    pub async fn get_with_tags(pool: &SqlitePool, id: i64) -> Result<Option<MealOptionWithTags>> {
        let option = match Self::get_by_id(pool, id).await? {
            Some(opt) => opt,
            None => return Ok(None),
        };

        let tag_ids = Self::get_tag_ids_for_option(pool, id).await?;

        Ok(Some(MealOptionWithTags {
            option,
            tags: tag_ids,
        }))
    }

    /// Get all meal options for a template with their tags
    pub async fn get_by_template_with_tags(
        pool: &SqlitePool,
        template_id: i64,
    ) -> Result<Vec<MealOptionWithTags>> {
        let options = Self::get_by_template_id(pool, template_id).await?;

        let mut options_with_tags = Vec::new();
        for option in options {
            let tag_ids = Self::get_tag_ids_for_option(pool, option.id).await?;
            options_with_tags.push(MealOptionWithTags {
                option,
                tags: tag_ids,
            });
        }

        Ok(options_with_tags)
    }

    /// Get all tag IDs associated with a meal option
    async fn get_tag_ids_for_option(pool: &SqlitePool, option_id: i64) -> Result<Vec<i64>> {
        let rows = sqlx::query(
            "SELECT tag_id FROM meal_option_tags WHERE meal_option_id = ? ORDER BY tag_id",
        )
        .bind(option_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.iter().map(|row| row.get("tag_id")).collect())
    }

    /// Add tags to a meal option
    pub async fn add_tags(pool: &SqlitePool, option_id: i64, tag_ids: Vec<i64>) -> Result<()> {
        // Verify option exists
        if Self::get_by_id(pool, option_id).await?.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        // Verify all tags exist
        for tag_id in &tag_ids {
            let tag_exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tags WHERE id = ?)")
                    .bind(tag_id)
                    .fetch_one(pool)
                    .await?;

            if !tag_exists {
                return Err(sqlx::Error::Protocol(format!(
                    "Tag with id {} does not exist",
                    tag_id
                )));
            }
        }

        // Insert tags (ignore duplicates)
        for tag_id in tag_ids {
            sqlx::query(
                "INSERT OR IGNORE INTO meal_option_tags (meal_option_id, tag_id) VALUES (?, ?)",
            )
            .bind(option_id)
            .bind(tag_id)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    /// Remove tags from a meal option
    pub async fn remove_tags(pool: &SqlitePool, option_id: i64, tag_ids: Vec<i64>) -> Result<()> {
        for tag_id in tag_ids {
            sqlx::query("DELETE FROM meal_option_tags WHERE meal_option_id = ? AND tag_id = ?")
                .bind(option_id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    /// Replace all tags for a meal option
    pub async fn set_tags(pool: &SqlitePool, option_id: i64, tag_ids: Vec<i64>) -> Result<()> {
        // Verify option exists
        if Self::get_by_id(pool, option_id).await?.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        // Verify all tags exist
        for tag_id in &tag_ids {
            let tag_exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tags WHERE id = ?)")
                    .bind(tag_id)
                    .fetch_one(pool)
                    .await?;

            if !tag_exists {
                return Err(sqlx::Error::Protocol(format!(
                    "Tag with id {} does not exist",
                    tag_id
                )));
            }
        }

        // Remove all existing tags
        sqlx::query("DELETE FROM meal_option_tags WHERE meal_option_id = ?")
            .bind(option_id)
            .execute(pool)
            .await?;

        // Add new tags
        for tag_id in tag_ids {
            sqlx::query("INSERT INTO meal_option_tags (meal_option_id, tag_id) VALUES (?, ?)")
                .bind(option_id)
                .bind(tag_id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }

    /// Search meal options by name or description
    pub async fn search(pool: &SqlitePool, query: &str) -> Result<Vec<MealOption>> {
        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query(
            "SELECT id, template_id, name, description, nutritional_notes, 
                    created_at, updated_at
             FROM meal_options 
             WHERE name LIKE ? OR description LIKE ?
             ORDER BY name",
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_option).collect()
    }

    /// Update a meal option
    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        update: UpdateMealOption,
    ) -> Result<MealOption> {
        // Check that option exists
        if Self::get_by_id(pool, id).await?.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        // Build dynamic update query based on which fields are Some
        let mut updates = Vec::new();
        let mut query_str = String::from("UPDATE meal_options SET ");

        if update.name.is_some() {
            updates.push("name = ?");
        }
        if update.description.is_some() {
            updates.push("description = ?");
        }
        if update.nutritional_notes.is_some() {
            updates.push("nutritional_notes = ?");
        }

        if updates.is_empty() {
            // No updates to make, just return the current option
            return Self::get_by_id(pool, id)
                .await?
                .ok_or_else(|| sqlx::Error::RowNotFound);
        }

        query_str.push_str(&updates.join(", "));
        query_str.push_str(", updated_at = CURRENT_TIMESTAMP WHERE id = ?");

        let mut query = sqlx::query(&query_str);

        // Bind parameters in the same order as the updates vector
        if let Some(name) = &update.name {
            query = query.bind(name);
        }
        if let Some(description) = &update.description {
            query = query.bind(description.as_ref());
        }
        if let Some(nutritional_notes) = &update.nutritional_notes {
            query = query.bind(nutritional_notes.as_ref());
        }

        query = query.bind(id);
        query.execute(pool).await?;

        Self::get_by_id(pool, id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)
    }

    /// Delete a meal option
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<()> {
        let result = sqlx::query("DELETE FROM meal_options WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::models::{CreateMealTemplate, CreateTag, LocationType, SlotType, TagCategory};
    use crate::repository::{MealTemplateRepository, TagRepository};
    use std::path::PathBuf;
    use tempfile::TempDir;

    async fn setup_test_db() -> (SqlitePool, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let pool = db::initialize_database(db_path).await.unwrap();
        (pool, temp_dir)
    }

    async fn create_test_template(pool: &SqlitePool) -> i64 {
        let template = CreateMealTemplate {
            name: "Test Template".to_string(),
            description: Some("Test Description".to_string()),
            location_type: LocationType::Home,
            compatible_slots: vec![SlotType::Breakfast],
            weekly_limit: None,
        };

        let created = MealTemplateRepository::create(pool, template)
            .await
            .unwrap();
        created.id
    }

    async fn create_test_tag(pool: &SqlitePool, name: &str, category: TagCategory) -> i64 {
        let tag = CreateTag {
            name: name.to_string(),
            display_name: name.to_string(),
            category,
            parent_tag_id: None,
            weekly_suggestion: Some(3),
        };

        let created = TagRepository::create(pool, tag).await.unwrap();
        created.id
    }

    #[tokio::test]
    async fn test_create_option() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        let option = CreateMealOption {
            template_id,
            name: "Ricotta Option".to_string(),
            description: Some("Fresh ricotta cheese".to_string()),
            nutritional_notes: Some("Low fat version".to_string()),
        };

        let created = MealOptionRepository::create(&pool, option).await.unwrap();

        assert_eq!(created.name, "Ricotta Option");
        assert_eq!(created.template_id, template_id);
        assert_eq!(
            created.description,
            Some("Fresh ricotta cheese".to_string())
        );
        assert_eq!(
            created.nutritional_notes,
            Some("Low fat version".to_string())
        );
    }

    #[tokio::test]
    async fn test_get_option_by_id() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        let option = CreateMealOption {
            template_id,
            name: "Philadelphia".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let created = MealOptionRepository::create(&pool, option).await.unwrap();
        let retrieved = MealOptionRepository::get_by_id(&pool, created.id)
            .await
            .unwrap();

        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.name, "Philadelphia");
    }

    #[tokio::test]
    async fn test_get_by_template_id() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        // Create multiple options for the template
        for name in &["Option 1", "Option 2", "Option 3"] {
            let option = CreateMealOption {
                template_id,
                name: name.to_string(),
                description: None,
                nutritional_notes: None,
            };
            MealOptionRepository::create(&pool, option).await.unwrap();
        }

        let options = MealOptionRepository::get_by_template_id(&pool, template_id)
            .await
            .unwrap();
        assert_eq!(options.len(), 3);
        assert!(options.iter().any(|o| o.name == "Option 1"));
        assert!(options.iter().any(|o| o.name == "Option 2"));
        assert!(options.iter().any(|o| o.name == "Option 3"));
    }

    #[tokio::test]
    async fn test_option_with_tags() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        // Create tags
        let cheese_tag_id = create_test_tag(&pool, "formaggio", TagCategory::Ingredient).await;
        let dairy_tag_id = create_test_tag(&pool, "latticini", TagCategory::Ingredient).await;

        // Create option
        let option = CreateMealOption {
            template_id,
            name: "Ricotta".to_string(),
            description: None,
            nutritional_notes: None,
        };
        let created = MealOptionRepository::create(&pool, option).await.unwrap();

        // Add tags
        MealOptionRepository::add_tags(&pool, created.id, vec![cheese_tag_id, dairy_tag_id])
            .await
            .unwrap();

        // Get with tags
        let with_tags = MealOptionRepository::get_with_tags(&pool, created.id)
            .await
            .unwrap();
        assert!(with_tags.is_some());
        let with_tags = with_tags.unwrap();

        assert_eq!(with_tags.tags.len(), 2);
        assert!(with_tags.tags.contains(&cheese_tag_id));
        assert!(with_tags.tags.contains(&dairy_tag_id));
    }

    #[tokio::test]
    async fn test_manage_tags() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        // Create tags (name must be lowercase with underscores)
        let tag1_id = create_test_tag(&pool, "tag_one", TagCategory::Ingredient).await;
        let tag2_id = create_test_tag(&pool, "tag_two", TagCategory::Ingredient).await;
        let tag3_id = create_test_tag(&pool, "tag_three", TagCategory::Ingredient).await;

        // Create option
        let option = CreateMealOption {
            template_id,
            name: "Test Option".to_string(),
            description: None,
            nutritional_notes: None,
        };
        let created = MealOptionRepository::create(&pool, option).await.unwrap();

        // Add initial tags
        MealOptionRepository::add_tags(&pool, created.id, vec![tag1_id, tag2_id])
            .await
            .unwrap();
        let with_tags = MealOptionRepository::get_with_tags(&pool, created.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(with_tags.tags.len(), 2);

        // Remove one tag
        MealOptionRepository::remove_tags(&pool, created.id, vec![tag1_id])
            .await
            .unwrap();
        let with_tags = MealOptionRepository::get_with_tags(&pool, created.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(with_tags.tags.len(), 1);
        assert_eq!(with_tags.tags[0], tag2_id);

        // Replace all tags
        MealOptionRepository::set_tags(&pool, created.id, vec![tag1_id, tag3_id])
            .await
            .unwrap();
        let with_tags = MealOptionRepository::get_with_tags(&pool, created.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(with_tags.tags.len(), 2);
        assert!(with_tags.tags.contains(&tag1_id));
        assert!(with_tags.tags.contains(&tag3_id));
        assert!(!with_tags.tags.contains(&tag2_id));
    }

    #[tokio::test]
    async fn test_search_options() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        // Create options with different names
        let options = vec![
            ("Ricotta cheese", "Fresh ricotta"),
            ("Philadelphia cream", "Cream cheese"),
            ("Mozzarella", "Fresh mozzarella"),
        ];

        for (name, desc) in options {
            let option = CreateMealOption {
                template_id,
                name: name.to_string(),
                description: Some(desc.to_string()),
                nutritional_notes: None,
            };
            MealOptionRepository::create(&pool, option).await.unwrap();
        }

        // Search by name
        let results = MealOptionRepository::search(&pool, "cream").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Philadelphia cream");

        // Search by description
        let results = MealOptionRepository::search(&pool, "Fresh").await.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_update_option() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        let option = CreateMealOption {
            template_id,
            name: "Original".to_string(),
            description: Some("Original desc".to_string()),
            nutritional_notes: Some("Original notes".to_string()),
        };
        let created = MealOptionRepository::create(&pool, option).await.unwrap();

        // Update name and description
        let update = UpdateMealOption {
            name: Some("Updated".to_string()),
            description: None,
            nutritional_notes: None,
        };
        let updated = MealOptionRepository::update(&pool, created.id, update)
            .await
            .unwrap();

        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.description, Some("Original desc".to_string()));
        assert_eq!(
            updated.nutritional_notes,
            Some("Original notes".to_string())
        );

        // Clear optional fields
        let update = UpdateMealOption {
            name: None,
            description: Some(None),
            nutritional_notes: Some(None),
        };
        let updated = MealOptionRepository::update(&pool, created.id, update)
            .await
            .unwrap();

        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.description, None);
        assert_eq!(updated.nutritional_notes, None);
    }

    #[tokio::test]
    async fn test_delete_option() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        let option = CreateMealOption {
            template_id,
            name: "To Delete".to_string(),
            description: None,
            nutritional_notes: None,
        };
        let created = MealOptionRepository::create(&pool, option).await.unwrap();

        // Delete the option
        MealOptionRepository::delete(&pool, created.id)
            .await
            .unwrap();

        // Verify it's gone
        let retrieved = MealOptionRepository::get_by_id(&pool, created.id)
            .await
            .unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_validation_error() {
        let (pool, _temp_dir) = setup_test_db().await;
        let template_id = create_test_template(&pool).await;

        // Empty name should fail
        let option = CreateMealOption {
            template_id,
            name: "".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let result = MealOptionRepository::create(&pool, option).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_template_id() {
        let (pool, _temp_dir) = setup_test_db().await;

        let option = CreateMealOption {
            template_id: 99999, // Non-existent template
            name: "Test".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let result = MealOptionRepository::create(&pool, option).await;
        assert!(result.is_err());
    }
}
