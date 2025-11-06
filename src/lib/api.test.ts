import { invoke } from "@tauri-apps/api/core";
import { beforeEach, describe, expect, it, vi } from "vitest";
import * as api from "./api";
import {
    LocationType,
    SlotType,
    TagCategory,
    type MealEntry,
    type MealOption,
    type MealTemplate,
    type Tag,
} from "./types";

// Mock the Tauri invoke function
vi.mock("@tauri-apps/api/core");

describe("Tag API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should get all tags", async () => {
    const mockTags: Tag[] = [
      {
        id: 1,
        name: "pasta",
        display_name: "Pasta",
        category: TagCategory.Ingredient,
        weekly_suggestion: 3,
        parent_tag_id: null,
        created_at: "2024-01-01T00:00:00Z",
      },
    ];

    vi.mocked(invoke).mockResolvedValue(mockTags);

    const result = await api.getAllTags();

    expect(invoke).toHaveBeenCalledWith("get_all_tags");
    expect(result).toEqual(mockTags);
  });

  it("should get tag by id", async () => {
    const mockTag: Tag = {
      id: 1,
      name: "pasta",
      display_name: "Pasta",
      category: TagCategory.Ingredient,
      weekly_suggestion: 3,
      parent_tag_id: null,
      created_at: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValue(mockTag);

    const result = await api.getTagById(1);

    expect(invoke).toHaveBeenCalledWith("get_tag_by_id", { id: 1 });
    expect(result).toEqual(mockTag);
  });

  it("should create a tag", async () => {
    const newTag = {
      name: "pasta",
      display_name: "Pasta",
      category: TagCategory.Ingredient,
      weekly_suggestion: 3,
    };

    const createdTag: Tag = {
      id: 1,
      ...newTag,
      parent_tag_id: null,
      created_at: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValue(createdTag);

    const result = await api.createTag(newTag);

    expect(invoke).toHaveBeenCalledWith("create_tag", { tag: newTag });
    expect(result).toEqual(createdTag);
  });

  it("should handle API errors", async () => {
    const apiError = { message: "Tag not found" };
    vi.mocked(invoke).mockResolvedValue(apiError);

    await expect(api.getTagById(999)).rejects.toThrow("Tag not found");
  });
});

describe("Template API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should get all templates", async () => {
    const mockTemplates: MealTemplate[] = [
      {
        id: 1,
        name: "Breakfast Template",
        description: "A healthy breakfast",
        compatible_slots: [SlotType.Breakfast],
        location_type: LocationType.Home,
        weekly_limit: null,
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
      },
    ];

    vi.mocked(invoke).mockResolvedValue(mockTemplates);

    const result = await api.getAllTemplates();

    expect(invoke).toHaveBeenCalledWith("get_all_templates");
    expect(result).toEqual(mockTemplates);
  });

  it("should search templates", async () => {
    const mockTemplates: MealTemplate[] = [
      {
        id: 1,
        name: "Pasta Template",
        description: "Italian pasta dishes",
        compatible_slots: [SlotType.Lunch, SlotType.Dinner],
        location_type: LocationType.Any,
        weekly_limit: 2,
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
      },
    ];

    vi.mocked(invoke).mockResolvedValue(mockTemplates);

    const result = await api.searchTemplates("pasta");

    expect(invoke).toHaveBeenCalledWith("search_templates", { query: "pasta" });
    expect(result).toEqual(mockTemplates);
  });

  it("should get templates by location", async () => {
    const mockTemplates: MealTemplate[] = [];
    vi.mocked(invoke).mockResolvedValue(mockTemplates);

    const result = await api.getTemplatesByLocation(LocationType.Office);

    expect(invoke).toHaveBeenCalledWith("get_templates_by_location", {
      location: LocationType.Office,
    });
    expect(result).toEqual(mockTemplates);
  });

  it("should get templates by slot", async () => {
    const mockTemplates: MealTemplate[] = [];
    vi.mocked(invoke).mockResolvedValue(mockTemplates);

    const result = await api.getTemplatesBySlot(SlotType.Breakfast);

    expect(invoke).toHaveBeenCalledWith("get_templates_by_slot", {
      slot: SlotType.Breakfast,
    });
    expect(result).toEqual(mockTemplates);
  });
});

describe("Option API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should get options by template", async () => {
    const mockOptions: MealOption[] = [
      {
        id: 1,
        template_id: 5,
        name: "Ricotta",
        description: "Fresh ricotta cheese",
        nutritional_notes: null,
        created_at: "2024-01-01T00:00:00Z",
        updated_at: "2024-01-01T00:00:00Z",
      },
    ];

    vi.mocked(invoke).mockResolvedValue(mockOptions);

    const result = await api.getOptionsByTemplate(5);

    expect(invoke).toHaveBeenCalledWith("get_options_by_template", {
      templateId: 5,
    });
    expect(result).toEqual(mockOptions);
  });

  it("should create an option", async () => {
    const newOption = {
      template_id: 5,
      name: "Philadelphia",
      description: "Cream cheese",
      nutritional_notes: null,
    };

    const createdOption: MealOption = {
      id: 1,
      ...newOption,
      created_at: "2024-01-01T00:00:00Z",
      updated_at: "2024-01-01T00:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValue(createdOption);

    const result = await api.createOption(newOption);

    expect(invoke).toHaveBeenCalledWith("create_option", { option: newOption });
    expect(result).toEqual(createdOption);
  });

  it("should add tags to option", async () => {
    vi.mocked(invoke).mockResolvedValue(undefined);

    await api.addTagsToOption(1, [10, 20, 30]);

    expect(invoke).toHaveBeenCalledWith("add_tags_to_option", {
      optionId: 1,
      tagIds: [10, 20, 30],
    });
  });

  it("should set option tags", async () => {
    vi.mocked(invoke).mockResolvedValue(undefined);

    await api.setOptionTags(1, [5, 10]);

    expect(invoke).toHaveBeenCalledWith("set_option_tags", {
      optionId: 1,
      tagIds: [5, 10],
    });
  });
});

describe("Entry API", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should get entries by date", async () => {
    const mockEntries: MealEntry[] = [
      {
        id: 1,
        meal_option_id: 10,
        date: "2024-01-15",
        slot_type: SlotType.Breakfast,
        location: LocationType.Home,
        servings: 1.0,
        notes: null,
        completed: true,
        created_at: "2024-01-15T08:00:00Z",
        updated_at: "2024-01-15T08:00:00Z",
      },
    ];

    vi.mocked(invoke).mockResolvedValue(mockEntries);

    const result = await api.getEntriesByDate("2024-01-15");

    expect(invoke).toHaveBeenCalledWith("get_entries_by_date", {
      date: "2024-01-15",
    });
    expect(result).toEqual(mockEntries);
  });

  it("should get entries by date range", async () => {
    const mockEntries: MealEntry[] = [];
    vi.mocked(invoke).mockResolvedValue(mockEntries);

    const result = await api.getEntriesByDateRange("2024-01-01", "2024-01-07");

    expect(invoke).toHaveBeenCalledWith("get_entries_by_date_range", {
      startDate: "2024-01-01",
      endDate: "2024-01-07",
    });
    expect(result).toEqual(mockEntries);
  });

  it("should create an entry", async () => {
    const newEntry = {
      meal_option_id: 10,
      date: "2024-01-15",
      slot_type: SlotType.Lunch,
      location: LocationType.Office,
      servings: 1.5,
      notes: "Extra serving",
      completed: false,
    };

    const createdEntry: MealEntry = {
      id: 1,
      ...newEntry,
      created_at: "2024-01-15T12:00:00Z",
      updated_at: "2024-01-15T12:00:00Z",
    };

    vi.mocked(invoke).mockResolvedValue(createdEntry);

    const result = await api.createEntry(newEntry);

    expect(invoke).toHaveBeenCalledWith("create_entry", { entry: newEntry });
    expect(result).toEqual(createdEntry);
  });

  it("should validate an entry", async () => {
    const entryToValidate = {
      meal_option_id: 10,
      date: "2024-01-15",
      slot_type: SlotType.Lunch,
      location: LocationType.Office,
    };

    const warnings = ["Template limit reached for this week"];
    vi.mocked(invoke).mockResolvedValue(warnings);

    const result = await api.validateEntry(entryToValidate);

    expect(invoke).toHaveBeenCalledWith("validate_entry", {
      entry: entryToValidate,
    });
    expect(result).toEqual(warnings);
  });

  it("should get weekly usage", async () => {
    const mockUsage = {
      meal_option_id: 10,
      week: "2024-W03",
      usage_count: 2,
    };

    vi.mocked(invoke).mockResolvedValue(mockUsage);

    const result = await api.getWeeklyUsage(10, "2024-W03");

    expect(invoke).toHaveBeenCalledWith("get_weekly_usage", {
      mealOptionId: 10,
      week: "2024-W03",
    });
    expect(result).toEqual(mockUsage);
  });
});
