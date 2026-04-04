use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::{
    api_module::purchases::purchases_dto::purchases_dto::{
        PurchaseDetailResponse, PurchaseIdResponse, PurchaseRequest,
    },
    api_utils::api_utils_fun::get_current_timestamp_now,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_purchase(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PurchaseRequest>,
) -> Result<Json<ApiResponse<PurchaseIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let p_create = schemas::purchases::ActiveModel::from(payload);

    let new_p = p_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_p.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create purchase".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        PurchaseIdResponse::from(new_p),
        "Purchase created successfully".to_string(),
        1,
    )))
}

pub async fn get_purchase_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<PurchaseDetailResponse>>, ApiError> {
    let p = schemas::purchases::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match p {
        Some(p) => Ok(Json(ApiResponse::success(
            PurchaseDetailResponse::from(p),
            "Purchase retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Purchase not found".to_string())),
    }
}

pub async fn get_purchases(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<PurchaseDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::purchases::Entity::find();

    if let Some(supplier) = pagination.supplier_id {
        select = select.filter(schemas::purchases::Column::SupplierId.eq(supplier));
    }

    if let Some(invoice) = pagination.invoice_no.clone()
        && !invoice.is_empty()
    {
        select = select.filter(schemas::purchases::Column::InvoiceNo.eq(invoice));
    }

    if let Some(status) = pagination.status.clone()
        && !status.is_empty()
    {
        select = select.filter(schemas::purchases::Column::Status.eq(status));
    }

    // date range
    if let Some(date_init) = pagination.date_init.clone()
        && !date_init.is_empty()
        && let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_init)
    {
        let dt_utc = dt.with_timezone(&chrono::Utc);
        select = select.filter(schemas::purchases::Column::Date.gte(dt_utc));
    }

    if let Some(date_end) = pagination.date_end.clone()
        && !date_end.is_empty()
        && let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_end)
    {
        let dt_utc = dt.with_timezone(&chrono::Utc);
        select = select.filter(schemas::purchases::Column::Date.lte(dt_utc));
    }

    let paginator = select
        .order_by_asc(schemas::purchases::Column::Id)
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
            .map(PurchaseDetailResponse::from)
            .collect(),
        "Purchases retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_purchase(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let p = schemas::purchases::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match p {
        Some(p) => {
            p.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Purchase deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Purchase not found".to_string())),
    }
}

pub async fn update_purchase(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PurchaseRequest>,
) -> Result<Json<ApiResponse<PurchaseIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let p = schemas::purchases::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match p {
        Some(p) => {
            let mut p_active = p.into_active_model();

            p_active.status = ActiveValue::Set(payload.status);
            p_active.created_at = ActiveValue::Set(get_current_timestamp_now());

            let updated = p_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                PurchaseIdResponse::from(updated),
                "Purchase updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Purchase not found".to_string())),
    }
}
