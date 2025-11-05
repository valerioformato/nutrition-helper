use crate::models::{
    CreateMealEntry, LocationType, MealEntry, SlotType, UpdateMealEntry, WeeklyTagUsage,
    WeeklyUsage,
};
use chrono::NaiveDate;
use sqlx::{Result, Row, SqlitePool};

pub struct MealEntryRepository;

impl MealEntryRepository {
    /// Helper to convert a database row to MealEntry
    fn row_to_entry(row: &sqlx::sqlite::SqliteRow) -> Result<MealEntry> {
        let slot_type_str: String = row.try_get("slot_type")?;
        let slot_type =
            SlotType::from_db_string(&slot_type_str).map_err(|e| sqlx::Error::Protocol(e))?;

        let location_str: String = row.try_get("location")?;
        let location =
            LocationType::from_db_string(&location_str).map_err(|e| sqlx::Error::Protocol(e))?;

        Ok(MealEntry {
            id: row.try_get("id")?,
            meal_option_id: row.try_get("meal_option_id")?,
            date: row.try_get("date")?,
            slot_type,
            location,
            servings: row.try_get("servings")?,
            notes: row.try_get("notes")?,
            completed: row.try_get("completed")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    /// Create a new meal entry
    pub async fn create(pool: &SqlitePool, entry: CreateMealEntry) -> Result<MealEntry> {
        // Validate using the model's validation method
        entry.validate().map_err(|e| sqlx::Error::Protocol(e))?;

        // Check that meal_option_id exists
        let option_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM meal_options WHERE id = ?)")
                .bind(entry.meal_option_id)
                .fetch_one(pool)
                .await?;

        if !option_exists {
            return Err(sqlx::Error::Protocol(format!(
                "Meal option with id {} does not exist",
                entry.meal_option_id
            )));
        }

        let servings = entry.servings_or_default();
        let completed = entry.completed_or_default();

        let result = sqlx::query(
            "INSERT INTO meal_entries (meal_option_id, date, slot_type, location, servings, notes, completed) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(entry.meal_option_id)
        .bind(entry.date)
        .bind(entry.slot_type.to_db_string())
        .bind(entry.location.to_db_string())
        .bind(servings)
        .bind(&entry.notes)
        .bind(completed)
        .execute(pool)
        .await?;

        let id = result.last_insert_rowid();
        Self::get_by_id(pool, id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)
    }

    /// Get a meal entry by ID
    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<Option<MealEntry>> {
        let row = sqlx::query(
            "SELECT id, meal_option_id, date, slot_type, location, servings, notes, completed,
                    created_at, updated_at
             FROM meal_entries 
             WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_entry(&row)?)),
            None => Ok(None),
        }
    }

    /// Get all entries for a specific date
    pub async fn get_by_date(pool: &SqlitePool, date: NaiveDate) -> Result<Vec<MealEntry>> {
        let rows = sqlx::query(
            "SELECT id, meal_option_id, date, slot_type, location, servings, notes, completed,
                    created_at, updated_at
             FROM meal_entries 
             WHERE date = ?
             ORDER BY CASE slot_type
                 WHEN 'breakfast' THEN 1
                 WHEN 'morning_snack' THEN 2
                 WHEN 'lunch' THEN 3
                 WHEN 'afternoon_snack' THEN 4
                 WHEN 'dinner' THEN 5
             END",
        )
        .bind(date)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_entry).collect()
    }

    /// Get entries for a date range
    pub async fn get_by_date_range(
        pool: &SqlitePool,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<MealEntry>> {
        let rows = sqlx::query(
            "SELECT id, meal_option_id, date, slot_type, location, servings, notes, completed,
                    created_at, updated_at
             FROM meal_entries 
             WHERE date BETWEEN ? AND ?
             ORDER BY date, CASE slot_type
                 WHEN 'breakfast' THEN 1
                 WHEN 'morning_snack' THEN 2
                 WHEN 'lunch' THEN 3
                 WHEN 'afternoon_snack' THEN 4
                 WHEN 'dinner' THEN 5
             END",
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_entry).collect()
    }

    /// Get entries by date and slot type
    pub async fn get_by_date_and_slot(
        pool: &SqlitePool,
        date: NaiveDate,
        slot: SlotType,
    ) -> Result<Vec<MealEntry>> {
        let rows = sqlx::query(
            "SELECT id, meal_option_id, date, slot_type, location, servings, notes, completed,
                    created_at, updated_at
             FROM meal_entries 
             WHERE date = ? AND slot_type = ?",
        )
        .bind(date)
        .bind(slot.to_db_string())
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_entry).collect()
    }

    /// Get entries by completion status
    pub async fn get_by_completed(pool: &SqlitePool, completed: bool) -> Result<Vec<MealEntry>> {
        let rows = sqlx::query(
            "SELECT id, meal_option_id, date, slot_type, location, servings, notes, completed,
                    created_at, updated_at
             FROM meal_entries 
             WHERE completed = ?
             ORDER BY date DESC, CASE slot_type
                 WHEN 'breakfast' THEN 1
                 WHEN 'morning_snack' THEN 2
                 WHEN 'lunch' THEN 3
                 WHEN 'afternoon_snack' THEN 4
                 WHEN 'dinner' THEN 5
             END",
        )
        .bind(completed)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_entry).collect()
    }

    /// Get all entries for a specific meal option
    pub async fn get_by_meal_option(
        pool: &SqlitePool,
        meal_option_id: i64,
    ) -> Result<Vec<MealEntry>> {
        let rows = sqlx::query(
            "SELECT id, meal_option_id, date, slot_type, location, servings, notes, completed,
                    created_at, updated_at
             FROM meal_entries 
             WHERE meal_option_id = ?
             ORDER BY date DESC",
        )
        .bind(meal_option_id)
        .fetch_all(pool)
        .await?;

        rows.iter().map(Self::row_to_entry).collect()
    }

    /// Get weekly usage statistics for a meal option
    pub async fn get_weekly_usage(
        pool: &SqlitePool,
        meal_option_id: i64,
        week: &str,
    ) -> Result<Option<WeeklyUsage>> {
        let row = sqlx::query_as::<_, WeeklyUsage>(
            "SELECT meal_option_id, week, usage_count 
             FROM weekly_meal_usage 
             WHERE meal_option_id = ? AND week = ?",
        )
        .bind(meal_option_id)
        .bind(week)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Get weekly usage statistics for a tag
    pub async fn get_weekly_tag_usage(
        pool: &SqlitePool,
        tag_id: i64,
        week: &str,
    ) -> Result<Option<WeeklyTagUsage>> {
        let row = sqlx::query_as::<_, WeeklyTagUsage>(
            "SELECT tag_id, tag_name, week, usage_count 
             FROM weekly_tag_usage 
             WHERE tag_id = ? AND week = ?",
        )
        .bind(tag_id)
        .bind(week)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// Update a meal entry
    pub async fn update(pool: &SqlitePool, id: i64, update: UpdateMealEntry) -> Result<MealEntry> {
        // Validate using the model's validation method
        update.validate().map_err(|e| sqlx::Error::Protocol(e))?;

        // Check that entry exists
        if Self::get_by_id(pool, id).await?.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        // Build dynamic update query based on which fields are Some
        let mut updates = Vec::new();
        let mut query_str = String::from("UPDATE meal_entries SET ");

        if update.location.is_some() {
            updates.push("location = ?");
        }
        if update.servings.is_some() {
            updates.push("servings = ?");
        }
        if update.notes.is_some() {
            updates.push("notes = ?");
        }
        if update.completed.is_some() {
            updates.push("completed = ?");
        }

        if updates.is_empty() {
            // No updates to make, just return the current entry
            return Self::get_by_id(pool, id)
                .await?
                .ok_or_else(|| sqlx::Error::RowNotFound);
        }

        query_str.push_str(&updates.join(", "));
        query_str.push_str(", updated_at = CURRENT_TIMESTAMP WHERE id = ?");

        let mut query = sqlx::query(&query_str);

        // Bind parameters in the same order as the updates vector
        if let Some(location) = &update.location {
            query = query.bind(location.to_db_string());
        }
        if let Some(servings) = update.servings {
            query = query.bind(servings);
        }
        if let Some(notes) = &update.notes {
            query = query.bind(notes.as_ref());
        }
        if let Some(completed) = update.completed {
            query = query.bind(completed);
        }

        query = query.bind(id);
        query.execute(pool).await?;

        Self::get_by_id(pool, id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)
    }

    /// Delete a meal entry
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<()> {
        let result = sqlx::query("DELETE FROM meal_entries WHERE id = ?")
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
    use crate::models::{CreateMealOption, CreateMealTemplate, CreateTag, TagCategory};
    use crate::repository::{MealOptionRepository, MealTemplateRepository, TagRepository};
    use chrono::NaiveDate;
    use std::path::PathBuf;
    use tempfile::TempDir;

    async fn setup_test_db() -> (SqlitePool, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let pool = db::initialize_database(PathBuf::from(db_path))
            .await
            .unwrap();
        (pool, temp_dir)
    }

    async fn create_test_option(pool: &SqlitePool) -> i64 {
        // Create a template first
        let template = CreateMealTemplate {
            name: "Test Template".to_string(),
            description: Some("Test Description".to_string()),
            location_type: LocationType::Home,
            compatible_slots: vec![SlotType::Breakfast, SlotType::Lunch],
        };
        let template = MealTemplateRepository::create(pool, template)
            .await
            .unwrap();

        // Create an option
        let option = CreateMealOption {
            template_id: template.id,
            name: "Test Option".to_string(),
            description: Some("Test option description".to_string()),
            nutritional_notes: None,
        };
        let option = MealOptionRepository::create(pool, option).await.unwrap();
        option.id
    }

    #[tokio::test]
    async fn test_create_entry() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date: NaiveDate::from_ymd_opt(2024, 11, 5).unwrap(),
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: Some(1.5),
            notes: Some("Extra avocado".to_string()),
            completed: Some(true),
        };

        let created = MealEntryRepository::create(&pool, entry).await.unwrap();

        assert_eq!(created.meal_option_id, option_id);
        assert_eq!(created.date, NaiveDate::from_ymd_opt(2024, 11, 5).unwrap());
        assert_eq!(created.slot_type, SlotType::Breakfast);
        assert_eq!(created.servings, 1.5);
        assert_eq!(created.completed, true);
    }

    #[tokio::test]
    async fn test_create_entry_with_defaults() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date: NaiveDate::from_ymd_opt(2024, 11, 5).unwrap(),
            slot_type: SlotType::Lunch,
            location: LocationType::Office,
            servings: None, // Should default to 1.0
            notes: None,
            completed: None, // Should default to false
        };

        let created = MealEntryRepository::create(&pool, entry).await.unwrap();

        assert_eq!(created.servings, 1.0);
        assert_eq!(created.completed, false);
    }

    #[tokio::test]
    async fn test_get_by_date() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;
        let date = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();

        // Create multiple entries for the same date
        for slot in [SlotType::Breakfast, SlotType::Lunch, SlotType::Dinner] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date,
                slot_type: slot,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: None,
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        let entries = MealEntryRepository::get_by_date(&pool, date).await.unwrap();

        assert_eq!(entries.len(), 3);
        // Verify they're sorted by slot order
        assert_eq!(entries[0].slot_type, SlotType::Breakfast);
        assert_eq!(entries[1].slot_type, SlotType::Lunch);
        assert_eq!(entries[2].slot_type, SlotType::Dinner);
    }

    #[tokio::test]
    async fn test_get_by_date_range() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        // Create entries across multiple dates
        for day in 1..=5 {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: NaiveDate::from_ymd_opt(2024, 11, day).unwrap(),
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: None,
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        let entries = MealEntryRepository::get_by_date_range(
            &pool,
            NaiveDate::from_ymd_opt(2024, 11, 2).unwrap(),
            NaiveDate::from_ymd_opt(2024, 11, 4).unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(entries.len(), 3);
        assert_eq!(
            entries[0].date,
            NaiveDate::from_ymd_opt(2024, 11, 2).unwrap()
        );
        assert_eq!(
            entries[1].date,
            NaiveDate::from_ymd_opt(2024, 11, 3).unwrap()
        );
        assert_eq!(
            entries[2].date,
            NaiveDate::from_ymd_opt(2024, 11, 4).unwrap()
        );
    }

    #[tokio::test]
    async fn test_get_by_date_and_slot() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;
        let date = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();

        // Create entries for different slots
        for slot in [SlotType::Breakfast, SlotType::Lunch] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date,
                slot_type: slot,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: None,
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        let entries = MealEntryRepository::get_by_date_and_slot(&pool, date, SlotType::Breakfast)
            .await
            .unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].slot_type, SlotType::Breakfast);
    }

    #[tokio::test]
    async fn test_get_by_completed() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        // Create planned and completed entries
        for (day, completed) in [(1, false), (2, false), (3, true), (4, true)] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: NaiveDate::from_ymd_opt(2024, 11, day).unwrap(),
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: Some(completed),
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        let planned = MealEntryRepository::get_by_completed(&pool, false)
            .await
            .unwrap();
        assert_eq!(planned.len(), 2);

        let completed = MealEntryRepository::get_by_completed(&pool, true)
            .await
            .unwrap();
        assert_eq!(completed.len(), 2);
    }

    #[tokio::test]
    async fn test_get_by_meal_option() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        // Create multiple entries for the same option
        for day in 1..=3 {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: NaiveDate::from_ymd_opt(2024, 11, day).unwrap(),
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: None,
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        let entries = MealEntryRepository::get_by_meal_option(&pool, option_id)
            .await
            .unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[tokio::test]
    async fn test_weekly_usage_view() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        // Create completed entries in the same week (Nov 4-6, 2024 are all in week 45)
        // Nov 4 = Monday, Nov 5 = Tuesday, Nov 6 = Wednesday
        for day in [4, 5, 6] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: NaiveDate::from_ymd_opt(2024, 11, day).unwrap(),
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: Some(true), // Only completed entries count
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        // Query the weekly usage view (Nov 4-6 are in ISO week 45)
        let usage = MealEntryRepository::get_weekly_usage(&pool, option_id, "2024-45")
            .await
            .unwrap();

        assert!(usage.is_some());
        let usage = usage.unwrap();
        assert_eq!(usage.usage_count, 3);
    }

    #[tokio::test]
    async fn test_update_entry() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date: NaiveDate::from_ymd_opt(2024, 11, 5).unwrap(),
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: Some(1.0),
            notes: Some("Original notes".to_string()),
            completed: Some(false),
        };
        let created = MealEntryRepository::create(&pool, entry).await.unwrap();

        // Update servings and mark as completed
        let update = UpdateMealEntry {
            location: Some(LocationType::Office),
            servings: Some(1.5),
            notes: None,
            completed: Some(true),
        };
        let updated = MealEntryRepository::update(&pool, created.id, update)
            .await
            .unwrap();

        assert_eq!(updated.location, LocationType::Office);
        assert_eq!(updated.servings, 1.5);
        assert_eq!(updated.notes, Some("Original notes".to_string()));
        assert_eq!(updated.completed, true);

        // Clear notes
        let update = UpdateMealEntry {
            location: None,
            servings: None,
            notes: Some(None),
            completed: None,
        };
        let updated = MealEntryRepository::update(&pool, created.id, update)
            .await
            .unwrap();
        assert_eq!(updated.notes, None);
    }

    #[tokio::test]
    async fn test_delete_entry() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date: NaiveDate::from_ymd_opt(2024, 11, 5).unwrap(),
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };
        let created = MealEntryRepository::create(&pool, entry).await.unwrap();

        // Delete the entry
        MealEntryRepository::delete(&pool, created.id)
            .await
            .unwrap();

        // Verify it's gone
        let retrieved = MealEntryRepository::get_by_id(&pool, created.id)
            .await
            .unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_validation_error() {
        let (pool, _temp_dir) = setup_test_db().await;
        let option_id = create_test_option(&pool).await;

        // Invalid servings
        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date: NaiveDate::from_ymd_opt(2024, 11, 5).unwrap(),
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: Some(0.0),
            notes: None,
            completed: None,
        };

        let result = MealEntryRepository::create(&pool, entry).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_meal_option_id() {
        let (pool, _temp_dir) = setup_test_db().await;

        let entry = CreateMealEntry {
            meal_option_id: 99999, // Non-existent option
            date: NaiveDate::from_ymd_opt(2024, 11, 5).unwrap(),
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: None,
        };

        let result = MealEntryRepository::create(&pool, entry).await;
        assert!(result.is_err());
    }
}
