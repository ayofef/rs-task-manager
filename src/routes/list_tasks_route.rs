use crate::route_utils::{
    api_error::ApiError, api_response::ApiResponse, create_json_body::create_json_body,
};
use axum::extract::State;
use serde_json::{Value, json};
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;

use crate::queries::{list_task_query::LIST_TASKS_QUERY, task::Task};

pub async fn list_tasks_route(State(pool): State<Arc<Pool<Postgres>>>) -> ApiResponse {
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
