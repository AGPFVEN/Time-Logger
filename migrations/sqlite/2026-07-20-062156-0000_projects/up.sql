-- Your SQL goes here
-- ===== Projects =====
CREATE TABLE IF NOT EXISTS projects (
  id          INTEGER PRIMARY KEY NOT NULL,
  name        TEXT NOT NULL,
  description TEXT,
  status_id   INTEGER REFERENCES project_statuses(id),
  finish_at   TEXT,
  created_at        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  time_spent        INTEGER DEFAULT 0
);
