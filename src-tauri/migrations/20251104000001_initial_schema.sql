-- Initial database schema for Nutrition Helper
-- Creates tables for meal templates and meal entries

-- Meal Templates (meal options available to choose from)
CREATE TABLE IF NOT EXISTS meal_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL CHECK(category IN ('breakfast', 'lunch', 'dinner', 'snack')),
    location_type TEXT NOT NULL CHECK(location_type IN ('home', 'office', 'restaurant', 'any')),
    weekly_limit INTEGER CHECK(weekly_limit IS NULL OR weekly_limit > 0),
    nutritional_notes TEXT,
    tags TEXT, -- JSON array stored as text
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Meal Entries (actual meals consumed/planned)
CREATE TABLE IF NOT EXISTS meal_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date DATE NOT NULL,
    slot_type TEXT NOT NULL CHECK(slot_type IN ('breakfast', 'morning_snack', 'lunch', 'afternoon_snack', 'dinner')),
    meal_template_id INTEGER NOT NULL,
    location TEXT NOT NULL CHECK(location IN ('home', 'office', 'restaurant', 'any')),
    portion_size REAL CHECK(portion_size IS NULL OR portion_size > 0),
    portion_unit TEXT,
    notes TEXT,
    completed BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (meal_template_id) REFERENCES meal_templates(id) ON DELETE RESTRICT
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_meal_entries_date ON meal_entries(date);
CREATE INDEX IF NOT EXISTS idx_meal_entries_template ON meal_entries(meal_template_id);
CREATE INDEX IF NOT EXISTS idx_meal_entries_date_slot ON meal_entries(date, slot_type);
CREATE INDEX IF NOT EXISTS idx_meal_templates_category ON meal_templates(category);
CREATE INDEX IF NOT EXISTS idx_meal_templates_location ON meal_templates(location_type);

-- View for tracking weekly meal usage (for enforcing limits)
CREATE VIEW IF NOT EXISTS weekly_meal_usage AS
SELECT
    meal_template_id,
    strftime('%Y-%W', date) as week,
    COUNT(*) as usage_count
FROM meal_entries
WHERE completed = 1
GROUP BY meal_template_id, week;

-- Trigger to update updated_at timestamp on meal_templates
CREATE TRIGGER IF NOT EXISTS update_meal_templates_timestamp
AFTER UPDATE ON meal_templates
FOR EACH ROW
BEGIN
    UPDATE meal_templates SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

-- Trigger to update updated_at timestamp on meal_entries
CREATE TRIGGER IF NOT EXISTS update_meal_entries_timestamp
AFTER UPDATE ON meal_entries
FOR EACH ROW
BEGIN
    UPDATE meal_entries SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
