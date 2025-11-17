import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import {
    getAllTags,
    getAllTemplates,
    getOptionById,
    getRecentEntries,
    getTemplateById,
    getWeeklyUsage
} from "../../lib/api";
import {
    MealEntry,
    MealOptionWithTags,
    MealTemplate,
    SlotType,
    Tag
} from "../../lib/types";
import { Modal } from "../common/Modal";

interface MealSelectionModalProps {
  isOpen: boolean;
  onClose: () => void;
  slotType: SlotType;
  slotName: string;
  onSelectTemplate: (template: MealTemplate) => void;
}

// Helper function to get current week string (YYYY-WW format)
// Must match SQLite's migration: shifts date back 1 day then applies %W
// This makes Monday the first day of the week (Mon-Sun weeks)
function getCurrentWeek(): string {
  const now = new Date();
  
  // Shift back 1 day to convert Mon-Sun weeks to SQLite's Sun-Sat %W format
  const shiftedDate = new Date(now.getTime() - 24 * 60 * 60 * 1000);
  const year = shiftedDate.getFullYear();
  
  // Get January 1st of the shifted year
  const jan1 = new Date(year, 0, 1);
  
  // Calculate days since January 1st
  const dayOfYear = Math.floor((shiftedDate.getTime() - jan1.getTime()) / (24 * 60 * 60 * 1000));
  
  // SQLite %W algorithm: (day_of_year + day_of_week_of_jan1) / 7
  const jan1Day = jan1.getDay();
  const weekNumber = Math.floor((dayOfYear + jan1Day) / 7);
  
  return `${year}-${weekNumber.toString().padStart(2, '0')}`;
}

export function MealSelectionModal({
  isOpen,
  onClose,
  slotType,
  slotName,
  onSelectTemplate,
}: MealSelectionModalProps) {
  const [templates, setTemplates] = useState<MealTemplate[]>([]);
  const [filteredTemplates, setFilteredTemplates] = useState<MealTemplate[]>(
    []
  );
  const [tags, setTags] = useState<Tag[]>([]);
  const [options, setOptions] = useState<MealOptionWithTags[]>([]);
  const [weeklyUsage, setWeeklyUsage] = useState<Map<number, number>>(new Map());
  const [recentTemplates, setRecentTemplates] = useState<MealTemplate[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedTags, setSelectedTags] = useState<Set<number>>(new Set());
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Fetch templates, tags, and options when modal opens
  useEffect(() => {
    if (isOpen) {
      loadData();
    }
  }, [isOpen]);

  // Filter templates based on search query, slot compatibility, and selected tags
  useEffect(() => {
    let filtered = templates.filter((template) =>
      template.compatible_slots.includes(slotType)
    );

    // Filter by search query
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        (template) =>
          template.name.toLowerCase().includes(query) ||
          (template.description &&
            template.description.toLowerCase().includes(query))
      );
    }

    // Filter by selected tags (if any tags are selected)
    if (selectedTags.size > 0) {
      filtered = filtered.filter((template) => {
        // Get all options for this template
        const templateOptions = options.filter(
          (opt) => opt.template_id === template.id
        );

        // Check if any option has at least one of the selected tags
        return templateOptions.some((option) =>
          option.tags.some((tagId: number) => selectedTags.has(tagId))
        );
      });
    }

    setFilteredTemplates(filtered);
  }, [templates, searchQuery, slotType, selectedTags, options]);

  const loadData = async () => {
    setLoading(true);
    setError(null);
    try {
      const [allTemplates, allTags, recentEntries] = await Promise.all([
        getAllTemplates(),
        getAllTags(),
        getRecentEntries(6), // Get last 6 unique meals
      ]);
      
      // Fetch templates for recent entries
      const recentTemplateIds = new Set<number>();
      const recentTemplatesPromises = recentEntries.map(async (entry: MealEntry) => {
        try {
          const option = await getOptionById(entry.meal_option_id);
          if (option && !recentTemplateIds.has(option.template_id)) {
            recentTemplateIds.add(option.template_id);
            return await getTemplateById(option.template_id);
          }
          return null;
        } catch {
          return null;
        }
      });
      
      const recentTemplatesResults = await Promise.all(recentTemplatesPromises);
      const uniqueRecentTemplates = recentTemplatesResults
        .filter((t): t is MealTemplate => t !== null)
        .filter((t) => t.compatible_slots.includes(slotType)); // Filter by slot compatibility
      
      // Fetch all options with tags for each template
      const optionsPromises = allTemplates.map(async (template) => {
        const result = await invoke<MealOptionWithTags[]>(
          "get_options_by_template_with_tags",
          { templateId: template.id }
        );
        return result;
      });
      
      const optionsArrays = await Promise.all(optionsPromises);
      const allOptions = optionsArrays.flat();
      
      // Fetch weekly usage for all options
      const currentWeek = getCurrentWeek();
      const usagePromises = allOptions.map(async (option) => {
        try {
          const usage = await getWeeklyUsage(option.id, currentWeek);
          return { optionId: option.id, count: usage?.usage_count || 0 };
        } catch {
          return { optionId: option.id, count: 0 };
        }
      });
      
      const usageResults = await Promise.all(usagePromises);
      const usageMap = new Map<number, number>();
      usageResults.forEach(({ optionId, count }) => {
        usageMap.set(optionId, count);
      });
      
      setTemplates(allTemplates);
      setTags(allTags);
      setOptions(allOptions);
      setWeeklyUsage(usageMap);
      setRecentTemplates(uniqueRecentTemplates);
    } catch (err) {
      console.error("Failed to load data:", err);
      setError("Failed to load meal data. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  const toggleTag = (tagId: number) => {
    setSelectedTags((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(tagId)) {
        newSet.delete(tagId);
      } else {
        newSet.add(tagId);
      }
      return newSet;
    });
  };

  // Get total weekly usage for a template (sum usage across all its options)
  const getTemplateUsage = (template: MealTemplate): number => {
    const templateOptions = options.filter((opt) => opt.template_id === template.id);
    if (templateOptions.length === 0) return 0;
    
    // Sum the usage count across all options in this template
    return templateOptions.reduce((sum, opt) => sum + (weeklyUsage.get(opt.id) || 0), 0);
  };

  const handleSelectTemplate = (template: MealTemplate) => {
    onSelectTemplate(template);
    // Don't call onClose() - let the parent (wizard or direct usage) handle the flow
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={`Add Meal to ${slotName}`}>
      {/* Search Bar */}
      <div className="mb-4">
        <input
          type="text"
          placeholder="Search templates..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Tag Filters */}
      {tags.length > 0 && (
        <div className="mb-4">
          <div className="text-sm font-medium text-gray-700 mb-2">
            Filter by tags:
          </div>
          <div className="flex flex-wrap gap-2">
            {tags.map((tag) => (
              <button
                key={tag.id}
                onClick={() => toggleTag(tag.id)}
                className={`px-3 py-1.5 rounded-full text-sm font-medium transition-colors ${
                  selectedTags.has(tag.id)
                    ? "bg-blue-500 text-white hover:bg-blue-600"
                    : "bg-gray-100 text-gray-700 hover:bg-gray-200"
                }`}
              >
                {tag.display_name}
              </button>
            ))}
          </div>
          {selectedTags.size > 0 && (
            <button
              onClick={() => setSelectedTags(new Set())}
              className="mt-2 text-sm text-blue-600 hover:text-blue-800 underline"
            >
              Clear all filters
            </button>
          )}
        </div>
      )}

      {/* Loading State */}
      {loading && (
        <div className="flex items-center justify-center py-8">
          <div className="text-gray-500">Loading templates...</div>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-4">
          <p className="text-red-800">{error}</p>
          <button
            onClick={loadData}
            className="mt-2 text-sm text-red-600 hover:text-red-800 underline"
          >
            Try again
          </button>
        </div>
      )}

      {/* Recently Used Section */}
      {!loading && !error && recentTemplates.length > 0 && !searchQuery && selectedTags.size === 0 && (
        <div className="mb-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-3 flex items-center gap-2">
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" />
            </svg>
            Recently Used
          </h3>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
            {recentTemplates.map((template) => {
              const usage = getTemplateUsage(template);
              const isExhausted = !!(template.weekly_limit && usage >= template.weekly_limit);
              
              return (
                <button
                  key={template.id}
                  onClick={() => !isExhausted && handleSelectTemplate(template)}
                  disabled={isExhausted}
                  className={`relative flex flex-col items-start p-3 rounded-lg border-2 transition-all text-left
                    ${isExhausted 
                      ? "bg-gray-100 border-gray-300 opacity-60 cursor-not-allowed"
                      : "bg-white border-gray-200 hover:border-blue-400 hover:shadow-md"
                    }`}
                >
                  {isExhausted && (
                    <div className="absolute top-2 right-2 bg-red-500 text-white text-xs px-2 py-1 rounded font-bold">
                      LIMIT REACHED
                    </div>
                  )}
                  
                  <div className="w-full">
                    <h4 className="font-semibold text-gray-900 text-sm mb-1">{template.name}</h4>
                    
                    <div className="flex flex-wrap items-center gap-2 text-xs">
                      <span className="px-2 py-0.5 bg-blue-100 text-blue-800 rounded-full">
                        {template.location_type === "home" && "🏠 Home"}
                        {template.location_type === "office" && "🏢 Office"}
                        {template.location_type === "restaurant" && "🍽️ Restaurant"}
                      </span>
                      
                      {template.weekly_limit ? (
                        <span className={`px-2 py-0.5 rounded-full font-medium
                          ${usage === 0 ? "bg-green-100 text-green-800" :
                            usage < template.weekly_limit ? "bg-yellow-100 text-yellow-800" :
                            "bg-red-100 text-red-800"}`}
                        >
                          {usage}/{template.weekly_limit} this week
                        </span>
                      ) : (
                        <span className="px-2 py-0.5 bg-blue-50 text-blue-600 rounded-full">
                          ∞ No limit
                        </span>
                      )}
                    </div>
                  </div>
                  
                  {!isExhausted && (
                    <div className="absolute bottom-2 right-2 text-gray-400">→</div>
                  )}
                </button>
              );
            })}
          </div>
          <div className="mt-4 border-t border-gray-200 pt-4">
            <h3 className="text-lg font-semibold text-gray-900 mb-3">All Templates</h3>
          </div>
        </div>
      )}

      {/* Templates Grid */}
      {!loading && !error && (
        <>
          {filteredTemplates.length === 0 ? (
            <div className="text-center py-8 text-gray-500">
              {searchQuery
                ? "No templates found matching your search."
                : "No templates available for this meal slot."}
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredTemplates.map((template) => {
                const usage = getTemplateUsage(template);
                const isExhausted = !!(template.weekly_limit && usage >= template.weekly_limit);
                
                return (
                  <button
                    key={template.id}
                    onClick={() => !isExhausted && handleSelectTemplate(template)}
                    disabled={isExhausted}
                    className={`group relative flex flex-col p-5 border-2 rounded-xl transition-all duration-200 text-left ${
                      isExhausted
                        ? "border-gray-300 bg-gray-50 cursor-not-allowed opacity-60"
                        : "border-gray-200 bg-white hover:border-blue-400 hover:shadow-lg hover:bg-blue-50"
                    }`}
                  >
                    {/* Exhausted Overlay Badge */}
                    {isExhausted && (
                      <div className="absolute top-2 right-2 bg-red-500 text-white text-xs font-bold px-2 py-1 rounded-full">
                        LIMIT REACHED
                      </div>
                    )}
                    
                    {/* Template Name */}
                    <h3 className={`font-semibold text-lg mb-2 transition-colors ${
                      isExhausted
                        ? "text-gray-500"
                        : "text-gray-900 group-hover:text-blue-700"
                    }`}>
                      {template.name}
                    </h3>

                  {/* Description */}
                  {template.description && (
                    <p className="text-sm text-gray-600 mb-3 line-clamp-2">
                      {template.description}
                    </p>
                  )}

                  {/* Metadata Footer */}
                  <div className="mt-auto pt-3 border-t border-gray-100 space-y-1">
                    {/* Location Badge */}
                    <div className="flex items-center gap-2 text-xs flex-wrap">
                      <span className="inline-flex items-center px-2 py-1 rounded-full bg-gray-100 text-gray-700 font-medium">
                        {template.location_type === "any"
                          ? "📍 Any"
                          : template.location_type === "home"
                          ? "🏠 Home"
                          : template.location_type === "office"
                          ? "🏢 Office"
                          : "🍽️ Restaurant"}
                      </span>
                      
                      {/* Weekly Limit Badge with Usage */}
                      {template.weekly_limit ? (
                        <span className={`inline-flex items-center px-2 py-1 rounded-full font-medium ${
                          getTemplateUsage(template) >= template.weekly_limit
                            ? "bg-red-100 text-red-700"
                            : getTemplateUsage(template) > 0
                            ? "bg-yellow-100 text-yellow-700"
                            : "bg-green-100 text-green-700"
                        }`}>
                          {getTemplateUsage(template)}/{template.weekly_limit} this week
                        </span>
                      ) : (
                        <span className="inline-flex items-center px-2 py-1 rounded-full bg-blue-100 text-blue-700 font-medium">
                          ∞ No limit
                        </span>
                      )}
                    </div>
                  </div>

                  {/* Hover Indicator */}
                  {!isExhausted && (
                    <div className="absolute top-3 right-3 opacity-0 group-hover:opacity-100 transition-opacity">
                      <svg
                        className="w-5 h-5 text-blue-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M9 5l7 7-7 7"
                        />
                      </svg>
                    </div>
                  )}
                </button>
              );
            })}
            </div>
          )}
        </>
      )}
    </Modal>
  );
}
