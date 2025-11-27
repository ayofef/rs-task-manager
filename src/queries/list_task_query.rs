pub const LIST_TASKS_QUERY: &str = "
    SELECT id, description, status::text as status, created_at, flagged, updated_at
    FROM rs_task_manager
    WHERE status != 'archived'
    ORDER BY created_at DESC
    LIMIT 100
";
