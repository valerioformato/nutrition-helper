use serde::{Deserialize, Serialize};
use sqlx::Type;

/// The five fixed meal slots per day
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SlotType {
    Breakfast,
    MorningSnack,
    Lunch,
    AfternoonSnack,
    Dinner,
}

impl SlotType {
    /// Get all slot types in order
    pub fn all() -> [SlotType; 5] {
        [
            SlotType::Breakfast,
            SlotType::MorningSnack,
            SlotType::Lunch,
            SlotType::AfternoonSnack,
            SlotType::Dinner,
        ]
    }

    /// Convert to database string representation
    pub fn to_db_string(&self) -> &'static str {
        match self {
            SlotType::Breakfast => "breakfast",
            SlotType::MorningSnack => "morning_snack",
            SlotType::Lunch => "lunch",
            SlotType::AfternoonSnack => "afternoon_snack",
            SlotType::Dinner => "dinner",
        }
    }

    /// Parse from database string
    pub fn from_db_string(s: &str) -> Result<Self, String> {
        match s {
            "breakfast" => Ok(SlotType::Breakfast),
            "morning_snack" => Ok(SlotType::MorningSnack),
            "lunch" => Ok(SlotType::Lunch),
            "afternoon_snack" => Ok(SlotType::AfternoonSnack),
            "dinner" => Ok(SlotType::Dinner),
            _ => Err(format!("Invalid slot type: {}", s)),
        }
    }
}

/// Where a meal can be prepared/consumed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    Home,
    Office,
    Restaurant,
    Any, // For meals that work anywhere
}

impl LocationType {
    pub fn to_db_string(&self) -> &'static str {
        match self {
            LocationType::Home => "home",
            LocationType::Office => "office",
            LocationType::Restaurant => "restaurant",
            LocationType::Any => "any",
        }
    }

    pub fn from_db_string(s: &str) -> Result<Self, String> {
        match s {
            "home" => Ok(LocationType::Home),
            "office" => Ok(LocationType::Office),
            "restaurant" => Ok(LocationType::Restaurant),
            "any" => Ok(LocationType::Any),
            _ => Err(format!("Invalid location type: {}", s)),
        }
    }

    /// Check if a location is compatible with this type
    pub fn is_compatible_with(&self, other: LocationType) -> bool {
        *self == LocationType::Any || other == LocationType::Any || *self == other
    }
}

/// Category for tags (ingredient tracking, dietary restrictions, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TagCategory {
    Ingredient,
    Dietary,
    PrepTime,
    Other,
}

impl TagCategory {
    pub fn to_db_string(&self) -> &'static str {
        match self {
            TagCategory::Ingredient => "ingredient",
            TagCategory::Dietary => "dietary",
            TagCategory::PrepTime => "prep_time",
            TagCategory::Other => "other",
        }
    }

    pub fn from_db_string(s: &str) -> Result<Self, String> {
        match s {
            "ingredient" => Ok(TagCategory::Ingredient),
            "dietary" => Ok(TagCategory::Dietary),
            "prep_time" => Ok(TagCategory::PrepTime),
            "other" => Ok(TagCategory::Other),
            _ => Err(format!("Invalid tag category: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_type_all() {
        let slots = SlotType::all();
        assert_eq!(slots.len(), 5);
        assert_eq!(slots[0], SlotType::Breakfast);
        assert_eq!(slots[4], SlotType::Dinner);
    }

    #[test]
    fn test_slot_type_db_conversion() {
        assert_eq!(SlotType::Breakfast.to_db_string(), "breakfast");
        assert_eq!(SlotType::MorningSnack.to_db_string(), "morning_snack");
        
        assert_eq!(
            SlotType::from_db_string("lunch").unwrap(),
            SlotType::Lunch
        );
        assert!(SlotType::from_db_string("invalid").is_err());
    }

    #[test]
    fn test_location_type_compatibility() {
        assert!(LocationType::Home.is_compatible_with(LocationType::Home));
        assert!(!LocationType::Home.is_compatible_with(LocationType::Office));
        assert!(LocationType::Any.is_compatible_with(LocationType::Home));
        assert!(LocationType::Home.is_compatible_with(LocationType::Any));
    }

    #[test]
    fn test_location_type_db_conversion() {
        assert_eq!(LocationType::Home.to_db_string(), "home");
        assert_eq!(
            LocationType::from_db_string("restaurant").unwrap(),
            LocationType::Restaurant
        );
        assert!(LocationType::from_db_string("invalid").is_err());
    }

    #[test]
    fn test_tag_category_db_conversion() {
        assert_eq!(TagCategory::Ingredient.to_db_string(), "ingredient");
        assert_eq!(
            TagCategory::from_db_string("dietary").unwrap(),
            TagCategory::Dietary
        );
        assert!(TagCategory::from_db_string("invalid").is_err());
    }

    #[test]
    fn test_enum_serialization() {
        // Test serde serialization (for IPC)
        let slot = SlotType::Breakfast;
        let json = serde_json::to_string(&slot).unwrap();
        assert_eq!(json, r#""breakfast""#);

        let location = LocationType::Home;
        let json = serde_json::to_string(&location).unwrap();
        assert_eq!(json, r#""home""#);
    }
}
