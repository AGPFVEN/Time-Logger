-- Your SQL goes here
ALTER TABLE time_entries ADD COLUMN duration INTEGER;

--  Calculate and Update column for time entries
UPDATE time_entries
SET duration = CAST(strftime('%s', end_time) - strftime('%s', start_time) AS INTEGER)
WHERE end_time IS NOT NULL AND start_time IS NOT NULL;
