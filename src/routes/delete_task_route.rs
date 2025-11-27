use axum::extract::{Json, State};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::queries::delete_task_query::DELETE_TASK_QUERY;
use crate::route_utils::{
    api_error::ApiError, api_response::ApiResponse, create_json_body::create_json_body,
};
use crate::types::db_pool_as_state::DbPoolAsState;

#[derive(Deserialize)]
pub struct DeleteTaskPayload {
    pub id: Uuid,
}

pub async fn delete_task_route(
    State(pool): DbPoolAsState,
    Json(payload): Json<DeleteTaskPayload>,
) -> ApiResponse {
    let id = payload.id;

    let delete_task_response = sqlx::query(DELETE_TASK_QUERY)
        .bind(id)
        .execute(pool.as_ref())
        .await;

    match delete_task_response {
        Ok(result) => {
            let deleted_rows = result.rows_affected();

            if deleted_rows == 0 {
                let error_message = format!("Task with id: {} not found", id);
                return Err(ApiError::InvalidInput(error_message));
            }

            Ok(create_json_body(
                json!({
                    "success": true,
                    "message": "Task deleted successfully"
                }),
                Value::Null,
            ))
        }

        Err(e) => {
            eprintln!("Error deleting task: {:?}", e);
            Err(ApiError::InternalError)
        }
    }
}
