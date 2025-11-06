// Services module
// Business logic layer

pub mod validation_service;

// Re-export for convenient access
pub use validation_service::{ValidationError, ValidationService, ValidationWarning, WarningType};
