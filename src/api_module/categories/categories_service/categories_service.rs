use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::categories::categories_dto::categories_dto::{
    CategoryDetailResponse, CategoryIdResponse, CategoryRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};
use log::info;

pub async fn create_category(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<CategoryRequest>,
) -> Result<Json<ApiResponse<CategoryIdResponse>>, ApiError> {
    info!("create_category called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let category_create = schemas::categories::ActiveModel::from(payload);

    if category_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_category = category_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_category.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create category".to_string(),
        ));
    }

    // invalidate category caches
    let _ = tokio::spawn(async move {
        let _ = crate::config::config_redis::del_pattern("categories:*").await;
        let _ = crate::config::config_redis::del_pattern("category:*").await;
    });

    Ok(Json(ApiResponse::success(
        CategoryIdResponse::from(new_category),
        "Category created successfully".to_string(),
        1,
    )))
}

pub async fn get_category_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<CategoryDetailResponse>>, ApiError> {
    info!("get_category_by_id called with id: {:?}", id);

    let category = schemas::categories::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match category {
        Some(category) => {
            let cache_key = format!("category:{}", id);
            match crate::config::config_redis::get_json::<CategoryDetailResponse>(&cache_key).await
            {
                Ok(Some(cached)) => {
                    return Ok(Json(ApiResponse::success(
                        cached,
                        "Category retrieved successfully (cache)".to_string(),
                        1,
                    )));
                }
                Ok(None) => (),
                Err(e) => info!("redis get_json error: {}", e),
            }

            let dto = CategoryDetailResponse::from(category.clone());
            let _ = tokio::spawn(async move {
                let _ = crate::config::config_redis::set_json(&cache_key, &dto, 3600).await;
            });

            Ok(Json(ApiResponse::success(
                CategoryDetailResponse::from(category),
                "Category retrieved successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::ValidationError("Category not found".to_string())),
    }
}

pub async fn get_categories(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<CategoryDetailResponse>>>, ApiError> {
    info!(
        "get_categories called with pagination: name={:?}, parent_id={:?}, page={:?}, limit={:?}, total={:?}",
        pagination.name, pagination.parent_id, pagination.page, pagination.limit, pagination.total
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::categories::Entity::find();

    // Apply optional filters
    if let Some(name_filter) = pagination.name.clone()
        && !name_filter.is_empty()
    {
        select = select.filter(schemas::categories::Column::Name.eq(name_filter));
    }

    if let Some(parent) = pagination.parent_id {
        select = select.filter(schemas::categories::Column::ParentId.eq(parent));
    }

    let paginator = select
        .order_by_asc(schemas::categories::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let categories = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        categories
            .into_iter()
            .map(CategoryDetailResponse::from)
            .collect(),
        "Categories retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_category(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_category called with id: {:?}", id);

    let category = schemas::categories::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match category {
        Some(category) => {
            category
                .delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            // invalidate category caches
            let _ = tokio::spawn(async move {
                let _ = crate::config::config_redis::del_pattern("categories:*").await;
                let _ = crate::config::config_redis::del_key(&format!("category:{}", id)).await;
            });
            Ok(Json(ApiResponse::success(
                (),
                "Category deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Category not found".to_string())),
    }
}

pub async fn get_categories_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<CategoryDetailResponse>>>, ApiError> {
    info!(
        "get_categories_by_name called with pagination: {:?}",
        pagination
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);
    let name_filter = pagination.name.clone().unwrap_or_default();

    if name_filter.is_empty() {
        return Err(ApiError::ValidationError(
            "Name filter cannot be empty".to_string(),
        ));
    }

    let paginator = schemas::categories::Entity::find()
        .filter(schemas::categories::Column::Name.eq(name_filter))
        .order_by_asc(schemas::categories::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let categories = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        categories
            .into_iter()
            .map(CategoryDetailResponse::from)
            .collect(),
        "Categories retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn update_category(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<CategoryRequest>,
) -> Result<Json<ApiResponse<CategoryIdResponse>>, ApiError> {
    info!(
        "update_category called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    let category = schemas::categories::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match category {
        Some(category) => {
            let mut category_active_model = category.into_active_model();

            category_active_model.name = ActiveValue::Set(payload.name);
            category_active_model.parent_id = ActiveValue::Set(payload.parent_id);
            category_active_model.description = ActiveValue::Set(payload.description);

            let updated_category = category_active_model
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            // invalidate caches for this category and lists
            let cid = id;
            let _ = tokio::spawn(async move {
                let _ = crate::config::config_redis::del_key(&format!("category:{}", cid)).await;
                let _ = crate::config::config_redis::del_pattern("categories:*").await;
            });

            Ok(Json(ApiResponse::success(
                CategoryIdResponse::from(updated_category),
                "Category updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Category not found".to_string())),
    }
}
