-- Allow weekly_suggestion to be 0 (meaning "avoid this ingredient")
-- Previous constraint: CHECK(weekly_suggestion IS NULL OR weekly_suggestion > 0)
-- New constraint: CHECK(weekly_suggestion IS NULL OR weekly_suggestion >= 0)

-- SQLite doesn't support ALTER TABLE to modify constraints directly,
-- so we need to recreate the table

-- Step 0: Drop views that depend on tags table
DROP VIEW IF EXISTS weekly_tag_usage;

-- Step 1: Create new tags table with updated constraint
CREATE TABLE tags_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    category TEXT NOT NULL CHECK(category IN ('ingredient', 'dietary', 'prep_time', 'other')),
    weekly_suggestion INTEGER CHECK(weekly_suggestion IS NULL OR weekly_suggestion >= 0),
    parent_tag_id INTEGER,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_tag_id) REFERENCES tags_new(id) ON DELETE SET NULL
);

-- Step 2: Copy data from old table
INSERT INTO tags_new (id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at)
SELECT id, name, display_name, category, weekly_suggestion, parent_tag_id, created_at
FROM tags;

-- Step 3: Drop old table
DROP TABLE tags;

-- Step 4: Rename new table
ALTER TABLE tags_new RENAME TO tags;

-- Step 5: Recreate indexes
CREATE INDEX IF NOT EXISTS idx_tags_category ON tags(category);
CREATE INDEX IF NOT EXISTS idx_tags_parent ON tags(parent_tag_id);

-- Step 6: Recreate views
CREATE VIEW IF NOT EXISTS weekly_tag_usage AS
SELECT
    t.id as tag_id,
    t.name as tag_name,
    strftime('%Y-%W', me.date) as week,
    COUNT(*) as usage_count
FROM tags t
JOIN meal_option_tags mot ON t.id = mot.tag_id
JOIN meal_options mo ON mot.meal_option_id = mo.id
JOIN meal_entries me ON mo.id = me.meal_option_id
WHERE me.completed = 1
GROUP BY t.id, t.name, week;
