-- Your SQL goes here
-- ===== Project Statuses =====
CREATE TABLE IF NOT EXISTS project_statuses (
  id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  key       TEXT NOT NULL UNIQUE,
  label     TEXT NOT NULL,
  is_closed BOOLEAN NOT NULL DEFAULT 0
);
