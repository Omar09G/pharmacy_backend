use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait,
    QueryOrder,
};
use validator::Validate;

use crate::{
    api_module::inventory_locations::inventory_location_dto::inventory_location_dto::{
        InventoryLocationIdResponse, InventoryLocationRequest,
    },
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_inventory_location(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<InventoryLocationRequest>,
) -> Result<Json<ApiResponse<InventoryLocationIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let inventory_location_create = schemas::inventory_locations::ActiveModel::from(payload);

    if inventory_location_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_inventory_location = inventory_location_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_inventory_location.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create inventory location".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        InventoryLocationIdResponse::from(new_inventory_location),
        "Inventory location created successfully".to_string(),
        1,
    )))
}

pub async fn get_inventory_location_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<InventoryLocationIdResponse>>, ApiError> {
    let inventory_location = schemas::inventory_locations::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match inventory_location {
        Some(il) => Ok(Json(ApiResponse::success(
            InventoryLocationIdResponse::from(il),
            "Inventory location retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Inventory location not found".to_string(),
        )),
    }
}

pub async fn delete_inventory_location(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let inventory_location = schemas::inventory_locations::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match inventory_location {
        Some(il) => {
            il.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Inventory location deleted successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Inventory location not found".to_string(),
        )),
    }
}

pub async fn get_inventory_locations(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<InventoryLocationIdResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let paginator = schemas::inventory_locations::Entity::find()
        .order_by_asc(schemas::inventory_locations::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let inventory_locations = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let inventory_location_responses = inventory_locations
        .into_iter()
        .map(InventoryLocationIdResponse::from)
        .collect();

    Ok(Json(ApiResponse::success(
        inventory_location_responses,
        "Inventory locations retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn update_inventory_location(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<InventoryLocationRequest>,
) -> Result<Json<ApiResponse<InventoryLocationIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let inventory_location = schemas::inventory_locations::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let mut inventory_location = match inventory_location {
        Some(il) => il.into_active_model(),
        None => {
            return Err(ApiError::ValidationError(
                "Inventory location not found".to_string(),
            ));
        }
    };

    inventory_location.r#type = ActiveValue::Set(payload.r#type);
    inventory_location.description = ActiveValue::Set(payload.description);

    let updated_inventory_location = inventory_location
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        InventoryLocationIdResponse::from(updated_inventory_location),
        "Inventory location updated successfully".to_string(),
        1,
    )))
}
