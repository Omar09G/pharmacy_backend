use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SalesDTO {
    pub id: i64,
    pub date_sale: Option<Date>,
    pub discount: f32,
    pub id_sale_detl: i64,
    pub iva: f32,
    pub msg: Option<String>,
    pub payment_method: Option<String>,
    pub payment_status: Option<String>,
    pub status: Option<String>,
    pub sub_total: f32,
    pub time_sale: Option<Time>,
    pub total: f32,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SalesDetailDTO {
    pub id: i64,
    pub date_sale: Option<Date>,
    pub product_code_bar: Option<String>,
    pub product_count: Option<i64>,
    pub product_id: i64,
    pub product_price: f32,
    pub time_sale: Option<Time>,
    pub id_sale: i64,
}

impl From<schemas::sale::Model> for SalesDTO {
    fn from(model: schemas::sale::Model) -> Self {
        SalesDTO {
            id: model.id,
            date_sale: model.date_sale,
            discount: model.discount,
            id_sale_detl: model.id_sale_detl,
            iva: model.iva,
            msg: model.msg,
            payment_method: model.payment_method,
            payment_status: model.payment_status,
            status: model.status,
            sub_total: model.sub_total,
            time_sale: model.time_sale,
            total: model.total,
            username: model.username,
        }
    }
}

impl From<schemas::saledetal::Model> for SalesDetailDTO {
    fn from(model: schemas::saledetal::Model) -> Self {
        SalesDetailDTO {
            id: model.id,
            date_sale: model.date_sale,
            product_code_bar: model.product_code_bar,
            product_count: model.product_count,
            product_id: model.product_id,
            product_price: model.product_price,
            time_sale: model.time_sale,
            id_sale: model.id_sale,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SalesResponseIdDTO {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SalesRequestDTO {
    pub id: i64,
    pub date_sale: Option<Date>,
    pub discount: f32,
    pub id_sale_detl: i64,
    pub iva: f32,
    pub msg: Option<String>,
    pub payment_method: Option<String>,
    pub payment_status: Option<String>,
    pub status: Option<String>,
    pub sub_total: f32,
    pub time_sale: Option<Time>,
    pub total: f32,
    pub username: Option<String>,
    pub details: Vec<SalesDetailDTO>,
}

impl From<schemas::sale::Model> for SalesResponseIdDTO {
    fn from(sale: schemas::sale::Model) -> Self {
        SalesResponseIdDTO { id: sale.id }
    }
}

impl From<schemas::sale::ActiveModel> for SalesResponseIdDTO {
    fn from(sale: schemas::sale::ActiveModel) -> Self {
        SalesResponseIdDTO {
            id: sale.id.unwrap(),
        }
    }
}

impl SalesRequestDTO {
    pub fn new(
        id: i64,
        date_sale: Option<Date>,
        discount: f32,
        id_sale_detl: i64,
        iva: f32,
        msg: Option<String>,
        payment_method: Option<String>,
        payment_status: Option<String>,
        status: Option<String>,
        sub_total: f32,
        time_sale: Option<Time>,
        total: f32,
        username: Option<String>,
        details: Vec<SalesDetailDTO>,
    ) -> Self {
        SalesRequestDTO {
            id,
            date_sale,
            discount,
            id_sale_detl,
            iva,
            msg,
            payment_method,
            payment_status,
            status,
            sub_total,
            time_sale,
            total,
            username,
            details,
        }
    }
}

impl From<SalesRequestDTO> for SalesDTO {
    fn from(request: SalesRequestDTO) -> Self {
        SalesDTO {
            id: request.id,
            date_sale: request.date_sale,
            discount: request.discount,
            id_sale_detl: request.id_sale_detl,
            iva: request.iva,
            msg: request.msg,
            payment_method: request.payment_method,
            payment_status: request.payment_status,
            status: request.status,
            sub_total: request.sub_total,
            time_sale: request.time_sale,
            total: request.total,
            username: request.username,
        }
    }
}

/*Get detalle de la venta para almacenar en la base de datos */
impl From<SalesRequestDTO> for SalesDetailDTO {
    fn from(request: SalesRequestDTO) -> Self {
        SalesDetailDTO {
            id: request.id,
            date_sale: request.date_sale,
            product_code_bar: None,
            product_count: None,
            product_id: 0,
            product_price: 0.0,
            time_sale: request.time_sale,
            id_sale: request.id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParamsSales {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub date_inicio: String,
    pub date_fin: String,
    pub username: Option<String>,
}

impl PaginationParamsSales {
    pub fn new(
        page: u64,
        limit: u64,
        total: u64,
        date_inicio: String,
        date_fin: String,
        username: Option<String>,
    ) -> Self {
        Self {
            page,
            limit,
            total,
            date_inicio,
            date_fin,
            username,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SalesResponseDTO {
    pub id: i64,
    pub date_sale: Option<Date>,
    pub discount: f32,
    pub id_sale_detl: i64,
    pub iva: f32,
    pub msg: Option<String>,
    pub payment_method: Option<String>,
    pub payment_status: Option<String>,
    pub status: Option<String>,
    pub sub_total: f32,
    pub time_sale: Option<Time>,
    pub total: f32,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SalesDetailResponseDTO {
    pub id: i64,
    pub date_sale: Option<Date>,
    pub product_code_bar: Option<String>,
    pub product_count: Option<i64>,
    pub product_id: i64,
    pub product_price: f32,
    pub time_sale: Option<Time>,
    pub id_sale: i64,
}

impl From<schemas::sale::Model> for SalesResponseDTO {
    fn from(model: schemas::sale::Model) -> Self {
        SalesResponseDTO {
            id: model.id,
            date_sale: model.date_sale,
            discount: model.discount,
            id_sale_detl: model.id_sale_detl,
            iva: model.iva,
            msg: model.msg,
            payment_method: model.payment_method,
            payment_status: model.payment_status,
            status: model.status,
            sub_total: model.sub_total,
            time_sale: model.time_sale,
            total: model.total,
            username: model.username,
        }
    }
}

impl From<schemas::saledetal::Model> for SalesDetailResponseDTO {
    fn from(model: schemas::saledetal::Model) -> Self {
        SalesDetailResponseDTO {
            id: model.id,
            date_sale: model.date_sale,
            product_code_bar: model.product_code_bar,
            product_count: model.product_count,
            product_id: model.product_id,
            product_price: model.product_price,
            time_sale: model.time_sale,
            id_sale: model.id_sale,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SalesResponse {
    pub id: i64,
    pub date_sale: Option<Date>,
    pub discount: f32,
    pub id_sale_detl: i64,
    pub iva: f32,
    pub msg: Option<String>,
    pub payment_method: Option<String>,
    pub payment_status: Option<String>,
    pub status: Option<String>,
    pub sub_total: f32,
    pub time_sale: Option<Time>,
    pub total: f32,
    pub username: Option<String>,
    pub details: Vec<SalesDetailResponseDTO>,
}

impl From<(schemas::sale::Model, Vec<schemas::saledetal::Model>)> for SalesResponse {
    fn from((sale, details): (schemas::sale::Model, Vec<schemas::saledetal::Model>)) -> Self {
        SalesResponse {
            id: sale.id,
            date_sale: sale.date_sale,
            discount: sale.discount,
            id_sale_detl: sale.id_sale_detl,
            iva: sale.iva,
            msg: sale.msg,
            payment_method: sale.payment_method,
            payment_status: sale.payment_status,
            status: sale.status,
            sub_total: sale.sub_total,
            time_sale: sale.time_sale,
            total: sale.total,
            username: sale.username,
            details: details
                .into_iter()
                .map(SalesDetailResponseDTO::from)
                .collect(),
        }
    }
}
