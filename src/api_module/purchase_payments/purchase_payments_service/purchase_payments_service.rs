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
use crate::api_utils::api_utils_fun::parse_local_date_range_to_utc;
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};
use log::info;

pub async fn create_purchase_payment(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PurchasePaymentRequest>,
) -> Result<Json<ApiResponse<PurchasePaymentIdResponse>>, ApiError> {
    info!("create_purchase_payment called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let pp_create = schemas::purchase_payments::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_pp = pp_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_pp.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create purchase payment".to_string(),
        ));
    }

    // enqueue async job for purchase payment processing
    match new_pp.id.clone() {
        sea_orm::ActiveValue::Set(id) => {
            let job = serde_json::json!({"type": "purchase_payment", "id": id});
            let _ = tokio::spawn(async move {
                let _ = crate::config::config_redis::enqueue_json("jobs:payments", &job).await;
            });
        }
        _ => {}
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
    info!("get_purchase_payment_by_id called with id: {:?}", id);

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
    info!(
        "get_purchase_payments called with pagination: page={:?}, limit={:?}, total={:?}, purchase_id={:?}, method_id={:?}, reference={:?}, date_init={:?}, date_end={:?}",
        pagination.page,
        pagination.limit,
        pagination.total,
        pagination.purchase_id,
        pagination.method_id,
        pagination.reference,
        pagination.date_init,
        pagination.date_end
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::purchase_payments::Entity::find();

    if let Some(purchase) = pagination.purchase_id {
        select = select.filter(schemas::purchase_payments::Column::PurchaseId.eq(purchase));
    }

    if let Some(method) = pagination.method_id {
        select = select.filter(schemas::purchase_payments::Column::MethodId.eq(method));
    }

    if let Some(reference) = pagination.reference.clone()
        && !reference.is_empty()
    {
        select = select.filter(schemas::purchase_payments::Column::Reference.eq(reference));
    }

    // paid_at date range (YYYY-MM-DD interpreted as local time → UTC)
    let (fecha_init, fecha_end) = parse_local_date_range_to_utc(
        &pagination.date_init.clone().unwrap_or_default(),
        &pagination.date_end.clone().unwrap_or_default(),
    )?;

    if let Some(date_init) = fecha_init {
        select = select.filter(schemas::purchase_payments::Column::PaidAt.gte(date_init));
    }

    if let Some(date_end) = fecha_end {
        select = select.filter(schemas::purchase_payments::Column::PaidAt.lte(date_end));
    }

    let paginator = select
        .order_by_asc(schemas::purchase_payments::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

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
    info!("delete_purchase_payment called with id: {:?}", id);

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
    Path(id): Path<i64>,
    Json(payload): Json<PurchasePaymentRequest>,
) -> Result<Json<ApiResponse<PurchasePaymentIdResponse>>, ApiError> {
    info!(
        "update_purchase_payment called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    let pp = schemas::purchase_payments::Entity::find_by_id(id)
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
