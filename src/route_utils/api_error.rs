use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::{Value, json};

use super::create_json_body::create_json_body;

struct PreparedApiError {
    status: StatusCode,
    error_message: String,
    error_code: &'static str,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
}

impl ApiError {
    fn prepare_response(&self) -> PreparedApiError {
        match self {
            ApiError::NotFound => PreparedApiError {
                status: StatusCode::NOT_FOUND,
                error_message: "Not Found".to_string(),
                error_code: "not.found",
            },
            ApiError::InvalidInput(msg) => PreparedApiError {
                status: StatusCode::BAD_REQUEST,
                error_message: msg.to_string(),
                error_code: "invalid.input",
            },
            ApiError::InternalError => PreparedApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                error_message: "Internal Server Error".to_string(),
                error_code: "internal.error",
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let parsed_error: PreparedApiError = match self {
            ApiError::NotFound => ApiError::NotFound.prepare_response(),
            ApiError::InvalidInput(msg) => ApiError::InvalidInput(msg).prepare_response(),
            ApiError::InternalError => ApiError::InternalError.prepare_response(),
        };

        let body = create_json_body(
            Value::Null,
            json!({
                "message": parsed_error.error_message,
                "code": parsed_error.error_code,
            }),
        );

        (parsed_error.status, body).into_response()
    }
}
