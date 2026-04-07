use axum::{
    Json,
    extract::{Path, Query, State},
};

use chrono::Timelike;
use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::{
    api_module::sales::sales_dto::sales_dto::{SaleDetailResponse, SaleIdResponse, SaleRequest},
    api_utils::api_utils_fun::parce_date_str_to_date_time_with_timezone_opt,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_sale(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SaleRequest>,
) -> Result<Json<ApiResponse<SaleIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let s_create = schemas::sales::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_s = s_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_s.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create sale".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        SaleIdResponse::from(new_s),
        "Sale created successfully".to_string(),
        1,
    )))
}

pub async fn get_sale_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<SaleDetailResponse>>, ApiError> {
    let s = schemas::sales::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match s {
        Some(s) => Ok(Json(ApiResponse::success(
            SaleDetailResponse::from(s),
            "Sale retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Sale not found".to_string())),
    }
}

pub async fn get_sales(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SaleDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    info!(
        "Fetching sales with pagination: page {}, limit {}",
        pagination.page, pagination.limit
    );

    let mut select = schemas::sales::Entity::find();

    if let Some(customer) = pagination.customer_id {
        select = select.filter(schemas::sales::Column::CustomerId.eq(customer));
    }

    if let Some(user) = pagination.user_id {
        select = select.filter(schemas::sales::Column::UserId.eq(user));
    }

    if let Some(invoice) = pagination.invoice_no.clone()
        && !invoice.is_empty()
    {
        select = select.filter(schemas::sales::Column::InvoiceNo.eq(invoice));
    }

    if let Some(status) = pagination.status.clone()
        && !status.is_empty()
    {
        select = select.filter(schemas::sales::Column::Status.eq(status));
    }

    let fecha_init = parce_date_str_to_date_time_with_timezone_opt(
        &pagination.date_init.clone().unwrap_or_default(),
    )?;
    let fecha_end = parce_date_str_to_date_time_with_timezone_opt(
        &pagination.date_end.clone().unwrap_or_default(),
    )?;
    // Reemplazar la hora de `fecha_end` por 23:59:59 para incluir toda la fecha en el filtro
    let fecha_end = fecha_end.and_then(|d| {
        d.with_hour(23)
            .and_then(|dt| dt.with_minute(59))
            .and_then(|dt| dt.with_second(59))
    });

    info!(
        "Applying filters - customer_id: {:?}, user_id: {:?}, invoice_no: {:?}, status: {:?}, date_init: {:?}, date_end: {:?}",
        pagination.customer_id,
        pagination.user_id,
        pagination.invoice_no,
        pagination.status,
        fecha_init,
        fecha_end
    );
    // date range
    if let Some(date_init) = fecha_init {
        select = select.filter(schemas::sales::Column::Date.gte(date_init));
    }

    if let Some(date_end) = fecha_end {
        select = select.filter(schemas::sales::Column::Date.lte(date_end));
    }

    let paginator = select
        .order_by_asc(schemas::sales::Column::Id)
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
        items.into_iter().map(SaleDetailResponse::from).collect(),
        "Sales retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_sale(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let s = schemas::sales::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match s {
        Some(s) => {
            s.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Sale deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Sale not found".to_string())),
    }
}

pub async fn update_sale(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<SaleRequest>,
) -> Result<Json<ApiResponse<SaleIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let s = schemas::sales::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match s {
        Some(s) => {
            let mut s_active = s.into_active_model();

            //s_active.customer_id = ActiveValue::Set(payload.customer_id);
            //s_active.user_id = ActiveValue::Set(payload.user_id);
            //s_active.invoice_no = ActiveValue::Set(payload.invoice_no);
            //s_active.date = ActiveValue::Set(payload.date);
            //s_active.subtotal = ActiveValue::Set(payload.subtotal);
            //s_active.tax_total = ActiveValue::Set(payload.tax_total);
            //s_active.discount_total = ActiveValue::Set(payload.discount_total);
            //s_active.total = ActiveValue::Set(payload.total);
            s_active.status = ActiveValue::Set(payload.status);
            //s_active.is_credit = ActiveValue::Set(payload.is_credit);
            //s_active.created_at = ActiveValue::Set(payload.created_at);

            let updated = s_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                SaleIdResponse::from(updated),
                "Sale updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Sale not found".to_string())),
    }
}
