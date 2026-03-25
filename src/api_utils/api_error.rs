use axum::{http::StatusCode, response::IntoResponse};
use log::error;
use sea_orm::DbErr;
use validator::ValidationErrors;

use crate::api_utils::api_response::{BadRequest, UnprocessableEntity};
pub enum ApiError {
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
    NotFound,
    BadRequest,
    Validation(ValidationErrors),
    UnprocessableEntity(String),
    Unauthorized,
    Forbidden(String),
    ValidationError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unexpected(e) => {
                error!("Unexpected error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
            ApiError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request").into_response(),
            ApiError::Validation(errs) => BadRequest(errs).into_response(),
            ApiError::UnprocessableEntity(msg) => UnprocessableEntity(msg).into_response(),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg).into_response(),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        Self::Unexpected(Box::new(value))
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        Self::Validation(value)
    }
}
