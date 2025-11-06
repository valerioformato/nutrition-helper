// Command handlers module
// Tauri commands for IPC communication between frontend and backend

pub mod tag_commands;

// Re-export all commands for easy registration
pub use tag_commands::*;
