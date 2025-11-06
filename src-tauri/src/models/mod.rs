// Data models module
// Rust structs representing database entities

// Re-export all model types
// Note: These will be used in Phase 2 (Tauri commands)
#![allow(dead_code)]

mod enums;
mod meal_entry;
mod meal_option;
mod meal_template;
mod tag;

pub use enums::*;
pub use meal_entry::*;
pub use meal_option::*;
pub use meal_template::*;
pub use tag::*;
