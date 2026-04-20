use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_utils::api_utils_fun::parse_local_date_range_to_utc;
use crate::{
    api_module::purchases::{
        PurchaseUpdateRequest,
        purchases_dto::purchases_dto::{
            PurchaseDetailResponse, PurchaseIdResponse, PurchaseRequest,
        },
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
use log::info;

pub async fn create_purchase(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PurchaseRequest>,
) -> Result<Json<ApiResponse<PurchaseIdResponse>>, ApiError> {
    info!("create_purchase called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    //Copiar payload a una variable para crear el pago después de crear la compra
    let purchase_payments = payload.clone().payment;

    let p_create = schemas::purchases::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_p = p_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_p.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create purchase".to_string(),
        ));
    }

    let id_created = new_p.id.clone().unwrap();

    let payment_create = schemas::purchase_payments::ActiveModel {
        id: ActiveValue::NotSet,
        purchase_id: ActiveValue::Set(id_created),
        amount: ActiveValue::Set(purchase_payments.amount),
        method_id: ActiveValue::Set(purchase_payments.method_id),
        paid_at: ActiveValue::Set(get_current_timestamp_now()),
        reference: ActiveValue::NotSet,
    };

    let new_payment = payment_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_payment.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create purchase payment".to_string(),
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
    info!("get_purchase_by_id called with id: {:?}", id);

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
    info!(
        "get_purchases called with pagination: page={:?}, limit={:?}, total={:?}, supplier_id={:?}, invoice_no={:?}, status={:?}, date_init={:?}, date_end={:?}",
        pagination.page,
        pagination.limit,
        pagination.total,
        pagination.supplier_id,
        pagination.invoice_no,
        pagination.status,
        pagination.date_init,
        pagination.date_end
    );

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

    // date range (YYYY-MM-DD interpreted as local time → UTC)
    let (fecha_init, fecha_end) = parse_local_date_range_to_utc(
        &pagination.date_init.clone().unwrap_or_default(),
        &pagination.date_end.clone().unwrap_or_default(),
    )?;

    if let Some(date_init) = fecha_init {
        select = select.filter(schemas::purchases::Column::Date.gte(date_init));
    }

    if let Some(date_end) = fecha_end {
        select = select.filter(schemas::purchases::Column::Date.lte(date_end));
    }

    let paginator = select
        .order_by_asc(schemas::purchases::Column::Id)
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
    info!("delete_purchase called with id: {:?}", id);

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
    Path(id): Path<i64>,
    Json(payload): Json<PurchaseUpdateRequest>,
) -> Result<Json<ApiResponse<PurchaseIdResponse>>, ApiError> {
    info!(
        "update_purchase called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    let p = schemas::purchases::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match p {
        Some(p) => {
            let mut p_active = p.into_active_model();

            p_active.status = ActiveValue::Set(payload.status);

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
