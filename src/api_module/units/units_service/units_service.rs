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
    api_module::units::units_dto::units_dto::{UnitIdResponse, UnitRequest, UnitResponse},
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_unit(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<UnitRequest>,
) -> Result<Json<ApiResponse<UnitIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let unit_create = schemas::units::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    if unit_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_unit = unit_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        UnitIdResponse::from(new_unit),
        "Unit created successfully".to_string(),
        1,
    )))
}

pub async fn get_unit_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<UnitResponse>>, ApiError> {
    let unit = schemas::units::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if let Some(unit) = unit {
        Ok(Json(ApiResponse::success(
            UnitResponse::from(unit),
            "Unit retrieved successfully".to_string(),
            1,
        )))
    } else {
        Err(ApiError::NotFoundErrorDescription(
            "Unit not found".to_string(),
        ))
    }
}

pub async fn list_units(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<UnitResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let paginator = schemas::units::Entity::find()
        .order_by_asc(schemas::units::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let units = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let unit_responses: Vec<UnitResponse> = units.into_iter().map(UnitResponse::from).collect();

    Ok(Json(ApiResponse::success(
        unit_responses,
        "Units retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_unit(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let unit = schemas::units::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match unit {
        Some(unit) => {
            unit.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Unit deleted successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::NotFoundErrorDescription(
            "Unit not found".to_string(),
        )),
    }
}
pub async fn update_unit(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<UnitRequest>,
) -> Result<Json<ApiResponse<UnitResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let unit = schemas::units::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if let Some(unit) = unit {
        let mut unit_active: schemas::units::ActiveModel = unit.into_active_model();
        unit_active.code = ActiveValue::Set(payload.code);
        unit_active.name = ActiveValue::Set(payload.name);
        unit_active.precision = ActiveValue::Set(payload.precision);

        let updated_unit = unit_active
            .save(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        Ok(Json(ApiResponse::success(
            UnitResponse::from(updated_unit),
            "Unit updated successfully".to_string(),
            1,
        )))
    } else {
        Err(ApiError::NotFoundErrorDescription(
            "Unit not found".to_string(),
        ))
    }
}

pub async fn search_units_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<UnitResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let inits_name = pagination.inits_name.clone().unwrap_or_default();
    if inits_name.is_empty() {
        return Err(ApiError::ValidationError(
            "Name filter cannot be empty".to_string(),
        ));
    }

    let paginator = schemas::units::Entity::find()
        .filter(schemas::units::Column::Code.eq(inits_name))
        .order_by_asc(schemas::units::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let units = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let unit_responses: Vec<UnitResponse> = units.into_iter().map(UnitResponse::from).collect();

    Ok(Json(ApiResponse::success(
        unit_responses,
        "Units retrieved successfully".to_string(),
        total_items as i32,
    )))
}
