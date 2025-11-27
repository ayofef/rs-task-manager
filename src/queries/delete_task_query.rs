pub const DELETE_TASK_QUERY: &str = "
    DELETE FROM rs_task_manager
    WHERE id = $1
";
