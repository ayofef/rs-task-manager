pub const LIST_TASKS_QUERY: &str = "
    SELECT id, description, status::text as status, created_at, flagged
    FROM rs_task_manager
    ORDER BY created_at DESC
";
