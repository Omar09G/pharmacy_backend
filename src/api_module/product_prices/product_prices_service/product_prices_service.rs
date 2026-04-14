use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_utils::api_utils_fun::parse_mexico_date_range_to_utc;
use crate::{
    api_module::product_prices::product_prices_dto::product_prices_dto::{
        ProductPriceDetailResponse, ProductPriceIdResponse, ProductPriceRequest,
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

pub async fn create_product_price(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ProductPriceRequest>,
) -> Result<Json<ApiResponse<ProductPriceIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pp_create = schemas::product_prices::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_pp = pp_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_pp.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create product price".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        ProductPriceIdResponse::from(new_pp),
        "Product price created successfully".to_string(),
        1,
    )))
}

pub async fn get_product_price_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<ProductPriceDetailResponse>>, ApiError> {
    let pp = schemas::product_prices::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pp {
        Some(pp) => Ok(Json(ApiResponse::success(
            ProductPriceDetailResponse::from(pp),
            "Product price retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Product price not found".to_string(),
        )),
    }
}

pub async fn get_product_prices(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductPriceDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::product_prices::Entity::find();

    if let Some(product) = pagination.product_id {
        select = select.filter(schemas::product_prices::Column::ProductId.eq(product));
    }

    if let Some(price_type_filter) = pagination.price_type.clone()
        && !price_type_filter.is_empty()
    {
        select = select.filter(schemas::product_prices::Column::PriceType.eq(price_type_filter));
    }

    // date range (YYYY-MM-DD interpreted as Mexico City local time → UTC)
    let (fecha_init, fecha_end) = parse_mexico_date_range_to_utc(
        &pagination.date_init.clone().unwrap_or_default(),
        &pagination.date_end.clone().unwrap_or_default(),
    )?;

    if let Some(date_init) = fecha_init {
        select = select.filter(schemas::product_prices::Column::StartsAt.gte(date_init));
    }

    if let Some(date_end) = fecha_end {
        select = select.filter(schemas::product_prices::Column::EndsAt.lte(date_end));
    }

    let paginator = select
        .order_by_asc(schemas::product_prices::Column::Id)
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
            .map(ProductPriceDetailResponse::from)
            .collect(),
        "Product prices retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_product_price(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let pp = schemas::product_prices::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pp {
        Some(pp) => {
            pp.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Product price deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Product price not found".to_string(),
        )),
    }
}

pub async fn update_product_price(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<ProductPriceRequest>,
) -> Result<Json<ApiResponse<ProductPriceIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pp = schemas::product_prices::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pp {
        Some(pp) => {
            let mut pp_active = pp.into_active_model();

            pp_active.price_type = ActiveValue::Set(payload.price_type);
            pp_active.price = ActiveValue::Set(payload.price);
            pp_active.starts_at = ActiveValue::Set(Some(get_current_timestamp_now()));

            let updated = pp_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                ProductPriceIdResponse::from(updated),
                "Product price updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Product price not found".to_string(),
        )),
    }
}
