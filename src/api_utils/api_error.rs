use axum::{Json, http::StatusCode, response::IntoResponse};
use log::error;
use sea_orm::DbErr;
use validator::ValidationErrors;

use crate::api_utils::api_response::{ApiResponse, BadRequest, ErrorType, UnprocessableEntity};

/// Application-level error enum.
///
/// Each variant maps to an HTTP status code AND an `ErrorType` so the
/// frontend can distinguish business errors (e.g. "stock insuficiente")
/// from system errors (DB down), validation errors, or auth errors.
pub enum ApiError {
    /// Unexpected internal failure (DB, Redis, IO). Maps to 500 / SYSTEM.
    Unexpected(Box<dyn std::error::Error + Send + Sync>),
    /// Generic not-found. Maps to 404 / BUSINESS.
    NotFound,
    /// Generic bad request. Maps to 400 / VALIDATION.
    BadRequest,
    /// Field-level validation errors from `validator`. Maps to 400 / VALIDATION.
    Validation(ValidationErrors),
    /// Semantically unprocessable entity (e.g. malformed payload). Maps to 422.
    UnprocessableEntity(String),
    /// Missing or invalid authentication. Maps to 401 / AUTH.
    Unauthorized,
    /// Authenticated but lacking permissions. Maps to 403 / AUTH.
    Forbidden(String),
    /// Custom validation message (single string). Maps to 400 / VALIDATION.
    ValidationError(String),
    /// Resource not found with a descriptive message. Maps to 404 / BUSINESS.
    NotFoundErrorDescription(String),
    /// Business rule violation (e.g. stock insuficiente, crédito excedido).
    /// Maps to 422 / BUSINESS so the UI can show a specific message.
    BusinessError(String),
    /// Conflict — a duplicate or conflicting resource state. Maps to 409 / BUSINESS.
    Conflict(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unexpected(e) => {
                error!("Unexpected error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::with_error_type(
                        (),
                        "Error interno del servidor. El equipo ha sido notificado.".to_string(),
                        500,
                        ErrorType::System,
                    )),
                )
                    .into_response()
            }
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::with_error_type(
                    (),
                    "El recurso solicitado no fue encontrado.".to_string(),
                    404,
                    ErrorType::Business,
                )),
            )
                .into_response(),
            ApiError::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::with_error_type(
                    (),
                    "La solicitud contiene datos inválidos.".to_string(),
                    400,
                    ErrorType::Validation,
                )),
            )
                .into_response(),
            ApiError::Validation(errs) => BadRequest(errs).into_response(),
            ApiError::UnprocessableEntity(msg) => UnprocessableEntity(msg).into_response(),
            ApiError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                Json(ApiResponse::with_error_type((), msg, 403, ErrorType::Auth)),
            )
                .into_response(),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::with_error_type(
                    (),
                    "No autenticado o sesión expirada. Inicia sesión nuevamente.".to_string(),
                    401,
                    ErrorType::Auth,
                )),
            )
                .into_response(),
            ApiError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::with_error_type(
                    (),
                    msg,
                    400,
                    ErrorType::Validation,
                )),
            )
                .into_response(),
            ApiError::NotFoundErrorDescription(msg) => (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::with_error_type(
                    (),
                    msg,
                    404,
                    ErrorType::Business,
                )),
            )
                .into_response(),
            ApiError::BusinessError(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponse::with_error_type(
                    (),
                    msg,
                    422,
                    ErrorType::Business,
                )),
            )
                .into_response(),
            ApiError::Conflict(msg) => (
                StatusCode::CONFLICT,
                Json(ApiResponse::with_error_type(
                    (),
                    msg,
                    409,
                    ErrorType::Business,
                )),
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
