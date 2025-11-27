use axum::extract::State;
use serde_json::{Value, json};

use crate::queries::{list_task_query::LIST_TASKS_QUERY, task::Task};
use crate::route_utils::{
    api_error::ApiError, api_response::ApiResponse, create_json_body::create_json_body,
};
use crate::types::db_pool_as_state::DbPoolAsState;

pub async fn list_tasks_route(State(pool): DbPoolAsState) -> ApiResponse {
    let tasks = sqlx::query_as::<_, Task>(LIST_TASKS_QUERY)
        .fetch_all(pool.as_ref())
        .await;

    match tasks {
        Ok(tasks) => Ok(create_json_body(json!(tasks), Value::Null)),
        Err(e) => {
            eprintln!("Error fetching tasks: {:?}", e);

            Err(ApiError::InternalError)
        }
    }
}
