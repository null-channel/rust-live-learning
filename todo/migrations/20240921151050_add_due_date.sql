-- Add migration script here
ALTER TABLE todos ADD COLUMN due_date DATE;
ALTER TABLE todos ADD COLUMN completion_date DATE;
UPDATE todos set completion_date = '2024-09-21' where completed = 1;
ALTER TABLE todos DROP COLUMN completed;
