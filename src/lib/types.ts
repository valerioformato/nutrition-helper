// TypeScript type definitions
// Types matching Rust models for type-safe IPC

// ============================================================================
// ENUMS
// ============================================================================

/**
 * The five fixed meal slots per day
 * Matches Rust: SlotType
 */
export enum SlotType {
  Breakfast = "breakfast",
  MorningSnack = "morning_snack",
  Lunch = "lunch",
  AfternoonSnack = "afternoon_snack",
  Dinner = "dinner",
}

/**
 * Where a meal can be prepared/consumed
 * Matches Rust: LocationType
 */
export enum LocationType {
  Home = "home",
  Office = "office",
  Restaurant = "restaurant",
  Any = "any",
}

/**
 * Category for tags (ingredient tracking, dietary restrictions, etc.)
 * Matches Rust: TagCategory
 */
export enum TagCategory {
  Ingredient = "ingredient",
  Dietary = "dietary",
  PrepTime = "prep_time",
  Other = "other",
}

// ============================================================================
// DOMAIN MODELS
// ============================================================================

/**
 * Tag for tracking ingredients, dietary restrictions, and frequency suggestions
 * Matches Rust: Tag
 */
export interface Tag {
  id: number;
  name: string; // Internal key: "pasta", "ricotta"
  display_name: string; // User-facing: "Pasta", "Ricotta"
  category: TagCategory;
  weekly_suggestion: number | null; // Soft limit (e.g., 3 for "max 3x/week")
  parent_tag_id: number | null; // For hierarchies: pasta_integrale -> pasta
  created_at: string; // ISO 8601 datetime string
}

/**
 * Meal Template - The "cards" that fill slots (the "Oppure" choices)
 * Example: "Pane con marmellata e formaggio spalmabile"
 * Matches Rust: MealTemplate
 */
export interface MealTemplate {
  id: number;
  name: string;
  description: string | null;
  compatible_slots: SlotType[]; // Which slots can this template fill
  location_type: LocationType; // Where this meal can be prepared
  weekly_limit: number | null; // Hard limit: max times per week (null = unlimited)
  created_at: string; // ISO 8601 datetime string
  updated_at: string; // ISO 8601 datetime string
}

/**
 * Meal Option - Ingredient/variation choices within a template
 * Example: "philadelphia", "ricotta", "crema spalmabile 100% frutta secca"
 * Matches Rust: MealOption
 */
export interface MealOption {
  id: number;
  template_id: number;
  name: string;
  description: string | null;
  nutritional_notes: string | null;
  created_at: string; // ISO 8601 datetime string
  updated_at: string; // ISO 8601 datetime string
}

/**
 * Meal option with its associated tags
 * Matches Rust: MealOptionWithTags
 */
export interface MealOptionWithTags {
  id: number;
  template_id: number;
  name: string;
  description: string | null;
  nutritional_notes: string | null;
  created_at: string;
  updated_at: string;
  tags: number[]; // Tag IDs associated with this option
}

/**
 * Meal Entry - Actual meal logging and planning
 * Tracks both planned meals (future) and logged meals (past/completed)
 * Matches Rust: MealEntry
 */
export interface MealEntry {
  id: number;
  meal_option_id: number;
  date: string; // ISO 8601 date string (YYYY-MM-DD)
  slot_type: SlotType;
  location: LocationType;
  servings: number; // Default 1.0, nutrition plan uses strict serving sizes
  notes: string | null;
  completed: boolean; // false = planned, true = consumed
  created_at: string; // ISO 8601 datetime string
  updated_at: string; // ISO 8601 datetime string
}

// ============================================================================
// CREATE/UPDATE TYPES
// ============================================================================

/**
 * Input for creating a new tag
 * Matches Rust: CreateTag
 */
export interface CreateTag {
  name: string;
  display_name: string;
  category: TagCategory;
  weekly_suggestion?: number | null;
  parent_tag_id?: number | null;
}

/**
 * Input for updating an existing tag
 * Matches Rust: UpdateTag
 * Note: Optional<Option<T>> in Rust maps to T | null | undefined in TypeScript
 * - undefined = no change
 * - null = clear value
 * - value = set to value
 */
export interface UpdateTag {
  display_name?: string;
  category?: TagCategory;
  weekly_suggestion?: number | null;
  parent_tag_id?: number | null;
}

/**
 * Input for creating a new meal template
 * Matches Rust: CreateMealTemplate
 */
export interface CreateMealTemplate {
  name: string;
  description?: string | null;
  compatible_slots: SlotType[];
  location_type: LocationType;
  weekly_limit?: number | null;
}

/**
 * Input for updating an existing meal template
 * Matches Rust: UpdateMealTemplate
 */
export interface UpdateMealTemplate {
  name?: string;
  description?: string | null;
  compatible_slots?: SlotType[];
  location_type?: LocationType;
  weekly_limit?: number | null;
}

/**
 * Input for creating a new meal option
 * Matches Rust: CreateMealOption
 */
export interface CreateMealOption {
  template_id: number;
  name: string;
  description?: string | null;
  nutritional_notes?: string | null;
}

/**
 * Input for updating an existing meal option
 * Matches Rust: UpdateMealOption
 */
export interface UpdateMealOption {
  name?: string;
  description?: string | null;
  nutritional_notes?: string | null;
}

/**
 * Input for creating a new meal entry
 * Matches Rust: CreateMealEntry
 */
export interface CreateMealEntry {
  meal_option_id: number;
  date: string; // ISO 8601 date string (YYYY-MM-DD)
  slot_type: SlotType;
  location: LocationType;
  servings?: number; // Defaults to 1.0 if not provided
  notes?: string | null;
  completed?: boolean; // Defaults to false (planned)
}

/**
 * Input for updating an existing meal entry
 * Matches Rust: UpdateMealEntry
 */
export interface UpdateMealEntry {
  location?: LocationType;
  servings?: number;
  notes?: string | null;
  completed?: boolean;
}

// ============================================================================
// WEEKLY USAGE TRACKING
// ============================================================================

/**
 * Helper struct for weekly usage tracking
 * Matches Rust: WeeklyUsage
 */
export interface WeeklyUsage {
  meal_option_id: number;
  week: string; // Format: "YYYY-WW" (ISO week)
  usage_count: number;
}

/**
 * Helper struct for weekly tag usage tracking
 * Matches Rust: WeeklyTagUsage
 */
export interface WeeklyTagUsage {
  tag_id: number;
  tag_name: string;
  week: string; // Format: "YYYY-WW" (ISO week)
  usage_count: number;
}

// ============================================================================
// API ERROR TYPES
// ============================================================================

/**
 * API error response from Tauri commands
 * Matches the error structure returned by commands
 */
export interface ApiError {
  message: string;
  code?: string;
}

/**
 * Result type for API calls that may fail
 */
export type ApiResult<T> = T | ApiError;

/**
 * Type guard to check if a result is an error
 */
export function isApiError(result: any): result is ApiError {
  return result && typeof result === "object" && "message" in result;
}

