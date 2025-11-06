import { MealEntry, MealOption, MealTemplate, LocationType } from "../../lib/types";

interface MealCardProps {
  entry: MealEntry;
  option: MealOption;
  template: MealTemplate;
  onClick?: () => void;
}

export function MealCard({ entry, option, template, onClick }: MealCardProps) {
  // Get location icon and label
  const getLocationDisplay = (location: LocationType) => {
    switch (location) {
      case LocationType.Home:
        return { icon: "🏠", label: "Home" };
      case LocationType.Office:
        return { icon: "🏢", label: "Office" };
      case LocationType.Restaurant:
        return { icon: "🍽️", label: "Restaurant" };
      case LocationType.Any:
        return { icon: "📍", label: "Any" };
    }
  };

  const location = getLocationDisplay(entry.location);

  return (
    <div
      className={`
        bg-white rounded-md border-2 border-gray-200 p-4
        transition-all hover:shadow-md
        ${onClick ? "cursor-pointer" : ""}
        ${entry.completed ? "bg-green-50 border-green-300" : ""}
      `}
      onClick={onClick}
    >
      {/* Template and Option Names */}
      <div className="mb-2">
        <h4 className="font-semibold text-gray-900 text-base">
          {option.name}
        </h4>
        <p className="text-sm text-gray-600">from {template.name}</p>
      </div>

      {/* Details Row */}
      <div className="flex items-center gap-3 text-sm text-gray-700 flex-wrap">
        {/* Location */}
        <div className="flex items-center gap-1">
          <span>{location.icon}</span>
          <span>{location.label}</span>
        </div>

        {/* Servings */}
        <div className="flex items-center gap-1">
          <span>🍽️</span>
          <span>{entry.servings} serving{entry.servings !== 1 ? "s" : ""}</span>
        </div>

        {/* Completed Badge */}
        {entry.completed && (
          <div className="flex items-center gap-1 px-2 py-0.5 bg-green-100 text-green-800 rounded-full text-xs font-medium">
            <span>✓</span>
            <span>Completed</span>
          </div>
        )}
      </div>

      {/* Notes (if present) */}
      {entry.notes && (
        <div className="mt-3 pt-3 border-t border-gray-200">
          <p className="text-sm text-gray-600 italic">
            {entry.notes}
          </p>
        </div>
      )}

      {/* Option Description (if present) */}
      {option.description && (
        <div className="mt-2">
          <p className="text-xs text-gray-500">
            {option.description}
          </p>
        </div>
      )}
    </div>
  );
}
