// Database module
// Database connection and initialization

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::PathBuf;
use tauri::Manager;

/// Initialize the database connection pool
/// Creates the database file if it doesn't exist and runs migrations
pub async fn initialize_database(db_path: PathBuf) -> Result<SqlitePool, sqlx::Error> {
    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
    }

    // Create connection string
    let connection_string = format!("sqlite://{}?mode=rwc", db_path.display());

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

/// Get the default database path for the application
/// Uses the app data directory provided by Tauri
pub fn get_database_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    app_data_dir.join("nutrition_helper.db")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_initialize_database() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let pool = initialize_database(db_path.clone()).await;
        assert!(pool.is_ok());

        // Verify the database file was created
        assert!(db_path.exists());
    }

    #[tokio::test]
    async fn test_database_creates_parent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("nested").join("dirs").join("test.db");

        let pool = initialize_database(db_path.clone()).await;
        assert!(pool.is_ok());

        // Verify nested directories were created
        assert!(db_path.parent().unwrap().exists());
        assert!(db_path.exists());
    }

    #[tokio::test]
    async fn test_migrations_create_tables() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let pool = initialize_database(db_path).await.unwrap();

        // Query to check if tables exist
        let tables: Vec<(String,)> = sqlx::query_as(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name != '_sqlx_migrations' ORDER BY name"
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        let table_names: Vec<String> = tables.into_iter().map(|(name,)| name).collect();

        // Verify all core tables exist (4-level hierarchy + tags system)
        assert!(
            table_names.contains(&"meal_templates".to_string()),
            "meal_templates table not found"
        );
        assert!(
            table_names.contains(&"meal_options".to_string()),
            "meal_options table not found"
        );
        assert!(
            table_names.contains(&"meal_entries".to_string()),
            "meal_entries table not found"
        );
        assert!(
            table_names.contains(&"tags".to_string()),
            "tags table not found"
        );
        assert!(
            table_names.contains(&"meal_option_tags".to_string()),
            "meal_option_tags junction table not found"
        );

        // Should have exactly 5 tables
        assert_eq!(
            table_names.len(),
            5,
            "Expected 5 tables, found: {:?}",
            table_names
        );
    }

    #[tokio::test]
    async fn test_indexes_are_created() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let pool = initialize_database(db_path).await.unwrap();

        // Query to check if indexes exist
        let indexes: Vec<(String,)> = sqlx::query_as(
            "SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%' ORDER BY name",
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        let index_names: Vec<String> = indexes.into_iter().map(|(name,)| name).collect();

        // Verify indexes exist (5 indexes total after schema update)
        assert!(index_names.contains(&"idx_meal_entries_date".to_string()));
        assert!(index_names.contains(&"idx_meal_entries_option".to_string()));
        assert!(index_names.contains(&"idx_meal_entries_date_slot".to_string()));
        assert!(index_names.contains(&"idx_meal_options_template".to_string()));
        assert!(index_names.contains(&"idx_meal_templates_location".to_string()));
        assert!(index_names.contains(&"idx_tags_category".to_string()));
        assert!(index_names.contains(&"idx_tags_parent".to_string()));
        assert!(index_names.contains(&"idx_meal_option_tags_option".to_string()));
        assert!(index_names.contains(&"idx_meal_option_tags_tag".to_string()));

        // Should have exactly 9 indexes (5 original + 4 for tags system)
        assert_eq!(
            index_names.len(),
            9,
            "Expected 9 indexes, found: {:?}",
            index_names
        );
    }

    #[tokio::test]
    async fn test_view_is_created() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let pool = initialize_database(db_path).await.unwrap();

        // Query to check if view exists
        let views: Vec<(String,)> =
            sqlx::query_as("SELECT name FROM sqlite_master WHERE type='view' ORDER BY name")
                .fetch_all(&pool)
                .await
                .unwrap();

        let view_names: Vec<String> = views.into_iter().map(|(name,)| name).collect();

        // Verify both views exist (meal usage + tag usage)
        assert!(view_names.contains(&"weekly_meal_usage".to_string()));
        assert!(view_names.contains(&"weekly_tag_usage".to_string()));

        // Should have exactly 2 views
        assert_eq!(
            view_names.len(),
            2,
            "Expected 2 views, found: {:?}",
            view_names
        );
    }
}
