use axum::{
    Json,
    extract::{Path, Query, State},
};

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
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_product_lot(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ProductLotRequest>,
) -> Result<Json<ApiResponse<ProductLotIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pl_create = schemas::product_lots::ActiveModel::from(payload);

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
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::product_lots::Entity::find();

    if let Some(product) = pagination.product_id {
        select = select.filter(schemas::product_lots::Column::ProductId.eq(product));
    }

    if let Some(lot_num) = pagination.lot_number.clone() {
        if !lot_num.is_empty() {
            select = select.filter(schemas::product_lots::Column::LotNumber.eq(lot_num));
        }
    }

    let paginator = select
        .order_by_asc(schemas::product_lots::Column::Id)
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
    Json(payload): Json<ProductLotRequest>,
) -> Result<Json<ApiResponse<ProductLotIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pl = schemas::product_lots::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pl {
        Some(pl) => {
            let mut pl_active = pl.into_active_model();

            pl_active.product_id = ActiveValue::Set(payload.product_id);
            pl_active.lot_number = ActiveValue::Set(payload.lot_number);
            pl_active.qty_on_hand = ActiveValue::Set(payload.qty_on_hand);
            pl_active.expiry_date = ActiveValue::Set(payload.expiry_date);
            pl_active.purchase_id = ActiveValue::Set(payload.purchase_id);
            pl_active.created_at = ActiveValue::Set(payload.created_at);

            let updated = pl_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

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
