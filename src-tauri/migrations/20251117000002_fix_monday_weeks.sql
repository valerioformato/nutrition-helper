-- Fix weekly views to properly use Monday-based weeks
-- SQLite's %W uses Sunday as first day (week runs Sun-Sat)
-- We need Monday as first day (week runs Mon-Sun)
-- Solution: Shift dates back by 1 day for %W calculation, so Monday becomes "Sunday" in %W's view

-- Drop existing views
DROP VIEW IF EXISTS weekly_meal_usage;
DROP VIEW IF EXISTS weekly_tag_usage;

-- Recreate weekly_meal_usage with proper Monday-based weeks
-- For %W calculation, shift all dates back 1 day so Monday=day0, Sunday=day6
CREATE VIEW IF NOT EXISTS weekly_meal_usage AS
SELECT
    meal_option_id,
    strftime('%Y-', date(date, '-1 day')) || printf('%02d', CAST(strftime('%W', date(date, '-1 day')) AS INTEGER)) as week,
    COUNT(*) as usage_count
FROM meal_entries
WHERE completed = 1
GROUP BY meal_option_id, week;

-- Recreate weekly_tag_usage with proper Monday-based weeks
CREATE VIEW IF NOT EXISTS weekly_tag_usage AS
SELECT
    t.id as tag_id,
    t.name as tag_name,
    strftime('%Y-', date(me.date, '-1 day')) || printf('%02d', CAST(strftime('%W', date(me.date, '-1 day')) AS INTEGER)) as week,
    COUNT(*) as usage_count
FROM meal_entries me
JOIN meal_option_tags mot ON me.meal_option_id = mot.meal_option_id
JOIN tags t ON mot.tag_id = t.id
WHERE me.completed = 1
GROUP BY t.id, t.name, week;
