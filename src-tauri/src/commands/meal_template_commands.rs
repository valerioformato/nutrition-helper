// MealTemplate-related Tauri commands
// Command handlers for meal template CRUD operations

use crate::error::ApiResult;
use crate::models::{CreateMealTemplate, LocationType, MealTemplate, SlotType, UpdateMealTemplate};
use crate::repository::MealTemplateRepository;
use sqlx::SqlitePool;
use tauri::State;

/// Get all meal templates
#[tauri::command]
pub async fn get_all_templates(pool: State<'_, SqlitePool>) -> ApiResult<Vec<MealTemplate>> {
    MealTemplateRepository::get_all(pool.inner())
        .await
        .map_err(Into::into)
}

/// Get a meal template by ID
#[tauri::command]
pub async fn get_template_by_id(
    id: i64,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Option<MealTemplate>> {
    MealTemplateRepository::get_by_id(pool.inner(), id)
        .await
        .map_err(Into::into)
}

/// Get meal templates by location type
#[tauri::command]
pub async fn get_templates_by_location(
    location: LocationType,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealTemplate>> {
    MealTemplateRepository::get_by_location(pool.inner(), location)
        .await
        .map_err(Into::into)
}

/// Get meal templates by slot type
#[tauri::command]
pub async fn get_templates_by_slot(
    slot: SlotType,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealTemplate>> {
    MealTemplateRepository::get_by_slot(pool.inner(), slot)
        .await
        .map_err(Into::into)
}

/// Search meal templates by name
#[tauri::command]
pub async fn search_templates(
    query: String,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealTemplate>> {
    MealTemplateRepository::search(pool.inner(), &query)
        .await
        .map_err(Into::into)
}

/// Create a new meal template
#[tauri::command]
pub async fn create_template(
    template: CreateMealTemplate,
    pool: State<'_, SqlitePool>,
) -> ApiResult<MealTemplate> {
    MealTemplateRepository::create(pool.inner(), template)
        .await
        .map_err(Into::into)
}

/// Update an existing meal template
#[tauri::command]
pub async fn update_template(
    id: i64,
    updates: UpdateMealTemplate,
    pool: State<'_, SqlitePool>,
) -> ApiResult<MealTemplate> {
    MealTemplateRepository::update(pool.inner(), id, updates)
        .await
        .map_err(Into::into)
}

/// Delete a meal template
#[tauri::command]
pub async fn delete_template(id: i64, pool: State<'_, SqlitePool>) -> ApiResult<bool> {
    MealTemplateRepository::delete(pool.inner(), id)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::MealTemplateRepository;
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
    async fn test_create_and_get_template_commands() {
        let pool = setup_test_pool().await;

        // Create a template
        let create_template = CreateMealTemplate {
            name: "Pane con marmellata".to_string(),
            description: Some("Bread with jam".to_string()),
            compatible_slots: vec![SlotType::Breakfast, SlotType::MorningSnack],
            location_type: LocationType::Home,
            weekly_limit: None,
        };

        let created = MealTemplateRepository::create(&pool, create_template)
            .await
            .unwrap();

        assert_eq!(created.name, "Pane con marmellata");
        assert_eq!(created.compatible_slots.len(), 2);
        assert_eq!(created.location_type, LocationType::Home);

        // Get by ID
        let fetched = MealTemplateRepository::get_by_id(&pool, created.id)
            .await
            .unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().name, "Pane con marmellata");
    }

    #[tokio::test]
    async fn test_get_all_templates_command() {
        let pool = setup_test_pool().await;

        // Create multiple templates
        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Yogurt".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Breakfast],
                location_type: LocationType::Any,
                weekly_limit: None,
            },
        )
        .await
        .unwrap();

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Pasta".to_string(),
                description: Some("Pasta dish".to_string()),
                compatible_slots: vec![SlotType::Lunch, SlotType::Dinner],
                location_type: LocationType::Home,
                weekly_limit: None,
            },
        )
        .await
        .unwrap();

        let all_templates = MealTemplateRepository::get_all(&pool).await.unwrap();
        assert_eq!(all_templates.len(), 2);
    }

    #[tokio::test]
    async fn test_get_templates_by_location_command() {
        let pool = setup_test_pool().await;

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Home Meal".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Home,
                weekly_limit: None,
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
                weekly_limit: None,
            },
        )
        .await
        .unwrap();

        let home_templates = MealTemplateRepository::get_by_location(&pool, LocationType::Home)
            .await
            .unwrap();
        assert_eq!(home_templates.len(), 1);
        assert_eq!(home_templates[0].name, "Home Meal");
    }

    #[tokio::test]
    async fn test_get_templates_by_slot_command() {
        let pool = setup_test_pool().await;

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Breakfast Only".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Breakfast],
                location_type: LocationType::Home,
                weekly_limit: None,
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
                weekly_limit: None,
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
    async fn test_search_templates_command() {
        let pool = setup_test_pool().await;

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Pasta carbonara".to_string(),
                description: Some("Classic pasta dish".to_string()),
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Home,
                weekly_limit: None,
            },
        )
        .await
        .unwrap();

        MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Chicken salad".to_string(),
                description: Some("Fresh salad".to_string()),
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Office,
                weekly_limit: None,
            },
        )
        .await
        .unwrap();

        let pasta_results = MealTemplateRepository::search(&pool, "pasta")
            .await
            .unwrap();
        assert_eq!(pasta_results.len(), 1);
        assert_eq!(pasta_results[0].name, "Pasta carbonara");

        let carbonara_results = MealTemplateRepository::search(&pool, "carbo")
            .await
            .unwrap();
        assert_eq!(carbonara_results.len(), 1);
    }

    #[tokio::test]
    async fn test_update_template_command() {
        let pool = setup_test_pool().await;

        let created = MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Original Name".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Home,
                weekly_limit: None,
            },
        )
        .await
        .unwrap();

        let updated = MealTemplateRepository::update(
            &pool,
            created.id,
            UpdateMealTemplate {
                name: Some("Updated Name".to_string()),
                description: Some(Some("New description".to_string())),
                compatible_slots: Some(vec![SlotType::Lunch, SlotType::Dinner]),
                location_type: Some(LocationType::Office),
                weekly_limit: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.description, Some("New description".to_string()));
        assert_eq!(updated.compatible_slots.len(), 2);
        assert_eq!(updated.location_type, LocationType::Office);
    }

    #[tokio::test]
    async fn test_delete_template_command() {
        let pool = setup_test_pool().await;

        let created = MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "To Delete".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Home,
                weekly_limit: None,
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
    async fn test_validation_error_command() {
        let pool = setup_test_pool().await;

        // Try to create template with empty name
        let result = MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "".to_string(),
                description: None,
                compatible_slots: vec![SlotType::Lunch],
                location_type: LocationType::Home,
                weekly_limit: None,
            },
        )
        .await;

        assert!(result.is_err());

        // Try with empty compatible_slots
        let result = MealTemplateRepository::create(
            &pool,
            CreateMealTemplate {
                name: "Test".to_string(),
                description: None,
                compatible_slots: vec![],
                location_type: LocationType::Home,
                weekly_limit: None,
            },
        )
        .await;

        assert!(result.is_err());
    }
}
