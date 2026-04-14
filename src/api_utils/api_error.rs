use axum::{Json, http::StatusCode, response::IntoResponse};
use log::error;
use sea_orm::DbErr;
use validator::ValidationErrors;

use crate::api_utils::api_response::{ApiResponse, BadRequest, UnprocessableEntity};
pub enum ApiError {
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
    NotFound,
    BadRequest,
    Validation(ValidationErrors),
    UnprocessableEntity(String),
    Unauthorized,
    Forbidden(String),
    ValidationError(String),
    NotFoundErrorDescription(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unexpected(e) => {
                error!("Unexpected error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::with_error_details(
                        (),
                        "Internal Server Error".to_string(),
                        500,
                    )),
                )
                    .into_response()
            }
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::with_error_details(
                    (),
                    "Not found".to_string(),
                    404,
                )),
            )
                .into_response(),
            ApiError::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::with_error_details(
                    (),
                    "Bad Request".to_string(),
                    400,
                )),
            )
                .into_response(),
            ApiError::Validation(errs) => BadRequest(errs).into_response(),
            ApiError::UnprocessableEntity(msg) => UnprocessableEntity(msg).into_response(),
            ApiError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                Json(ApiResponse::with_error_details((), msg, 403)),
            )
                .into_response(),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::with_error_details(
                    (),
                    "Unauthorized".to_string(),
                    401,
                )),
            )
                .into_response(),
            ApiError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::with_error_details((), msg, 400)),
            )
                .into_response(),
            ApiError::NotFoundErrorDescription(msg) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::with_error_details((), msg, 404)),
            )
                .into_response(),
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
