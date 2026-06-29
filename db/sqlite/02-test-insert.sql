-- MANDATORY IN SQLITE: Enable Foreign Key enforcement
PRAGMA foreign_keys = ON;

-- Seed status dictionaries
INSERT INTO project_statuses (key, label, is_closed) VALUES 
  ('planning', 'In Planning', 0),
  ('active', 'In Development', 0),
  ('closed', 'Completed', 1);

INSERT INTO task_statuses (key, label, is_closed) VALUES 
  ('todo', 'To Do', 0),
  ('doing', 'In Progress', 0),
  ('done', 'Done', 1);

-- Seed tags
INSERT INTO tags (name, color_hex) VALUES 
  ('backend', '#0080FF'),
  ('urgent', '#FF0000'),
  ('tech-debt', '#E0E0E0');

-- Create 2 projects
INSERT INTO projects (id, name, description, status_id) VALUES 
  ('PRJ-WEB', 'Corporate Portal', 'Full redesign 2026', 2),
  ('PRJ-APP', 'Customer App', 'iOS Version', 1);

-- Create 4 flat tasks
INSERT INTO tasks (id, title, status_id, time_spent) VALUES 
  ('TSK-1', 'Design database schema', 3, 120),
  ('TSK-2', 'Implement user endpoints', 2, 300),
  ('TSK-3', 'Layout login screen', 1, 0),
  ('TSK-4', 'Configure CI/CD pipeline', 1, 45);
  