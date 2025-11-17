import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { createEntry, getWeeklyUsage } from "../../lib/api";
import {
    CreateMealEntry,
    LocationType,
    MealOptionWithTags,
    MealTemplate,
    SlotType
} from "../../lib/types";
import { Modal } from "../common/Modal";

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

interface OptionSelectionModalProps {
  isOpen: boolean;
  onClose: () => void;
  template: MealTemplate;
  slotType: SlotType;
  slotName: string;
  date: string; // ISO date string
  onSuccess: () => void;
  onBack?: () => void; // Optional back button callback
}

export function OptionSelectionModal({
  isOpen,
  onClose,
  template,
  slotType,
  slotName,
  date,
  onSuccess,
  onBack,
}: OptionSelectionModalProps) {
  const [options, setOptions] = useState<MealOptionWithTags[]>([]);
  const [selectedOption, setSelectedOption] = useState<MealOptionWithTags | null>(null);
  const [weeklyUsage, setWeeklyUsage] = useState<Map<number, number>>(new Map());
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  // Form state
  const [servings, setServings] = useState(1.0);
  const [location, setLocation] = useState<LocationType>(
    template.location_type
  );
  const [notes, setNotes] = useState("");

  // Fetch options when modal opens
  useEffect(() => {
    if (isOpen) {
      loadOptions();
      // Reset form
      setSelectedOption(null);
      setServings(1.0);
      setLocation(template.location_type);
      setNotes("");
    }
  }, [isOpen, template]);

  const loadOptions = async () => {
    setLoading(true);
    setError(null);
    try {
      // Fetch options with tags
      const templateOptions = await invoke<MealOptionWithTags[]>(
        "get_options_by_template_with_tags",
        { templateId: template.id }
      );
      
      // Fetch weekly usage for all options
      const currentWeek = getCurrentWeek();
      const usagePromises = templateOptions.map(async (option) => {
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
      
      setOptions(templateOptions);
      setWeeklyUsage(usageMap);
    } catch (err) {
      console.error("Failed to load options:", err);
      setError("Failed to load meal options. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  // Get total weekly usage for the template (sum usage across all its options)
  const getTemplateUsage = (): number => {
    if (options.length === 0) return 0;
    // Sum the usage count across all options in this template
    return options.reduce((sum, opt) => sum + (weeklyUsage.get(opt.id) || 0), 0);
  };

  const handleSave = async () => {
    if (!selectedOption) return;

    setSaving(true);
    setError(null);

    try {
      // Auto-complete meals added to past dates
      const today = new Date();
      today.setHours(0, 0, 0, 0);
      const mealDate = new Date(date);
      mealDate.setHours(0, 0, 0, 0);
      const isPastDate = mealDate < today;

      const entry: CreateMealEntry = {
        meal_option_id: selectedOption.id,
        date,
        slot_type: slotType,
        location,
        servings,
        notes: notes.trim() || null,
        completed: isPastDate, // Auto-complete if past date
      };

      await createEntry(entry);
      onSuccess();
      onClose();
    } catch (err) {
      console.error("Failed to save entry:", err);
      setError("Failed to save meal. Please try again.");
    } finally {
      setSaving(false);
    }
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title={`${template.name} - ${slotName}`}
    >
      {/* Back Button (when used in wizard) */}
      {onBack && (
        <div className="mb-4">
          <button
            onClick={onBack}
            className="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
          >
            <svg
              className="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 19l-7-7 7-7"
              />
            </svg>
            Back to Meal Selection
          </button>
        </div>
      )}

      {/* Loading State */}
      {loading && (
        <div className="flex items-center justify-center py-8">
          <div className="text-gray-500">Loading options...</div>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-4">
          <p className="text-red-800">{error}</p>
          <button
            onClick={loadOptions}
            className="mt-2 text-sm text-red-600 hover:text-red-800 underline"
          >
            Try again
          </button>
        </div>
      )}

      {/* Options List */}
      {!loading && !error && (
        <>
          {options.length === 0 ? (
            <div className="text-center py-8 text-gray-500">
              No options available for this template.
            </div>
          ) : (
            <>
              {/* Step 1: Select Option */}
              <div className="mb-6">
                <h3 className="text-sm font-semibold text-gray-700 mb-3">
                  Select an option:
                </h3>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                  {options.map((option) => {
                    // Use template-level usage (sum of all options) for limit checking
                    const templateUsage = getTemplateUsage();
                    const isExhausted = !!(template.weekly_limit && templateUsage >= template.weekly_limit);
                    
                    return (
                      <button
                        key={option.id}
                        onClick={() => !isExhausted && setSelectedOption(option)}
                        disabled={isExhausted}
                        className={`relative text-left p-4 border-2 rounded-lg transition-all ${
                          selectedOption?.id === option.id
                            ? "border-blue-500 bg-blue-50 shadow-md"
                            : isExhausted
                            ? "border-gray-200 bg-gray-50 opacity-60 cursor-not-allowed"
                            : "border-gray-200 hover:border-blue-300 hover:bg-gray-50 hover:shadow-sm"
                        }`}
                      >
                        {isExhausted && (
                          <div className="absolute top-2 right-2 bg-red-500 text-white text-xs px-2 py-0.5 rounded font-bold">
                            LIMIT
                          </div>
                        )}
                        
                        <h4 className="font-semibold text-gray-900 mb-2 pr-12">
                          {option.name}
                        </h4>
                        
                        {option.description && (
                          <p className="text-sm text-gray-600 mb-2">
                            {option.description}
                          </p>
                        )}
                        
                        {option.nutritional_notes && (
                          <p className="text-xs text-gray-500 mb-3 italic">
                            {option.nutritional_notes}
                          </p>
                        )}
                        
                        <div className="flex flex-wrap gap-2 mt-2">
                          {/* Weekly usage indicator - shows template-level usage */}
                          {template.weekly_limit ? (
                            <span className={`text-xs px-2 py-1 rounded-full font-medium ${
                              templateUsage === 0 ? "bg-green-100 text-green-800" :
                              templateUsage < template.weekly_limit ? "bg-yellow-100 text-yellow-800" :
                              "bg-red-100 text-red-800"
                            }`}>
                              {templateUsage}/{template.weekly_limit} this week
                            </span>
                          ) : (
                            <span className="text-xs px-2 py-1 bg-blue-50 text-blue-600 rounded-full">
                              ∞ No limit
                            </span>
                          )}
                          
                          {/* Location compatibility */}
                          {template.location_type !== "any" && (
                            <span className="text-xs px-2 py-1 bg-purple-100 text-purple-700 rounded-full">
                              {template.location_type === "home" && "🏠 Home"}
                              {template.location_type === "office" && "🏢 Office"}
                              {template.location_type === "restaurant" && "🍽️ Restaurant"}
                            </span>
                          )}
                        </div>
                        
                        {selectedOption?.id === option.id && (
                          <div className="absolute bottom-2 right-2 text-blue-500">
                            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                              <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                            </svg>
                          </div>
                        )}
                      </button>
                    );
                  })}
                </div>
              </div>

              {/* Step 2: Configure Meal Details (shown when option selected) */}
              {selectedOption && (
                <>
                  <div className="border-t border-gray-200 pt-6 space-y-4">
                    <h3 className="text-sm font-semibold text-gray-700 mb-3">
                      Meal details:
                    </h3>

                    {/* Location */}
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Location
                      </label>
                      <select
                        value={location}
                        onChange={(e) =>
                          setLocation(e.target.value as LocationType)
                        }
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      >
                        <option value={LocationType.Home}>🏠 Home</option>
                        <option value={LocationType.Office}>🏢 Office</option>
                        <option value={LocationType.Restaurant}>
                          🍽️ Restaurant
                        </option>
                        <option value={LocationType.Any}>📍 Any</option>
                      </select>
                    </div>

                    {/* Servings */}
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Servings
                      </label>
                      <input
                        type="number"
                        min="0.1"
                        step="0.1"
                        value={servings}
                        onChange={(e) =>
                          setServings(parseFloat(e.target.value) || 1.0)
                        }
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      />
                    </div>

                    {/* Notes */}
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Notes (optional)
                      </label>
                      <textarea
                        value={notes}
                        onChange={(e) => setNotes(e.target.value)}
                        placeholder="Add any notes about this meal..."
                        rows={3}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                      />
                    </div>
                  </div>

                  {/* Action Buttons */}
                  <div className="flex gap-3 mt-6 pt-4 border-t border-gray-200">
                    <button
                      onClick={onClose}
                      disabled={saving}
                      className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors disabled:opacity-50"
                    >
                      Cancel
                    </button>
                    <button
                      onClick={handleSave}
                      disabled={saving}
                      className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:opacity-50"
                    >
                      {saving ? "Saving..." : "Save Meal"}
                    </button>
                  </div>
                </>
              )}
            </>
          )}
        </>
      )}
    </Modal>
  );
}
