use axum::Json;
use serde_json::{Value, json};

pub fn create_json_body(data: Value, error: Value) -> Json<Value> {
    Json(json!({
        "data": data,
        "error": error
    }))
}
