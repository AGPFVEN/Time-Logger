-- Your SQL goes here
-- ===== Tasks-Projects relation (many2many) =====
CREATE TABLE project_task_dependencies (
  project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
  task_id  INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  PRIMARY KEY (project_id, task_id),
  CONSTRAINT no_self_loop CHECK (project_id <> task_id)
);
