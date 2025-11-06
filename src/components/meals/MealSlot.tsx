import { SlotType } from "../../lib/types";

interface MealSlotProps {
  slotType: SlotType;
  slotName: string;
  isEmpty: boolean;
  onAddMeal?: () => void;
  onClick?: () => void;
  children?: React.ReactNode;
}

export function MealSlot({
  slotType,
  slotName,
  isEmpty,
  onAddMeal,
  onClick,
  children,
}: MealSlotProps) {
  const handleClick = () => {
    if (isEmpty && onAddMeal) {
      onAddMeal();
    } else if (!isEmpty && onClick) {
      onClick();
    }
  };

  // Color classes based on slot type
  const getSlotColor = () => {
    switch (slotType) {
      case SlotType.Breakfast:
        return "border-yellow-400 bg-yellow-50";
      case SlotType.MorningSnack:
        return "border-green-400 bg-green-50";
      case SlotType.Lunch:
        return "border-blue-400 bg-blue-50";
      case SlotType.AfternoonSnack:
        return "border-green-400 bg-green-50";
      case SlotType.Dinner:
        return "border-purple-400 bg-purple-50";
    }
  };

  const getSlotAccent = () => {
    switch (slotType) {
      case SlotType.Breakfast:
        return "text-yellow-700";
      case SlotType.MorningSnack:
        return "text-green-700";
      case SlotType.Lunch:
        return "text-blue-700";
      case SlotType.AfternoonSnack:
        return "text-green-700";
      case SlotType.Dinner:
        return "text-purple-700";
    }
  };

  return (
    <div
      className={`
        border-2 rounded-lg p-4 transition-all
        ${getSlotColor()}
        ${isEmpty ? "border-dashed" : "border-solid"}
        ${onClick || onAddMeal ? "cursor-pointer hover:shadow-md" : ""}
      `}
      onClick={handleClick}
    >
      {/* Slot Header */}
      <div className="flex items-center justify-between mb-3">
        <h3 className={`font-semibold text-lg ${getSlotAccent()}`}>
          {slotName}
        </h3>
      </div>

      {/* Content */}
      {isEmpty ? (
        <div className="flex items-center justify-center py-6">
          <button
            className={`
              px-4 py-2 rounded-md font-medium transition-colors
              ${getSlotAccent()}
              hover:bg-white hover:shadow-sm
            `}
            onClick={(e) => {
              e.stopPropagation();
              if (onAddMeal) onAddMeal();
            }}
          >
            + Add Meal
          </button>
        </div>
      ) : (
        <div className="space-y-2">{children}</div>
      )}
    </div>
  );
}
