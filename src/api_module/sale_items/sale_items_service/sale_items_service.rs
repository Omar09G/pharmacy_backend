use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::sale_items::sale_items_dto::sale_items_dto::{
    SaleItemDetailResponse, SaleItemIdResponse, SaleItemRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_sale_item(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SaleItemRequest>,
) -> Result<Json<ApiResponse<SaleItemIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let si_create = schemas::sale_items::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_si = si_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_si.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create sale item".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        SaleItemIdResponse::from(new_si),
        "Sale item created successfully".to_string(),
        1,
    )))
}

pub async fn get_sale_item_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<SaleItemDetailResponse>>, ApiError> {
    let si = schemas::sale_items::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match si {
        Some(si) => Ok(Json(ApiResponse::success(
            SaleItemDetailResponse::from(si),
            "Sale item retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Sale item not found".to_string())),
    }
}

pub async fn get_sale_items(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SaleItemDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::sale_items::Entity::find();

    if let Some(sale) = pagination.sale_id {
        select = select.filter(schemas::sale_items::Column::SaleId.eq(sale));
    }

    if let Some(product) = pagination.product_id {
        select = select.filter(schemas::sale_items::Column::ProductId.eq(product));
    }

    let paginator = select
        .order_by_asc(schemas::sale_items::Column::Id)
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
            .map(SaleItemDetailResponse::from)
            .collect(),
        "Sale items retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_sale_item(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let si = schemas::sale_items::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match si {
        Some(si) => {
            si.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Sale item deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Sale item not found".to_string())),
    }
}

pub async fn update_sale_item(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<SaleItemRequest>,
) -> Result<Json<ApiResponse<SaleItemIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let si = schemas::sale_items::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match si {
        Some(si) => {
            let mut si_active = si.into_active_model();

            si_active.sale_id = ActiveValue::Set(payload.sale_id);
            si_active.product_id = ActiveValue::Set(payload.product_id);
            si_active.lot_id = ActiveValue::Set(payload.lot_id);
            si_active.qty = ActiveValue::Set(payload.qty);
            si_active.unit_price = ActiveValue::Set(payload.unit_price);
            si_active.discount = ActiveValue::Set(payload.discount);
            si_active.tax_amount = ActiveValue::Set(payload.tax_amount);
            si_active.line_total = ActiveValue::Set(payload.line_total);

            let updated = si_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                SaleItemIdResponse::from(updated),
                "Sale item updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Sale item not found".to_string())),
    }
}
