-- Fix weekly views to use Monday-based weeks (ISO 8601)
-- SQLite's %W uses Sunday as first day of week, but we need Monday

-- Drop existing views
DROP VIEW IF EXISTS weekly_meal_usage;
DROP VIEW IF EXISTS weekly_tag_usage;

-- Recreate weekly_meal_usage with Monday-based weeks
-- Strategy: Use %W (week number) but adjust by checking if day is Sunday
-- If Sunday (day 0), it belongs to next week in Monday-based system
CREATE VIEW IF NOT EXISTS weekly_meal_usage AS
SELECT
    meal_option_id,
    CASE 
        WHEN CAST(strftime('%w', date) AS INTEGER) = 0 
        THEN strftime('%Y-', date(date, '+1 day')) || printf('%02d', CAST(strftime('%W', date(date, '+1 day')) AS INTEGER))
        ELSE strftime('%Y-') || printf('%02d', CAST(strftime('%W', date) AS INTEGER))
    END as week,
    COUNT(*) as usage_count
FROM meal_entries
WHERE completed = 1
GROUP BY meal_option_id, week;

-- Recreate weekly_tag_usage with Monday-based weeks
CREATE VIEW IF NOT EXISTS weekly_tag_usage AS
SELECT
    t.id as tag_id,
    t.name as tag_name,
    CASE 
        WHEN CAST(strftime('%w', me.date) AS INTEGER) = 0 
        THEN strftime('%Y-', date(me.date, '+1 day')) || printf('%02d', CAST(strftime('%W', date(me.date, '+1 day')) AS INTEGER))
        ELSE strftime('%Y-') || printf('%02d', CAST(strftime('%W', me.date) AS INTEGER))
    END as week,
    COUNT(*) as usage_count
FROM meal_entries me
JOIN meal_option_tags mot ON me.meal_option_id = mot.meal_option_id
JOIN tags t ON mot.tag_id = t.id
WHERE me.completed = 1
GROUP BY t.id, t.name, week;
