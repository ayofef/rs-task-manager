use axum::extract::State;
use crate::route_utils::{api_response::ApiResponse, create_json_body::create_json_body};
use serde_json::{Value, json};
use std::sync::Arc; 
use sqlx::{pool::Pool, postgres::Postgres};

use crate::queries::{list_task_query::LIST_TASKS_QUERY, task::Task};



pub async fn list_tasks_route(State(pool): State<Arc<Pool<Postgres>>>) -> ApiResponse {
    let tasks = sqlx::query_as::<_, Task>(LIST_TASKS_QUERY)
        .fetch_all(pool.as_ref())
        .await?;
 

    Ok(create_json_body(json!(tasks), Value::Null))
}
