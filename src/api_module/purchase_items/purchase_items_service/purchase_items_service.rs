use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::purchase_items::purchase_items_dto::purchase_items_dto::{
    PurchaseItemDetailResponse, PurchaseItemIdResponse, PurchaseItemRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_purchase_item(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PurchaseItemRequest>,
) -> Result<Json<ApiResponse<PurchaseItemIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pi_create = schemas::purchase_items::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_pi = pi_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_pi.id.is_not_set() {
        return Err(ApiError::NotFoundErrorDescription(
            "Failed to create purchase item".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        PurchaseItemIdResponse::from(new_pi),
        "Purchase item created successfully".to_string(),
        1,
    )))
}

pub async fn get_purchase_item_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<PurchaseItemDetailResponse>>, ApiError> {
    let pi = schemas::purchase_items::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pi {
        Some(pi) => Ok(Json(ApiResponse::success(
            PurchaseItemDetailResponse::from(pi),
            "Purchase item retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::NotFoundErrorDescription(
            "Purchase item not found".to_string(),
        )),
    }
}

pub async fn get_purchase_items(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<PurchaseItemDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::purchase_items::Entity::find();

    if let Some(purchase) = pagination.id.or(pagination.purchase_id) {
        select = select.filter(schemas::purchase_items::Column::PurchaseId.eq(purchase));
    }

    if let Some(product) = pagination.product_id {
        select = select.filter(schemas::purchase_items::Column::ProductId.eq(product));
    }

    if let Some(lot) = pagination.lot_number.clone()
        && !lot.is_empty()
    {
        // we have lot_number in PaginationParams but purchase_items stores lot_id
        // join or filter by related lot id is out of scope; skip unless product_lots exist
    }

    let paginator = select
        .order_by_asc(schemas::purchase_items::Column::Id)
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

    if items.is_empty() {
        return Err(ApiError::NotFoundErrorDescription(
            "No purchase items found".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        items
            .into_iter()
            .map(PurchaseItemDetailResponse::from)
            .collect(),
        "Purchase items retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_purchase_item(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let pi = schemas::purchase_items::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pi {
        Some(pi) => {
            pi.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Purchase item deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::NotFoundErrorDescription(
            "Purchase item not found".to_string(),
        )),
    }
}

pub async fn update_purchase_item(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<PurchaseItemRequest>,
) -> Result<Json<ApiResponse<PurchaseItemIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pi = schemas::purchase_items::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pi {
        Some(pi) => {
            let mut pi_active = pi.into_active_model();

            // pi_active.purchase_id = ActiveValue::Set(payload.purchase_id);
            // pi_active.product_id = ActiveValue::Set(payload.product_id);
            //  pi_active.lot_id = ActiveValue::Set(payload.lot_id);
            pi_active.qty = ActiveValue::Set(payload.qty);
            pi_active.unit_cost = ActiveValue::Set(payload.unit_cost);
            pi_active.discount = ActiveValue::Set(payload.discount);
            pi_active.tax_amount = ActiveValue::Set(payload.tax_amount);
            pi_active.line_total = ActiveValue::Set(payload.line_total);

            let updated = pi_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                PurchaseItemIdResponse::from(updated),
                "Purchase item updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::NotFoundErrorDescription(
            "Purchase item not found".to_string(),
        )),
    }
}
