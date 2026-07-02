-- ===== Task Statuses =====
CREATE TABLE IF NOT EXISTS task_statuses (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  key       TEXT NOT NULL UNIQUE,
  label     TEXT NOT NULL,
  is_closed INTEGER NOT NULL DEFAULT 0
);

-- ===== Tasks =====
CREATE TABLE IF NOT EXISTS tasks (
  id                TEXT PRIMARY KEY,
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

-- ===== Tasks relation (many2many) =====
CREATE TABLE task_task_dependencies (
  parent_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  child_id  TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  PRIMARY KEY (parent_id, child_id),
  CONSTRAINT no_self_loop CHECK (parent_id <> child_id)
);

-- ===== Project Statuses =====
CREATE TABLE IF NOT EXISTS project_statuses (
  id        INTEGER PRIMARY KEY AUTOINCREMENT,
  key       TEXT NOT NULL UNIQUE,
  label     TEXT NOT NULL,
  is_closed INTEGER NOT NULL DEFAULT 0
);

-- ===== Projects =====
CREATE TABLE IF NOT EXISTS projects (
  id          TEXT PRIMARY KEY,
  name        TEXT NOT NULL,
  description TEXT,
  status_id   INTEGER REFERENCES project_statuses(id),
  finish_at   TEXT,
  created_at        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at        TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  time_spent        INTEGER DEFAULT 0
);

-- ===== Tasks-Projects relation (many2many) =====
CREATE TABLE project_task_dependencies (
  project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  task_id  TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  PRIMARY KEY (project_id, task_id),
  CONSTRAINT no_self_loop CHECK (project_id <> task_id)
);

-- ===== Tags =====
CREATE TABLE IF NOT EXISTS tags (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  name       TEXT NOT NULL UNIQUE,
  color_hex  TEXT DEFAULT '#808080',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ===== Tasks-Tags relation (many2many) =====
CREATE TABLE IF NOT EXISTS task_tags (
  task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  tag_id  INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (task_id, tag_id)
);

-- ===== Projects-Tags relation (many2many) =====
CREATE TABLE IF NOT EXISTS project_tags (
  project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  tag_id     INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (project_id, tag_id)
);
