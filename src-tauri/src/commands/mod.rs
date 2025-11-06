// Command handlers module
// Tauri commands for IPC communication between frontend and backend

pub mod meal_template_commands;
pub mod tag_commands;

// Re-export all commands for easy registration
pub use meal_template_commands::*;
pub use tag_commands::*;
