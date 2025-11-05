use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::TagCategory;

/// Tag for tracking ingredients, dietary restrictions, and frequency suggestions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,         // Internal key: "pasta", "ricotta"
    pub display_name: String, // User-facing: "Pasta", "Ricotta"
    pub category: TagCategory,
    pub weekly_suggestion: Option<i32>, // Soft limit (e.g., 3 for "max 3x/week")
    pub parent_tag_id: Option<i64>,     // For hierarchies: pasta_integrale -> pasta
    pub created_at: DateTime<Utc>,
}

/// Input for creating a new tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTag {
    pub name: String,
    pub display_name: String,
    pub category: TagCategory,
    pub weekly_suggestion: Option<i32>,
    pub parent_tag_id: Option<i64>,
}

/// Input for updating an existing tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTag {
    pub display_name: Option<String>,
    pub category: Option<TagCategory>,
    pub weekly_suggestion: Option<Option<i32>>, // None = no change, Some(None) = clear value
    pub parent_tag_id: Option<Option<i64>>,
}

impl CreateTag {
    /// Validate tag creation data
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Tag name cannot be empty".to_string());
        }

        if self.display_name.trim().is_empty() {
            return Err("Tag display name cannot be empty".to_string());
        }

        // Name should be lowercase with underscores (internal identifier)
        if !self
            .name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '_')
        {
            return Err("Tag name must be lowercase with underscores only".to_string());
        }

        if let Some(suggestion) = self.weekly_suggestion {
            if suggestion <= 0 {
                return Err("Weekly suggestion must be positive".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tag_validation() {
        let valid_tag = CreateTag {
            name: "pasta_integrale".to_string(),
            display_name: "Pasta Integrale".to_string(),
            category: TagCategory::Ingredient,
            weekly_suggestion: Some(3),
            parent_tag_id: None,
        };
        assert!(valid_tag.validate().is_ok());

        // Empty name
        let invalid = CreateTag {
            name: "".to_string(),
            display_name: "Test".to_string(),
            category: TagCategory::Ingredient,
            weekly_suggestion: None,
            parent_tag_id: None,
        };
        assert!(invalid.validate().is_err());

        // Invalid name format (uppercase)
        let invalid = CreateTag {
            name: "PastaIntegrale".to_string(),
            display_name: "Pasta Integrale".to_string(),
            category: TagCategory::Ingredient,
            weekly_suggestion: None,
            parent_tag_id: None,
        };
        assert!(invalid.validate().is_err());

        // Invalid weekly suggestion
        let invalid = CreateTag {
            name: "pasta".to_string(),
            display_name: "Pasta".to_string(),
            category: TagCategory::Ingredient,
            weekly_suggestion: Some(0),
            parent_tag_id: None,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_tag_serialization() {
        let tag = CreateTag {
            name: "ricotta".to_string(),
            display_name: "Ricotta".to_string(),
            category: TagCategory::Ingredient,
            weekly_suggestion: Some(2),
            parent_tag_id: None,
        };

        let json = serde_json::to_string(&tag).unwrap();
        let deserialized: CreateTag = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, tag.name);
        assert_eq!(deserialized.weekly_suggestion, tag.weekly_suggestion);
    }
}
