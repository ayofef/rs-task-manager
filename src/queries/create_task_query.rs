// Query when ID is provided - uses upsert (ON CONFLICT)
pub const CREATE_TASK_WITH_ID_QUERY: &str = "
    INSERT INTO rs_task_manager (id, description)
    VALUES ($1, $2)
    ON CONFLICT (id) DO UPDATE SET description = EXCLUDED.description
    RETURNING id, description, status::text as status, created_at, flagged, updated_at
";

// Query when ID is not provided - let database generate it
pub const CREATE_TASK_WITHOUT_ID_QUERY: &str = "
    INSERT INTO rs_task_manager (description)
    VALUES ($1)
    RETURNING id, description, status::text as status, created_at, flagged, updated_at
";
