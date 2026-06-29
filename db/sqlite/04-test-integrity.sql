-- ERROR TEST A: Attempt to duplicate an existing tag
-- Expected: [SQLITE_CONSTRAINT_UNIQUE] UNIQUE constraint failed: tags.name
INSERT INTO tags (name) VALUES ('backend');

-- ERROR TEST B: Attempt to make a task its own sub-task
-- Expected: [SQLITE_CONSTRAINT_CHECK] CHECK constraint failed: no_self_loop
INSERT INTO task_task_dependencies (parent_id, child_id) VALUES ('TSK-1', 'TSK-1');
