use crate::route_utils::{api_response::ApiResponse, create_json_body::create_json_body};
use serde_json::{Value, json};

pub async fn delete_task_route() -> ApiResponse {
    Ok(create_json_body(json!({ "message": "WIP" }), Value::Null))
}
