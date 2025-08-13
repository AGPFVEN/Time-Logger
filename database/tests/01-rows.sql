BEGIN;

-- ----- Status dictionaries -----
INSERT INTO project_statuses (key, label, is_closed) VALUES
  ('planned',   'Planned',   false),
  ('active',    'Active',    false),
  ('on_hold',   'On hold',   false),
  ('completed', 'Completed', true),
  ('cancelled', 'Cancelled', true)
ON CONFLICT (key) DO NOTHING;

INSERT INTO task_statuses (key, label, is_closed) VALUES
  ('todo',        'To do',        false),
  ('in_progress', 'In progress',  false),
  ('blocked',     'Blocked',      false),
  ('done',        'Done',         true),
  ('cancelled',   'Cancelled',    true)
ON CONFLICT (key) DO NOTHING;

-- ----- Tags -----
INSERT INTO tags (name, color, description) VALUES
  ('frontend', '#3b82f6', 'UI and client code'),
  ('backend',  '#22c55e', 'APIs and services'),
  ('urgent',   '#ef4444', 'High priority')
ON CONFLICT (name) DO NOTHING;

-- ----- Projects -----
INSERT INTO projects (name, description, status_id, finish_at)
VALUES
  ('Website Redesign',
   'Revamp marketing site with new branding',
   (SELECT id FROM project_statuses WHERE key='active'),
   '2025-10-01 00:00+00'),
  ('Mobile App MVP',
   'Initial MVP for iOS/Android',
   (SELECT id FROM project_statuses WHERE key='planned'),
   '2025-12-15 00:00+00');

-- tag projects
INSERT INTO project_tags (project_id, tag_id)
SELECT p.id, t.id
FROM projects p
JOIN tags t ON t.name IN ('frontend','backend')
WHERE p.name = 'Website Redesign'
ON CONFLICT DO NOTHING;

-- ----- Tasks for "Website Redesign" -----
-- parents first
INSERT INTO tasks (project_id, title, status_id, position, finish_at_initial)
SELECT id,
       'Design Phase',
       (SELECT id FROM task_statuses WHERE key='in_progress'),
       10,
       '2025-08-20 00:00+00'
FROM projects WHERE name='Website Redesign';

INSERT INTO tasks (project_id, title, status_id, position, finish_at_initial)
SELECT id,
       'Implementation',
       (SELECT id FROM task_statuses WHERE key='todo'),
       20,
       '2025-09-15 00:00+00'
FROM projects WHERE name='Website Redesign';

-- children referencing parent by title within same project
INSERT INTO tasks (project_id, parent_id, title, status_id, position, finish_at_initial, finish_at)
SELECT p.id,
       (SELECT id FROM tasks WHERE project_id=p.id AND title='Design Phase'),
       'Wireframes',
       (SELECT id FROM task_statuses WHERE key='done'),
       11,
       '2025-08-10 00:00+00',
       '2025-08-09 12:00+00'
FROM projects p WHERE p.name='Website Redesign';

INSERT INTO tasks (project_id, parent_id, title, status_id, position, finish_at_initial)
SELECT p.id,
       (SELECT id FROM tasks WHERE project_id=p.id AND title='Design Phase'),
       'UI Mockups',
       (SELECT id FROM task_statuses WHERE key='in_progress'),
       12,
       '2025-08-18 00:00+00'
FROM projects p WHERE p.name='Website Redesign';

INSERT INTO tasks (project_id, parent_id, title, status_id, position)
SELECT p.id,
       (SELECT id FROM tasks WHERE project_id=p.id AND title='Implementation'),
       'Frontend',
       (SELECT id FROM task_statuses WHERE key='todo'),
       21
FROM projects p WHERE p.name='Website Redesign';

INSERT INTO tasks (project_id, parent_id, title, status_id, position)
SELECT p.id,
       (SELECT id FROM tasks WHERE project_id=p.id AND title='Implementation'),
       'Backend',
       (SELECT id FROM task_statuses WHERE key='todo'),
       22
FROM projects p WHERE p.name='Website Redesign';

-- tag tasks
INSERT INTO task_tags (task_id, tag_id)
SELECT t.id, tg.id
FROM tasks t
JOIN projects p ON p.id=t.project_id AND p.name='Website Redesign'
JOIN tags tg ON tg.name IN ('frontend','urgent')
WHERE t.title IN ('UI Mockups','Frontend')
ON CONFLICT DO NOTHING;

-- ----- Time entries (one finished, one running) -----
-- finished entry for "UI Mockups"
INSERT INTO time_entries (project_id, task_id, started_at, ended_at, description)
SELECT p.id,
       (SELECT id FROM tasks WHERE project_id=p.id AND title='UI Mockups'),
       '2025-08-01 09:00+00',
       '2025-08-01 11:30+00',
       'Explored color palette'
FROM projects p WHERE p.name='Website Redesign';

-- running entry (no ended_at) for "Frontend"
INSERT INTO time_entries (project_id, task_id, started_at, description)
SELECT p.id,
       (SELECT id FROM tasks WHERE project_id=p.id AND title='Frontend'),
       '2025-08-13 08:00+00',
       'Implementing navbar'
FROM projects p WHERE p.name='Website Redesign';

-- ----- Project progress pages -----
INSERT INTO project_progress_pages (project_id, title, content_md, progress_percent, period_start, period_end)
SELECT p.id,
       'Week 1 – Kickoff',
       'Started with **wireframes** and brand exploration.',
       15.00,
       DATE '2025-07-29',
       DATE '2025-08-04'
FROM projects p WHERE p.name='Website Redesign';

INSERT INTO project_progress_pages (project_id, title, content_md, progress_percent, period_start, period_end)
SELECT p.id,
       'Week 2 – Visual design',
       'UI mockups in progress; backend planning started.',
       28.50,
       DATE '2025-08-05',
       DATE '2025-08-11'
FROM projects p WHERE p.name='Website Redesign';

COMMIT;

-- Quick sanity checks (optional):
-- SELECT * FROM project_statuses;
-- SELECT name, status_id FROM projects;
-- SELECT title, parent_id, project_id FROM tasks;
-- SELECT project_id, task_id, started_at, ended_at, duration_seconds FROM time_entries;
-- SELECT title, progress_percent FROM project_progress_pages;
