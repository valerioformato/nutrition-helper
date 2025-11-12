import { useEffect, useState } from "react";
import { createTemplate, updateTemplate } from "../../lib/api";
import { LocationType, MealTemplate, SlotType } from "../../lib/types";

interface TemplateFormProps {
  template?: MealTemplate | null;
  onClose: () => void;
  onSave: () => void;
}

export function TemplateForm({ template, onClose, onSave }: TemplateFormProps) {
  const isEdit = !!template;

  // Form state
  const [name, setName] = useState(template?.name || "");
  const [description, setDescription] = useState(template?.description || "");
  const [locationType, setLocationType] = useState<LocationType>(
    template?.location_type || LocationType.Any
  );
  const [compatibleSlots, setCompatibleSlots] = useState<SlotType[]>(
    template?.compatible_slots || []
  );
  const [weeklyLimit, setWeeklyLimit] = useState<number | null>(
    template?.weekly_limit || null
  );

  // UI state
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Validation
  const [nameError, setNameError] = useState<string | null>(null);
  const [slotsError, setSlotsError] = useState<string | null>(null);

  const allSlots: SlotType[] = [
    SlotType.Breakfast,
    SlotType.MorningSnack,
    SlotType.Lunch,
    SlotType.AfternoonSnack,
    SlotType.Dinner,
  ];

  const getSlotDisplayName = (slot: SlotType): string => {
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
    }
  };

  const toggleSlot = (slot: SlotType) => {
    setCompatibleSlots((prev) =>
      prev.includes(slot)
        ? prev.filter((s) => s !== slot)
        : [...prev, slot]
    );
  };

  const validate = (): boolean => {
    let valid = true;

    // Name validation
    if (!name.trim()) {
      setNameError("Template name is required");
      valid = false;
    } else if (name.trim().length < 3) {
      setNameError("Name must be at least 3 characters");
      valid = false;
    } else {
      setNameError(null);
    }

    // Slots validation
    if (compatibleSlots.length === 0) {
      setSlotsError("Select at least one compatible meal slot");
      valid = false;
    } else {
      setSlotsError(null);
    }

    return valid;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validate()) {
      return;
    }

    setSaving(true);
    setError(null);

    try {
      const templateData = {
        name: name.trim(),
        description: description.trim() || null,
        location_type: locationType,
        compatible_slots: compatibleSlots,
        weekly_limit: weeklyLimit,
      };

      if (isEdit && template) {
        await updateTemplate(template.id, templateData);
      } else {
        await createTemplate(templateData);
      }

      onSave();
    } catch (err) {
      console.error("Failed to save template:", err);
      setError(
        `Failed to ${isEdit ? "update" : "create"} template. Please try again.`
      );
    } finally {
      setSaving(false);
    }
  };

  // Close on Escape key
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape" && !saving) {
        onClose();
      }
    };
    window.addEventListener("keydown", handleEscape);
    return () => window.removeEventListener("keydown", handleEscape);
  }, [saving, onClose]);

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="sticky top-0 bg-white border-b border-gray-200 px-6 py-4">
          <h2 className="text-2xl font-bold text-gray-900">
            {isEdit ? "Edit Template" : "New Template"}
          </h2>
        </div>

        {/* Form */}
        <form onSubmit={handleSubmit} className="p-6 space-y-6">
          {/* Error Alert */}
          {error && (
            <div className="bg-red-50 border border-red-200 rounded-md p-4">
              <p className="text-red-800">{error}</p>
            </div>
          )}

          {/* Name */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Template Name <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              onBlur={validate}
              placeholder="e.g., Pane con marmellata"
              className={`w-full px-4 py-2 border rounded-md focus:outline-none focus:ring-2 ${
                nameError
                  ? "border-red-300 focus:ring-red-500"
                  : "border-gray-300 focus:ring-blue-500"
              }`}
              disabled={saving}
            />
            {nameError && (
              <p className="text-red-600 text-sm mt-1">{nameError}</p>
            )}
          </div>

          {/* Description */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Description
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Optional description or notes about this template"
              rows={3}
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              disabled={saving}
            />
          </div>

          {/* Location Type */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Location Type <span className="text-red-500">*</span>
            </label>
            <select
              value={locationType}
              onChange={(e) => setLocationType(e.target.value as LocationType)}
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              disabled={saving}
            >
              <option value={LocationType.Any}>📍 Any Location</option>
              <option value={LocationType.Home}>🏠 Home</option>
              <option value={LocationType.Office}>🏢 Office</option>
              <option value={LocationType.Restaurant}>🍽️ Restaurant</option>
            </select>
            <p className="text-sm text-gray-500 mt-1">
              Where this meal is typically prepared or consumed
            </p>
          </div>

          {/* Compatible Slots */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Compatible Meal Slots <span className="text-red-500">*</span>
            </label>
            <div className="space-y-2">
              {allSlots.map((slot) => (
                <label
                  key={slot}
                  className="flex items-center space-x-3 cursor-pointer p-2 hover:bg-gray-50 rounded-md"
                >
                  <input
                    type="checkbox"
                    checked={compatibleSlots.includes(slot)}
                    onChange={() => toggleSlot(slot)}
                    disabled={saving}
                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                  />
                  <span className="text-gray-900">
                    {getSlotDisplayName(slot)}
                  </span>
                </label>
              ))}
            </div>
            {slotsError && (
              <p className="text-red-600 text-sm mt-1">{slotsError}</p>
            )}
            <p className="text-sm text-gray-500 mt-2">
              Select which meal slots this template can fill
            </p>
          </div>

          {/* Weekly Limit */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Weekly Limit (Optional)
            </label>
            <div className="flex items-center space-x-4">
              <input
                type="number"
                min="0"
                step="1"
                value={weeklyLimit === null ? "" : weeklyLimit}
                onChange={(e) => {
                  const value = e.target.value;
                  setWeeklyLimit(value === "" ? null : parseInt(value, 10));
                }}
                placeholder="No limit"
                className="w-32 px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                disabled={saving}
              />
              <span className="text-gray-600">times per week</span>
              {weeklyLimit !== null && (
                <button
                  type="button"
                  onClick={() => setWeeklyLimit(null)}
                  className="text-sm text-blue-600 hover:text-blue-800"
                  disabled={saving}
                >
                  Remove limit
                </button>
              )}
            </div>
            <p className="text-sm text-gray-500 mt-1">
              Soft recommendation for dietary variety (optional)
            </p>
          </div>

          {/* Form Actions */}
          <div className="flex gap-3 pt-4 border-t border-gray-200">
            <button
              type="button"
              onClick={onClose}
              disabled={saving}
              className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={saving}
              className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {saving
                ? "Saving..."
                : isEdit
                  ? "Update Template"
                  : "Create Template"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
