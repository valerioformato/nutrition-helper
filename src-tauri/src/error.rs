// Error types for IPC communication
// Custom error types that can be serialized across the Tauri IPC boundary

use serde::{Deserialize, Serialize};

/// Result type alias for Tauri commands
pub type ApiResult<T> = Result<T, ApiError>;

/// Main error type for API responses
/// All variants can be serialized and sent to the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum ApiError {
    /// Database operation failed
    DatabaseError(String),

    /// Resource not found (404)
    NotFound(String),

    /// Validation error (400)
    ValidationError(String),

    /// Business logic validation failed (from ValidationService)
    BusinessValidationError(crate::services::ValidationError),

    /// Duplicate resource (409)
    Conflict(String),

    /// Foreign key constraint violation
    ForeignKeyViolation(String),

    /// Internal server error (500)
    InternalError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ApiError::BusinessValidationError(err) => {
                write!(f, "Business validation error: {}", err)
            }
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::ForeignKeyViolation(msg) => write!(f, "Foreign key violation: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

/// Convert sqlx errors to ApiError
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                // Check for specific database errors
                let error_code = db_err.code().map(|c| c.to_string());
                let error_message = db_err.message().to_string();

                // SQLite constraint errors
                if let Some(code) = error_code {
                    match code.as_str() {
                        // UNIQUE constraint failed
                        "1555" | "2067" => {
                            return ApiError::Conflict(format!(
                                "Resource already exists: {}",
                                error_message
                            ));
                        }
                        // FOREIGN KEY constraint failed
                        "787" | "1811" => {
                            return ApiError::ForeignKeyViolation(format!(
                                "Referenced resource not found: {}",
                                error_message
                            ));
                        }
                        _ => {}
                    }
                }

                // Check error message for constraint violations
                if error_message.contains("UNIQUE constraint") {
                    ApiError::Conflict(format!("Resource already exists: {}", error_message))
                } else if error_message.contains("FOREIGN KEY constraint") {
                    ApiError::ForeignKeyViolation(format!(
                        "Referenced resource not found: {}",
                        error_message
                    ))
                } else {
                    ApiError::DatabaseError(error_message)
                }
            }
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

/// Convert ValidationError to ApiError
impl From<crate::services::ValidationError> for ApiError {
    fn from(err: crate::services::ValidationError) -> Self {
        ApiError::BusinessValidationError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let errors = vec![
            (
                ApiError::DatabaseError("Connection failed".to_string()),
                "Database error: Connection failed",
            ),
            (
                ApiError::NotFound("Meal not found".to_string()),
                "Not found: Meal not found",
            ),
            (
                ApiError::ValidationError("Invalid portion size".to_string()),
                "Validation error: Invalid portion size",
            ),
            (
                ApiError::Conflict("Tag already exists".to_string()),
                "Conflict: Tag already exists",
            ),
            (
                ApiError::ForeignKeyViolation("Template not found".to_string()),
                "Foreign key violation: Template not found",
            ),
            (
                ApiError::InternalError("Unexpected error".to_string()),
                "Internal error: Unexpected error",
            ),
        ];

        for (error, expected) in errors {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn test_api_error_serialization() {
        let error = ApiError::ValidationError("Invalid input".to_string());
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("ValidationError"));
        assert!(json.contains("Invalid input"));

        let deserialized: ApiError = serde_json::from_str(&json).unwrap();
        match deserialized {
            ApiError::ValidationError(msg) => assert_eq!(msg, "Invalid input"),
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_sqlx_error_conversion() {
        // Test RowNotFound conversion
        let sqlx_err = sqlx::Error::RowNotFound;
        let api_err: ApiError = sqlx_err.into();
        match api_err {
            ApiError::NotFound(_) => {}
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_business_validation_error_display() {
        use crate::services::ValidationError;

        let val_err = ValidationError::WeeklyLimitExceeded {
            item_name: "Pasta".to_string(),
            limit: 2,
            current_usage: 3,
        };
        let api_err = ApiError::BusinessValidationError(val_err);
        let display = format!("{}", api_err);
        assert!(display.contains("Business validation error"));
        assert!(display.contains("Weekly limit exceeded"));
    }

    #[test]
    fn test_business_validation_error_conversion() {
        use crate::models::SlotType;
        use crate::services::ValidationError;

        let val_err = ValidationError::IncompatibleSlot {
            option_name: "Pizza".to_string(),
            slot: SlotType::Breakfast,
            compatible_slots: vec![SlotType::Lunch],
        };

        let api_err: ApiError = val_err.into();
        match api_err {
            ApiError::BusinessValidationError(_) => {}
            _ => panic!("Expected BusinessValidationError"),
        }
    }
}
