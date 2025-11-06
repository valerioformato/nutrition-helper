// MealOption-related Tauri commands
// Command handlers for meal option CRUD operations and tag management

use crate::error::ApiResult;
use crate::models::{CreateMealOption, MealOption, MealOptionWithTags, UpdateMealOption};
use crate::repository::MealOptionRepository;
use sqlx::SqlitePool;
use tauri::State;

/// Get all meal options
#[tauri::command]
pub async fn get_all_options(pool: State<'_, SqlitePool>) -> ApiResult<Vec<MealOption>> {
    MealOptionRepository::get_all(pool.inner())
        .await
        .map_err(Into::into)
}

/// Get a meal option by ID
#[tauri::command]
pub async fn get_option_by_id(
    id: i64,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Option<MealOption>> {
    MealOptionRepository::get_by_id(pool.inner(), id)
        .await
        .map_err(Into::into)
}

/// Get a meal option with its tags by ID
#[tauri::command]
pub async fn get_option_with_tags(
    id: i64,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Option<MealOptionWithTags>> {
    MealOptionRepository::get_with_tags(pool.inner(), id)
        .await
        .map_err(Into::into)
}

/// Get all meal options for a template
#[tauri::command]
pub async fn get_options_by_template(
    template_id: i64,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealOption>> {
    MealOptionRepository::get_by_template_id(pool.inner(), template_id)
        .await
        .map_err(Into::into)
}

/// Get all meal options for a template with tags
#[tauri::command]
pub async fn get_options_by_template_with_tags(
    template_id: i64,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealOptionWithTags>> {
    MealOptionRepository::get_by_template_with_tags(pool.inner(), template_id)
        .await
        .map_err(Into::into)
}

/// Search meal options by name
#[tauri::command]
pub async fn search_options(
    query: String,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealOption>> {
    MealOptionRepository::search(pool.inner(), &query)
        .await
        .map_err(Into::into)
}

/// Create a new meal option
#[tauri::command]
pub async fn create_option(
    option: CreateMealOption,
    pool: State<'_, SqlitePool>,
) -> ApiResult<MealOption> {
    MealOptionRepository::create(pool.inner(), option)
        .await
        .map_err(Into::into)
}

/// Update an existing meal option
#[tauri::command]
pub async fn update_option(
    id: i64,
    updates: UpdateMealOption,
    pool: State<'_, SqlitePool>,
) -> ApiResult<MealOption> {
    MealOptionRepository::update(pool.inner(), id, updates)
        .await
        .map_err(Into::into)
}

/// Delete a meal option
#[tauri::command]
pub async fn delete_option(id: i64, pool: State<'_, SqlitePool>) -> ApiResult<()> {
    MealOptionRepository::delete(pool.inner(), id)
        .await
        .map_err(Into::into)
}

/// Add tags to a meal option
#[tauri::command]
pub async fn add_tags_to_option(
    option_id: i64,
    tag_ids: Vec<i64>,
    pool: State<'_, SqlitePool>,
) -> ApiResult<()> {
    MealOptionRepository::add_tags(pool.inner(), option_id, tag_ids)
        .await
        .map_err(Into::into)
}

/// Remove tags from a meal option
#[tauri::command]
pub async fn remove_tags_from_option(
    option_id: i64,
    tag_ids: Vec<i64>,
    pool: State<'_, SqlitePool>,
) -> ApiResult<()> {
    MealOptionRepository::remove_tags(pool.inner(), option_id, tag_ids)
        .await
        .map_err(Into::into)
}

/// Set all tags for a meal option (replaces existing tags)
#[tauri::command]
pub async fn set_option_tags(
    option_id: i64,
    tag_ids: Vec<i64>,
    pool: State<'_, SqlitePool>,
) -> ApiResult<()> {
    MealOptionRepository::set_tags(pool.inner(), option_id, tag_ids)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CreateMealTemplate, CreateTag, LocationType, SlotType, TagCategory};
    use crate::repository::{MealTemplateRepository, TagRepository};
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test pool");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    async fn create_test_template(pool: &SqlitePool) -> i64 {
        let template = CreateMealTemplate {
            name: "Test Template".to_string(),
            description: Some("Test".to_string()),
            location_type: LocationType::Home,
            compatible_slots: vec![SlotType::Breakfast],
            weekly_limit: None,
        };

        MealTemplateRepository::create(pool, template)
            .await
            .expect("Failed to create template")
            .id
    }

    async fn create_test_tag(pool: &SqlitePool, name: &str) -> i64 {
        let tag = CreateTag {
            name: name.to_string(),
            display_name: name.to_string(),
            category: TagCategory::Ingredient,
            parent_tag_id: None,
            weekly_suggestion: None,
        };

        TagRepository::create(pool, tag)
            .await
            .expect("Failed to create tag")
            .id
    }

    #[tokio::test]
    async fn test_create_and_get_option() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template(&pool).await;

        let option = CreateMealOption {
            template_id,
            name: "Test Option".to_string(),
            description: Some("A test option".to_string()),
            nutritional_notes: Some("Test notes".to_string()),
        };

        let created = MealOptionRepository::create(&pool, option)
            .await
            .expect("Failed to create option");

        assert_eq!(created.name, "Test Option");
        assert_eq!(created.template_id, template_id);

        let fetched = MealOptionRepository::get_by_id(&pool, created.id)
            .await
            .expect("Failed to get option")
            .expect("Option not found");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, created.name);
    }

    #[tokio::test]
    async fn test_get_all_options() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template(&pool).await;

        let option1 = CreateMealOption {
            template_id,
            name: "Option 1".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let option2 = CreateMealOption {
            template_id,
            name: "Option 2".to_string(),
            description: None,
            nutritional_notes: None,
        };

        MealOptionRepository::create(&pool, option1)
            .await
            .expect("Failed to create option 1");
        MealOptionRepository::create(&pool, option2)
            .await
            .expect("Failed to create option 2");

        let all_options = MealOptionRepository::get_all(&pool)
            .await
            .expect("Failed to get all options");

        assert_eq!(all_options.len(), 2);
    }

    #[tokio::test]
    async fn test_get_options_by_template() {
        let pool = setup_test_pool().await;
        let template_id1 = create_test_template(&pool).await;

        // Create second template
        let template2 = CreateMealTemplate {
            name: "Template 2".to_string(),
            description: None,
            location_type: LocationType::Home,
            compatible_slots: vec![SlotType::Lunch],
            weekly_limit: None,
        };
        let template_id2 = MealTemplateRepository::create(&pool, template2)
            .await
            .expect("Failed to create template 2")
            .id;

        // Create options for both templates
        let option1 = CreateMealOption {
            template_id: template_id1,
            name: "Template 1 Option".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let option2 = CreateMealOption {
            template_id: template_id2,
            name: "Template 2 Option".to_string(),
            description: None,
            nutritional_notes: None,
        };

        MealOptionRepository::create(&pool, option1)
            .await
            .expect("Failed to create option 1");
        MealOptionRepository::create(&pool, option2)
            .await
            .expect("Failed to create option 2");

        let template1_options = MealOptionRepository::get_by_template_id(&pool, template_id1)
            .await
            .expect("Failed to get options for template 1");

        assert_eq!(template1_options.len(), 1);
        assert_eq!(template1_options[0].name, "Template 1 Option");
    }

    #[tokio::test]
    async fn test_search_options() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template(&pool).await;

        let option1 = CreateMealOption {
            template_id,
            name: "Philadelphia Cheese".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let option2 = CreateMealOption {
            template_id,
            name: "Ricotta Cheese".to_string(),
            description: None,
            nutritional_notes: None,
        };

        MealOptionRepository::create(&pool, option1)
            .await
            .expect("Failed to create option 1");
        MealOptionRepository::create(&pool, option2)
            .await
            .expect("Failed to create option 2");

        let results = MealOptionRepository::search(&pool, "cheese")
            .await
            .expect("Failed to search options");

        assert_eq!(results.len(), 2);

        let philly_results = MealOptionRepository::search(&pool, "philadelphia")
            .await
            .expect("Failed to search for philadelphia");

        assert_eq!(philly_results.len(), 1);
        assert_eq!(philly_results[0].name, "Philadelphia Cheese");
    }

    #[tokio::test]
    async fn test_update_option() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template(&pool).await;

        let option = CreateMealOption {
            template_id,
            name: "Original Name".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let created = MealOptionRepository::create(&pool, option)
            .await
            .expect("Failed to create option");

        let updates = UpdateMealOption {
            name: Some("Updated Name".to_string()),
            description: Some(Some("New description".to_string())),
            nutritional_notes: Some(Some("New notes".to_string())),
        };

        let updated = MealOptionRepository::update(&pool, created.id, updates)
            .await
            .expect("Failed to update option");

        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.description, Some("New description".to_string()));
        assert_eq!(updated.nutritional_notes, Some("New notes".to_string()));
    }

    #[tokio::test]
    async fn test_delete_option() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template(&pool).await;

        let option = CreateMealOption {
            template_id,
            name: "To Delete".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let created = MealOptionRepository::create(&pool, option)
            .await
            .expect("Failed to create option");

        MealOptionRepository::delete(&pool, created.id)
            .await
            .expect("Failed to delete option");

        let fetched = MealOptionRepository::get_by_id(&pool, created.id)
            .await
            .expect("Failed to query option");

        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn test_tag_management() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template(&pool).await;
        let tag1_id = create_test_tag(&pool, "pasta").await;
        let tag2_id = create_test_tag(&pool, "integrale").await;

        let option = CreateMealOption {
            template_id,
            name: "Pasta Integrale".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let created = MealOptionRepository::create(&pool, option)
            .await
            .expect("Failed to create option");

        // Add tags
        MealOptionRepository::add_tags(&pool, created.id, vec![tag1_id, tag2_id])
            .await
            .expect("Failed to add tags");

        // Get option with tags
        let with_tags = MealOptionRepository::get_with_tags(&pool, created.id)
            .await
            .expect("Failed to get option with tags")
            .expect("Option not found");

        assert_eq!(with_tags.tags.len(), 2);
        assert!(with_tags.tags.contains(&tag1_id));
        assert!(with_tags.tags.contains(&tag2_id));

        // Remove one tag
        MealOptionRepository::remove_tags(&pool, created.id, vec![tag1_id])
            .await
            .expect("Failed to remove tag");

        let with_tags = MealOptionRepository::get_with_tags(&pool, created.id)
            .await
            .expect("Failed to get option with tags")
            .expect("Option not found");

        assert_eq!(with_tags.tags.len(), 1);
        assert!(with_tags.tags.contains(&tag2_id));

        // Set tags (replaces all)
        MealOptionRepository::set_tags(&pool, created.id, vec![tag1_id])
            .await
            .expect("Failed to set tags");

        let with_tags = MealOptionRepository::get_with_tags(&pool, created.id)
            .await
            .expect("Failed to get option with tags")
            .expect("Option not found");

        assert_eq!(with_tags.tags.len(), 1);
        assert!(with_tags.tags.contains(&tag1_id));
    }

    #[tokio::test]
    async fn test_get_options_by_template_with_tags() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template(&pool).await;
        let tag_id = create_test_tag(&pool, "cheese").await;

        let option1 = CreateMealOption {
            template_id,
            name: "Philadelphia".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let option2 = CreateMealOption {
            template_id,
            name: "Ricotta".to_string(),
            description: None,
            nutritional_notes: None,
        };

        let created1 = MealOptionRepository::create(&pool, option1)
            .await
            .expect("Failed to create option 1");

        let created2 = MealOptionRepository::create(&pool, option2)
            .await
            .expect("Failed to create option 2");

        // Add tag to first option
        MealOptionRepository::add_tags(&pool, created1.id, vec![tag_id])
            .await
            .expect("Failed to add tag");

        // Get all options with tags
        let options_with_tags = MealOptionRepository::get_by_template_with_tags(&pool, template_id)
            .await
            .expect("Failed to get options with tags");

        assert_eq!(options_with_tags.len(), 2);

        // Find the option with tags
        let option_with_tag = options_with_tags
            .iter()
            .find(|o| o.tags.len() > 0)
            .expect("Should find option with tag");

        assert_eq!(option_with_tag.option.id, created1.id);
        assert_eq!(option_with_tag.tags.len(), 1);
        assert_eq!(option_with_tag.tags[0], tag_id);
    }
}
