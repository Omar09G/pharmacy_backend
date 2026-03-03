use crate::{
    api_handlers::product::dto::product_dto::{
        ProductRequestCount, ProductRequestDTO, ProductRequestPrice, ProductResponse,
    },
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
    },
    config::config_database::config_db_context::AppContext,
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait,
    QueryOrder,
};
use validator::Validate;

pub async fn get_product_by_id(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("Fetching product with ID: {}", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    match ProductResponse::try_from(product) {
        Ok(product_response) => Ok(Json(ApiResponse::success(
            product_response,
            "Product retrieved successfully".to_string(),
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn get_all_product(
    State(app_context): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, ApiError> {
    info!("Fetching product all");

    let product_list = schemas::product::Entity::find()
        .order_by_asc(schemas::product::Column::ProductId)
        .paginate(&app_context.conn, pagination.limit)
        .fetch_page(pagination.page)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        product_list.into_iter().map(Into::into).collect(),
        "Product retrieved successfully".to_string(),
    )))
}

pub async fn create_new_product(
    State(app_context): State<AppContext>,
    Json(payload): Json<ProductRequestDTO>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn create new product ");

    payload.validate().map_err(ApiError::Validation)?;

    let new_user = schemas::product::ActiveModel {
        product_id: ActiveValue::NotSet,
        product_name: ActiveValue::Set(payload.product_name),
        product_catalog: ActiveValue::Set(payload.product_catalog),
        product_count: ActiveValue::Set(payload.product_count),
        product_code_bar: ActiveValue::Set(payload.product_code_bar),
        product_price: ActiveValue::Set(payload.product_price),
        product_desc: ActiveValue::Set(payload.product_desc),
        product_lote: ActiveValue::Set(payload.product_lote),
        product_date: ActiveValue::NotSet,
        product_lastmdate: ActiveValue::NotSet,
    }
    .save(&app_context.conn)
    .await?;

    Ok(Json(ApiResponse::success(
        new_user.into(),
        "message".to_string(),
    )))
}

pub async fn delete_product(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("fn delete product by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    product
        .delete(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        (),
        "Product delete correct".to_string(),
    )))
}

pub async fn update_product(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
    Json(payload): Json<ProductRequestDTO>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn update_product by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut update_product = product.into_active_model();

    update_product.product_count = ActiveValue::Set(payload.product_count);
    update_product.product_name = ActiveValue::Set(payload.product_name);
    update_product.product_desc = ActiveValue::Set(payload.product_desc);

    let new_update_product = update_product.save(&app_context.conn).await?;

    Ok(Json(ApiResponse::success(
        new_update_product.into(),
        "Update Correcto".to_string(),
    )))
}

pub async fn update_product_price(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
    Json(payload): Json<ProductRequestPrice>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn update_product_price by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut update_product = product.into_active_model();

    update_product.product_price = ActiveValue::Set(payload.product_price);

    let new_update_product = update_product.save(&app_context.conn).await?;

    Ok(Json(ApiResponse::success(
        new_update_product.into(),
        "Update Correcto".to_string(),
    )))
}

pub async fn update_product_count(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
    Json(payload): Json<ProductRequestCount>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn update_product_price by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut update_product = product.into_active_model();

    update_product.product_count = ActiveValue::Set(payload.product_count);

    let new_update_product = update_product.save(&app_context.conn).await?;

    Ok(Json(ApiResponse::success(
        new_update_product.into(),
        "Update Correcto".to_string(),
    )))
}
