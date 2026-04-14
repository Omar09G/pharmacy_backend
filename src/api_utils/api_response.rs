use axum::{Json, http::StatusCode, response::IntoResponse};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::{ValidationError, ValidationErrors};

use crate::api_utils::api_response;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub data: T,
    pub total: i32,
    pub message: String,
    pub status: String,
    pub code_error: u16,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn new(
        data: T,
        total: i32,
        message: String,
        status: String,
        code_error: u16,
        timestamp: String,
    ) -> Self {
        Self {
            data,
            total,
            message,
            status,
            code_error,
            timestamp,
        }
    }
}

#[derive(Serialize)]
pub struct ProblemDetails {
    pub detail: String,
    pub errors: Vec<Field>,
}

#[derive(Serialize)]
pub struct Field {
    pub field: String,
    pub reason: String,
    pub code: String,
}

impl Field {
    pub fn new(field: &str, reason: &str, code: &str) -> Self {
        Self {
            field: field.to_string(),
            reason: reason.to_string(),
            code: code.to_string(),
        }
    }
}

const INVALID_DEFAULT_MESSAGE: Cow<'static, str> = Cow::Borrowed("Invalid information");

// This module defines the structure of API responses and error handling for the application. It includes a generic `ApiResponse` struct for successful responses, a `ProblemDetails` struct for
trait IntoFields {
    fn into_fields(self) -> Vec<Field>;
}

pub struct BadRequest(pub ValidationErrors);

impl IntoResponse for BadRequest {
    fn into_response(self) -> axum::response::Response {
        let fields_with_errors = self.0.into_fields();

        let total_errors = fields_with_errors.len() as i32;

        let api_response = api_response::ApiResponse {
            data: fields_with_errors,
            total: total_errors,
            message: "Validation failed".to_string(),
            status: "error".to_string(),
            code_error: 400,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        (StatusCode::BAD_REQUEST, Json(api_response)).into_response()
    }
}
/* Implementacion para integrar errores de Validacion  */

impl IntoFields for ValidationErrors {
    fn into_fields(self) -> Vec<Field> {
        let mut fields: Vec<Field> = Vec::with_capacity(self.field_errors().len());

        let field_errors = self.field_errors();

        fields.extend(field_errors.into_iter().map(|(field_name, errs)| {
            let error: &ValidationError = &errs[0];
            let field_message = error.message.as_ref().unwrap_or(&INVALID_DEFAULT_MESSAGE);

            Field::new(&field_name, field_message, &error.code)
        }));

        fields.sort_by(|a, b| a.field.to_lowercase().cmp(&b.field.to_lowercase()));

        fields
    }
}

pub struct UnprocessableEntity(pub String);

impl IntoResponse for UnprocessableEntity {
    fn into_response(self) -> axum::response::Response {
        let problem_details = ProblemDetails {
            detail: self.0,
            errors: Vec::new(),
        };

        (StatusCode::UNPROCESSABLE_ENTITY, Json(problem_details)).into_response()
    }
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: String, total: i32) -> Self {
        Self {
            data,
            total,
            message,
            status: "success".to_string(),
            code_error: 200,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_error_details(data: T, message: String, code_error: u16) -> Self {
        Self {
            data,
            total: 0,
            message,
            status: "error".to_string(),
            code_error,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    pub fn with_custom_status(data: T, message: String, status: String, code_error: u16) -> Self {
        Self {
            data,
            total: 10,
            message,
            status,
            code_error,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    pub fn warring(data: T, message: String) -> Self {
        Self {
            data,
            total: 10,
            message,
            status: "warning".to_string(),
            code_error: 200,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub date_init: Option<String>,
    pub date_end: Option<String>,
    pub username: Option<String>,
    pub client_id: Option<i64>,
    pub user_id: Option<i64>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub discount_type: Option<String>,
    pub applies_to: Option<String>,
    pub active: Option<bool>,
    pub id: Option<i64>,
    pub document_id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub lot_number: Option<String>,
    pub role_id: Option<i64>,
    pub permission_id: Option<i64>,
    pub product_id: Option<i64>,
    pub category_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub supplier_id: Option<i64>,
    pub invoice_no: Option<String>,
    pub customer_id: Option<i64>,
    pub unit_id: Option<i64>,
    pub tax_profile_id: Option<i64>,
    pub method_id: Option<i64>,
    pub brand: Option<String>,
    pub reference: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub location_id: Option<i64>,
    pub lot_id: Option<i64>,
    pub recorded_by: Option<i64>,
    pub status: Option<String>,
    pub is_sellable: Option<bool>,
    pub track_batches: Option<bool>,
    pub inventory_location_name: Option<String>,
    pub inits_name: Option<String>,
    pub price_min: Option<Decimal>,
    pub price_max: Option<Decimal>,
    pub price_type: Option<String>,
    pub sale_id: Option<i64>,
    pub purchase_id: Option<i64>,
    pub purchase_item_id: Option<i64>,
    pub entity_type: Option<String>,
    pub table_name: Option<String>,
    pub action: Option<String>,
    pub entity_id: Option<i64>,
    pub changed_by: Option<i64>,
    pub payment_id: Option<i64>,
    pub credit_invoice_id: Option<i64>,
}
