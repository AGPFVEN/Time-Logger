-- Your SQL goes here
-- ===== Time Entries =====
CREATE TABLE IF NOT EXISTS time_entries (
  id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  task_id     INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  start_time  TEXT NOT NULL,
  end_time    TEXT,
  description TEXT,
  created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
