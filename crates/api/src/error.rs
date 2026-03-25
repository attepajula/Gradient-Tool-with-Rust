use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid color: {0}")]
    InvalidColor(String),
    #[error("image processing failed: {0}")]
    ImageError(String),
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("payload too large (max 10 MB)")]
    PayloadTooLarge,
    #[error("bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::InvalidColor(_)
            | ApiError::BadRequest(_)
            | ApiError::MissingField(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::PayloadTooLarge => (StatusCode::PAYLOAD_TOO_LARGE, self.to_string()),
            ApiError::ImageError(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
