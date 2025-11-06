// Tag-related Tauri commands
// Command handlers for tag CRUD operations

use crate::error::ApiResult;
use crate::models::{CreateTag, Tag, TagCategory, UpdateTag};
use crate::repository::TagRepository;
use sqlx::SqlitePool;
use tauri::State;

/// Get all tags
#[tauri::command]
pub async fn get_all_tags(pool: State<'_, SqlitePool>) -> ApiResult<Vec<Tag>> {
    TagRepository::get_all(pool.inner())
        .await
        .map_err(Into::into)
}

/// Get a tag by ID
#[tauri::command]
pub async fn get_tag_by_id(id: i64, pool: State<'_, SqlitePool>) -> ApiResult<Option<Tag>> {
    TagRepository::get_by_id(pool.inner(), id)
        .await
        .map_err(Into::into)
}

/// Get a tag by name
#[tauri::command]
pub async fn get_tag_by_name(name: String, pool: State<'_, SqlitePool>) -> ApiResult<Option<Tag>> {
    TagRepository::get_by_name(pool.inner(), &name)
        .await
        .map_err(Into::into)
}

/// Get all tags by category
#[tauri::command]
pub async fn get_tags_by_category(
    category: TagCategory,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<Tag>> {
    TagRepository::get_by_category(pool.inner(), category)
        .await
        .map_err(Into::into)
}

/// Get child tags of a parent tag
#[tauri::command]
pub async fn get_tag_children(parent_id: i64, pool: State<'_, SqlitePool>) -> ApiResult<Vec<Tag>> {
    TagRepository::get_children(pool.inner(), parent_id)
        .await
        .map_err(Into::into)
}

/// Create a new tag
#[tauri::command]
pub async fn create_tag(tag: CreateTag, pool: State<'_, SqlitePool>) -> ApiResult<Tag> {
    TagRepository::create(pool.inner(), tag)
        .await
        .map_err(Into::into)
}

/// Update an existing tag
#[tauri::command]
pub async fn update_tag(
    id: i64,
    updates: UpdateTag,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Tag> {
    TagRepository::update(pool.inner(), id, updates)
        .await
        .map_err(Into::into)
}

/// Delete a tag
#[tauri::command]
pub async fn delete_tag(id: i64, pool: State<'_, SqlitePool>) -> ApiResult<bool> {
    TagRepository::delete(pool.inner(), id)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::TagRepository;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn test_create_and_get_tag_commands() {
        let pool = setup_test_pool().await;

        // Create a tag directly using repository
        let create_tag = CreateTag {
            name: "pasta".to_string(),
            display_name: "Pasta".to_string(),
            category: TagCategory::Ingredient,
            weekly_suggestion: Some(3),
            parent_tag_id: None,
        };

        let created = TagRepository::create(&pool, create_tag).await.unwrap();
        assert_eq!(created.name, "pasta");
        assert_eq!(created.display_name, "Pasta");

        // Test get by ID
        let fetched = TagRepository::get_by_id(&pool, created.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().name, "pasta");

        // Test get by name
        let by_name = TagRepository::get_by_name(&pool, "pasta").await.unwrap();
        assert!(by_name.is_some());
        assert_eq!(by_name.unwrap().id, created.id);
    }

    #[tokio::test]
    async fn test_get_all_tags_command() {
        let pool = setup_test_pool().await;

        // Create multiple tags
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

        let all_tags = TagRepository::get_all(&pool).await.unwrap();
        assert_eq!(all_tags.len(), 2);
    }

    #[tokio::test]
    async fn test_get_tags_by_category_command() {
        let pool = setup_test_pool().await;

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
    async fn test_tag_hierarchy_command() {
        let pool = setup_test_pool().await;

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
    async fn test_update_tag_command() {
        let pool = setup_test_pool().await;

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
    async fn test_delete_tag_command() {
        let pool = setup_test_pool().await;

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
    async fn test_duplicate_tag_name_error_command() {
        let pool = setup_test_pool().await;

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

        // Try to create with same name
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
