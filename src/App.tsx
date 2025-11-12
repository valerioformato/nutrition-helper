import { useState } from "react";
import { DailyView } from "./views/DailyView";
import { TemplatesView } from "./views/TemplatesView";

type View = "daily" | "templates";

function App() {
  const [currentView, setCurrentView] = useState<View>("daily");

  return (
    <div>
      {/* Temporary Navigation - Will be replaced with proper routing in Task 7 */}
      <nav className="bg-gray-800 text-white p-4">
        <div className="max-w-7xl mx-auto flex gap-4">
          <button
            onClick={() => setCurrentView("daily")}
            className={`px-4 py-2 rounded-md transition-colors ${
              currentView === "daily"
                ? "bg-blue-600"
                : "bg-gray-700 hover:bg-gray-600"
            }`}
          >
            📅 Daily View
          </button>
          <button
            onClick={() => setCurrentView("templates")}
            className={`px-4 py-2 rounded-md transition-colors ${
              currentView === "templates"
                ? "bg-blue-600"
                : "bg-gray-700 hover:bg-gray-600"
            }`}
          >
            📋 Templates Manager
          </button>
        </div>
      </nav>

      {/* View Content */}
      {currentView === "daily" && <DailyView />}
      {currentView === "templates" && <TemplatesView />}
    </div>
  );
}

export default App;

