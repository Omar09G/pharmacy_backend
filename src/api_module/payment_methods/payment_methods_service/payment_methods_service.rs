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
    api_module::payment_methods::payment_methods_dto::payment_methods_dto::{
        PaymentMethodIdResponse, PaymentMethodRequest, PaymentMethodResponse,
    },
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};
use log::info;

pub async fn create_payment_method(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PaymentMethodRequest>,
) -> Result<Json<ApiResponse<PaymentMethodIdResponse>>, ApiError> {
    info!("create_payment_method called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let payment_method_create = schemas::payment_methods::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    if payment_method_create.name.is_not_set() || payment_method_create.method_type.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_payment_method = payment_method_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        PaymentMethodIdResponse::from(new_payment_method),
        "Payment method created successfully".to_string(),
        1,
    )))
}

pub async fn get_payment_method_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<PaymentMethodResponse>>, ApiError> {
    info!("get_payment_method_by_id called with id: {:?}", id);

    let payment_method = schemas::payment_methods::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match payment_method {
        Some(pm) => Ok(Json(ApiResponse::success(
            PaymentMethodResponse::from(pm),
            "Payment method retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::NotFound),
    }
}

pub async fn delete_payment_method(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_payment_method called with id: {:?}", id);

    let payment_method = schemas::payment_methods::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match payment_method {
        Some(pm) => {
            pm.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Payment method deleted successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::NotFound),
    }
}

pub async fn get_payment_methods(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<PaymentMethodResponse>>>, ApiError> {
    info!(
        "get_payment_methods called with pagination: page={:?}, limit={:?}, total={:?}",
        pagination.page, pagination.limit, pagination.total
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let paginator = schemas::payment_methods::Entity::find()
        .order_by_asc(schemas::payment_methods::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let payment_methods = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .into_iter()
        .map(PaymentMethodResponse::from)
        .collect();

    Ok(Json(ApiResponse::success(
        payment_methods,
        "Payment methods retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn update_payment_method(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<PaymentMethodRequest>,
) -> Result<Json<ApiResponse<PaymentMethodIdResponse>>, ApiError> {
    info!(
        "update_payment_method called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    let payment_method = schemas::payment_methods::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let mut payment_method = match payment_method {
        Some(pm) => pm.into_active_model(),
        None => return Err(ApiError::NotFound),
    };

    payment_method.name = ActiveValue::Set(payload.name);
    payment_method.method_type = ActiveValue::Set(payload.method_type);
    payment_method.active = ActiveValue::Set(payload.active);

    let updated_payment_method = payment_method
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        PaymentMethodIdResponse::from(updated_payment_method),
        "Payment method updated successfully".to_string(),
        1,
    )))
}

pub async fn search_payment_methods_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<PaymentMethodIdResponse>>>, ApiError> {
    info!(
        "search_payment_methods_by_name called with pagination: page={:?}, limit={:?}, total={:?}, name={:?}",
        pagination.page, pagination.limit, pagination.total, pagination.name
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let name_filter = pagination.name.unwrap_or_default();

    if name_filter.is_empty() {
        return Err(ApiError::ValidationError(
            "Name filter cannot be empty".to_string(),
        ));
    }

    let paginator = schemas::payment_methods::Entity::find()
        .filter(schemas::payment_methods::Column::Name.contains(&name_filter))
        .order_by_asc(schemas::payment_methods::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let payment_methods = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .into_iter()
        .map(PaymentMethodIdResponse::from)
        .collect();

    Ok(Json(ApiResponse::success(
        payment_methods,
        "Payment methods retrieved successfully".to_string(),
        total_items as i32,
    )))
}
