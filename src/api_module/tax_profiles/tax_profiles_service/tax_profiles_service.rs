use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::{
    api_module::tax_profiles::tax_profiles_dto::tax_profiles_dto::{
        TaxProfileIdResponse, TaxProfileRequest, TaxProfileResponse,
    },
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};
use log::info;

pub async fn create_tax_profile(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<TaxProfileRequest>,
) -> Result<Json<ApiResponse<TaxProfileIdResponse>>, ApiError> {
    info!("create_tax_profile called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let tax_profile_create = schemas::tax_profiles::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    if tax_profile_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_tax_profile = tax_profile_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        TaxProfileIdResponse::from(new_tax_profile),
        "Tax profile created successfully".to_string(),
        1,
    )))
}

pub async fn get_tax_profile_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<TaxProfileResponse>>, ApiError> {
    info!("get_tax_profile_by_id called with id: {:?}", id);

    let tax_profile = schemas::tax_profiles::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if let Some(tax_profile) = tax_profile {
        Ok(Json(ApiResponse::success(
            TaxProfileResponse::from(tax_profile),
            "Tax profile retrieved successfully".to_string(),
            1,
        )))
    } else {
        Err(ApiError::NotFoundErrorDescription(
            "Tax profile not found".to_string(),
        ))
    }
}
pub async fn list_tax_profiles(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<TaxProfileResponse>>>, ApiError> {
    info!(
        "list_tax_profiles called with pagination: page={:?}, limit={:?}, total={:?}",
        pagination.page, pagination.limit, pagination.total
    );

    let paginator = schemas::tax_profiles::Entity::find()
        .order_by_asc(schemas::tax_profiles::Column::Id)
        .paginate(&app_ctx.conn, to_page_limit(pagination.limit));

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let tax_profiles = paginator
        .fetch_page(to_page_index(pagination.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        tax_profiles
            .into_iter()
            .map(TaxProfileResponse::from)
            .collect(),
        "Tax profiles retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_tax_profile(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_tax_profile called with id: {:?}", id);

    let tax_profile = schemas::tax_profiles::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if let Some(tax_profile) = tax_profile {
        tax_profile
            .delete(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        Ok(Json(ApiResponse::success(
            (),
            "Tax profile deleted successfully".to_string(),
            0,
        )))
    } else {
        Err(ApiError::NotFoundErrorDescription(
            "Tax profile not found".to_string(),
        ))
    }
}

pub async fn update_tax_profile(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<TaxProfileRequest>,
) -> Result<Json<ApiResponse<TaxProfileIdResponse>>, ApiError> {
    info!("update_tax_profile called with payload: {:?}, id: {:?}", payload, id);

    payload.validate().map_err(ApiError::Validation)?;

    let tax_profile = schemas::tax_profiles::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if let Some(tax_profile) = tax_profile {
        let mut tax_profile_active: schemas::tax_profiles::ActiveModel =
            tax_profile.into_active_model();

        tax_profile_active.name = ActiveValue::Set(payload.name);
        tax_profile_active.rate = ActiveValue::Set(payload.rate);
        tax_profile_active.is_inclusive = ActiveValue::Set(payload.is_inclusive);
        tax_profile_active.description = ActiveValue::Set(payload.description);

        let updated_tax_profile = tax_profile_active
            .save(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        Ok(Json(ApiResponse::success(
            TaxProfileIdResponse::from(updated_tax_profile),
            "Tax profile updated successfully".to_string(),
            1,
        )))
    } else {
        Err(ApiError::NotFoundErrorDescription(
            "Tax profile not found".to_string(),
        ))
    }
}

pub async fn search_tax_profiles_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<TaxProfileResponse>>>, ApiError> {
    info!(
        "search_tax_profiles_by_name called with pagination: page={:?}, limit={:?}, total={:?}, name={:?}",
        pagination.page,
        pagination.limit,
        pagination.total,
        pagination.name
    );

    let paginator = schemas::tax_profiles::Entity::find()
        .filter(
            schemas::tax_profiles::Column::Name
                .contains(pagination.name.clone().unwrap_or_default()),
        )
        .order_by_asc(schemas::tax_profiles::Column::Id)
        .paginate(&app_ctx.conn, to_page_limit(pagination.limit));

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let tax_profiles = paginator
        .fetch_page(to_page_index(pagination.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        tax_profiles
            .into_iter()
            .map(TaxProfileResponse::from)
            .collect(),
        "Tax profiles retrieved successfully".to_string(),
        total_items as i32,
    )))
}
