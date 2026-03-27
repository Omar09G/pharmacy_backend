use crate::{
    api_handlers::client::client_dto::{ClientRequest, ClientResponse},
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
    ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, PaginatorTrait, QueryOrder,
};
use validator::Validate;

pub async fn create_client_handler(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ClientRequest>,
) -> Result<Json<ApiResponse<ClientResponse>>, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    info!("Creating client: {:?}", payload);

    let client_active_model: schemas::client::ActiveModel = payload.into();

    let client = client_active_model
        .insert(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        client.into(),
        "Cliente creado exitosamente".to_string(),
        1,
    )))
}

pub async fn get_all_clients_handler(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ClientResponse>>>, ApiError> {
    info!(
        "Fetching clients with pagination: page={}, limit={}",
        pagination.page, pagination.limit
    );

    let paginator = schemas::client::Entity::find()
        .order_by_asc(schemas::client::Column::ClientId)
        .paginate(&app_ctx.conn, pagination.limit);

    let total_clients = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let clients = paginator
        .fetch_page(pagination.page)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        clients.into_iter().map(|c| c.into()).collect(),
        "Clientes obtenidos exitosamente".to_string(),
        total_clients as i32,
    )))
}

pub async fn get_client_by_id_handler(
    State(app_ctx): State<AppContext>,
    Path(client_id): Path<i64>,
) -> Result<Json<ApiResponse<ClientResponse>>, ApiError> {
    info!("Fetching client with ID: {}", client_id);

    let client = schemas::client::Entity::find_by_id(client_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match client {
        Some(c) => Ok(Json(ApiResponse::success(
            c.into(),
            "Cliente obtenido exitosamente".to_string(),
            1,
        ))),
        None => Err(ApiError::NotFound),
    }
}

pub async fn delete_client_handler(
    State(app_ctx): State<AppContext>,
    Path(client_id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("Deleting client with ID: {}", client_id);

    let client = schemas::client::Entity::find_by_id(client_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match client {
        Some(c) => {
            c.into_active_model()
                .delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Cliente eliminado exitosamente".to_string(),
                0,
            )))
        }
        None => Err(ApiError::NotFound),
    }
}

pub async fn update_client_handler(
    State(app_ctx): State<AppContext>,
    Path(client_id): Path<i64>,
    Json(payload): Json<ClientRequest>,
) -> Result<Json<ApiResponse<ClientResponse>>, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::ValidationError(e.to_string()))?;

    info!("Updating client with ID: {}", client_id);

    let client = schemas::client::Entity::find_by_id(client_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match client {
        Some(c) => {
            let mut active_model: schemas::client::ActiveModel = c.into_active_model();
            info!("Updating client: {:?}", active_model);
            active_model.client_address = ActiveValue::Set(payload.client_address);
            active_model.client_city = ActiveValue::Set(payload.client_city);
            active_model.client_country = ActiveValue::Set(payload.client_country);
            active_model.client_last_name = ActiveValue::Set(payload.client_last_name);
            active_model.client_name = ActiveValue::Set(payload.client_name);
            active_model.client_phone = ActiveValue::Set(payload.client_phone);
            active_model.client_status = ActiveValue::Set(payload.client_status);
            active_model.client_type = ActiveValue::Set(payload.client_type);

            let updated_client = active_model
                .update(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                updated_client.into(),
                "Cliente actualizado exitosamente".to_string(),
                1,
            )))
        }
        None => Err(ApiError::NotFound),
    }
}
