use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::sale_payments::sale_payments_dto::sale_payments_dto::{
    SalePaymentDetailResponse, SalePaymentIdResponse, SalePaymentRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_sale_payment(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SalePaymentRequest>,
) -> Result<Json<ApiResponse<SalePaymentIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let sp_create = schemas::sale_payments::ActiveModel::from(payload);

    let new_sp = sp_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_sp.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create sale payment".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        SalePaymentIdResponse::from(new_sp),
        "Sale payment created successfully".to_string(),
        1,
    )))
}

pub async fn get_sale_payment_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<SalePaymentDetailResponse>>, ApiError> {
    let sp = schemas::sale_payments::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match sp {
        Some(sp) => Ok(Json(ApiResponse::success(
            SalePaymentDetailResponse::from(sp),
            "Sale payment retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Sale payment not found".to_string(),
        )),
    }
}

pub async fn get_sale_payments(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SalePaymentDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::sale_payments::Entity::find();

    if let Some(sale) = pagination.sale_id {
        select = select.filter(schemas::sale_payments::Column::SaleId.eq(sale));
    }

    if let Some(method) = pagination.method_id {
        select = select.filter(schemas::sale_payments::Column::MethodId.eq(method));
    }

    if let Some(reference) = pagination.reference.clone() {
        if !reference.is_empty() {
            select = select.filter(schemas::sale_payments::Column::Reference.eq(reference));
        }
    }

    // date range for paid_at
    if let Some(date_init) = pagination.date_init.clone() {
        if !date_init.is_empty() {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_init) {
                let dt_utc = dt.with_timezone(&chrono::Utc);
                select = select.filter(schemas::sale_payments::Column::PaidAt.gte(dt_utc));
            }
        }
    }

    if let Some(date_end) = pagination.date_end.clone() {
        if !date_end.is_empty() {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_end) {
                let dt_utc = dt.with_timezone(&chrono::Utc);
                select = select.filter(schemas::sale_payments::Column::PaidAt.lte(dt_utc));
            }
        }
    }

    let paginator = select
        .order_by_asc(schemas::sale_payments::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let items = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        items
            .into_iter()
            .map(SalePaymentDetailResponse::from)
            .collect(),
        "Sale payments retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_sale_payment(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let sp = schemas::sale_payments::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match sp {
        Some(sp) => {
            sp.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Sale payment deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Sale payment not found".to_string(),
        )),
    }
}

pub async fn update_sale_payment(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SalePaymentRequest>,
) -> Result<Json<ApiResponse<SalePaymentIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let sp = schemas::sale_payments::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match sp {
        Some(sp) => {
            let mut sp_active = sp.into_active_model();

            sp_active.sale_id = ActiveValue::Set(payload.sale_id);
            sp_active.amount = ActiveValue::Set(payload.amount);
            sp_active.method_id = ActiveValue::Set(payload.method_id);
            sp_active.paid_at = ActiveValue::Set(payload.paid_at);
            sp_active.reference = ActiveValue::Set(payload.reference);

            let updated = sp_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                SalePaymentIdResponse::from(updated),
                "Sale payment updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Sale payment not found".to_string(),
        )),
    }
}
