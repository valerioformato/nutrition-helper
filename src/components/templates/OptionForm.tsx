import { useEffect, useState } from "react";
import {
    addTagsToOption,
    createOption,
    getAllTags,
    removeTagsFromOption,
    updateOption,
} from "../../lib/api";
import { MealOption, Tag } from "../../lib/types";

interface OptionFormProps {
  templateId: number;
  option?: MealOption | null;
  existingTags?: number[]; // Tag IDs if editing
  onClose: () => void;
  onSave: () => void;
}

export function OptionForm({
  templateId,
  option,
  existingTags = [],
  onClose,
  onSave,
}: OptionFormProps) {
  const isEdit = !!option;

  // Form state
  const [name, setName] = useState(option?.name || "");
  const [description, setDescription] = useState(option?.description || "");
  const [nutritionalNotes, setNutritionalNotes] = useState(
    option?.nutritional_notes || ""
  );
  const [selectedTagIds, setSelectedTagIds] = useState<number[]>(existingTags);

  // Available tags
  const [allTags, setAllTags] = useState<Tag[]>([]);
  const [tagsLoading, setTagsLoading] = useState(false);

  // UI state
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Validation
  const [nameError, setNameError] = useState<string | null>(null);

  useEffect(() => {
    loadTags();
  }, []);

  const loadTags = async () => {
    setTagsLoading(true);
    try {
      const tags = await getAllTags();
      setAllTags(tags);
    } catch (err) {
      console.error("Failed to load tags:", err);
      setError("Failed to load tags. You can still create the option without tags.");
    } finally {
      setTagsLoading(false);
    }
  };

  const toggleTag = (tagId: number) => {
    setSelectedTagIds((prev) =>
      prev.includes(tagId)
        ? prev.filter((id) => id !== tagId)
        : [...prev, tagId]
    );
  };

  const validate = (): boolean => {
    let valid = true;

    // Name validation
    if (!name.trim()) {
      setNameError("Option name is required");
      valid = false;
    } else if (name.trim().length < 2) {
      setNameError("Name must be at least 2 characters");
      valid = false;
    } else {
      setNameError(null);
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
      if (isEdit && option) {
        // Update existing option
        const updateData = {
          name: name.trim(),
          description: description.trim() || null,
          nutritional_notes: nutritionalNotes.trim() || null,
        };
        await updateOption(option.id, updateData);

        // Update tags: determine which to add/remove
        const tagsToAdd = selectedTagIds.filter(
          (id) => !existingTags.includes(id)
        );
        const tagsToRemove = existingTags.filter(
          (id) => !selectedTagIds.includes(id)
        );

        if (tagsToAdd.length > 0) {
          await addTagsToOption(option.id, tagsToAdd);
        }
        if (tagsToRemove.length > 0) {
          await removeTagsFromOption(option.id, tagsToRemove);
        }
      } else {
        // Create new option
        const createData = {
          template_id: templateId,
          name: name.trim(),
          description: description.trim() || null,
          nutritional_notes: nutritionalNotes.trim() || null,
        };
        const newOption = await createOption(createData);

        // Add tags to the new option
        if (selectedTagIds.length > 0) {
          await addTagsToOption(newOption.id, selectedTagIds);
        }
      }

      onSave();
    } catch (err) {
      console.error("Failed to save option:", err);
      setError(
        `Failed to ${isEdit ? "update" : "create"} option. Please try again.`
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

  // Group tags by hierarchy (if they have parent_tag_id)
  const rootTags = allTags.filter((tag) => !tag.parent_tag_id);
  const childTagsMap = new Map<number, Tag[]>();
  allTags.forEach((tag) => {
    if (tag.parent_tag_id) {
      const children = childTagsMap.get(tag.parent_tag_id) || [];
      children.push(tag);
      childTagsMap.set(tag.parent_tag_id, children);
    }
  });

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="sticky top-0 bg-white border-b border-gray-200 px-6 py-4">
          <h2 className="text-2xl font-bold text-gray-900">
            {isEdit ? "Edit Option" : "New Option"}
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
              Option Name <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              onBlur={validate}
              placeholder="e.g., Philadelphia, Ricotta, Crema spalmabile"
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
            <p className="text-sm text-gray-500 mt-1">
              The specific ingredient or variation name
            </p>
          </div>

          {/* Description */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Description
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Optional description or preparation notes"
              rows={2}
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              disabled={saving}
            />
          </div>

          {/* Nutritional Notes */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Nutritional Notes
            </label>
            <textarea
              value={nutritionalNotes}
              onChange={(e) => setNutritionalNotes(e.target.value)}
              placeholder="Optional nutritional information or dietary notes"
              rows={2}
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              disabled={saving}
            />
            <p className="text-sm text-gray-500 mt-1">
              e.g., "High protein", "Low carb", "Contains nuts"
            </p>
          </div>

          {/* Tags Selection */}
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Tags (Optional)
            </label>
            {tagsLoading ? (
              <div className="text-sm text-gray-500">Loading tags...</div>
            ) : allTags.length === 0 ? (
              <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-md">
                <p className="text-sm text-yellow-800">
                  No tags available yet. You can create tags in the Tags Manager
                  (Task 6) and then come back to add them to options.
                </p>
              </div>
            ) : (
              <div className="border border-gray-300 rounded-md p-4 max-h-64 overflow-y-auto">
                <div className="space-y-3">
                  {rootTags.map((tag) => {
                    const children = childTagsMap.get(tag.id) || [];
                    return (
                      <div key={tag.id} className="space-y-2">
                        {/* Parent Tag */}
                        <label className="flex items-center space-x-3 cursor-pointer p-2 hover:bg-gray-50 rounded-md">
                          <input
                            type="checkbox"
                            checked={selectedTagIds.includes(tag.id)}
                            onChange={() => toggleTag(tag.id)}
                            disabled={saving}
                            className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                          />
                          <span className="text-gray-900 font-medium">
                            🏷️ {tag.name}
                            {tag.weekly_suggestion && (
                              <span className="ml-2 text-xs text-gray-500">
                                ({tag.weekly_suggestion}x/week)
                              </span>
                            )}
                          </span>
                        </label>

                        {/* Child Tags */}
                        {children.length > 0 && (
                          <div className="ml-6 space-y-1 border-l-2 border-gray-200 pl-3">
                            {children.map((childTag) => (
                              <label
                                key={childTag.id}
                                className="flex items-center space-x-3 cursor-pointer p-2 hover:bg-gray-50 rounded-md"
                              >
                                <input
                                  type="checkbox"
                                  checked={selectedTagIds.includes(childTag.id)}
                                  onChange={() => toggleTag(childTag.id)}
                                  disabled={saving}
                                  className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                                />
                                <span className="text-gray-700 text-sm">
                                  {childTag.name}
                                  {childTag.weekly_suggestion && (
                                    <span className="ml-2 text-xs text-gray-500">
                                      ({childTag.weekly_suggestion}x/week)
                                    </span>
                                  )}
                                </span>
                              </label>
                            ))}
                          </div>
                        )}
                      </div>
                    );
                  })}
                </div>
              </div>
            )}
            <p className="text-sm text-gray-500 mt-2">
              Select tags to categorize this option and track weekly limits
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
                  ? "Update Option"
                  : "Create Option"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
