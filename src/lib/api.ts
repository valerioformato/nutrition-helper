// Tauri API wrapper functions
// Typed functions that call Tauri commands

import { invoke } from "@tauri-apps/api/core";
import type {
    CreateMealEntry,
    CreateMealOption,
    CreateMealTemplate,
    CreateTag,
    LocationType,
    MealEntry,
    MealOption,
    MealOptionWithTags,
    MealTemplate,
    SlotType,
    Tag,
    TagCategory,
    UpdateMealEntry,
    UpdateMealOption,
    UpdateMealTemplate,
    UpdateTag,
    WeeklyTagUsage,
    WeeklyUsage,
} from "./types";
import { isApiError } from "./types";

// ============================================================================
// TAG API
// ============================================================================

/**
 * Get all tags from the database
 */
export async function getAllTags(): Promise<Tag[]> {
  const result = await invoke<Tag[]>("get_all_tags");
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get a tag by its ID
 */
export async function getTagById(id: number): Promise<Tag | null> {
  const result = await invoke<Tag | null>("get_tag_by_id", { id });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get a tag by its name (internal key)
 */
export async function getTagByName(name: string): Promise<Tag | null> {
  const result = await invoke<Tag | null>("get_tag_by_name", { name });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get all tags in a specific category
 */
export async function getTagsByCategory(category: TagCategory): Promise<Tag[]> {
  const result = await invoke<Tag[]>("get_tags_by_category", { category });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get all child tags for a parent tag
 */
export async function getTagChildren(parentId: number): Promise<Tag[]> {
  const result = await invoke<Tag[]>("get_tag_children", { parentId });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Create a new tag
 */
export async function createTag(tag: CreateTag): Promise<Tag> {
  const result = await invoke<Tag>("create_tag", { tag });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Update an existing tag
 */
export async function updateTag(id: number, tag: UpdateTag): Promise<Tag> {
  const result = await invoke<Tag>("update_tag", { id, tag });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Delete a tag by ID
 * @returns true if deleted, false if not found
 */
export async function deleteTag(id: number): Promise<boolean> {
  const result = await invoke<boolean>("delete_tag", { id });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

// ============================================================================
// MEAL TEMPLATE API
// ============================================================================

/**
 * Get all meal templates
 */
export async function getAllTemplates(): Promise<MealTemplate[]> {
  const result = await invoke<MealTemplate[]>("get_all_templates");
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get a meal template by its ID
 */
export async function getTemplateById(
  id: number
): Promise<MealTemplate | null> {
  const result = await invoke<MealTemplate | null>("get_template_by_id", {
    id,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get templates compatible with a specific location
 */
export async function getTemplatesByLocation(
  location: LocationType
): Promise<MealTemplate[]> {
  const result = await invoke<MealTemplate[]>("get_templates_by_location", {
    location,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get templates compatible with a specific slot
 */
export async function getTemplatesBySlot(
  slot: SlotType
): Promise<MealTemplate[]> {
  const result = await invoke<MealTemplate[]>("get_templates_by_slot", {
    slot,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Search templates by name or description
 */
export async function searchTemplates(query: string): Promise<MealTemplate[]> {
  const result = await invoke<MealTemplate[]>("search_templates", { query });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Create a new meal template
 */
export async function createTemplate(
  template: CreateMealTemplate
): Promise<MealTemplate> {
  const result = await invoke<MealTemplate>("create_template", { template });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Update an existing meal template
 */
export async function updateTemplate(
  id: number,
  template: UpdateMealTemplate
): Promise<MealTemplate> {
  const result = await invoke<MealTemplate>("update_template", {
    id,
    updates: template,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Delete a meal template
 * @returns true if deleted, false if not found
 */
export async function deleteTemplate(id: number): Promise<boolean> {
  const result = await invoke<boolean>("delete_template", { id });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

// ============================================================================
// MEAL OPTION API
// ============================================================================

/**
 * Get all meal options
 */
export async function getAllOptions(): Promise<MealOption[]> {
  const result = await invoke<MealOption[]>("get_all_options");
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get a meal option by its ID
 */
export async function getOptionById(id: number): Promise<MealOption | null> {
  const result = await invoke<MealOption | null>("get_option_by_id", { id });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get a meal option with its associated tags
 */
export async function getOptionWithTags(
  id: number
): Promise<MealOptionWithTags | null> {
  const result = await invoke<MealOptionWithTags | null>("get_option_with_tags", {
    id,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get all options for a specific template
 */
export async function getOptionsByTemplate(
  templateId: number
): Promise<MealOption[]> {
  const result = await invoke<MealOption[]>("get_options_by_template", {
    templateId,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get all options for a template with their tags
 */
export async function getOptionsByTemplateWithTags(
  templateId: number
): Promise<MealOptionWithTags[]> {
  const result = await invoke<MealOptionWithTags[]>(
    "get_options_by_template_with_tags",
    { templateId }
  );
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Search options by name or description
 */
export async function searchOptions(query: string): Promise<MealOption[]> {
  const result = await invoke<MealOption[]>("search_options", { query });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Create a new meal option
 */
export async function createOption(
  option: CreateMealOption
): Promise<MealOption> {
  const result = await invoke<MealOption>("create_option", { option });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Update an existing meal option
 */
export async function updateOption(
  id: number,
  option: UpdateMealOption
): Promise<MealOption> {
  const result = await invoke<MealOption>("update_option", { id, updates: option });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Delete a meal option
 */
export async function deleteOption(id: number): Promise<void> {
  const result = await invoke<void>("delete_option", { id });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
}

/**
 * Add tags to a meal option
 */
export async function addTagsToOption(
  optionId: number,
  tagIds: number[]
): Promise<void> {
  const result = await invoke<void>("add_tags_to_option", {
    optionId,
    tagIds,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
}

/**
 * Remove tags from a meal option
 */
export async function removeTagsFromOption(
  optionId: number,
  tagIds: number[]
): Promise<void> {
  const result = await invoke<void>("remove_tags_from_option", {
    optionId,
    tagIds,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
}

/**
 * Replace all tags for a meal option
 */
export async function setOptionTags(
  optionId: number,
  tagIds: number[]
): Promise<void> {
  const result = await invoke<void>("set_option_tags", { optionId, tagIds });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
}

// ============================================================================
// MEAL ENTRY API
// ============================================================================

/**
 * Get a meal entry by its ID
 */
export async function getEntryById(id: number): Promise<MealEntry | null> {
  const result = await invoke<MealEntry | null>("get_entry_by_id", { id });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get all entries for a specific date
 */
export async function getEntriesByDate(date: string): Promise<MealEntry[]> {
  const result = await invoke<MealEntry[]>("get_entries_by_date", { date });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get all entries within a date range
 */
export async function getEntriesByDateRange(
  startDate: string,
  endDate: string
): Promise<MealEntry[]> {
  const result = await invoke<MealEntry[]>("get_entries_by_date_range", {
    startDate,
    endDate,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get entry for a specific date and slot
 */
export async function getEntryByDateAndSlot(
  date: string,
  slotType: SlotType
): Promise<MealEntry | null> {
  const result = await invoke<MealEntry | null>("get_entry_by_date_and_slot", {
    date,
    slotType,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get entries by completion status
 */
export async function getEntriesByCompleted(
  completed: boolean
): Promise<MealEntry[]> {
  const result = await invoke<MealEntry[]>("get_entries_by_completed", {
    completed,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get all entries for a specific meal option
 */
export async function getEntriesByMealOption(
  mealOptionId: number
): Promise<MealEntry[]> {
  const result = await invoke<MealEntry[]>("get_entries_by_meal_option", {
    mealOptionId,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get recently used meal entries (for quick reselection)
 */
export async function getRecentEntries(limit: number): Promise<MealEntry[]> {
  const result = await invoke<MealEntry[]>("get_recent_entries", { limit });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get weekly usage statistics for a meal option
 */
export async function getWeeklyUsage(
  mealOptionId: number,
  week: string
): Promise<WeeklyUsage | null> {
  const result = await invoke<WeeklyUsage | null>("get_weekly_usage", {
    mealOptionId,
    week,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Get weekly usage statistics for all tags
 */
export async function getWeeklyTagUsage(
  week: string
): Promise<WeeklyTagUsage[]> {
  const result = await invoke<WeeklyTagUsage[]>("get_weekly_tag_usage", {
    week,
  });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Create a new meal entry
 */
export async function createEntry(entry: CreateMealEntry): Promise<MealEntry> {
  const result = await invoke<MealEntry>("create_entry", { entry });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Update an existing meal entry
 */
export async function updateEntry(
  id: number,
  entry: UpdateMealEntry
): Promise<MealEntry> {
  const result = await invoke<MealEntry>("update_entry", { id, entry });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}

/**
 * Delete a meal entry
 */
export async function deleteEntry(id: number): Promise<void> {
  const result = await invoke<void>("delete_entry", { id });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
}

/**
 * Validate a meal entry against business rules
 * @returns validation warnings/errors
 */
export async function validateEntry(
  entry: CreateMealEntry
): Promise<string[]> {
  const result = await invoke<string[]>("validate_entry", { entry });
  if (isApiError(result)) {
    throw new Error(result.message);
  }
  return result;
}


