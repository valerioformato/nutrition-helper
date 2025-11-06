import { useEffect, useState } from "react";
import { getAllTemplates } from "../../lib/api";
import { MealTemplate, SlotType } from "../../lib/types";
import { Modal } from "../common/Modal";

interface MealSelectionModalProps {
  isOpen: boolean;
  onClose: () => void;
  slotType: SlotType;
  slotName: string;
  onSelectTemplate: (template: MealTemplate) => void;
}

export function MealSelectionModal({
  isOpen,
  onClose,
  slotType,
  slotName,
  onSelectTemplate,
}: MealSelectionModalProps) {
  const [templates, setTemplates] = useState<MealTemplate[]>([]);
  const [filteredTemplates, setFilteredTemplates] = useState<MealTemplate[]>(
    []
  );
  const [searchQuery, setSearchQuery] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Fetch templates when modal opens
  useEffect(() => {
    if (isOpen) {
      loadTemplates();
    }
  }, [isOpen]);

  // Filter templates based on search query and slot compatibility
  useEffect(() => {
    let filtered = templates.filter((template) =>
      template.compatible_slots.includes(slotType)
    );

    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        (template) =>
          template.name.toLowerCase().includes(query) ||
          (template.description &&
            template.description.toLowerCase().includes(query))
      );
    }

    setFilteredTemplates(filtered);
  }, [templates, searchQuery, slotType]);

  const loadTemplates = async () => {
    setLoading(true);
    setError(null);
    try {
      const allTemplates = await getAllTemplates();
      setTemplates(allTemplates);
    } catch (err) {
      console.error("Failed to load templates:", err);
      setError("Failed to load meal templates. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  const handleSelectTemplate = (template: MealTemplate) => {
    onSelectTemplate(template);
    onClose();
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={`Add Meal to ${slotName}`}>
      {/* Search Bar */}
      <div className="mb-4">
        <input
          type="text"
          placeholder="Search templates..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      {/* Loading State */}
      {loading && (
        <div className="flex items-center justify-center py-8">
          <div className="text-gray-500">Loading templates...</div>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-4">
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
            <div className="text-center py-8 text-gray-500">
              {searchQuery
                ? "No templates found matching your search."
                : "No templates available for this meal slot."}
            </div>
          ) : (
            <div className="space-y-2">
              {filteredTemplates.map((template) => (
                <button
                  key={template.id}
                  onClick={() => handleSelectTemplate(template)}
                  className="w-full text-left p-4 border border-gray-200 rounded-md hover:bg-blue-50 hover:border-blue-300 transition-colors"
                >
                  <h3 className="font-semibold text-gray-900">
                    {template.name}
                  </h3>
                  {template.description && (
                    <p className="text-sm text-gray-600 mt-1">
                      {template.description}
                    </p>
                  )}
                  <div className="flex items-center gap-2 mt-2 text-xs text-gray-500">
                    <span>
                      {template.location_type === "any"
                        ? "📍 Any location"
                        : template.location_type === "home"
                        ? "🏠 Home"
                        : template.location_type === "office"
                        ? "🏢 Office"
                        : "🍽️ Restaurant"}
                    </span>
                    {template.weekly_limit && (
                      <span>• Max {template.weekly_limit}x/week</span>
                    )}
                  </div>
                </button>
              ))}
            </div>
          )}
        </>
      )}
    </Modal>
  );
}
