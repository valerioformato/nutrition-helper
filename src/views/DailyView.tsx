import { useEffect, useState } from "react";
import { MealCard } from "../components/meals/MealCard";
import { MealSelectionModal } from "../components/meals/MealSelectionModal";
import { MealSlot } from "../components/meals/MealSlot";
import { OptionSelectionModal } from "../components/meals/OptionSelectionModal";
import { getEntriesByDate, getOptionById, getTemplateById } from "../lib/api";
import {
    MealEntry,
    MealOption,
    MealTemplate,
    SlotType,
} from "../lib/types";

// Helper type for entry with full meal details
interface EntryWithDetails {
  entry: MealEntry;
  option: MealOption;
  template: MealTemplate;
}

/**
 * DailyView - Main view for displaying daily meal slots
 * Phase 2: Basic implementation with 5 meal slots
 */
export function DailyView() {
  const [selectedDate, setSelectedDate] = useState<Date>(new Date());
  const [templateModalOpen, setTemplateModalOpen] = useState(false);
  const [optionModalOpen, setOptionModalOpen] = useState(false);
  const [selectedSlot, setSelectedSlot] = useState<SlotType | null>(null);
  const [selectedTemplate, setSelectedTemplate] = useState<MealTemplate | null>(
    null
  );
  
  // Entries data
  const [entries, setEntries] = useState<Map<SlotType, EntryWithDetails>>(
    new Map()
  );
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Fetch entries when date changes
  useEffect(() => {
    loadEntries();
  }, [selectedDate]);

  const loadEntries = async () => {
    setLoading(true);
    setError(null);
    try {
      const dateStr = selectedDate.toISOString().split("T")[0];
      const dayEntries = await getEntriesByDate(dateStr);

      // Fetch full details for each entry
      const entriesMap = new Map<SlotType, EntryWithDetails>();
      
      for (const entry of dayEntries) {
        try {
          const option = await getOptionById(entry.meal_option_id);
          if (!option) {
            console.error(`Option not found for entry ${entry.id}`);
            continue;
          }
          
          const template = await getTemplateById(option.template_id);
          if (!template) {
            console.error(`Template not found for option ${option.id}`);
            continue;
          }
          
          entriesMap.set(entry.slot_type, {
            entry,
            option,
            template,
          });
        } catch (err) {
          console.error(`Failed to load details for entry ${entry.id}:`, err);
        }
      }

      setEntries(entriesMap);
    } catch (err) {
      console.error("Failed to load entries:", err);
      setError("Failed to load meals. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  // All 5 meal slots in order
  const slots: SlotType[] = [
    SlotType.Breakfast,
    SlotType.MorningSnack,
    SlotType.Lunch,
    SlotType.AfternoonSnack,
    SlotType.Dinner,
  ];

  // Format date for display
  const formatDate = (date: Date): string => {
    return date.toLocaleDateString("en-US", {
      weekday: "long",
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  };

  // Navigate to previous day
  const goToPreviousDay = () => {
    const newDate = new Date(selectedDate);
    newDate.setDate(newDate.getDate() - 1);
    setSelectedDate(newDate);
  };

  // Navigate to next day
  const goToNextDay = () => {
    const newDate = new Date(selectedDate);
    newDate.setDate(newDate.getDate() + 1);
    setSelectedDate(newDate);
  };

  // Navigate to today
  const goToToday = () => {
    setSelectedDate(new Date());
  };

  // Check if selected date is today
  const isToday = (): boolean => {
    const today = new Date();
    return (
      selectedDate.getDate() === today.getDate() &&
      selectedDate.getMonth() === today.getMonth() &&
      selectedDate.getFullYear() === today.getFullYear()
    );
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-4xl mx-auto px-4 py-6">
          <h1 className="text-3xl font-bold text-gray-900">
            Nutrition Helper
          </h1>
          <p className="text-gray-600 mt-1">Daily Meal Planner</p>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-4xl mx-auto px-4 py-8">
        {/* Date Navigator */}
        <div className="bg-white rounded-lg shadow-md p-6 mb-6">
          <div className="flex items-center justify-between">
            <button
              onClick={goToPreviousDay}
              className="px-4 py-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-md transition-colors"
              aria-label="Previous day"
            >
              <svg
                className="w-6 h-6"
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
            </button>

            <div className="flex flex-col items-center">
              <h2 className="text-xl font-semibold text-gray-900">
                {formatDate(selectedDate)}
              </h2>
              {!isToday() && (
                <button
                  onClick={goToToday}
                  className="mt-2 text-sm text-blue-600 hover:text-blue-800 transition-colors"
                >
                  Go to Today
                </button>
              )}
            </div>

            <button
              onClick={goToNextDay}
              className="px-4 py-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-md transition-colors"
              aria-label="Next day"
            >
              <svg
                className="w-6 h-6"
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
            </button>
          </div>
        </div>

        {/* Meal Slots Timeline */}
        <div className="space-y-4">
          {/* Loading State */}
          {loading && (
            <div className="text-center py-8 text-gray-500">
              Loading meals...
            </div>
          )}

          {/* Error State */}
          {error && (
            <div className="bg-red-50 border border-red-200 rounded-md p-4">
              <p className="text-red-800">{error}</p>
              <button
                onClick={loadEntries}
                className="mt-2 text-sm text-red-600 hover:text-red-800 underline"
              >
                Try again
              </button>
            </div>
          )}

          {/* Meal Slots */}
          {!loading &&
            !error &&
            slots.map((slot) => {
              const entryWithDetails = entries.get(slot);
              const isEmpty = !entryWithDetails;

              return (
                <MealSlot
                  key={slot}
                  slotType={slot}
                  slotName={getSlotDisplayName(slot)}
                  isEmpty={isEmpty}
                  onAddMeal={
                    isEmpty
                      ? () => {
                          setSelectedSlot(slot);
                          setTemplateModalOpen(true);
                        }
                      : undefined
                  }
                  onClick={
                    !isEmpty
                      ? () => {
                          console.log(
                            `Edit meal in ${getSlotDisplayName(slot)}`
                          );
                          // TODO: Open meal detail editor (Phase 4)
                        }
                      : undefined
                  }
                >
                  {entryWithDetails && (
                    <MealCard
                      entry={entryWithDetails.entry}
                      option={entryWithDetails.option}
                      template={entryWithDetails.template}
                    />
                  )}
                </MealSlot>
              );
            })}
        </div>
      </div>

      {/* Meal Selection Modal - Step 1: Choose Template */}
      {selectedSlot && (
        <MealSelectionModal
          isOpen={templateModalOpen}
          onClose={() => {
            setTemplateModalOpen(false);
            setSelectedSlot(null);
          }}
          slotType={selectedSlot}
          slotName={getSlotDisplayName(selectedSlot)}
          onSelectTemplate={(template) => {
            setSelectedTemplate(template);
            setTemplateModalOpen(false);
            setOptionModalOpen(true);
          }}
        />
      )}

      {/* Option Selection Modal - Step 2: Choose Option & Configure */}
      {selectedSlot && selectedTemplate && (
        <OptionSelectionModal
          isOpen={optionModalOpen}
          onClose={() => {
            setOptionModalOpen(false);
            setSelectedTemplate(null);
            setSelectedSlot(null);
          }}
          template={selectedTemplate}
          slotType={selectedSlot}
          slotName={getSlotDisplayName(selectedSlot)}
          date={selectedDate.toISOString().split("T")[0]}
          onSuccess={() => {
            // Refresh entries to show the newly added meal
            loadEntries();
          }}
        />
      )}
    </div>
  );
}

// Helper function to get human-readable slot names
function getSlotDisplayName(slot: SlotType): string {
  switch (slot) {
    case SlotType.Breakfast:
      return "Breakfast";
    case SlotType.MorningSnack:
      return "Morning Snack";
    case SlotType.Lunch:
      return "Lunch";
    case SlotType.AfternoonSnack:
      return "Afternoon Snack";
    case SlotType.Dinner:
      return "Dinner";
    default:
      return slot;
  }
}
