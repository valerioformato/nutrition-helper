-- Add weekly_limit column to meal_templates table
-- This is a hard constraint that blocks entry creation if exceeded
-- NULL means unlimited, positive integers enforce the limit

ALTER TABLE meal_templates 
ADD COLUMN weekly_limit INTEGER CHECK(weekly_limit IS NULL OR weekly_limit > 0);

-- Add index for better query performance when checking limits
CREATE INDEX IF NOT EXISTS idx_meal_templates_weekly_limit 
ON meal_templates(weekly_limit) 
WHERE weekly_limit IS NOT NULL;
