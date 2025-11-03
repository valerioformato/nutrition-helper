import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [loading, setLoading] = useState(false);

  async function testBackend() {
    setLoading(true);
    try {
      const response = await invoke<string>("greet", { name: "User" });
      setGreetMsg(response);
    } catch (error) {
      setGreetMsg(`Error: ${error}`);
    } finally {
      setLoading(false);
    }
  }

  return (
        <div className="min-h-screen bg-gray-100 flex items-center justify-center">
            <div className="bg-white p-8 rounded-lg shadow-lg max-w-md w-full">
                <h1 className="text-3xl font-bold text-gray-800 mb-4">
                    Nutrition Helper
                </h1>
                <p className="text-gray-600 mb-6">
                    Your meal planning companion - Phase 0 Setup Complete! 🎉
                </p>

                <div className="space-y-4">
                    <div className="p-4 bg-blue-50 rounded-md border border-blue-200">
                        <h2 className="text-lg font-semibold text-blue-800 mb-2">
                            Ready to Build
                        </h2>
                        <p className="text-sm text-blue-600">
                            Tauri + React + TypeScript project initialized successfully.
                        </p>
                    </div>

                    {greetMsg && (
                        <div className="p-4 bg-green-50 rounded-md border border-green-200">
                            <p className="text-sm text-green-700">{greetMsg}</p>
                        </div>
                    )}

                    <button
                        onClick={testBackend}
                        disabled={loading}
                        className="w-full bg-blue-500 hover:bg-blue-600 text-white font-medium py-2 px-4 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {loading ? "Testing..." : "Test Backend Communication"}
                    </button>
                </div>
            </div>
        </div>
    );
}

export default App;
