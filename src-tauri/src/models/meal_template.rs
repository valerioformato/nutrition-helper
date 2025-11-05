use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{LocationType, SlotType};

/// Level 2: Meal Template - The "cards" that fill slots (the "Oppure" choices)
/// Example: "Pane con marmellata e formaggio spalmabile"
/// Note: compatible_slots is stored as JSON in the database
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MealTemplate {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub compatible_slots: Vec<SlotType>,  // Which slots can this template fill
    pub location_type: LocationType,       // Where this meal can be prepared
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Row structure for fetching from database (compatible_slots as String)
#[derive(Debug, FromRow)]
pub struct MealTemplateRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub compatible_slots: String,  // JSON string from DB
    pub location_type: String,     // TEXT from DB
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<MealTemplateRow> for MealTemplate {
    type Error = String;

    fn try_from(row: MealTemplateRow) -> Result<Self, Self::Error> {
        let compatible_slots = MealTemplate::parse_compatible_slots(&row.compatible_slots)
            .map_err(|e| format!("Failed to parse compatible_slots: {}", e))?;
        
        let location_type = LocationType::from_db_string(&row.location_type)?;

        Ok(MealTemplate {
            id: row.id,
            name: row.name,
            description: row.description,
            compatible_slots,
            location_type,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

/// Input for creating a new meal template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMealTemplate {
    pub name: String,
    pub description: Option<String>,
    pub compatible_slots: Vec<SlotType>,
    pub location_type: LocationType,
}

/// Input for updating an existing meal template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMealTemplate {
    pub name: Option<String>,
    pub description: Option<Option<String>>,  // None = no change, Some(None) = clear
    pub compatible_slots: Option<Vec<SlotType>>,
    pub location_type: Option<LocationType>,
}

impl CreateMealTemplate {
    /// Validate template creation data
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Template name cannot be empty".to_string());
        }

        if self.compatible_slots.is_empty() {
            return Err("Template must be compatible with at least one slot".to_string());
        }

        Ok(())
    }
}

// Helper functions for converting compatible_slots to/from JSON
impl MealTemplate {
    /// Parse compatible slots from JSON string (from database)
    pub fn parse_compatible_slots(json: &str) -> Result<Vec<SlotType>, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Convert compatible slots to JSON string (for database)
    pub fn serialize_compatible_slots(slots: &[SlotType]) -> String {
        serde_json::to_string(slots).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_template_validation() {
        let valid = CreateMealTemplate {
            name: "Pane con marmellata".to_string(),
            description: Some("Breakfast bread with jam".to_string()),
            compatible_slots: vec![SlotType::Breakfast, SlotType::MorningSnack],
            location_type: LocationType::Home,
        };
        assert!(valid.validate().is_ok());

        // Empty name
        let invalid = CreateMealTemplate {
            name: "".to_string(),
            description: None,
            compatible_slots: vec![SlotType::Breakfast],
            location_type: LocationType::Home,
        };
        assert!(invalid.validate().is_err());

        // No compatible slots
        let invalid = CreateMealTemplate {
            name: "Test".to_string(),
            description: None,
            compatible_slots: vec![],
            location_type: LocationType::Home,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_slot_compatibility() {
        let template = CreateMealTemplate {
            name: "Yogurt".to_string(),
            description: None,
            compatible_slots: vec![SlotType::Breakfast, SlotType::MorningSnack],
            location_type: LocationType::Any,
        };

        assert!(template.compatible_slots.contains(&SlotType::Breakfast));
        assert!(!template.compatible_slots.contains(&SlotType::Dinner));
    }

    #[test]
    fn test_template_serialization() {
        let template = CreateMealTemplate {
            name: "Pasta con verdure".to_string(),
            description: Some("Whole wheat pasta with vegetables".to_string()),
            compatible_slots: vec![SlotType::Lunch, SlotType::Dinner],
            location_type: LocationType::Home,
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: CreateMealTemplate = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.name, template.name);
        assert_eq!(deserialized.compatible_slots.len(), 2);
    }

    #[test]
    fn test_compatible_slots_json_conversion() {
        let slots = vec![SlotType::Breakfast, SlotType::Lunch];
        let json_str = MealTemplate::serialize_compatible_slots(&slots);
        
        assert!(json_str.contains("breakfast"));
        assert!(json_str.contains("lunch"));

        let parsed = MealTemplate::parse_compatible_slots(&json_str).unwrap();
        assert_eq!(parsed, slots);
    }
}
