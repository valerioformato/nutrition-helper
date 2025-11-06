import { useState } from "react";
import { MealSlot } from "../components/meals/MealSlot";
import { MealCard } from "../components/meals/MealCard";
import {
  SlotType,
  LocationType,
  MealEntry,
  MealOption,
  MealTemplate,
} from "../lib/types";

/**
 * DailyView - Main view for displaying daily meal slots
 * Phase 2: Basic implementation with 5 meal slots
 */
export function DailyView() {
  const [selectedDate, setSelectedDate] = useState<Date>(new Date());

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

  // Sample data to demonstrate MealCard (TODO: Remove in Task 8 when fetching real data)
  const sampleTemplate: MealTemplate = {
    id: 1,
    name: "Yogurt con cereali e frutta secca",
    description: "A healthy breakfast option",
    compatible_slots: [SlotType.Breakfast],
    location_type: LocationType.Home,
    weekly_limit: null,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  };

  const sampleOption: MealOption = {
    id: 1,
    template_id: 1,
    name: "Greek yogurt with granola",
    description: "Low-fat greek yogurt with homemade granola",
    nutritional_notes: null,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  };

  const sampleEntry: MealEntry = {
    id: 1,
    meal_option_id: 1,
    date: selectedDate.toISOString().split("T")[0],
    slot_type: SlotType.Breakfast,
    location: LocationType.Home,
    servings: 1.0,
    notes: "Added extra blueberries today",
    completed: true,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
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
          {slots.map((slot) => {
            // Show sample filled breakfast slot for demonstration
            const isBreakfast = slot === SlotType.Breakfast;
            const isEmpty = !isBreakfast;

            return (
              <MealSlot
                key={slot}
                slotType={slot}
                slotName={getSlotDisplayName(slot)}
                isEmpty={isEmpty}
                onAddMeal={
                  isEmpty
                    ? () => {
                        console.log(
                          `Add meal to ${getSlotDisplayName(slot)}`
                        );
                        // TODO: Open meal selection modal (Task 6)
                      }
                    : undefined
                }
                onClick={
                  !isEmpty
                    ? () => {
                        console.log(`Edit meal in ${getSlotDisplayName(slot)}`);
                        // TODO: Open meal detail editor (Phase 4)
                      }
                    : undefined
                }
              >
                {!isEmpty && (
                  <MealCard
                    entry={sampleEntry}
                    option={sampleOption}
                    template={sampleTemplate}
                  />
                )}
              </MealSlot>
            );
          })}
        </div>
      </div>
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
