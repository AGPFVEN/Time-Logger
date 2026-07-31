-- Your SQL goes here
-- ===== Tasks relation (many2many) =====
CREATE TABLE task_task_dependencies (
  parent_id INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  child_id  INTEGER NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
  PRIMARY KEY (parent_id, child_id),
  CONSTRAINT no_self_loop CHECK (parent_id <> child_id)
);
