-- Your SQL goes here
-- ===== Tasks =====
CREATE TABLE IF NOT EXISTS tasks (
  id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title             TEXT NOT NULL,
  description       TEXT,
  status_id         INTEGER REFERENCES task_statuses(id),
  position          INTEGER DEFAULT 1000,
  finish_at_initial TEXT,
  finish_at         TEXT,
  created_at        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  time_spent        INTEGER DEFAULT 0
);
