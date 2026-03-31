use sea_orm::ActiveValue;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClientOrdenRequestDTO {
    pub id: i64,
    #[validate(range(min = 1, message = "Orden ID must be a positive integer"))]
    pub orden_id: i64,
    #[validate(range(min = 1, message = "Client ID must be a positive integer"))]
    pub client_id: i64,
    #[validate(range(min = 1, message = "Employee ID must be a positive integer"))]
    pub employee_id: i64,
    #[validate(range(min = 0.0, message = "Total orden must be a non-negative number"))]
    pub total_orden: f32,
    pub date_orden: Option<Date>,
    pub time_orden: Option<Time>,
    pub status_orden: bool,
    pub payment_method: Option<String>,
    pub payment_status: Option<String>,
    pub payment_partial: Option<f32>,
    pub payment_date: Option<Date>,
    pub payment_time: Option<Time>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientOrdenIdResponseDTO {
    pub id: i64,
}

impl From<schemas::client_orden::Model> for ClientOrdenIdResponseDTO {
    fn from(model: schemas::client_orden::Model) -> Self {
        ClientOrdenIdResponseDTO { id: model.id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientOrdenResponseDTO {
    pub id: i64,
    pub orden_id: i64,
    pub client_id: i64,
    pub employee_id: i64,
    pub total_orden: f32,
    pub date_orden: Option<Date>,
    pub time_orden: Option<Time>,
    pub status_orden: bool,
    pub payment_method: Option<String>,
    pub payment_status: Option<String>,
    pub payment_partial: Option<f32>,
    pub payment_date: Option<Date>,
    pub payment_time: Option<Time>,
}

impl From<schemas::client_orden::Model> for ClientOrdenResponseDTO {
    fn from(model: schemas::client_orden::Model) -> Self {
        ClientOrdenResponseDTO {
            id: model.id,
            orden_id: model.orden_id,
            client_id: model.client_id,
            employee_id: model.employee_id,
            total_orden: model.total_orden,
            date_orden: model.date_orden,
            time_orden: model.time_orden,
            status_orden: model.status_orden,
            payment_method: model.payment_method,
            payment_status: model.payment_status,
            payment_partial: model.payment_partial,
            payment_date: model.payment_date,
            payment_time: model.payment_time,
        }
    }
}

impl From<ClientOrdenRequestDTO> for schemas::client_orden::ActiveModel {
    fn from(dto: ClientOrdenRequestDTO) -> Self {
        schemas::client_orden::ActiveModel {
            id: ActiveValue::NotSet,
            orden_id: ActiveValue::Set(dto.orden_id),
            client_id: ActiveValue::Set(dto.client_id),
            employee_id: ActiveValue::Set(dto.employee_id),
            total_orden: ActiveValue::Set(dto.total_orden),
            date_orden: ActiveValue::Set(dto.date_orden),
            time_orden: ActiveValue::Set(dto.time_orden),
            status_orden: ActiveValue::Set(dto.status_orden),
            payment_method: ActiveValue::Set(dto.payment_method),
            payment_status: ActiveValue::Set(dto.payment_status),
            payment_partial: ActiveValue::Set(dto.payment_partial),
            payment_date: ActiveValue::Set(dto.payment_date),
            payment_time: ActiveValue::Set(dto.payment_time),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParamsOrden {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub date_inicio: Option<String>,
    pub date_fin: Option<String>,
    pub username: Option<String>,
    pub client_id: Option<i64>,
    pub id_orden: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsOrdenPaymentPartial {
    pub id: i64,
    pub payment_partial: Option<f32>,
    pub payment_date: Option<Date>,
    pub payment_time: Option<Time>,
}
