import { useEffect, useState } from "react";
import { ConfirmDialog } from "../components/common/ConfirmDialog";
import { TagForm } from "../components/templates/TagForm";
import { deleteTag, getAllTags } from "../lib/api";
import { Tag } from "../lib/types";

interface TagsViewProps {
  onBack: () => void;
}

export function TagsView({ onBack }: TagsViewProps) {
  const [tags, setTags] = useState<Tag[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Form modal state
  const [showForm, setShowForm] = useState(false);
  const [editingTag, setEditingTag] = useState<Tag | null>(null);

  // Delete confirmation state
  const [tagToDelete, setTagToDelete] = useState<Tag | null>(null);

  useEffect(() => {
    loadTags();
  }, []);

  const loadTags = async () => {
    setLoading(true);
    setError(null);
    try {
      const allTags = await getAllTags();
      setTags(allTags);
    } catch (err) {
      console.error("Failed to load tags:", err);
      setError("Failed to load tags. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteClick = (tag: Tag) => {
    setTagToDelete(tag);
  };

  const confirmDelete = async () => {
    if (!tagToDelete) return;

    try {
      await deleteTag(tagToDelete.id);
      setTagToDelete(null);
      await loadTags();
    } catch (err) {
      console.error("Failed to delete tag:", err);
      alert(
        "Failed to delete tag. It may be in use by meal options or have child tags."
      );
      setTagToDelete(null);
    }
  };

  const cancelDelete = () => {
    setTagToDelete(null);
  };

  // Organize tags by hierarchy
  const rootTags = tags.filter((tag) => !tag.parent_tag_id);
  const childTagsMap = new Map<number, Tag[]>();
  tags.forEach((tag) => {
    if (tag.parent_tag_id) {
      const children = childTagsMap.get(tag.parent_tag_id) || [];
      children.push(tag);
      childTagsMap.set(tag.parent_tag_id, children);
    }
  });

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
                  Tags Manager
                </h1>
                <p className="text-gray-600 mt-1">
                  Manage ingredient tags and weekly suggestions
                </p>
              </div>
            </div>
            <button
              onClick={() => {
                setEditingTag(null);
                setShowForm(true);
              }}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
            >
              + Add Tag
            </button>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 py-8">
        {/* Loading State */}
        {loading && (
          <div className="text-center py-12">
            <div className="text-gray-500">Loading tags...</div>
          </div>
        )}

        {/* Error State */}
        {error && (
          <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
            <p className="text-red-800">{error}</p>
            <button
              onClick={loadTags}
              className="mt-2 text-sm text-red-600 hover:text-red-800 underline"
            >
              Try again
            </button>
          </div>
        )}

        {/* Tags List */}
        {!loading && !error && (
          <>
            {tags.length === 0 ? (
              <div className="text-center py-12 bg-white rounded-lg shadow-md">
                <p className="text-gray-500 mb-4">
                  No tags yet. Create your first tag to categorize meal options
                  and track weekly limits.
                </p>
                <button
                  onClick={() => {
                    setEditingTag(null);
                    setShowForm(true);
                  }}
                  className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
                >
                  + Add Your First Tag
                </button>
              </div>
            ) : (
              <div className="bg-white rounded-lg shadow-md">
                <div className="p-6">
                  <h2 className="text-lg font-semibold text-gray-900 mb-4">
                    All Tags ({tags.length})
                  </h2>
                  <div className="space-y-2">
                    {rootTags.map((tag) => {
                      const children = childTagsMap.get(tag.id) || [];
                      return (
                        <div key={tag.id} className="space-y-2">
                          {/* Parent Tag */}
                          <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors">
                            <div className="flex-1">
                              <div className="flex items-center gap-3">
                                <span className="text-xl">🏷️</span>
                                <div>
                                  <h3 className="font-semibold text-gray-900">
                                    {tag.display_name}
                                  </h3>
                                  <p className="text-xs text-gray-500 font-mono mt-1">
                                    {tag.name}
                                  </p>
                                  {tag.weekly_suggestion !== null && (
                                    <p className="text-sm text-blue-600 mt-1">
                                      {tag.weekly_suggestion === 0
                                        ? "Suggested: avoid if possible"
                                        : `Suggested: ${tag.weekly_suggestion}x/week`}
                                    </p>
                                  )}
                                </div>
                              </div>
                            </div>
                            <div className="flex gap-2">
                              <button
                                onClick={() => {
                                  setEditingTag(tag);
                                  setShowForm(true);
                                }}
                                className="px-3 py-2 text-sm text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
                              >
                                Edit
                              </button>
                              <button
                                onClick={() => handleDeleteClick(tag)}
                                className="px-3 py-2 text-sm text-red-600 hover:bg-red-50 rounded-md transition-colors"
                              >
                                Delete
                              </button>
                            </div>
                          </div>

                          {/* Child Tags */}
                          {children.length > 0 && (
                            <div className="ml-12 space-y-2">
                              {children.map((childTag) => (
                                <div
                                  key={childTag.id}
                                  className="flex items-center justify-between p-3 bg-white border border-gray-200 rounded-lg hover:border-gray-300 transition-colors"
                                >
                                  <div className="flex-1">
                                    <div className="flex items-center gap-2">
                                      <span className="text-sm text-gray-400">
                                        ↳
                                      </span>
                                      <div>
                                        <h4 className="text-sm font-medium text-gray-900">
                                          {childTag.display_name}
                                        </h4>
                                        <p className="text-xs text-gray-500 font-mono">
                                          {childTag.name}
                                        </p>
                                        {childTag.weekly_suggestion !== null && (
                                          <p className="text-xs text-blue-600 mt-1">
                                            {childTag.weekly_suggestion === 0
                                              ? "Suggested: avoid if possible"
                                              : `Suggested: ${childTag.weekly_suggestion}x/week`}
                                          </p>
                                        )}
                                      </div>
                                    </div>
                                  </div>
                                  <div className="flex gap-2">
                                    <button
                                      onClick={() => {
                                        setEditingTag(childTag);
                                        setShowForm(true);
                                      }}
                                      className="px-3 py-1 text-xs text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
                                    >
                                      Edit
                                    </button>
                                    <button
                                      onClick={() =>
                                        handleDeleteClick(childTag)
                                      }
                                      className="px-3 py-1 text-xs text-red-600 hover:bg-red-50 rounded-md transition-colors"
                                    >
                                      Delete
                                    </button>
                                  </div>
                                </div>
                              ))}
                            </div>
                          )}
                        </div>
                      );
                    })}
                  </div>
                </div>
              </div>
            )}
          </>
        )}
      </div>

      {/* Tag Form Modal */}
      {showForm && (
        <TagForm
          tag={editingTag}
          onClose={() => {
            setShowForm(false);
            setEditingTag(null);
          }}
          onSave={async () => {
            setShowForm(false);
            setEditingTag(null);
            await loadTags();
          }}
        />
      )}

      {/* Delete Confirmation Dialog */}
      {tagToDelete && (
        <ConfirmDialog
          title="Delete Tag"
          message={`Are you sure you want to delete the tag "${tagToDelete.display_name}"? ${
            childTagsMap.get(tagToDelete.id)?.length
              ? "This will also delete all child tags. "
              : ""
          }This action cannot be undone.`}
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
