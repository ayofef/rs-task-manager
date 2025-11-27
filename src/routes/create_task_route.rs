use axum::extract::{Json, State};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::queries::{
    create_task_query::{CREATE_TASK_WITH_ID_QUERY, CREATE_TASK_WITHOUT_ID_QUERY},
    task::Task,
};
use crate::route_utils::{
    api_error::ApiError, api_response::ApiResponse, create_json_body::create_json_body,
};
use crate::types::db_pool_as_state::DbPoolAsState;

#[derive(Deserialize)]
pub struct CreateTaskPayload {
    pub description: String,
    pub id: Option<Uuid>,
}

pub async fn create_task_route(
    State(pool): DbPoolAsState,
    Json(payload): Json<CreateTaskPayload>,
) -> ApiResponse {
    // Use query_as with fetch_one to get the RETURNING row
    let task_result = match payload.id {
        Some(id) => {
            sqlx::query_as::<_, Task>(CREATE_TASK_WITH_ID_QUERY)
                .bind(id)
                .bind(payload.description)
                .fetch_one(pool.as_ref())
                .await
        }
        None => {
            sqlx::query_as::<_, Task>(CREATE_TASK_WITHOUT_ID_QUERY)
                .bind(payload.description)
                .fetch_one(pool.as_ref())
                .await
        }
    };

    match task_result {
        Ok(task) => Ok(create_json_body(json!(task), Value::Null)),
        Err(e) => {
            eprintln!("Error creating task: {:?}", e);
            Err(ApiError::InternalError)
        }
    }
}
