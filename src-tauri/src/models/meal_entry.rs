use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{LocationType, SlotType};

/// Level 4: Meal Entry - Actual meal logging and planning
/// Tracks both planned meals (future) and logged meals (past/completed)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct MealEntry {
    pub id: i64,
    pub meal_option_id: i64,
    pub date: NaiveDate,
    pub slot_type: SlotType,
    pub location: LocationType,
    pub servings: f64, // Default 1.0, nutrition plan uses strict serving sizes
    pub notes: Option<String>,
    pub completed: bool, // FALSE = planned, TRUE = consumed
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input for creating a new meal entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMealEntry {
    pub meal_option_id: i64,
    pub date: NaiveDate,
    pub slot_type: SlotType,
    pub location: LocationType,
    pub servings: Option<f64>, // Defaults to 1.0 if not provided
    pub notes: Option<String>,
    pub completed: Option<bool>, // Defaults to false (planned)
}

/// Input for updating an existing meal entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMealEntry {
    pub location: Option<LocationType>,
    pub servings: Option<f64>,
    pub notes: Option<Option<String>>,
    pub completed: Option<bool>,
}

impl CreateMealEntry {
    /// Validate entry creation data
    pub fn validate(&self) -> Result<(), String> {
        if self.meal_option_id <= 0 {
            return Err("Invalid meal option ID".to_string());
        }

        if let Some(servings) = self.servings {
            if servings <= 0.0 {
                return Err("Servings must be positive".to_string());
            }
        }

        Ok(())
    }

    /// Get servings value, defaulting to 1.0 if not provided
    pub fn servings_or_default(&self) -> f64 {
        self.servings.unwrap_or(1.0)
    }

    /// Get completed value, defaulting to false (planned) if not provided
    pub fn completed_or_default(&self) -> bool {
        self.completed.unwrap_or(false)
    }
}

impl UpdateMealEntry {
    /// Validate entry update data
    pub fn validate(&self) -> Result<(), String> {
        if let Some(servings) = self.servings {
            if servings <= 0.0 {
                return Err("Servings must be positive".to_string());
            }
        }

        Ok(())
    }
}

/// Helper struct for weekly usage tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WeeklyUsage {
    pub meal_option_id: i64,
    pub week: String, // Format: "YYYY-WW" (ISO week)
    pub usage_count: i64,
}

/// Helper struct for weekly tag usage tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WeeklyTagUsage {
    pub tag_id: i64,
    pub tag_name: String,
    pub week: String,
    pub usage_count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_create_entry_validation() {
        let valid = CreateMealEntry {
            meal_option_id: 1,
            date: NaiveDate::from_ymd_opt(2024, 11, 4).unwrap(),
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: Some(1.0),
            notes: None,
            completed: Some(false),
        };
        assert!(valid.validate().is_ok());

        // Invalid meal option ID
        let invalid = CreateMealEntry {
            meal_option_id: 0,
            date: NaiveDate::from_ymd_opt(2024, 11, 4).unwrap(),
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: Some(1.0),
            notes: None,
            completed: None,
        };
        assert!(invalid.validate().is_err());

        // Invalid servings
        let invalid = CreateMealEntry {
            meal_option_id: 1,
            date: NaiveDate::from_ymd_opt(2024, 11, 4).unwrap(),
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: Some(0.0),
            notes: None,
            completed: None,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_create_entry_defaults() {
        let entry = CreateMealEntry {
            meal_option_id: 1,
            date: NaiveDate::from_ymd_opt(2024, 11, 4).unwrap(),
            slot_type: SlotType::Lunch,
            location: LocationType::Office,
            servings: None,
            notes: None,
            completed: None,
        };

        assert_eq!(entry.servings_or_default(), 1.0);
        assert!(!entry.completed_or_default());
    }

    #[test]
    fn test_update_entry_validation() {
        let valid = UpdateMealEntry {
            location: Some(LocationType::Restaurant),
            servings: Some(1.5),
            notes: Some(Some("Had extra avocado".to_string())),
            completed: Some(true),
        };
        assert!(valid.validate().is_ok());

        // Invalid servings
        let invalid = UpdateMealEntry {
            location: None,
            servings: Some(-1.0),
            notes: None,
            completed: None,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_entry_serialization() {
        let entry = CreateMealEntry {
            meal_option_id: 5,
            date: NaiveDate::from_ymd_opt(2024, 11, 5).unwrap(),
            slot_type: SlotType::Dinner,
            location: LocationType::Home,
            servings: Some(1.2),
            notes: Some("Extra vegetables".to_string()),
            completed: Some(true),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: CreateMealEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.meal_option_id, 5);
        assert_eq!(deserialized.servings, Some(1.2));
        assert_eq!(deserialized.completed, Some(true));
    }

    #[test]
    fn test_weekly_usage_serialization() {
        let usage = WeeklyUsage {
            meal_option_id: 3,
            week: "2024-45".to_string(),
            usage_count: 2,
        };

        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("2024-45"));

        let deserialized: WeeklyUsage = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.usage_count, 2);
    }
}
