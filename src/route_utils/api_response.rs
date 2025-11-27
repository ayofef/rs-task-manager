use axum::Json;
use serde_json::Value;

use super::api_error::ApiError;

pub type ApiResponse = Result<Json<Value>, ApiError>;
