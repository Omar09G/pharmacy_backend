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

use crate::api_module::product_lots::product_lots_dto::product_lots_dto::{
    ProductLotDetailResponse, ProductLotIdResponse, ProductLotRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{get_current_timestamp_now, to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_product_lot(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ProductLotRequest>,
) -> Result<Json<ApiResponse<ProductLotIdResponse>>, ApiError> {
    info!("create_product_lot called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let pl_create = schemas::product_lots::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_pl = pl_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_pl.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create product lot".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        ProductLotIdResponse::from(new_pl),
        "Product lot created successfully".to_string(),
        1,
    )))
}

pub async fn get_product_lot_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<ProductLotDetailResponse>>, ApiError> {
    info!("get_product_lot_by_id called with id: {:?}", id);

    let pl = schemas::product_lots::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pl {
        Some(pl) => Ok(Json(ApiResponse::success(
            ProductLotDetailResponse::from(pl),
            "Product lot retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Product lot not found".to_string(),
        )),
    }
}

pub async fn get_product_lots(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductLotDetailResponse>>>, ApiError> {
    info!(
        "get_product_lots called with pagination: page={:?}, limit={:?}, total={:?}, product_id={:?}, lot_number={:?}",
        pagination.page,
        pagination.limit,
        pagination.total,
        pagination.product_id,
        pagination.lot_number
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::product_lots::Entity::find();

    if let Some(product) = pagination.product_id {
        select = select.filter(schemas::product_lots::Column::ProductId.eq(product));
    }

    if let Some(lot_num) = pagination.lot_number.clone()
        && !lot_num.is_empty()
    {
        select = select.filter(schemas::product_lots::Column::LotNumber.eq(lot_num));
    }

    let paginator = select
        .order_by_asc(schemas::product_lots::Column::Id)
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
            .map(ProductLotDetailResponse::from)
            .collect(),
        "Product lots retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_product_lot(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_product_lot called with id: {:?}", id);

    let pl = schemas::product_lots::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pl {
        Some(pl) => {
            pl.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Product lot deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Product lot not found".to_string(),
        )),
    }
}

pub async fn update_product_lot(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<ProductLotRequest>,
) -> Result<Json<ApiResponse<ProductLotIdResponse>>, ApiError> {
    info!(
        "update_product_lot called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    info!("Updating product lot with ID {}: {:?}", id, payload);

    let pl = schemas::product_lots::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pl {
        Some(pl) => {
            // Stock itself is mutated exclusively by trg_update_lot_after_insert
            // when a movement row lands; here we only compute and validate.
            let qyt_on_hand_current = pl.qty_on_hand;
            let mut pl_active = pl.into_active_model();

            let qty_on_hand_update = payload.qty_on_hand;
            let qty_on_hand_final = qyt_on_hand_current + qty_on_hand_update;

            if qty_on_hand_final < sea_orm::prelude::Decimal::ZERO {
                return Err(ApiError::ValidationError(
                    "Resulting quantity cannot be negative".to_string(),
                ));
            }

            info!(
                "Current qty_on_hand: {}, Update: {}, Final qty_on_hand: {}",
                qyt_on_hand_current, qty_on_hand_update, qty_on_hand_final
            );

            if let Some(lot_number) = payload.lot_number.clone() {
                pl_active.lot_number = ActiveValue::Set(Some(lot_number));
            }
            if let Some(expiry_date) = payload.expiry_date {
                pl_active.expiry_date = ActiveValue::Set(Some(expiry_date));
            }
            let updated = pl_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            // Keep the movements ledger in sync; the trigger applies the delta.
            if qty_on_hand_update != sea_orm::prelude::Decimal::ZERO {
                let lot_id = updated.id.clone().unwrap();
                let movement = schemas::inventory_movements::ActiveModel {
                    id: sea_orm::ActiveValue::NotSet,
                    product_id: sea_orm::ActiveValue::Set(updated.product_id.clone().unwrap()),
                    lot_id: sea_orm::ActiveValue::Set(Some(lot_id)),
                    location_id: sea_orm::ActiveValue::NotSet,
                    change_qty: sea_orm::ActiveValue::Set(qty_on_hand_update),
                    reason: sea_orm::ActiveValue::Set("restock".to_string()),
                    reference_type: sea_orm::ActiveValue::Set(Some("manual".to_string())),
                    reference_id: sea_orm::ActiveValue::NotSet,
                    cost: sea_orm::ActiveValue::NotSet,
                    created_at: sea_orm::ActiveValue::Set(get_current_timestamp_now()),
                    created_by: sea_orm::ActiveValue::NotSet,
                };
                movement
                    .insert(&app_ctx.conn)
                    .await
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            }

            Ok(Json(ApiResponse::success(
                ProductLotIdResponse::from(updated),
                "Product lot updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Product lot not found".to_string(),
        )),
    }
}

pub async fn adjust_product_lot(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<ProductLotRequest>,
) -> Result<Json<ApiResponse<ProductLotIdResponse>>, ApiError> {
    info!(
        "adjust_product_lot called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    info!("Adjusting product lot with ID {}: {:?}", id, payload);

    let pl = schemas::product_lots::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pl {
        Some(pl) => {
            // The trigger trg_update_lot_after_insert owns qty_on_hand; we
            // compute the delta, validate, and let the movement apply it.
            let qyt_on_hand_current = pl.qty_on_hand;
            let mut pl_active = pl.into_active_model();

            if payload.qty_on_hand < sea_orm::prelude::Decimal::ZERO {
                return Err(ApiError::ValidationError(
                    "Quantity cannot be negative".to_string(),
                ));
            }

            info!(
                "Current qty_on_hand: {}, Adjusting to: {}",
                qyt_on_hand_current, payload.qty_on_hand
            );

            let delta = payload.qty_on_hand - qyt_on_hand_current;
            if let Some(lot_number) = payload.lot_number.clone() {
                pl_active.lot_number = ActiveValue::Set(Some(lot_number));
            }
            if let Some(expiry_date) = payload.expiry_date {
                pl_active.expiry_date = ActiveValue::Set(Some(expiry_date));
            }
            let updated = pl_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            // Keep the movements ledger in sync; the trigger applies the delta.
            if delta != sea_orm::prelude::Decimal::ZERO {
                let lot_id = updated.id.clone().unwrap();
                let movement = schemas::inventory_movements::ActiveModel {
                    id: ActiveValue::NotSet,
                    product_id: ActiveValue::Set(updated.product_id.clone().unwrap()),
                    lot_id: ActiveValue::Set(Some(lot_id)),
                    location_id: ActiveValue::NotSet,
                    change_qty: ActiveValue::Set(delta),
                    reason: ActiveValue::Set("adjustment".to_string()),
                    reference_type: ActiveValue::Set(Some("manual".to_string())),
                    reference_id: ActiveValue::NotSet,
                    cost: ActiveValue::NotSet,
                    created_at: ActiveValue::Set(get_current_timestamp_now()),
                    created_by: ActiveValue::NotSet,
                };
                movement
                    .insert(&app_ctx.conn)
                    .await
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            }

            Ok(Json(ApiResponse::success(
                ProductLotIdResponse::from(updated),
                "Product lot adjusted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Product lot not found".to_string(),
        )),
    }
}

pub async fn get_product_lot_by_barcode(
    State(app_ctx): State<AppContext>,
    Path(bar_code): Path<String>,
) -> Result<Json<ApiResponse<ProductLotDetailResponse>>, ApiError> {
    info!(
        "get_product_lot_by_barcode called with bar_code: {:?}",
        bar_code
    );

    let pbar_code = schemas::product_barcodes::Entity::find()
        .filter(schemas::product_barcodes::Column::Barcode.eq(bar_code))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let id = pbar_code
        .map(|pb| pb.product_id)
        .ok_or_else(|| ApiError::ValidationError("Bar code not found".to_string()))?;

    let pl = schemas::product_lots::Entity::find()
        .filter(schemas::product_lots::Column::ProductId.eq(id))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pl {
        Some(pl) => Ok(Json(ApiResponse::success(
            ProductLotDetailResponse::from(pl),
            "Product lot retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Product lot not found".to_string(),
        )),
    }
}
