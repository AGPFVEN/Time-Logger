WITH RECURSIVE execution_chain(id, title, step) AS (
    SELECT id, title, 1 
    FROM tasks WHERE id = 'TSK-1'
    
    UNION ALL
    
    SELECT t.id, t.title, c.step + 1
    FROM tasks t
    JOIN task_task_dependencies ttd ON t.id = ttd.child_id
    JOIN execution_chain c ON ttd.parent_id = c.id
)
SELECT step, id, title FROM execution_chain ORDER BY step;
