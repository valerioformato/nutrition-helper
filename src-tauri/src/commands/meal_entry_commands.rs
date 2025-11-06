// MealEntry-related Tauri commands
// Command handlers for meal entry CRUD operations and weekly usage tracking

use crate::error::ApiResult;
use crate::models::{
    CreateMealEntry, MealEntry, SlotType, UpdateMealEntry, WeeklyTagUsage, WeeklyUsage,
};
use crate::repository::MealEntryRepository;
use chrono::NaiveDate;
use sqlx::SqlitePool;
use tauri::State;

/// Get a meal entry by ID
#[tauri::command]
pub async fn get_entry_by_id(id: i64, pool: State<'_, SqlitePool>) -> ApiResult<Option<MealEntry>> {
    MealEntryRepository::get_by_id(pool.inner(), id)
        .await
        .map_err(Into::into)
}

/// Get all meal entries for a specific date
#[tauri::command]
pub async fn get_entries_by_date(
    date: String, // Format: "YYYY-MM-DD"
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealEntry>> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| {
        crate::error::ApiError::ValidationError(format!("Invalid date format: {}", e))
    })?;

    MealEntryRepository::get_by_date(pool.inner(), date)
        .await
        .map_err(Into::into)
}

/// Get all meal entries in a date range
#[tauri::command]
pub async fn get_entries_by_date_range(
    start_date: String, // Format: "YYYY-MM-DD"
    end_date: String,   // Format: "YYYY-MM-DD"
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealEntry>> {
    let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").map_err(|e| {
        crate::error::ApiError::ValidationError(format!("Invalid start date: {}", e))
    })?;
    let end = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
        .map_err(|e| crate::error::ApiError::ValidationError(format!("Invalid end date: {}", e)))?;

    MealEntryRepository::get_by_date_range(pool.inner(), start, end)
        .await
        .map_err(Into::into)
}

/// Get a specific meal entry by date and slot
#[tauri::command]
pub async fn get_entry_by_date_and_slot(
    date: String, // Format: "YYYY-MM-DD"
    slot: SlotType,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealEntry>> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| {
        crate::error::ApiError::ValidationError(format!("Invalid date format: {}", e))
    })?;

    MealEntryRepository::get_by_date_and_slot(pool.inner(), date, slot)
        .await
        .map_err(Into::into)
}

/// Get all entries by completion status
#[tauri::command]
pub async fn get_entries_by_completed(
    completed: bool,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealEntry>> {
    MealEntryRepository::get_by_completed(pool.inner(), completed)
        .await
        .map_err(Into::into)
}

/// Get all entries for a specific meal option
#[tauri::command]
pub async fn get_entries_by_meal_option(
    meal_option_id: i64,
    pool: State<'_, SqlitePool>,
) -> ApiResult<Vec<MealEntry>> {
    MealEntryRepository::get_by_meal_option(pool.inner(), meal_option_id)
        .await
        .map_err(Into::into)
}

/// Get weekly usage count for a specific meal option
#[tauri::command]
pub async fn get_weekly_usage(
    meal_option_id: i64,
    week: String, // Format: "YYYY-WW" - ISO week format
    pool: State<'_, SqlitePool>,
) -> ApiResult<Option<WeeklyUsage>> {
    MealEntryRepository::get_weekly_usage(pool.inner(), meal_option_id, &week)
        .await
        .map_err(Into::into)
}

/// Get weekly usage count for a specific tag
#[tauri::command]
pub async fn get_weekly_tag_usage(
    tag_id: i64,
    week: String, // Format: "YYYY-WW" - ISO week format
    pool: State<'_, SqlitePool>,
) -> ApiResult<Option<WeeklyTagUsage>> {
    MealEntryRepository::get_weekly_tag_usage(pool.inner(), tag_id, &week)
        .await
        .map_err(Into::into)
}

/// Create a new meal entry
#[tauri::command]
pub async fn create_entry(
    entry: CreateMealEntry,
    pool: State<'_, SqlitePool>,
) -> ApiResult<MealEntry> {
    MealEntryRepository::create(pool.inner(), entry)
        .await
        .map_err(Into::into)
}

/// Update an existing meal entry
#[tauri::command]
pub async fn update_entry(
    id: i64,
    updates: UpdateMealEntry,
    pool: State<'_, SqlitePool>,
) -> ApiResult<MealEntry> {
    MealEntryRepository::update(pool.inner(), id, updates)
        .await
        .map_err(Into::into)
}

/// Delete a meal entry
#[tauri::command]
pub async fn delete_entry(id: i64, pool: State<'_, SqlitePool>) -> ApiResult<()> {
    MealEntryRepository::delete(pool.inner(), id)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        CreateMealOption, CreateMealTemplate, CreateTag, LocationType, TagCategory,
    };
    use crate::repository::{MealOptionRepository, MealTemplateRepository, TagRepository};
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    async fn create_test_option(pool: &SqlitePool) -> i64 {
        // Create template first
        let template = CreateMealTemplate {
            name: "Test Template".to_string(),
            description: None,
            location_type: LocationType::Home,
            compatible_slots: vec![SlotType::Breakfast],
            weekly_limit: None,
        };
        let template_id = MealTemplateRepository::create(pool, template)
            .await
            .expect("Failed to create template")
            .id;

        // Create option
        let option = CreateMealOption {
            template_id,
            name: "Test Option".to_string(),
            description: None,
            nutritional_notes: None,
        };
        MealOptionRepository::create(pool, option)
            .await
            .expect("Failed to create option")
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
    async fn test_create_and_get_entry() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: Some(1.0),
            notes: Some("Test entry".to_string()),
            completed: Some(false),
        };

        let created = MealEntryRepository::create(&pool, entry)
            .await
            .expect("Failed to create entry");

        assert_eq!(created.meal_option_id, option_id);
        assert_eq!(created.date, date);
        assert_eq!(created.slot_type, SlotType::Breakfast);
        assert!(!created.completed);

        let fetched = MealEntryRepository::get_by_id(&pool, created.id)
            .await
            .expect("Failed to get entry")
            .expect("Entry not found");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.meal_option_id, option_id);
    }

    #[tokio::test]
    async fn test_get_entries_by_date() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        let date1 = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();

        // Create entries for different dates
        let entry1 = CreateMealEntry {
            meal_option_id: option_id,
            date: date1,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };

        let entry2 = CreateMealEntry {
            meal_option_id: option_id,
            date: date1,
            slot_type: SlotType::Lunch,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };

        let entry3 = CreateMealEntry {
            meal_option_id: option_id,
            date: date2,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };

        MealEntryRepository::create(&pool, entry1)
            .await
            .expect("Failed to create entry 1");
        MealEntryRepository::create(&pool, entry2)
            .await
            .expect("Failed to create entry 2");
        MealEntryRepository::create(&pool, entry3)
            .await
            .expect("Failed to create entry 3");

        let date1_entries = MealEntryRepository::get_by_date(&pool, date1)
            .await
            .expect("Failed to get entries");

        assert_eq!(date1_entries.len(), 2);
    }

    #[tokio::test]
    async fn test_get_entries_by_date_range() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        let date1 = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();
        let date3 = NaiveDate::from_ymd_opt(2024, 11, 6).unwrap();

        for date in &[date1, date2, date3] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: *date,
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: None,
            };
            MealEntryRepository::create(&pool, entry)
                .await
                .expect("Failed to create entry");
        }

        let range_entries = MealEntryRepository::get_by_date_range(&pool, date1, date2)
            .await
            .expect("Failed to get entries");

        assert_eq!(range_entries.len(), 2);
    }

    #[tokio::test]
    async fn test_get_entry_by_date_and_slot() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();

        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };

        let created = MealEntryRepository::create(&pool, entry)
            .await
            .expect("Failed to create entry");

        let fetched = MealEntryRepository::get_by_date_and_slot(&pool, date, SlotType::Breakfast)
            .await
            .expect("Failed to get entry");

        assert_eq!(fetched.len(), 1);
        assert_eq!(fetched[0].id, created.id);

        // Try getting a different slot (should be empty)
        let not_found = MealEntryRepository::get_by_date_and_slot(&pool, date, SlotType::Lunch)
            .await
            .expect("Failed to query");

        assert!(not_found.is_empty());
    }

    #[tokio::test]
    async fn test_get_entries_by_completed() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();

        // Create planned entry
        let planned = CreateMealEntry {
            meal_option_id: option_id,
            date,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: Some(false),
        };

        // Create completed entry
        let completed = CreateMealEntry {
            meal_option_id: option_id,
            date,
            slot_type: SlotType::Lunch,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: Some(true),
        };

        MealEntryRepository::create(&pool, planned)
            .await
            .expect("Failed to create planned entry");
        MealEntryRepository::create(&pool, completed)
            .await
            .expect("Failed to create completed entry");

        let planned_entries = MealEntryRepository::get_by_completed(&pool, false)
            .await
            .expect("Failed to get planned entries");

        let completed_entries = MealEntryRepository::get_by_completed(&pool, true)
            .await
            .expect("Failed to get completed entries");

        assert_eq!(planned_entries.len(), 1);
        assert_eq!(completed_entries.len(), 1);
    }

    #[tokio::test]
    async fn test_update_entry() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: Some(1.0),
            notes: None,
            completed: Some(false),
        };

        let created = MealEntryRepository::create(&pool, entry)
            .await
            .expect("Failed to create entry");

        let updates = UpdateMealEntry {
            location: Some(LocationType::Office),
            servings: Some(1.5),
            notes: Some(Some("Updated notes".to_string())),
            completed: Some(true),
        };

        let updated = MealEntryRepository::update(&pool, created.id, updates)
            .await
            .expect("Failed to update entry");

        assert_eq!(updated.location, LocationType::Office);
        assert_eq!(updated.servings, 1.5);
        assert_eq!(updated.notes, Some("Updated notes".to_string()));
        assert!(updated.completed);
    }

    #[tokio::test]
    async fn test_delete_entry() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };

        let created = MealEntryRepository::create(&pool, entry)
            .await
            .expect("Failed to create entry");

        MealEntryRepository::delete(&pool, created.id)
            .await
            .expect("Failed to delete entry");

        let fetched = MealEntryRepository::get_by_id(&pool, created.id)
            .await
            .expect("Failed to query entry");

        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn test_get_entries_by_meal_option() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        let date1 = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();

        let entry1 = CreateMealEntry {
            meal_option_id: option_id,
            date: date1,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };

        let entry2 = CreateMealEntry {
            meal_option_id: option_id,
            date: date2,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };

        MealEntryRepository::create(&pool, entry1)
            .await
            .expect("Failed to create entry 1");
        MealEntryRepository::create(&pool, entry2)
            .await
            .expect("Failed to create entry 2");

        let option_entries = MealEntryRepository::get_by_meal_option(&pool, option_id)
            .await
            .expect("Failed to get entries");

        assert_eq!(option_entries.len(), 2);
    }

    #[tokio::test]
    async fn test_weekly_usage() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;

        // Create entries in the same week (Nov 4-10, 2024)
        let monday = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap(); // Monday
        let wednesday = NaiveDate::from_ymd_opt(2024, 11, 6).unwrap();
        let friday = NaiveDate::from_ymd_opt(2024, 11, 8).unwrap();

        for date in &[monday, wednesday, friday] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: *date,
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: Some(true), // Only completed entries count
            };
            MealEntryRepository::create(&pool, entry)
                .await
                .expect("Failed to create entry");
        }

        // Week format: "2024-45" (ISO week 45 of 2024)
        let usage = MealEntryRepository::get_weekly_usage(&pool, option_id, "2024-45")
            .await
            .expect("Failed to get weekly usage")
            .expect("No usage data found");

        assert_eq!(usage.usage_count, 3);
        assert_eq!(usage.meal_option_id, option_id);
    }

    #[tokio::test]
    async fn test_weekly_tag_usage() {
        let pool = setup_test_pool().await;
        let option_id = create_test_option(&pool).await;
        let tag_id = create_test_tag(&pool, "pasta").await;

        // Add tag to option
        MealOptionRepository::add_tags(&pool, option_id, vec![tag_id])
            .await
            .expect("Failed to add tag");

        // Create entries
        let monday = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();

        for date in &[monday, tuesday] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: *date,
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: Some(true),
            };
            MealEntryRepository::create(&pool, entry)
                .await
                .expect("Failed to create entry");
        }

        // Week format: "2024-45" (ISO week 45 of 2024)
        let usage = MealEntryRepository::get_weekly_tag_usage(&pool, tag_id, "2024-45")
            .await
            .expect("Failed to get weekly tag usage")
            .expect("No usage data found");

        assert_eq!(usage.usage_count, 2);
        assert_eq!(usage.tag_name, "pasta");
        assert_eq!(usage.tag_id, tag_id);
    }
}
