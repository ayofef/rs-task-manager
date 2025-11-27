use crate::route_utils::{api_response::ApiResponse, create_json_body::create_json_body};
use serde_json::{Value, json};

pub async fn index_route() -> ApiResponse {
    Ok(create_json_body(
        json!({  "name": "rs-task-manager", "version": "1.0.0" }),
        Value::Null,
    ))
}
