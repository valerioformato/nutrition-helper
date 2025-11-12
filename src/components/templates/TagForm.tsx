import { useEffect, useState } from "react";
import { createTag, getAllTags, updateTag } from "../../lib/api";
import { Tag, TagCategory } from "../../lib/types";

interface TagFormProps {
  tag?: Tag | null;
  onClose: () => void;
  onSave: () => void;
}

export function TagForm({ tag, onClose, onSave }: TagFormProps) {
  const isEdit = !!tag;

  // Form state
  const [name, setName] = useState(tag?.name || "");
  const [displayName, setDisplayName] = useState(tag?.display_name || "");
  const [category, setCategory] = useState<TagCategory>(
    tag?.category || TagCategory.Ingredient
  );
  const [weeklySuggestion, setWeeklySuggestion] = useState<number | null>(
    tag?.weekly_suggestion || null
  );
  const [parentTagId, setParentTagId] = useState<number | null>(
    tag?.parent_tag_id || null
  );

  // Available tags for parent selection
  const [availableTags, setAvailableTags] = useState<Tag[]>([]);

  // UI state
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Validation
  const [nameError, setNameError] = useState<string | null>(null);
  const [displayNameError, setDisplayNameError] = useState<string | null>(
    null
  );

  useEffect(() => {
    loadTags();
  }, []);

  const loadTags = async () => {
    try {
      const tags = await getAllTags();
      // Filter out current tag (can't be its own parent) and its descendants
      const filteredTags = tags.filter((t) => {
        if (isEdit && tag) {
          // Can't be parent of itself
          if (t.id === tag.id) return false;
          // Can't be parent of its own child
          if (t.parent_tag_id === tag.id) return false;
        }
        return true;
      });
      setAvailableTags(filteredTags);
    } catch (err) {
      console.error("Failed to load tags:", err);
    }
  };

  const validate = (): boolean => {
    let valid = true;

    // Name validation
    if (!name.trim()) {
      setNameError("Tag name is required");
      valid = false;
    } else if (!/^[a-z0-9_]+$/.test(name.trim())) {
      setNameError("Name must be lowercase letters, numbers, and underscores only");
      valid = false;
    } else {
      setNameError(null);
    }

    // Display name validation
    if (!displayName.trim()) {
      setDisplayNameError("Display name is required");
      valid = false;
    } else if (displayName.trim().length < 2) {
      setDisplayNameError("Display name must be at least 2 characters");
      valid = false;
    } else {
      setDisplayNameError(null);
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
      const tagData = {
        name: name.trim(),
        display_name: displayName.trim(),
        category,
        weekly_suggestion: weeklySuggestion,
        parent_tag_id: parentTagId,
      };

      if (isEdit && tag) {
        await updateTag(tag.id, tagData);
      } else {
        await createTag(tagData);
      }

      onSave();
    } catch (err) {
      console.error("Failed to save tag:", err);
      
      // Try to extract a more specific error message
      let errorMessage = `Failed to ${isEdit ? "update" : "create"} tag.`;
      
      if (err instanceof Error && err.message) {
        // Parse the error message for specific issues
        const msg = err.message.toLowerCase();
        if (msg.includes("unique") || msg.includes("already exists")) {
          errorMessage += " A tag with this name already exists.";
        } else if (msg.includes("weekly_suggestion") || msg.includes("weekly suggestion")) {
          errorMessage += " Invalid weekly suggestion value.";
        } else if (msg.includes("name")) {
          errorMessage += " Invalid tag name format.";
        } else {
          errorMessage += ` ${err.message}`;
        }
      }
      
      setError(errorMessage);
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
            {isEdit ? "Edit Tag" : "New Tag"}
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

          {/* Name (Internal Key) */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Internal Name <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value.toLowerCase())}
              onBlur={validate}
              placeholder="e.g., pasta, ricotta, pasta_integrale"
              className={`w-full px-4 py-2 border rounded-md font-mono focus:outline-none focus:ring-2 ${
                nameError
                  ? "border-red-300 focus:ring-red-500"
                  : "border-gray-300 focus:ring-blue-500"
              }`}
              disabled={saving}
            />
            {nameError && (
              <p className="text-red-600 text-sm mt-1">{nameError}</p>
            )}
            <p className="text-sm text-gray-500 mt-1">
              Lowercase letters, numbers, and underscores only. Used internally.
            </p>
          </div>

          {/* Display Name */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Display Name <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={displayName}
              onChange={(e) => setDisplayName(e.target.value)}
              onBlur={validate}
              placeholder="e.g., Pasta, Ricotta, Pasta Integrale"
              className={`w-full px-4 py-2 border rounded-md focus:outline-none focus:ring-2 ${
                displayNameError
                  ? "border-red-300 focus:ring-red-500"
                  : "border-gray-300 focus:ring-blue-500"
              }`}
              disabled={saving}
            />
            {displayNameError && (
              <p className="text-red-600 text-sm mt-1">{displayNameError}</p>
            )}
            <p className="text-sm text-gray-500 mt-1">
              User-friendly name shown in the UI
            </p>
          </div>

          {/* Category */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Category <span className="text-red-500">*</span>
            </label>
            <select
              value={category}
              onChange={(e) => setCategory(e.target.value as TagCategory)}
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              disabled={saving}
            >
              <option value={TagCategory.Ingredient}>Ingredient</option>
              <option value={TagCategory.Dietary}>Dietary</option>
              <option value={TagCategory.PrepTime}>Prep Time</option>
              <option value={TagCategory.Other}>Other</option>
            </select>
            <p className="text-sm text-gray-500 mt-1">
              Categorize the tag for better organization
            </p>
          </div>

          {/* Parent Tag */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Parent Tag (Optional)
            </label>
            <select
              value={parentTagId || ""}
              onChange={(e) =>
                setParentTagId(e.target.value ? parseInt(e.target.value) : null)
              }
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              disabled={saving}
            >
              <option value="">No parent (root tag)</option>
              {availableTags
                .filter((t) => !t.parent_tag_id) // Only show root tags as parents
                .map((t) => (
                  <option key={t.id} value={t.id}>
                    {t.display_name}
                  </option>
                ))}
            </select>
            <p className="text-sm text-gray-500 mt-1">
              Create hierarchies (e.g., "Pasta Integrale" → "Pasta")
            </p>
          </div>

          {/* Weekly Suggestion */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Weekly Suggestion (Optional)
            </label>
            <div className="flex items-center space-x-4">
              <input
                type="number"
                min="0"
                step="1"
                value={weeklySuggestion === null ? "" : weeklySuggestion}
                onChange={(e) => {
                  const value = e.target.value;
                  setWeeklySuggestion(value === "" ? null : parseInt(value, 10));
                }}
                placeholder="No limit"
                className="w-32 px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                disabled={saving}
              />
              <span className="text-gray-600">times per week</span>
              {weeklySuggestion !== null && (
                <button
                  type="button"
                  onClick={() => setWeeklySuggestion(null)}
                  className="text-sm text-blue-600 hover:text-blue-800"
                  disabled={saving}
                >
                  Remove limit
                </button>
              )}
            </div>
            <p className="text-sm text-gray-500 mt-1">
              Soft recommendation for dietary variety
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
              {saving ? "Saving..." : isEdit ? "Update Tag" : "Create Tag"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
