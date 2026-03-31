use axum::{
    Json,
    extract::{Path, Query, State},
};
use log::{error, info};

use sea_orm::{IntoActiveModel, entity::prelude::*};

use crate::{
    api_handlers::{
        client::client_handler::get_client_by_id_handler,
        client_orden::client_orden_dto::{
            ClientOrdenIdResponseDTO, ClientOrdenRequestDTO, ClientOrdenResponseDTO,
            PaginationParamsOrden, ParamsOrdenPaymentPartial,
        },
    },
    api_utils::{api_error::ApiError, api_response::ApiResponse},
    config::config_database::config_db_context::AppContext,
};

use validator::Validate;

use sea_orm::QueryOrder;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};

// ---------------------- Helpers ----------------------
/// Name: `parse_date_str`
/// Description: Parse a `YYYY-MM-DD` string into a `sea_orm::Date`.
/// Parameters: `date_str` - string slice with date in `YYYY-MM-DD` format.
/// Outputs: `Ok(Date)` on success or `Err(ApiError::ValidationError)` on parse failure.
fn parse_date_str(date_str: &str) -> Result<Date, ApiError> {
    let naive = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
    match naive {
        Ok(d) => {
            let date: Date = d;
            Ok(date)
        }
        Err(e) => Err(ApiError::ValidationError(format!("Invalid date: {}", e))),
    }
}

/// Name: `to_page_index`
/// Description: Convert client 1-based `page` to 0-based page index used by paginators.
/// Parameters: `page` - page number from client (u64).
/// Outputs: 0-based `usize` page index.
fn to_page_index(page: u64) -> u64 {
    if page == 0 {
        0
    } else {
        (page.saturating_sub(1)) as u64
    }
}

pub async fn create_client_orden(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ClientOrdenRequestDTO>,
) -> Result<Json<ApiResponse<ClientOrdenIdResponseDTO>>, ApiError> {
    // Validate the incoming payload
    payload.validate().map_err(ApiError::Validation)?;

    // Create a new active model instance
    let new_client_orden = schemas::client_orden::ActiveModel::from(payload);

    // Insert the new record into the database
    let inserted_client_orden = new_client_orden
        .insert(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::ValidationError(format!("Database insertion error: {}", e)))?;

    //Validar si se guardo de forma correcta sino enviar error
    if inserted_client_orden.id == 0 {
        error!("Failed to create client order: No ID returned");
        return Err(ApiError::ValidationError(
            "Failed to create client order".to_string(),
        ));
    }

    // Return the API response
    Ok(Json(ApiResponse::success(
        ClientOrdenIdResponseDTO::from(inserted_client_orden),
        "Cliente Order Guardada Correctamente".to_string(),
        1,
    )))
}

pub async fn get_client_orden_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<ClientOrdenResponseDTO>>, ApiError> {
    // Fetch the client order by ID
    let client_orden = schemas::client_orden::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::ValidationError(format!("Database query error: {}", e)))?;

    // Check if the client order exists
    if let Some(client_orden) = client_orden {
        // Convert the model to a response DTO

        Ok(Json(ApiResponse::success(
            ClientOrdenResponseDTO::from(client_orden),
            "Cliente Order encontrada".to_string(),
            1,
        )))
    } else {
        Err(ApiError::ValidationError(format!(
            "Client order with ID {} not found",
            id
        )))
    }
}

pub async fn get_clent_ordens_date_range(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsOrden>,
) -> Result<Json<ApiResponse<Vec<ClientOrdenResponseDTO>>>, ApiError> {
    // Fetch client orders within the specified date range
    let date_inicio_str = payload.date_inicio.clone().unwrap_or_default();
    let date_fin_str = payload.date_fin.clone().unwrap_or_default();
    let client_id = payload.client_id.clone().unwrap_or_default();

    info!(
        "Fetching client orders from {} to {} for client ID {}",
        date_inicio_str, date_fin_str, client_id
    );

    if client_id <= 0 {
        return Err(ApiError::ValidationError(
            "Client ID must be provided and greater than 0".to_string(),
        ));
    }

    let client_exit = get_client_by_id_handler(State(app_ctx.clone()), Path(client_id)).await;
    if let Err(_) = client_exit {
        return Err(ApiError::ValidationError(format!(
            "Client with ID {} not found",
            client_id
        )));
    }

    let date_inicio = parse_date_str(&date_inicio_str)?;
    let date_fin = parse_date_str(&date_fin_str)?;

    let paginator = schemas::client_orden::Entity::find()
        .filter(
            schemas::client_orden::Column::DateOrden.between(date_inicio.clone(), date_fin.clone()),
        )
        .filter(schemas::client_orden::Column::ClientId.eq(payload.client_id))
        .filter(schemas::client_orden::Column::StatusOrden.eq(true))
        .filter(schemas::client_orden::Column::PaymentStatus.eq("PENDING"))
        .order_by_asc(schemas::client_orden::Column::Id)
        .paginate(&app_ctx.conn, payload.limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let client_ordens = paginator
        .fetch_page(to_page_index(payload.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let response_dtos: Vec<ClientOrdenResponseDTO> = client_ordens
        .into_iter()
        .map(ClientOrdenResponseDTO::from)
        .collect();

    if response_dtos.is_empty() {
        return Err(ApiError::ValidationError(format!(
            "No client orders found for client ID {} between {} and {}",
            client_id, date_inicio_str, date_fin_str
        )));
    }

    Ok(Json(ApiResponse::success(
        response_dtos,
        "Cliente Orders encontradas".to_string(),
        total_items as i32,
    )))
}

pub async fn get_clent_ordens_by_client(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsOrden>,
) -> Result<Json<ApiResponse<Vec<ClientOrdenResponseDTO>>>, ApiError> {
    // Fetch client orders within the specified date range
    let client_id = payload.client_id.clone().unwrap_or_default();

    info!("Fetching client orders for client ID {}", client_id);

    let paginator = schemas::client_orden::Entity::find()
        .filter(schemas::client_orden::Column::ClientId.eq(payload.client_id))
        .filter(schemas::client_orden::Column::StatusOrden.eq(true))
        .filter(schemas::client_orden::Column::PaymentStatus.eq("PENDING"))
        .order_by_asc(schemas::client_orden::Column::Id)
        .paginate(&app_ctx.conn, payload.limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let client_ordens = paginator
        .fetch_page(to_page_index(payload.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let response_dtos: Vec<ClientOrdenResponseDTO> = client_ordens
        .into_iter()
        .map(ClientOrdenResponseDTO::from)
        .collect();

    Ok(Json(ApiResponse::success(
        response_dtos,
        "Cliente Orders encontradas".to_string(),
        total_items as i32,
    )))
}

pub async fn paymet_total_client_orden(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // Fetch the client order by ID
    let client_orden = schemas::client_orden::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    let mut client_orden_active = client_orden.into_active_model();

    client_orden_active.payment_status = ActiveValue::Set(Some("PAID".to_string()));
    client_orden_active.save(&app_ctx.conn).await?;

    Ok(Json(ApiResponse::success(
        (),
        "Cliente Order pagada".to_string(),
        1,
    )))
}

pub async fn cancel_client_orden(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // Fetch the client order by ID
    let client_orden = schemas::client_orden::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    let mut client_orden_active = client_orden.into_active_model();

    client_orden_active.status_orden = ActiveValue::Set(false);
    client_orden_active.payment_status = ActiveValue::Set(Some("CANCELLED".to_string()));
    client_orden_active.payment_partial = ActiveValue::Set(None);
    client_orden_active.payment_date = ActiveValue::Set(None);
    client_orden_active.payment_time = ActiveValue::Set(None);
    client_orden_active.save(&app_ctx.conn).await?;

    Ok(Json(ApiResponse::success(
        (),
        "Cliente Order cancelada".to_string(),
        1,
    )))
}

pub async fn partial_payment_client_orden(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<ParamsOrdenPaymentPartial>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    // Fetch the client order by ID
    let client_orden = schemas::client_orden::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    let mut client_orden_active = client_orden.into_active_model();

    client_orden_active.payment_partial = ActiveValue::Set(payload.payment_partial);
    client_orden_active.payment_date = ActiveValue::Set(payload.payment_date);
    client_orden_active.payment_time = ActiveValue::Set(payload.payment_time);
    client_orden_active.payment_status = ActiveValue::Set(Some("PARTIAL".to_string()));
    client_orden_active.save(&app_ctx.conn).await?;

    Ok(Json(ApiResponse::success(
        (),
        "Pago parcial registrado para Cliente Order".to_string(),
        1,
    )))
}

pub async fn get_client_orden_summary_by_client(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsOrden>,
) -> Result<Json<ApiResponse<f32>>, ApiError> {
    // Fetch client orders within the specified date range
    let date_inicio_str = payload.date_inicio.clone().unwrap_or_default();
    let date_fin_str = payload.date_fin.clone().unwrap_or_default();
    let client_id = payload.client_id.clone().unwrap_or_default();

    info!(
        "Fetching client order summary from {} to {} for client ID {}",
        date_inicio_str, date_fin_str, client_id
    );

    if client_id <= 0 {
        return Err(ApiError::ValidationError(
            "Client ID must be provided and greater than 0".to_string(),
        ));
    }

    let client_exit = get_client_by_id_handler(State(app_ctx.clone()), Path(client_id)).await;
    if let Err(_) = client_exit {
        return Err(ApiError::ValidationError(format!(
            "Client with ID {} not found",
            client_id
        )));
    }

    let date_inicio = parse_date_str(&date_inicio_str)?;
    let date_fin = parse_date_str(&date_fin_str)?;

    let total_orden: f32 = schemas::client_orden::Entity::find()
        .filter(
            schemas::client_orden::Column::DateOrden.between(date_inicio.clone(), date_fin.clone()),
        )
        .filter(schemas::client_orden::Column::ClientId.eq(payload.client_id))
        .filter(schemas::client_orden::Column::StatusOrden.eq(true))
        .filter(schemas::client_orden::Column::PaymentStatus.eq("PENDING"))
        .all(&app_ctx.conn)
        .await?
        .into_iter()
        .map(|orden| orden.total_orden)
        .sum();

    Ok(Json(ApiResponse::new(
        total_orden,
        1,
        "Total sales calculated successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}
