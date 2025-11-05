use crate::models::{CreateTag, Tag, TagCategory, UpdateTag};
use chrono::{DateTime, Utc};
use sqlx::{Result, Row, SqlitePool};

pub struct TagRepository;

impl TagRepository {
    /// Helper to map a row to a Tag
    fn row_to_tag(row: &sqlx::sqlite::SqliteRow) -> Result<Tag> {
        let category_str: String = row.try_get("category")?;
        let category = TagCategory::from_db_string(&category_str).map_err(|e| {
            sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e,
            )))
        })?;

        Ok(Tag {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            display_name: row.try_get("display_name")?,
            category,
            weekly_suggestion: row.try_get("weekly_suggestion")?,
            parent_tag_id: row.try_get("parent_tag_id")?,
            created_at: row.try_get("created_at")?,
        })
    }

    /// Create a new tag
    pub async fn create(pool: &SqlitePool, tag: CreateTag) -> Result<Tag> {
        tag.validate().map_err(sqlx::Error::Protocol)?;

        let category_str = tag.category.to_db_string();

        let row = sqlx::query(
            r#"
            INSERT INTO tags (name, display_name, category, weekly_suggestion, parent_tag_id)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at
            "#,
        )
        .bind(&tag.name)
        .bind(&tag.display_name)
        .bind(category_str)
        .bind(tag.weekly_suggestion)
        .bind(tag.parent_tag_id)
        .fetch_one(pool)
        .await?;

        Self::row_to_tag(&row)
    }

    /// Get a tag by ID
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Tag>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at
            FROM tags
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_tag(&r)?)),
            None => Ok(None),
        }
    }

    /// Get a tag by name
    pub async fn get_by_name(pool: &SqlitePool, name: &str) -> Result<Option<Tag>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at
            FROM tags
            WHERE name = ?1
            "#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_tag(&r)?)),
            None => Ok(None),
        }
    }

    /// Get all tags
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Tag>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at
            FROM tags
            ORDER BY name
            "#,
        )
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_tag).collect()
    }

    /// Get all tags by category
    pub async fn get_by_category(pool: &SqlitePool, category: TagCategory) -> Result<Vec<Tag>> {
        let category_str = category.to_db_string();

        let rows = sqlx::query(
            r#"
            SELECT id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at
            FROM tags
            WHERE category = ?1
            ORDER BY name
            "#,
        )
        .bind(category_str)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_tag).collect()
    }

    /// Get child tags of a parent tag
    pub async fn get_children(pool: &SqlitePool, parent_id: i64) -> Result<Vec<Tag>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at
            FROM tags
            WHERE parent_tag_id = ?1
            ORDER BY name
            "#,
        )
        .bind(parent_id)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_tag).collect()
    }

    /// Update a tag
    pub async fn update(pool: &SqlitePool, id: i64, update: UpdateTag) -> Result<Tag> {
        // Get existing tag first
        let existing = Self::get_by_id(pool, id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        // Apply updates
        let display_name = update.display_name.unwrap_or(existing.display_name);
        let category = update.category.unwrap_or(existing.category);
        let category_str = category.to_db_string();

        // Handle Option<Option<T>> for nullable fields
        let weekly_suggestion = match update.weekly_suggestion {
            Some(val) => val,                   // Use the new value (which might be None)
            None => existing.weekly_suggestion, // Keep existing
        };

        let parent_tag_id = match update.parent_tag_id {
            Some(val) => val,
            None => existing.parent_tag_id,
        };

        let row = sqlx::query(
            r#"
            UPDATE tags
            SET display_name = ?1, category = ?2, weekly_suggestion = ?3, parent_tag_id = ?4
            WHERE id = ?5
            RETURNING id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at
            "#,
        )
        .bind(&display_name)
        .bind(category_str)
        .bind(weekly_suggestion)
        .bind(parent_tag_id)
        .bind(id)
        .fetch_one(pool)
        .await?;

        Self::row_to_tag(&row)
    }

    /// Delete a tag
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM tags WHERE id = ?1")
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
    async fn test_create_tag() {
        let pool = setup_test_db().await;

        let create_tag = CreateTag {
            name: "pasta".to_string(),
            display_name: "Pasta".to_string(),
            category: TagCategory::Ingredient,
            weekly_suggestion: Some(3),
            parent_tag_id: None,
        };

        let tag = TagRepository::create(&pool, create_tag).await.unwrap();

        assert_eq!(tag.name, "pasta");
        assert_eq!(tag.display_name, "Pasta");
        assert_eq!(tag.category, TagCategory::Ingredient);
        assert_eq!(tag.weekly_suggestion, Some(3));
        assert!(tag.id > 0);
    }

    #[tokio::test]
    async fn test_get_tag_by_id() {
        let pool = setup_test_db().await;

        let created = TagRepository::create(
            &pool,
            CreateTag {
                name: "ricotta".to_string(),
                display_name: "Ricotta".to_string(),
                category: TagCategory::Ingredient,
                weekly_suggestion: Some(2),
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        let fetched = TagRepository::get_by_id(&pool, created.id).await.unwrap();

        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().name, "ricotta");
    }

    #[tokio::test]
    async fn test_get_tag_by_name() {
        let pool = setup_test_db().await;

        TagRepository::create(
            &pool,
            CreateTag {
                name: "philadelphia".to_string(),
                display_name: "Philadelphia".to_string(),
                category: TagCategory::Ingredient,
                weekly_suggestion: None,
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        let fetched = TagRepository::get_by_name(&pool, "philadelphia")
            .await
            .unwrap();

        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().display_name, "Philadelphia");
    }

    #[tokio::test]
    async fn test_tag_hierarchy() {
        let pool = setup_test_db().await;

        // Create parent tag
        let parent = TagRepository::create(
            &pool,
            CreateTag {
                name: "pasta".to_string(),
                display_name: "Pasta".to_string(),
                category: TagCategory::Ingredient,
                weekly_suggestion: Some(3),
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        // Create child tag
        let child = TagRepository::create(
            &pool,
            CreateTag {
                name: "pasta_integrale".to_string(),
                display_name: "Pasta Integrale".to_string(),
                category: TagCategory::Ingredient,
                weekly_suggestion: Some(2),
                parent_tag_id: Some(parent.id),
            },
        )
        .await
        .unwrap();

        assert_eq!(child.parent_tag_id, Some(parent.id));

        // Get children
        let children = TagRepository::get_children(&pool, parent.id).await.unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "pasta_integrale");
    }

    #[tokio::test]
    async fn test_update_tag() {
        let pool = setup_test_db().await;

        let created = TagRepository::create(
            &pool,
            CreateTag {
                name: "test_tag".to_string(),
                display_name: "Test Tag".to_string(),
                category: TagCategory::Other,
                weekly_suggestion: Some(5),
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        let updated = TagRepository::update(
            &pool,
            created.id,
            UpdateTag {
                display_name: Some("Updated Tag".to_string()),
                category: Some(TagCategory::Dietary),
                weekly_suggestion: Some(Some(10)),
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.display_name, "Updated Tag");
        assert_eq!(updated.category, TagCategory::Dietary);
        assert_eq!(updated.weekly_suggestion, Some(10));
    }

    #[tokio::test]
    async fn test_delete_tag() {
        let pool = setup_test_db().await;

        let created = TagRepository::create(
            &pool,
            CreateTag {
                name: "to_delete".to_string(),
                display_name: "To Delete".to_string(),
                category: TagCategory::Other,
                weekly_suggestion: None,
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        let deleted = TagRepository::delete(&pool, created.id).await.unwrap();
        assert!(deleted);

        let fetched = TagRepository::get_by_id(&pool, created.id).await.unwrap();
        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn test_get_by_category() {
        let pool = setup_test_db().await;

        // Create tags in different categories
        TagRepository::create(
            &pool,
            CreateTag {
                name: "pasta".to_string(),
                display_name: "Pasta".to_string(),
                category: TagCategory::Ingredient,
                weekly_suggestion: None,
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        TagRepository::create(
            &pool,
            CreateTag {
                name: "vegetarian".to_string(),
                display_name: "Vegetarian".to_string(),
                category: TagCategory::Dietary,
                weekly_suggestion: None,
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        let ingredients = TagRepository::get_by_category(&pool, TagCategory::Ingredient)
            .await
            .unwrap();
        assert_eq!(ingredients.len(), 1);
        assert_eq!(ingredients[0].name, "pasta");

        let dietary = TagRepository::get_by_category(&pool, TagCategory::Dietary)
            .await
            .unwrap();
        assert_eq!(dietary.len(), 1);
        assert_eq!(dietary[0].name, "vegetarian");
    }

    #[tokio::test]
    async fn test_unique_tag_name() {
        let pool = setup_test_db().await;

        TagRepository::create(
            &pool,
            CreateTag {
                name: "duplicate".to_string(),
                display_name: "First".to_string(),
                category: TagCategory::Other,
                weekly_suggestion: None,
                parent_tag_id: None,
            },
        )
        .await
        .unwrap();

        // Try to create with same name - should fail
        let result = TagRepository::create(
            &pool,
            CreateTag {
                name: "duplicate".to_string(),
                display_name: "Second".to_string(),
                category: TagCategory::Other,
                weekly_suggestion: None,
                parent_tag_id: None,
            },
        )
        .await;

        assert!(result.is_err());
    }
}
