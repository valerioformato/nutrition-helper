// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod db;
mod error;
mod models;
mod repository;
mod services;

use sqlx::SqlitePool;
use tauri::Manager;

// Re-export error types for use in commands
pub use error::{ApiError, ApiResult};

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

    Ok(format!(
        "Database connection successful! Test query returned: {}",
        result.0
    ))
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
        .invoke_handler(tauri::generate_handler![
            greet,
            test_database,
            // Tag commands
            commands::get_all_tags,
            commands::get_tag_by_id,
            commands::get_tag_by_name,
            commands::get_tags_by_category,
            commands::get_tag_children,
            commands::create_tag,
            commands::update_tag,
            commands::delete_tag,
            // MealTemplate commands
            commands::get_all_templates,
            commands::get_template_by_id,
            commands::get_templates_by_location,
            commands::get_templates_by_slot,
            commands::search_templates,
            commands::create_template,
            commands::update_template,
            commands::delete_template,
            // MealOption commands
            commands::get_all_options,
            commands::get_option_by_id,
            commands::get_option_with_tags,
            commands::get_options_by_template,
            commands::get_options_by_template_with_tags,
            commands::search_options,
            commands::create_option,
            commands::update_option,
            commands::delete_option,
            commands::add_tags_to_option,
            commands::remove_tags_from_option,
            commands::set_option_tags,
            // MealEntry commands
            commands::get_entry_by_id,
            commands::get_entries_by_date,
            commands::get_entries_by_date_range,
            commands::get_entry_by_date_and_slot,
            commands::get_entries_by_completed,
            commands::get_entries_by_meal_option,
            commands::get_weekly_usage,
            commands::get_weekly_tag_usage,
            commands::create_entry,
            commands::update_entry,
            commands::delete_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
