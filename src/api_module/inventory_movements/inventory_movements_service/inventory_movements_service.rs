use axum::{
    Json,
    extract::{Path, Query, State},
};

use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::{
    api_module::inventory_movements::inventory_movements_dto::inventory_movements_dto::{
        InventoryMovementDetailResponse, InventoryMovementIdResponse, InventoryMovementRequest,
    },
    api_utils::api_utils_fun::validate_date_time_range_date,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_inventory_movement(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<InventoryMovementRequest>,
) -> Result<Json<ApiResponse<InventoryMovementIdResponse>>, ApiError> {
    info!(
        "create_inventory_movement called with payload: {:?}",
        payload
    );

    payload.validate().map_err(ApiError::Validation)?;

    let im_create = schemas::inventory_movements::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_im = im_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_im.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create inventory movement".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        InventoryMovementIdResponse::from(new_im),
        "Inventory movement created successfully".to_string(),
        1,
    )))
}

pub async fn get_inventory_movement_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<InventoryMovementDetailResponse>>, ApiError> {
    info!("get_inventory_movement_by_id called with id: {:?}", id);

    let im = schemas::inventory_movements::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match im {
        Some(im) => Ok(Json(ApiResponse::success(
            InventoryMovementDetailResponse::from(im),
            "Inventory movement retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Inventory movement not found".to_string(),
        )),
    }
}

pub async fn get_inventory_movements(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<InventoryMovementDetailResponse>>>, ApiError> {
    info!(
        "get_inventory_movements called with pagination: page={:?}, limit={:?}, total={:?}, product_id={:?}, lot_id={:?}, location_id={:?}, reference_type={:?}, id={:?}, date_init={:?}, date_end={:?}",
        pagination.page,
        pagination.limit,
        pagination.total,
        pagination.product_id,
        pagination.lot_id,
        pagination.location_id,
        pagination.reference_type,
        pagination.id,
        pagination.date_init,
        pagination.date_end
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::inventory_movements::Entity::find();

    if let Some(product) = pagination.product_id {
        select = select.filter(schemas::inventory_movements::Column::ProductId.eq(product));
    }

    if let Some(lot) = pagination.lot_id {
        select = select.filter(schemas::inventory_movements::Column::LotId.eq(lot));
    }

    if let Some(location) = pagination.location_id {
        select = select.filter(schemas::inventory_movements::Column::LocationId.eq(location));
    }

    if let Some(ref_type) = pagination.reference_type.clone()
        && !ref_type.is_empty()
    {
        select = select.filter(schemas::inventory_movements::Column::ReferenceType.eq(ref_type));
    }

    if let Some(ref_id) = pagination.id {
        select = select.filter(schemas::inventory_movements::Column::ReferenceId.eq(ref_id));
    }

    // date range
    let start_date = pagination.date_init.clone().unwrap_or_default();
    let end_date = pagination.date_end.clone().unwrap_or_default();

    let (date_ini, date_end) = validate_date_time_range_date(&start_date, &end_date)?;

    info!(
        "Filtering inventory movements with date range: {} - {}",
        start_date, end_date
    );
    info!(
        "Fecha convertida a DateTimeWithTimeZone: {:?} - {:?}",
        date_ini, date_end
    );

    if !start_date.is_empty() && !end_date.is_empty() {
        select = select
            .filter(schemas::inventory_movements::Column::CreatedAt.between(date_ini, date_end));
    } else if !start_date.is_empty() {
        select = select.filter(schemas::inventory_movements::Column::CreatedAt.gte(date_ini));
    } else if !end_date.is_empty() {
        select = select.filter(schemas::inventory_movements::Column::CreatedAt.lte(date_end));
    }

    let paginator = select
        .order_by_asc(schemas::inventory_movements::Column::Id)
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
            .map(InventoryMovementDetailResponse::from)
            .collect(),
        "Inventory movements retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_inventory_movement(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_inventory_movement called with id: {:?}", id);

    let im = schemas::inventory_movements::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match im {
        Some(im) => {
            im.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Inventory movement deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Inventory movement not found".to_string(),
        )),
    }
}

pub async fn update_inventory_movement(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<InventoryMovementRequest>,
) -> Result<Json<ApiResponse<InventoryMovementIdResponse>>, ApiError> {
    info!(
        "update_inventory_movement called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    let im = schemas::inventory_movements::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match im {
        Some(im) => {
            let mut im_active = im.into_active_model();

            im_active.product_id = ActiveValue::Set(payload.product_id);
            im_active.lot_id = ActiveValue::Set(payload.lot_id);
            im_active.location_id = ActiveValue::Set(payload.location_id);
            im_active.change_qty = ActiveValue::Set(payload.change_qty);
            im_active.reason = ActiveValue::Set(payload.reason);
            im_active.reference_type = ActiveValue::Set(payload.reference_type);
            im_active.reference_id = ActiveValue::Set(payload.reference_id);
            im_active.cost = ActiveValue::Set(payload.cost);
            im_active.created_at = ActiveValue::Set(payload.created_at);
            im_active.created_by = ActiveValue::Set(payload.created_by);

            let updated = im_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                InventoryMovementIdResponse::from(updated),
                "Inventory movement updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Inventory movement not found".to_string(),
        )),
    }
}
