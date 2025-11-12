import { useEffect, useState } from "react";
import { ConfirmDialog } from "../components/common/ConfirmDialog";
import { TemplateForm } from "../components/templates/TemplateForm";
import { deleteTemplate, getAllTemplates } from "../lib/api";
import { LocationType, MealTemplate, SlotType } from "../lib/types";
import { OptionsView } from "./OptionsView";
import { TagsView } from "./TagsView";

export function TemplatesView() {
  const [templates, setTemplates] = useState<MealTemplate[]>([]);
  const [filteredTemplates, setFilteredTemplates] = useState<MealTemplate[]>(
    []
  );
  const [searchQuery, setSearchQuery] = useState("");
  const [locationFilter, setLocationFilter] = useState<string>("all");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Form modal state
  const [showForm, setShowForm] = useState(false);
  const [editingTemplate, setEditingTemplate] = useState<MealTemplate | null>(
    null
  );
  
  // Options view state
  const [selectedTemplate, setSelectedTemplate] = useState<MealTemplate | null>(
    null
  );
  
  // Tags view state
  const [showTagsView, setShowTagsView] = useState(false);
  
  // Delete confirmation state
  const [templateToDelete, setTemplateToDelete] = useState<MealTemplate | null>(
    null
  );

  useEffect(() => {
    loadTemplates();
  }, []);

  useEffect(() => {
    filterTemplates();
  }, [templates, searchQuery, locationFilter]);

  const loadTemplates = async () => {
    setLoading(true);
    setError(null);
    try {
      const allTemplates = await getAllTemplates();
      setTemplates(allTemplates);
    } catch (err) {
      console.error("Failed to load templates:", err);
      setError("Failed to load templates. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  const filterTemplates = () => {
    let filtered = templates;

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

    // Filter by location
    if (locationFilter !== "all") {
      filtered = filtered.filter(
        (template) =>
          template.location_type === locationFilter ||
          template.location_type === LocationType.Any
      );
    }

    setFilteredTemplates(filtered);
  };

  const handleDeleteClick = (template: MealTemplate) => {
    setTemplateToDelete(template);
  };

  const confirmDelete = async () => {
    if (!templateToDelete) return;

    try {
      await deleteTemplate(templateToDelete.id);
      setTemplateToDelete(null);
      await loadTemplates();
    } catch (err) {
      console.error("Failed to delete template:", err);
      alert("Failed to delete template. It may be in use by meal entries.");
      setTemplateToDelete(null);
    }
  };

  const cancelDelete = () => {
    setTemplateToDelete(null);
  };

  const getLocationIcon = (location: LocationType) => {
    switch (location) {
      case LocationType.Home:
        return "🏠";
      case LocationType.Office:
        return "🏢";
      case LocationType.Restaurant:
        return "🍽️";
      case LocationType.Any:
        return "📍";
    }
  };

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

  // If showing tags view, render TagsView
  if (showTagsView) {
    return <TagsView onBack={() => setShowTagsView(false)} />;
  }

  // If a template is selected, show the OptionsView
  if (selectedTemplate) {
    return (
      <OptionsView
        template={selectedTemplate}
        onBack={() => setSelectedTemplate(null)}
      />
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">
                Templates Manager
              </h1>
              <p className="text-gray-600 mt-1">
                Manage your meal templates and options
              </p>
            </div>
            <div className="flex gap-3">
              <button
                onClick={() => setShowTagsView(true)}
                className="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 transition-colors"
              >
                Manage Tags
              </button>
              <button
                onClick={() => {
                  setEditingTemplate(null);
                  setShowForm(true);
                }}
                className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
              >
                + Add Template
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 py-8">
        {/* Filters */}
        <div className="bg-white rounded-lg shadow-md p-4 mb-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {/* Search */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Search
              </label>
              <input
                type="text"
                placeholder="Search templates..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            {/* Location Filter */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Location
              </label>
              <select
                value={locationFilter}
                onChange={(e) => setLocationFilter(e.target.value)}
                className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">All Locations</option>
                <option value={LocationType.Home}>🏠 Home</option>
                <option value={LocationType.Office}>🏢 Office</option>
                <option value={LocationType.Restaurant}>
                  🍽️ Restaurant
                </option>
                <option value={LocationType.Any}>📍 Any</option>
              </select>
            </div>
          </div>
        </div>

        {/* Loading State */}
        {loading && (
          <div className="text-center py-12">
            <div className="text-gray-500">Loading templates...</div>
          </div>
        )}

        {/* Error State */}
        {error && (
          <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
            <p className="text-red-800">{error}</p>
            <button
              onClick={loadTemplates}
              className="mt-2 text-sm text-red-600 hover:text-red-800 underline"
            >
              Try again
            </button>
          </div>
        )}

        {/* Templates List */}
        {!loading && !error && (
          <>
            {filteredTemplates.length === 0 ? (
              <div className="text-center py-12 bg-white rounded-lg shadow-md">
                <p className="text-gray-500 mb-4">
                  {searchQuery || locationFilter !== "all"
                    ? "No templates found matching your filters."
                    : "No templates yet. Create your first template to get started!"}
                </p>
                {!searchQuery && locationFilter === "all" && (
                  <button
                    onClick={() => {
                      setEditingTemplate(null);
                      setShowForm(true);
                    }}
                    className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
                  >
                    + Add Your First Template
                  </button>
                )}
              </div>
            ) : (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {filteredTemplates.map((template) => (
                  <div
                    key={template.id}
                    className="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow"
                  >
                    {/* Template Header */}
                    <div className="mb-4">
                      <h3 className="text-lg font-semibold text-gray-900 mb-2">
                        {template.name}
                      </h3>
                      {template.description && (
                        <p className="text-sm text-gray-600">
                          {template.description}
                        </p>
                      )}
                    </div>

                    {/* Template Details */}
                    <div className="space-y-2 mb-4">
                      {/* Location */}
                      <div className="flex items-center text-sm text-gray-700">
                        <span className="mr-2">
                          {getLocationIcon(template.location_type)}
                        </span>
                        <span className="capitalize">
                          {template.location_type}
                        </span>
                      </div>

                      {/* Compatible Slots */}
                      <div className="text-sm text-gray-700">
                        <span className="font-medium">Slots: </span>
                        <span>
                          {template.compatible_slots
                            .map(getSlotDisplayName)
                            .join(", ")}
                        </span>
                      </div>

                      {/* Weekly Limit */}
                      {template.weekly_limit && (
                        <div className="text-sm text-gray-700">
                          <span className="font-medium">Weekly limit: </span>
                          <span>{template.weekly_limit}x/week</span>
                        </div>
                      )}
                    </div>

                    {/* Actions */}
                    <div className="flex gap-2 pt-4 border-t border-gray-200">
                      <button
                        onClick={() => setSelectedTemplate(template)}
                        className="flex-1 px-3 py-2 text-sm bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors"
                      >
                        View Options
                      </button>
                      <button
                        onClick={() => {
                          setEditingTemplate(template);
                          setShowForm(true);
                        }}
                        className="px-3 py-2 text-sm text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
                      >
                        Edit
                      </button>
                      <button
                        onClick={() => handleDeleteClick(template)}
                        className="px-3 py-2 text-sm text-red-600 hover:bg-red-50 rounded-md transition-colors"
                      >
                        Delete
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </>
        )}
      </div>

      {/* Template Form Modal */}
      {showForm && (
        <TemplateForm
          template={editingTemplate}
          onClose={() => {
            setShowForm(false);
            setEditingTemplate(null);
          }}
          onSave={async () => {
            setShowForm(false);
            setEditingTemplate(null);
            await loadTemplates();
          }}
        />
      )}

      {/* Delete Confirmation Dialog */}
      {templateToDelete && (
        <ConfirmDialog
          title="Delete Template"
          message={`Are you sure you want to delete "${templateToDelete.name}"? This will also delete all associated meal options. This action cannot be undone.`}
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
