use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum ApiError {
    Error(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Error(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json! ({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
