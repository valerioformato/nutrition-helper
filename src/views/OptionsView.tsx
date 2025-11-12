import { useEffect, useState } from "react";
import { ConfirmDialog } from "../components/common/ConfirmDialog";
import { OptionForm } from "../components/templates/OptionForm";
import {
    deleteOption,
    getAllTags,
    getOptionsByTemplateWithTags,
} from "../lib/api";
import { MealOption, MealOptionWithTags, MealTemplate, Tag } from "../lib/types";

interface OptionsViewProps {
  template: MealTemplate;
  onBack: () => void;
}

export function OptionsView({ template, onBack }: OptionsViewProps) {
  const [options, setOptions] = useState<MealOptionWithTags[]>([]);
  const [allTags, setAllTags] = useState<Map<number, Tag>>(new Map());
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Form modal state
  const [showForm, setShowForm] = useState(false);
  const [editingOption, setEditingOption] = useState<MealOption | null>(null);
  const [editingOptionTags, setEditingOptionTags] = useState<number[]>([]);
  
  // Delete confirmation state
  const [optionToDelete, setOptionToDelete] = useState<MealOptionWithTags | null>(
    null
  );

  useEffect(() => {
    loadData();
  }, [template.id]);

  const loadData = async () => {
    setLoading(true);
    setError(null);
    try {
      // Load tags and options in parallel
      const [tags, templateOptions] = await Promise.all([
        getAllTags(),
        getOptionsByTemplateWithTags(template.id),
      ]);

      // Create a map for quick tag lookup
      const tagsMap = new Map<number, Tag>();
      tags.forEach((tag) => tagsMap.set(tag.id, tag));
      setAllTags(tagsMap);
      setOptions(templateOptions);
    } catch (err) {
      console.error("Failed to load data:", err);
      setError("Failed to load meal options. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  const loadOptions = async () => {
    try {
      const templateOptions = await getOptionsByTemplateWithTags(template.id);
      setOptions(templateOptions);
    } catch (err) {
      console.error("Failed to reload options:", err);
    }
  };

  const handleDeleteClick = (option: MealOptionWithTags) => {
    setOptionToDelete(option);
  };

  const confirmDelete = async () => {
    if (!optionToDelete) return;

    try {
      await deleteOption(optionToDelete.id);
      setOptionToDelete(null);
      await loadOptions();
    } catch (err) {
      console.error("Failed to delete option:", err);
      alert("Failed to delete option. It may be in use by meal entries.");
      setOptionToDelete(null);
    }
  };

  const cancelDelete = () => {
    setOptionToDelete(null);
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <button
                onClick={onBack}
                className="px-3 py-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-md transition-colors"
              >
                ← Back
              </button>
              <div>
                <h1 className="text-3xl font-bold text-gray-900">
                  {template.name}
                </h1>
                <p className="text-gray-600 mt-1">
                  Manage meal options and variations
                </p>
              </div>
            </div>
            <button
              onClick={() => {
                setEditingOption(null);
                setEditingOptionTags([]);
                setShowForm(true);
              }}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
            >
              + Add Option
            </button>
          </div>

          {/* Template Info Card */}
          <div className="mt-4 p-4 bg-gray-50 rounded-lg">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
              <div>
                <span className="font-medium text-gray-700">Location: </span>
                <span className="text-gray-900 capitalize">
                  {template.location_type}
                </span>
              </div>
              {template.description && (
                <div className="md:col-span-2">
                  <span className="font-medium text-gray-700">
                    Description:{" "}
                  </span>
                  <span className="text-gray-900">{template.description}</span>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 py-8">
        {/* Loading State */}
        {loading && (
          <div className="text-center py-12">
            <div className="text-gray-500">Loading options...</div>
          </div>
        )}

        {/* Error State */}
        {error && (
          <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
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
              <div className="text-center py-12 bg-white rounded-lg shadow-md">
                <p className="text-gray-500 mb-4">
                  No options yet. Add your first option to create variations for
                  this meal template.
                </p>
                <button
                  onClick={() => {
                    setEditingOption(null);
                    setEditingOptionTags([]);
                    setShowForm(true);
                  }}
                  className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
                >
                  + Add Your First Option
                </button>
              </div>
            ) : (
              <div className="space-y-4">
                {options.map((option) => {
                  // Get Tag objects for this option's tag IDs
                  const optionTags = option.tags
                    .map((tagId) => allTags.get(tagId))
                    .filter((tag): tag is Tag => tag !== undefined);

                  return (
                    <div
                      key={option.id}
                      className="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow"
                    >
                      <div className="flex items-start justify-between">
                        {/* Option Info */}
                        <div className="flex-1">
                          <h3 className="text-xl font-semibold text-gray-900 mb-2">
                            {option.name}
                          </h3>

                          {option.description && (
                            <p className="text-gray-600 mb-3">
                              {option.description}
                            </p>
                          )}

                          {option.nutritional_notes && (
                            <div className="mb-3 p-3 bg-blue-50 rounded-md">
                              <p className="text-sm text-blue-900">
                                <span className="font-medium">
                                  Nutritional Notes:{" "}
                                </span>
                                {option.nutritional_notes}
                              </p>
                            </div>
                          )}

                          {/* Tags */}
                          {optionTags.length > 0 && (
                            <div className="flex flex-wrap gap-2">
                              {optionTags.map((tag) => (
                                <span
                                  key={tag.id}
                                  className="px-3 py-1 text-sm bg-green-100 text-green-800 rounded-full"
                                >
                                  🏷️ {tag.name}
                                  {tag.weekly_suggestion && (
                                    <span className="ml-1 text-xs">
                                      ({tag.weekly_suggestion}x/week)
                                    </span>
                                  )}
                                </span>
                              ))}
                            </div>
                          )}
                        </div>

                        {/* Actions */}
                        <div className="flex gap-2 ml-4">
                          <button
                            onClick={() => {
                              // Convert MealOptionWithTags to MealOption for the form
                              const { tags, ...mealOption } = option;
                              setEditingOption(mealOption as MealOption);
                              setEditingOptionTags(option.tags);
                              setShowForm(true);
                            }}
                            className="px-3 py-2 text-sm text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
                          >
                            Edit
                          </button>
                          <button
                            onClick={() => handleDeleteClick(option)}
                            className="px-3 py-2 text-sm text-red-600 hover:bg-red-50 rounded-md transition-colors"
                          >
                            Delete
                          </button>
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </>
        )}
      </div>

      {/* Option Form Modal */}
      {showForm && (
        <OptionForm
          templateId={template.id}
          option={editingOption}
          existingTags={editingOptionTags}
          onClose={() => {
            setShowForm(false);
            setEditingOption(null);
            setEditingOptionTags([]);
          }}
          onSave={async () => {
            setShowForm(false);
            setEditingOption(null);
            setEditingOptionTags([]);
            await loadOptions();
          }}
        />
      )}

      {/* Delete Confirmation Dialog */}
      {optionToDelete && (
        <ConfirmDialog
          title="Delete Option"
          message={`Are you sure you want to delete the option "${optionToDelete.name}"? This action cannot be undone.`}
          confirmLabel="Delete"
          cancelLabel="Cancel"
          confirmVariant="danger"
          onConfirm={confirmDelete}
          onCancel={cancelDelete}
        />
      )}
    </div>
  );
}
