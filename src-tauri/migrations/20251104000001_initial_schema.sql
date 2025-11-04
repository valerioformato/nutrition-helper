-- Initial database schema for Nutrition Helper
-- Four-level hierarchy: Slot → Template → Option → Entry

-- Level 1: Meal Slots (fixed 5 slots per day)
-- These are implicitly defined in the application logic:
-- 'breakfast', 'morning_snack', 'lunch', 'afternoon_snack', 'dinner'

-- Level 2: Meal Templates (the "cards" that fill slots, separated by "Oppure")
-- Example: "Pane con marmellata e formaggio spalmabile"
-- Each template can be compatible with one or more meal slots
CREATE TABLE IF NOT EXISTS meal_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    compatible_slots TEXT NOT NULL, -- JSON array of SlotType values
    location_type TEXT NOT NULL CHECK(location_type IN ('home', 'office', 'restaurant', 'any')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Level 3: Meal Options (ingredient/variation choices within a template)
-- Example: "philadelphia", "ricotta", "crema spalmabile 100% frutta secca"
-- These are the specific choices separated by "o", "e/o", "+", etc. within each template
CREATE TABLE IF NOT EXISTS meal_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    template_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    nutritional_notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (template_id) REFERENCES meal_templates(id) ON DELETE CASCADE
);

-- Tags System: For tracking ingredients, dietary restrictions, and frequency suggestions
-- Tags can have parent-child relationships (e.g., "pasta_integrale" -> "pasta")
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,              -- Internal key: "pasta", "pasta_integrale", "ricotta"
    display_name TEXT NOT NULL,             -- User-facing: "Pasta", "Pasta Integrale", "Ricotta"
    category TEXT NOT NULL CHECK(category IN ('ingredient', 'dietary', 'prep_time', 'other')),
    weekly_suggestion INTEGER CHECK(weekly_suggestion IS NULL OR weekly_suggestion > 0),
    parent_tag_id INTEGER,                  -- For hierarchies: pasta_integrale references pasta
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_tag_id) REFERENCES tags(id) ON DELETE SET NULL
);

-- Many-to-many relationship: meal options can have multiple tags
CREATE TABLE IF NOT EXISTS meal_option_tags (
    meal_option_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (meal_option_id, tag_id),
    FOREIGN KEY (meal_option_id) REFERENCES meal_options(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Level 4: Meal Entries (actual meal options logged/consumed)
-- This is when the user logs which specific option they chose on a given date/slot
CREATE TABLE IF NOT EXISTS meal_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    meal_option_id INTEGER NOT NULL,
    date DATE NOT NULL,
    slot_type TEXT NOT NULL CHECK(slot_type IN ('breakfast', 'morning_snack', 'lunch', 'afternoon_snack', 'dinner')),
    location TEXT NOT NULL CHECK(location IN ('home', 'office', 'restaurant', 'any')),
    portion_size REAL CHECK(portion_size IS NULL OR portion_size > 0),
    portion_unit TEXT,
    notes TEXT,
    completed BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (meal_option_id) REFERENCES meal_options(id) ON DELETE RESTRICT
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_meal_entries_date ON meal_entries(date);
CREATE INDEX IF NOT EXISTS idx_meal_entries_option ON meal_entries(meal_option_id);
CREATE INDEX IF NOT EXISTS idx_meal_entries_date_slot ON meal_entries(date, slot_type);
CREATE INDEX IF NOT EXISTS idx_meal_options_template ON meal_options(template_id);
CREATE INDEX IF NOT EXISTS idx_meal_templates_location ON meal_templates(location_type);
CREATE INDEX IF NOT EXISTS idx_tags_category ON tags(category);
CREATE INDEX IF NOT EXISTS idx_tags_parent ON tags(parent_tag_id);
CREATE INDEX IF NOT EXISTS idx_meal_option_tags_option ON meal_option_tags(meal_option_id);
CREATE INDEX IF NOT EXISTS idx_meal_option_tags_tag ON meal_option_tags(tag_id);

-- View for tracking weekly meal option usage
CREATE VIEW IF NOT EXISTS weekly_meal_usage AS
SELECT
    meal_option_id,
    strftime('%Y-%W', date) as week,
    COUNT(*) as usage_count
FROM meal_entries
WHERE completed = 1
GROUP BY meal_option_id, week;

-- View for tracking weekly tag usage (for frequency suggestions)
-- This counts how many times each tag was used, including parent tag counts
CREATE VIEW IF NOT EXISTS weekly_tag_usage AS
SELECT
    t.id as tag_id,
    t.name as tag_name,
    strftime('%Y-%W', me.date) as week,
    COUNT(*) as usage_count
FROM meal_entries me
JOIN meal_option_tags mot ON me.meal_option_id = mot.meal_option_id
JOIN tags t ON mot.tag_id = t.id
WHERE me.completed = 1
GROUP BY t.id, week;

-- Triggers to update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_meal_templates_timestamp
AFTER UPDATE ON meal_templates
FOR EACH ROW
BEGIN
    UPDATE meal_templates SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER IF NOT EXISTS update_meal_options_timestamp
AFTER UPDATE ON meal_options
FOR EACH ROW
BEGIN
    UPDATE meal_options SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER IF NOT EXISTS update_meal_entries_timestamp
AFTER UPDATE ON meal_entries
FOR EACH ROW
BEGIN
    UPDATE meal_entries SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

-- No trigger for tags as they rarely change once created
