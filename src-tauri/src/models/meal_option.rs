use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Level 3: Meal Option - Ingredient/variation choices within a template
/// Example: "philadelphia", "ricotta", "crema spalmabile 100% frutta secca"
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct MealOption {
    pub id: i64,
    pub template_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub nutritional_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new meal option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMealOption {
    pub template_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub nutritional_notes: Option<String>,
}

/// Input for updating an existing meal option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMealOption {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub nutritional_notes: Option<Option<String>>,
}

/// Meal option with its associated tags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealOptionWithTags {
    #[serde(flatten)]
    pub option: MealOption,
    pub tags: Vec<i64>,  // Tag IDs associated with this option
}

impl CreateMealOption {
    /// Validate option creation data
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Option name cannot be empty".to_string());
        }

        if self.template_id <= 0 {
            return Err("Invalid template ID".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_option_validation() {
        let valid = CreateMealOption {
            template_id: 1,
            name: "philadelphia".to_string(),
            description: Some("Philadelphia cream cheese".to_string()),
            nutritional_notes: None,
        };
        assert!(valid.validate().is_ok());

        // Empty name
        let invalid = CreateMealOption {
            template_id: 1,
            name: "".to_string(),
            description: None,
            nutritional_notes: None,
        };
        assert!(invalid.validate().is_err());

        // Invalid template ID
        let invalid = CreateMealOption {
            template_id: 0,
            name: "ricotta".to_string(),
            description: None,
            nutritional_notes: None,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_option_serialization() {
        let option = CreateMealOption {
            template_id: 5,
            name: "ricotta".to_string(),
            description: Some("Low-fat ricotta cheese".to_string()),
            nutritional_notes: Some("High in protein".to_string()),
        };

        let json = serde_json::to_string(&option).unwrap();
        let deserialized: CreateMealOption = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.name, option.name);
        assert_eq!(deserialized.template_id, 5);
    }

    #[test]
    fn test_option_with_tags_serialization() {
        use chrono::Utc;

        let option_with_tags = MealOptionWithTags {
            option: MealOption {
                id: 1,
                template_id: 2,
                name: "pasta_integrale".to_string(),
                description: Some("Whole wheat pasta".to_string()),
                nutritional_notes: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            tags: vec![10, 11, 12],  // Tag IDs: pasta, pasta_integrale, high_fiber
        };

        let json = serde_json::to_string(&option_with_tags).unwrap();
        let deserialized: MealOptionWithTags = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.option.name, "pasta_integrale");
        assert_eq!(deserialized.tags.len(), 3);
        assert!(deserialized.tags.contains(&10));
    }
}
