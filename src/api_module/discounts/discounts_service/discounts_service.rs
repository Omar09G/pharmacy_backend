use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::discounts::discounts_dto::discounts_dto::{
    DiscountDetailResponse, DiscountIdResponse, DiscountRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_discount(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<DiscountRequest>,
) -> Result<Json<ApiResponse<DiscountIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let d_create = schemas::discounts::ActiveModel::from(payload);

    let new_d = d_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_d.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create discount".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        DiscountIdResponse::from(new_d),
        "Discount created successfully".to_string(),
        1,
    )))
}

pub async fn get_discount_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<DiscountDetailResponse>>, ApiError> {
    let d = schemas::discounts::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match d {
        Some(d) => Ok(Json(ApiResponse::success(
            DiscountDetailResponse::from(d),
            "Discount retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Discount not found".to_string())),
    }
}

pub async fn get_discounts(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<DiscountDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::discounts::Entity::find();

    if let Some(code) = pagination.name.clone()
        && !code.is_empty() {
            select = select.filter(schemas::discounts::Column::Name.eq(code));
        }

    if let Some(product) = pagination.product_id {
        select = select.filter(schemas::discounts::Column::ProductId.eq(product));
    }

    if let Some(category) = pagination.category_id {
        select = select.filter(schemas::discounts::Column::CategoryId.eq(category));
    }

    if let Some(customer) = pagination.customer_id {
        select = select.filter(schemas::discounts::Column::CustomerId.eq(customer));
    }

    if let Some(active) = pagination.is_sellable {
        // reuse is_sellable field as `active` filter if provided
        select = select.filter(schemas::discounts::Column::Active.eq(active));
    }

    let paginator = select
        .order_by_asc(schemas::discounts::Column::Id)
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
            .map(DiscountDetailResponse::from)
            .collect(),
        "Discounts retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_discount(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let d = schemas::discounts::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match d {
        Some(d) => {
            d.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Discount deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Discount not found".to_string())),
    }
}

pub async fn update_discount(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<DiscountRequest>,
) -> Result<Json<ApiResponse<DiscountIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let d = schemas::discounts::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match d {
        Some(d) => {
            let mut d_active = d.into_active_model();

            d_active.code = ActiveValue::Set(payload.code);
            d_active.name = ActiveValue::Set(payload.name);
            d_active.description = ActiveValue::Set(payload.description);
            d_active.discount_type = ActiveValue::Set(payload.discount_type);
            d_active.value = ActiveValue::Set(payload.value);
            d_active.applies_to = ActiveValue::Set(payload.applies_to);
            d_active.product_id = ActiveValue::Set(payload.product_id);
            d_active.category_id = ActiveValue::Set(payload.category_id);
            d_active.customer_id = ActiveValue::Set(payload.customer_id);
            d_active.min_qty = ActiveValue::Set(payload.min_qty);
            d_active.max_uses = ActiveValue::Set(payload.max_uses);
            d_active.priority = ActiveValue::Set(payload.priority);
            d_active.start_at = ActiveValue::Set(payload.start_at);
            d_active.end_at = ActiveValue::Set(payload.end_at);
            d_active.active = ActiveValue::Set(payload.active);
            d_active.created_at = ActiveValue::Set(payload.created_at);
            d_active.created_by = ActiveValue::Set(payload.created_by);

            let updated = d_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                DiscountIdResponse::from(updated),
                "Discount updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Discount not found".to_string())),
    }
}
