import { useEffect, useState } from "react";
import { createEntry, getOptionsByTemplate } from "../../lib/api";
import {
    CreateMealEntry,
    LocationType,
    MealOption,
    MealTemplate,
    SlotType,
} from "../../lib/types";
import { Modal } from "../common/Modal";

interface OptionSelectionModalProps {
  isOpen: boolean;
  onClose: () => void;
  template: MealTemplate;
  slotType: SlotType;
  slotName: string;
  date: string; // ISO date string
  onSuccess: () => void;
}

export function OptionSelectionModal({
  isOpen,
  onClose,
  template,
  slotType,
  slotName,
  date,
  onSuccess,
}: OptionSelectionModalProps) {
  const [options, setOptions] = useState<MealOption[]>([]);
  const [selectedOption, setSelectedOption] = useState<MealOption | null>(null);
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
      const templateOptions = await getOptionsByTemplate(template.id);
      setOptions(templateOptions);
    } catch (err) {
      console.error("Failed to load options:", err);
      setError("Failed to load meal options. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    if (!selectedOption) return;

    setSaving(true);
    setError(null);

    try {
      const entry: CreateMealEntry = {
        meal_option_id: selectedOption.id,
        date,
        slot_type: slotType,
        location,
        servings,
        notes: notes.trim() || null,
        completed: false,
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
                <div className="space-y-2">
                  {options.map((option) => (
                    <button
                      key={option.id}
                      onClick={() => setSelectedOption(option)}
                      className={`w-full text-left p-3 border-2 rounded-md transition-colors ${
                        selectedOption?.id === option.id
                          ? "border-blue-500 bg-blue-50"
                          : "border-gray-200 hover:border-blue-300 hover:bg-gray-50"
                      }`}
                    >
                      <h4 className="font-medium text-gray-900">
                        {option.name}
                      </h4>
                      {option.description && (
                        <p className="text-sm text-gray-600 mt-1">
                          {option.description}
                        </p>
                      )}
                      {option.nutritional_notes && (
                        <p className="text-xs text-gray-500 mt-1 italic">
                          {option.nutritional_notes}
                        </p>
                      )}
                    </button>
                  ))}
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
