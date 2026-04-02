use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::sale_payment_allocations::sale_payment_allocations_dto::sale_payment_allocations_dto::{
    SalePaymentAllocationDetailResponse, SalePaymentAllocationIdResponse, SalePaymentAllocationRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_sale_payment_allocation(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SalePaymentAllocationRequest>,
) -> Result<Json<ApiResponse<SalePaymentAllocationIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let spa_create = schemas::sale_payment_allocations::ActiveModel::from(payload);

    let new_spa = spa_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_spa.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create sale payment allocation".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        SalePaymentAllocationIdResponse::from(new_spa),
        "Sale payment allocation created successfully".to_string(),
        1,
    )))
}

pub async fn get_sale_payment_allocation_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<SalePaymentAllocationDetailResponse>>, ApiError> {
    let spa = schemas::sale_payment_allocations::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match spa {
        Some(spa) => Ok(Json(ApiResponse::success(
            SalePaymentAllocationDetailResponse::from(spa),
            "Sale payment allocation retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Sale payment allocation not found".to_string(),
        )),
    }
}

pub async fn get_sale_payment_allocations(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SalePaymentAllocationDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::sale_payment_allocations::Entity::find();

    if let Some(payment) = pagination.payment_id {
        select = select.filter(schemas::sale_payment_allocations::Column::PaymentId.eq(payment));
    }

    if let Some(credit) = pagination.credit_invoice_id {
        select =
            select.filter(schemas::sale_payment_allocations::Column::CreditInvoiceId.eq(credit));
    }

    let paginator = select
        .order_by_asc(schemas::sale_payment_allocations::Column::Id)
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
            .map(SalePaymentAllocationDetailResponse::from)
            .collect(),
        "Sale payment allocations retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_sale_payment_allocation(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let spa = schemas::sale_payment_allocations::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match spa {
        Some(spa) => {
            spa.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Sale payment allocation deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Sale payment allocation not found".to_string(),
        )),
    }
}

pub async fn update_sale_payment_allocation(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SalePaymentAllocationRequest>,
) -> Result<Json<ApiResponse<SalePaymentAllocationIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let spa = schemas::sale_payment_allocations::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match spa {
        Some(spa) => {
            let mut spa_active = spa.into_active_model();

            spa_active.payment_id = ActiveValue::Set(payload.payment_id);
            spa_active.credit_invoice_id = ActiveValue::Set(payload.credit_invoice_id);
            spa_active.amount = ActiveValue::Set(payload.amount);

            let updated = spa_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                SalePaymentAllocationIdResponse::from(updated),
                "Sale payment allocation updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Sale payment allocation not found".to_string(),
        )),
    }
}
