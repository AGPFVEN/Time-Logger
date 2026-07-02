-- Check how many project-task links exist right now (Should be 5)
SELECT count(*) AS total_links_before FROM project_task_dependencies;

-- Delete the mobile App project
DELETE FROM projects WHERE id = 'PRJ-APP';

-- Count again (Should be 3 left, belonging to PRJ-WEB)
SELECT count(*) AS total_links_after FROM project_task_dependencies;

-- Verify if task TSK-4 survived the deletion of its project (Must return 1 row)
SELECT id, title FROM tasks WHERE id = 'TSK-4';
