// Validation Service
// Business logic for validating meal entries and enforcing business rules

use crate::models::{MealTemplate, SlotType};
use crate::repository::{MealEntryRepository, MealOptionRepository, TagRepository};
use chrono::{Datelike, IsoWeek, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// Result type for validation operations
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validation errors with detailed messages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ValidationError {
    /// Weekly limit would be exceeded
    WeeklyLimitExceeded {
        item_name: String,
        limit: i32,
        current_usage: i64,
    },
    /// Meal option is not compatible with the requested slot
    IncompatibleSlot {
        option_name: String,
        slot: SlotType,
        compatible_slots: Vec<SlotType>,
    },
    /// Tag weekly suggestion would be exceeded (warning, not error)
    TagSuggestionExceeded {
        tag_name: String,
        suggestion: i32,
        current_usage: i64,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::WeeklyLimitExceeded {
                item_name,
                limit,
                current_usage,
            } => write!(
                f,
                "Weekly limit exceeded for '{}': {}/{} uses this week",
                item_name, current_usage, limit
            ),
            ValidationError::IncompatibleSlot {
                option_name,
                slot,
                compatible_slots,
            } => write!(
                f,
                "'{}' is not compatible with {:?}. Compatible slots: {:?}",
                option_name, slot, compatible_slots
            ),
            ValidationError::TagSuggestionExceeded {
                tag_name,
                suggestion,
                current_usage,
            } => write!(
                f,
                "Tag '{}' suggestion exceeded: {}/{} uses this week",
                tag_name, current_usage, suggestion
            ),
        }
    }
}

/// Validation warnings (non-blocking)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub message: String,
    pub warning_type: WarningType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WarningType {
    TagSuggestion,
    HighFrequency,
}

pub struct ValidationService;

impl ValidationService {
    /// Get the ISO week string for a given date (format: "YYYY-WW")
    /// Weeks start on Monday as per ISO 8601
    pub fn get_week_string(date: NaiveDate) -> String {
        let iso_week: IsoWeek = date.iso_week();
        format!("{}-{:02}", iso_week.year(), iso_week.week())
    }

    /// Get the Monday of the week for a given date
    pub fn get_week_start(date: NaiveDate) -> NaiveDate {
        let weekday = date.weekday().num_days_from_monday();
        date - chrono::Duration::days(weekday as i64)
    }

    /// Validate that a meal option is compatible with a specific slot
    pub fn validate_slot_compatibility(
        template: &MealTemplate,
        slot: SlotType,
    ) -> ValidationResult<()> {
        if template.compatible_slots.contains(&slot) {
            Ok(())
        } else {
            Err(ValidationError::IncompatibleSlot {
                option_name: template.name.clone(),
                slot,
                compatible_slots: template.compatible_slots.clone(),
            })
        }
    }

    /// Check if adding a meal entry would exceed weekly limits
    /// Returns Ok(()) if within limits, Err with details if exceeded
    pub async fn check_weekly_limit(
        pool: &SqlitePool,
        meal_option_id: i64,
        date: NaiveDate,
    ) -> ValidationResult<()> {
        // Get the meal option to check if it has a weekly limit
        let option = MealOptionRepository::get_by_id(pool, meal_option_id)
            .await
            .map_err(|_| ValidationError::WeeklyLimitExceeded {
                item_name: "Unknown".to_string(),
                limit: 0,
                current_usage: 0,
            })?
            .ok_or_else(|| ValidationError::WeeklyLimitExceeded {
                item_name: "Unknown".to_string(),
                limit: 0,
                current_usage: 0,
            })?;

        // Get the template to check for weekly limit
        let template =
            crate::repository::MealTemplateRepository::get_by_id(pool, option.template_id)
                .await
                .map_err(|_| ValidationError::WeeklyLimitExceeded {
                    item_name: option.name.clone(),
                    limit: 0,
                    current_usage: 0,
                })?
                .ok_or_else(|| ValidationError::WeeklyLimitExceeded {
                    item_name: option.name.clone(),
                    limit: 0,
                    current_usage: 0,
                })?;

        // Check if template has a weekly limit
        if let Some(weekly_limit) = template.weekly_limit {
            let week_str = Self::get_week_string(date);

            // Get current usage for this week
            let usage = MealEntryRepository::get_weekly_usage(pool, meal_option_id, &week_str)
                .await
                .map_err(|_| ValidationError::WeeklyLimitExceeded {
                    item_name: option.name.clone(),
                    limit: weekly_limit,
                    current_usage: 0,
                })?;

            let current_count = usage.map(|u| u.usage_count).unwrap_or(0);

            // Check if adding one more would exceed the limit
            if current_count >= weekly_limit as i64 {
                return Err(ValidationError::WeeklyLimitExceeded {
                    item_name: option.name,
                    limit: weekly_limit,
                    current_usage: current_count,
                });
            }
        }

        Ok(())
    }

    /// Check tag weekly suggestions (returns warnings, not errors)
    /// Tag suggestions are soft limits that generate warnings but don't block
    pub async fn check_tag_suggestions(
        pool: &SqlitePool,
        meal_option_id: i64,
        date: NaiveDate,
    ) -> ValidationResult<Vec<ValidationWarning>> {
        let mut warnings = Vec::new();

        // Get option with tags
        let option_with_tags = MealOptionRepository::get_with_tags(pool, meal_option_id)
            .await
            .map_err(|_| ValidationError::WeeklyLimitExceeded {
                item_name: "Unknown".to_string(),
                limit: 0,
                current_usage: 0,
            })?
            .ok_or_else(|| ValidationError::WeeklyLimitExceeded {
                item_name: "Unknown".to_string(),
                limit: 0,
                current_usage: 0,
            })?;

        let week_str = Self::get_week_string(date);

        // Check each tag for weekly suggestions
        for tag_id in option_with_tags.tags {
            let tag = TagRepository::get_by_id(pool, tag_id)
                .await
                .map_err(|_| ValidationError::WeeklyLimitExceeded {
                    item_name: "Unknown".to_string(),
                    limit: 0,
                    current_usage: 0,
                })?
                .ok_or_else(|| ValidationError::WeeklyLimitExceeded {
                    item_name: "Unknown".to_string(),
                    limit: 0,
                    current_usage: 0,
                })?;

            if let Some(suggestion) = tag.weekly_suggestion {
                let usage = MealEntryRepository::get_weekly_tag_usage(pool, tag_id, &week_str)
                    .await
                    .map_err(|_| ValidationError::WeeklyLimitExceeded {
                        item_name: tag.name.clone(),
                        limit: suggestion,
                        current_usage: 0,
                    })?;

                let current_count = usage.map(|u| u.usage_count).unwrap_or(0);

                // If adding one more would exceed suggestion, create warning
                if current_count >= suggestion as i64 {
                    warnings.push(ValidationWarning {
                        message: format!(
                            "Tag '{}' suggestion exceeded: {}/{} uses this week",
                            tag.display_name, current_count, suggestion
                        ),
                        warning_type: WarningType::TagSuggestion,
                    });
                }
            }
        }

        Ok(warnings)
    }

    /// Comprehensive validation before creating a meal entry
    /// Returns Ok(warnings) if valid, Err if validation fails
    pub async fn validate_meal_entry(
        pool: &SqlitePool,
        meal_option_id: i64,
        slot: SlotType,
        date: NaiveDate,
    ) -> ValidationResult<Vec<ValidationWarning>> {
        // Get meal option and template
        let option = MealOptionRepository::get_by_id(pool, meal_option_id)
            .await
            .map_err(|_| ValidationError::WeeklyLimitExceeded {
                item_name: "Unknown".to_string(),
                limit: 0,
                current_usage: 0,
            })?
            .ok_or_else(|| ValidationError::WeeklyLimitExceeded {
                item_name: "Unknown".to_string(),
                limit: 0,
                current_usage: 0,
            })?;

        let template =
            crate::repository::MealTemplateRepository::get_by_id(pool, option.template_id)
                .await
                .map_err(|_| ValidationError::WeeklyLimitExceeded {
                    item_name: option.name.clone(),
                    limit: 0,
                    current_usage: 0,
                })?
                .ok_or_else(|| ValidationError::WeeklyLimitExceeded {
                    item_name: option.name.clone(),
                    limit: 0,
                    current_usage: 0,
                })?;

        // 1. Check slot compatibility (hard requirement)
        Self::validate_slot_compatibility(&template, slot)?;

        // 2. Check weekly limits (hard requirement)
        Self::check_weekly_limit(pool, meal_option_id, date).await?;

        // 3. Check tag suggestions (soft warnings)
        let warnings = Self::check_tag_suggestions(pool, meal_option_id, date).await?;

        Ok(warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        CreateMealEntry, CreateMealOption, CreateMealTemplate, CreateTag, LocationType, TagCategory,
    };
    use crate::repository::MealTemplateRepository;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create test pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    async fn create_test_template_with_limit(pool: &SqlitePool, weekly_limit: Option<i32>) -> i64 {
        let template = CreateMealTemplate {
            name: "Test Template".to_string(),
            description: None,
            location_type: LocationType::Home,
            compatible_slots: vec![SlotType::Breakfast, SlotType::Lunch],
            weekly_limit,
        };

        MealTemplateRepository::create(pool, template)
            .await
            .expect("Failed to create template")
            .id
    }

    async fn create_test_option(pool: &SqlitePool, template_id: i64) -> i64 {
        let option = CreateMealOption {
            template_id,
            name: "Test Option".to_string(),
            description: None,
            nutritional_notes: None,
        };

        MealOptionRepository::create(pool, option)
            .await
            .expect("Failed to create option")
            .id
    }

    async fn create_test_tag(pool: &SqlitePool, name: &str, suggestion: Option<i32>) -> i64 {
        let tag = CreateTag {
            name: name.to_string(),
            display_name: name.to_string(),
            category: TagCategory::Ingredient,
            parent_tag_id: None,
            weekly_suggestion: suggestion,
        };

        TagRepository::create(pool, tag)
            .await
            .expect("Failed to create tag")
            .id
    }

    #[test]
    fn test_get_week_string() {
        // Monday, November 4, 2024 is in ISO week 45
        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        assert_eq!(ValidationService::get_week_string(date), "2024-45");

        // Friday in the same week
        let date = NaiveDate::from_ymd_opt(2024, 11, 8).unwrap();
        assert_eq!(ValidationService::get_week_string(date), "2024-45");

        // Different week
        let date = NaiveDate::from_ymd_opt(2024, 11, 11).unwrap();
        assert_eq!(ValidationService::get_week_string(date), "2024-46");
    }

    #[test]
    fn test_get_week_start() {
        // Any day in week should return Monday
        let wednesday = NaiveDate::from_ymd_opt(2024, 11, 6).unwrap();
        let monday = ValidationService::get_week_start(wednesday);
        assert_eq!(monday, NaiveDate::from_ymd_opt(2024, 11, 4).unwrap());

        // Monday should return itself
        let monday = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let week_start = ValidationService::get_week_start(monday);
        assert_eq!(week_start, monday);

        // Sunday should return previous Monday
        let sunday = NaiveDate::from_ymd_opt(2024, 11, 10).unwrap();
        let monday = ValidationService::get_week_start(sunday);
        assert_eq!(monday, NaiveDate::from_ymd_opt(2024, 11, 4).unwrap());
    }

    #[tokio::test]
    async fn test_slot_compatibility() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template_with_limit(&pool, None).await;

        let template = MealTemplateRepository::get_by_id(&pool, template_id)
            .await
            .unwrap()
            .unwrap();

        // Compatible slots should pass
        assert!(
            ValidationService::validate_slot_compatibility(&template, SlotType::Breakfast).is_ok()
        );
        assert!(ValidationService::validate_slot_compatibility(&template, SlotType::Lunch).is_ok());

        // Incompatible slot should fail
        let result = ValidationService::validate_slot_compatibility(&template, SlotType::Dinner);
        assert!(result.is_err());

        if let Err(ValidationError::IncompatibleSlot {
            option_name, slot, ..
        }) = result
        {
            assert_eq!(option_name, "Test Template");
            assert_eq!(slot, SlotType::Dinner);
        }
    }

    #[tokio::test]
    async fn test_weekly_limit_not_exceeded() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template_with_limit(&pool, Some(3)).await;
        let option_id = create_test_option(&pool, template_id).await;

        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();

        // Create one entry
        let entry = CreateMealEntry {
            meal_option_id: option_id,
            date,
            slot_type: SlotType::Breakfast,
            location: LocationType::Home,
            servings: None,
            notes: None,
            completed: Some(true),
        };
        MealEntryRepository::create(&pool, entry).await.unwrap();

        // Should still be under limit (1/3)
        let result = ValidationService::check_weekly_limit(&pool, option_id, date).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_weekly_limit_exceeded() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template_with_limit(&pool, Some(2)).await;
        let option_id = create_test_option(&pool, template_id).await;

        let monday = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();
        let wednesday = NaiveDate::from_ymd_opt(2024, 11, 6).unwrap();

        // Create 2 entries (at the limit)
        for date in &[monday, tuesday] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: *date,
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: Some(true),
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        // Trying to add a third should fail
        let result = ValidationService::check_weekly_limit(&pool, option_id, wednesday).await;
        assert!(result.is_err());

        if let Err(ValidationError::WeeklyLimitExceeded {
            item_name,
            limit,
            current_usage,
        }) = result
        {
            assert_eq!(item_name, "Test Option");
            assert_eq!(limit, 2);
            assert_eq!(current_usage, 2);
        }
    }

    #[tokio::test]
    async fn test_no_weekly_limit() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template_with_limit(&pool, None).await;
        let option_id = create_test_option(&pool, template_id).await;

        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();

        // Create multiple entries
        for day in 0..5 {
            let entry_date = date + chrono::Duration::days(day);
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: entry_date,
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: Some(true),
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        // Should always pass when no limit
        let result = ValidationService::check_weekly_limit(&pool, option_id, date).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tag_suggestions() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template_with_limit(&pool, None).await;
        let option_id = create_test_option(&pool, template_id).await;
        let tag_id = create_test_tag(&pool, "pasta", Some(2)).await;

        // Add tag to option
        MealOptionRepository::add_tags(&pool, option_id, vec![tag_id])
            .await
            .unwrap();

        let monday = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2024, 11, 5).unwrap();
        let wednesday = NaiveDate::from_ymd_opt(2024, 11, 6).unwrap();

        // Create 2 entries (at suggestion)
        for date in &[monday, tuesday] {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date: *date,
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: Some(true),
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        // Check for warnings
        let warnings = ValidationService::check_tag_suggestions(&pool, option_id, wednesday)
            .await
            .unwrap();

        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].message.contains("pasta"));
        assert_eq!(warnings[0].warning_type, WarningType::TagSuggestion);
    }

    #[tokio::test]
    async fn test_comprehensive_validation() {
        let pool = setup_test_pool().await;
        let template_id = create_test_template_with_limit(&pool, Some(2)).await;
        let option_id = create_test_option(&pool, template_id).await;

        let date = NaiveDate::from_ymd_opt(2024, 11, 4).unwrap();

        // Valid: Compatible slot, within limit
        let result =
            ValidationService::validate_meal_entry(&pool, option_id, SlotType::Breakfast, date)
                .await;
        assert!(result.is_ok());

        // Invalid: Incompatible slot
        let result =
            ValidationService::validate_meal_entry(&pool, option_id, SlotType::Dinner, date).await;
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ValidationError::IncompatibleSlot { .. })
        ));

        // Create entries to hit the limit
        for _ in 0..2 {
            let entry = CreateMealEntry {
                meal_option_id: option_id,
                date,
                slot_type: SlotType::Breakfast,
                location: LocationType::Home,
                servings: None,
                notes: None,
                completed: Some(true),
            };
            MealEntryRepository::create(&pool, entry).await.unwrap();
        }

        // Invalid: Weekly limit exceeded
        let result =
            ValidationService::validate_meal_entry(&pool, option_id, SlotType::Breakfast, date)
                .await;
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(ValidationError::WeeklyLimitExceeded { .. })
        ));
    }
}
