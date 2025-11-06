import { useState } from "react";
import { SlotType } from "../lib/types";

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
          {slots.map((slot) => (
            <div
              key={slot}
              className="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow"
            >
              <h3 className="text-lg font-semibold text-gray-900 mb-2">
                {getSlotDisplayName(slot)}
              </h3>
              {/* Placeholder for MealSlot component */}
              <div className="text-gray-500 italic">No meal added yet</div>
              <button className="mt-3 px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors">
                Add Meal
              </button>
            </div>
          ))}
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
