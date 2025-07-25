-- Add migration script here
ALTER TABLE todos ADD COLUMN due_date DATETIME;
ALTER TABLE todos ADD COLUMN completion_date DATETIME;
UPDATE todos set completion_date = DateTime('now') where completed = 1;
ALTER TABLE todos DROP COLUMN completed;
