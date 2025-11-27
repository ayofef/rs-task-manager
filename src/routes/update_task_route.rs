use axum::extract::{Json, State};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::queries::{task::Task, update_task_query::UPDATE_TASK_QUERY};
use crate::route_utils::{
    api_error::ApiError, api_response::ApiResponse, create_json_body::create_json_body,
};
use crate::types::db_pool_as_state::DbPoolAsState;

#[derive(Deserialize)]
pub struct UpdateTaskPayload {
    pub id: Uuid,
    pub description: Option<String>,
    pub status: Option<String>,
    pub flagged: Option<bool>,
}

pub async fn update_task_route(
    State(pool): DbPoolAsState,
    Json(payload): Json<UpdateTaskPayload>,
) -> ApiResponse {
    if payload.id.to_string().is_empty() || payload.id.to_string().len() == 0 {
        return Err(ApiError::InvalidInput("Task id is required".to_string()));
    }

    let task_result = sqlx::query_as::<_, Task>(UPDATE_TASK_QUERY)
        .bind(payload.id)
        .bind(payload.description)
        .bind(payload.status)
        .bind(payload.flagged)
        .fetch_one(pool.as_ref())
        .await;

    match task_result {
        Ok(task) => Ok(create_json_body(json!(task), Value::Null)),
        Err(e) => {
            eprintln!("Error updating task: {:?}", e);
            Err(ApiError::InternalError)
        }
    }
}
