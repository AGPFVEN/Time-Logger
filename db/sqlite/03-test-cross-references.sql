-- Assign tasks to projects
-- (Notice how TSK-1 is shared by both Web and App projects)
INSERT INTO project_task_dependencies (project_id, task_id) VALUES 
  ('PRJ-WEB', 'TSK-1'),
  ('PRJ-WEB', 'TSK-2'),
  ('PRJ-WEB', 'TSK-3'),
  ('PRJ-APP', 'TSK-1'), 
  ('PRJ-APP', 'TSK-4');

-- Attach tags to tasks
INSERT INTO task_tags (task_id, tag_id) VALUES 
  ('TSK-1', 1), -- Task 1 -> #backend
  ('TSK-2', 1), -- Task 2 -> #backend
  ('TSK-2', 2), -- Task 2 -> #urgent (multi-tag)
  ('TSK-4', 3); -- Task 4 -> #tech-debt

-- Attach tag to a project
INSERT INTO project_tags (project_id, tag_id) VALUES 
  ('PRJ-WEB', 2); -- Web Project -> #urgent

-- Create task hierarchy (TSK-2 depends on TSK-1 completion)
INSERT INTO task_task_dependencies (parent_id, child_id) VALUES 
  ('TSK-1', 'TSK-2'),
  ('TSK-2', 'TSK-3');
  