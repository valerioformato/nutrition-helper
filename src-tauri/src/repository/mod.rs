// Repository module
// Data access layer with CRUD operations
// Note: Repositories will be used in Phase 2 (Tauri commands)
#![allow(dead_code)]

mod meal_entry_repository;
mod meal_option_repository;
mod meal_template_repository;
mod tag_repository;

// Re-export repositories (will be used in Phase 2)
#[allow(unused_imports)]
pub use meal_entry_repository::MealEntryRepository;
#[allow(unused_imports)]
pub use meal_option_repository::MealOptionRepository;
#[allow(unused_imports)]
pub use meal_template_repository::MealTemplateRepository;
#[allow(unused_imports)]
pub use tag_repository::TagRepository;
