// Using COALESCE to handle optional parameters:
// - If a value is provided (not NULL), it updates the field
// - If NULL is passed, the field keeps its current value
pub const UPDATE_TASK_QUERY: &str = "
    UPDATE rs_task_manager
    SET 
        description = COALESCE($2, description),
        status = COALESCE($3::task_status, status),
        flagged = COALESCE($4, flagged),
        updated_at = now()
    WHERE id = $1
    RETURNING id, description, status::text as status, created_at, flagged, updated_at
";
