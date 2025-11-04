// Test database utilities
// Functions for creating and managing test databases

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

/// Creates an in-memory SQLite database for testing
/// The database is automatically dropped when the pool is dropped
pub async fn create_test_database() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .expect("Failed to create in-memory database");
    
    // Run migrations
    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");
    
    pool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_test_database() {
        let pool = create_test_database().await;
        assert!(pool.acquire().await.is_ok());
    }
}
