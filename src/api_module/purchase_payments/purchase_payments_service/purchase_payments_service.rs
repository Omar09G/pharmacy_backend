use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::purchase_payments::purchase_payments_dto::purchase_payments_dto::{
    PurchasePaymentDetailResponse, PurchasePaymentIdResponse, PurchasePaymentRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_purchase_payment(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PurchasePaymentRequest>,
) -> Result<Json<ApiResponse<PurchasePaymentIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pp_create = schemas::purchase_payments::ActiveModel::from(payload);

    let new_pp = pp_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_pp.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create purchase payment".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        PurchasePaymentIdResponse::from(new_pp),
        "Purchase payment created successfully".to_string(),
        1,
    )))
}

pub async fn get_purchase_payment_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<PurchasePaymentDetailResponse>>, ApiError> {
    let pp = schemas::purchase_payments::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pp {
        Some(pp) => Ok(Json(ApiResponse::success(
            PurchasePaymentDetailResponse::from(pp),
            "Purchase payment retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Purchase payment not found".to_string(),
        )),
    }
}

pub async fn get_purchase_payments(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<PurchasePaymentDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::purchase_payments::Entity::find();

    if let Some(purchase) = pagination.purchase_id {
        select = select.filter(schemas::purchase_payments::Column::PurchaseId.eq(purchase));
    }

    if let Some(method) = pagination.method_id {
        select = select.filter(schemas::purchase_payments::Column::MethodId.eq(method));
    }

    if let Some(reference) = pagination.reference.clone() {
        if !reference.is_empty() {
            select = select.filter(schemas::purchase_payments::Column::Reference.eq(reference));
        }
    }

    // paid_at date range using date_init/date_end if provided
    if let Some(date_init) = pagination.date_init.clone() {
        if !date_init.is_empty() {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_init) {
                let dt_utc = dt.with_timezone(&chrono::Utc);
                select = select.filter(schemas::purchase_payments::Column::PaidAt.gte(dt_utc));
            }
        }
    }

    if let Some(date_end) = pagination.date_end.clone() {
        if !date_end.is_empty() {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_end) {
                let dt_utc = dt.with_timezone(&chrono::Utc);
                select = select.filter(schemas::purchase_payments::Column::PaidAt.lte(dt_utc));
            }
        }
    }

    let paginator = select
        .order_by_asc(schemas::purchase_payments::Column::Id)
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
            .map(PurchasePaymentDetailResponse::from)
            .collect(),
        "Purchase payments retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_purchase_payment(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let pp = schemas::purchase_payments::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pp {
        Some(pp) => {
            pp.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Purchase payment deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Purchase payment not found".to_string(),
        )),
    }
}

pub async fn update_purchase_payment(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PurchasePaymentRequest>,
) -> Result<Json<ApiResponse<PurchasePaymentIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pp = schemas::purchase_payments::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pp {
        Some(pp) => {
            let mut pp_active = pp.into_active_model();

            pp_active.purchase_id = ActiveValue::Set(payload.purchase_id);
            pp_active.amount = ActiveValue::Set(payload.amount);
            pp_active.method_id = ActiveValue::Set(payload.method_id);
            pp_active.paid_at = ActiveValue::Set(payload.paid_at);
            pp_active.reference = ActiveValue::Set(payload.reference);

            let updated = pp_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                PurchasePaymentIdResponse::from(updated),
                "Purchase payment updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Purchase payment not found".to_string(),
        )),
    }
}
