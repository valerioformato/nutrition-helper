// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod db;
mod models;
mod commands;
mod repository;
mod services;

use sqlx::SqlitePool;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Nutrition Helper.", name)
}

/// Test command to verify database connection
#[tauri::command]
async fn test_database(state: tauri::State<'_, SqlitePool>) -> Result<String, String> {
    let pool = state.inner();
    
    // Simple query to verify connection
    let result: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    Ok(format!("Database connection successful! Test query returned: {}", result.0))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database
            let db_path = db::get_database_path(app.handle());
            
            tauri::async_runtime::block_on(async move {
                let pool = db::initialize_database(db_path)
                    .await
                    .expect("Failed to initialize database");
                
                // Make the pool available to commands
                app.manage(pool);
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, test_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
