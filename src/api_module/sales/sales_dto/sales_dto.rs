use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::{
    get_current_timestamp_at_zone_mexico, get_current_timestamp_now,
};

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaleRequest {
    pub id: i64,
    pub customer_id: Option<i64>,
    pub user_id: Option<i64>,
    pub invoice_no: Option<String>,
    pub date: DateTimeWithTimeZone,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub total: Decimal,
    pub status: String,
    pub is_credit: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaleIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaleDetailResponse {
    pub id: i64,
    pub customer_id: Option<i64>,
    pub user_id: Option<i64>,
    pub invoice_no: Option<String>,
    pub date: DateTimeWithTimeZone,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub total: Decimal,
    pub status: String,
    pub is_credit: bool,
    pub created_at: DateTimeWithTimeZone,
}

impl TryFrom<SaleRequest> for schemas::sales::ActiveModel {
    type Error = String;

    fn try_from(request: SaleRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            customer_id: ActiveValue::Set(request.customer_id),
            user_id: ActiveValue::Set(request.user_id),
            invoice_no: ActiveValue::Set(request.invoice_no),
            date: ActiveValue::Set(get_current_timestamp_now()),
            subtotal: ActiveValue::Set(request.subtotal),
            tax_total: ActiveValue::Set(request.tax_total),
            discount_total: ActiveValue::Set(request.discount_total),
            total: ActiveValue::Set(request.total),
            status: ActiveValue::Set(request.status),
            is_credit: ActiveValue::Set(request.is_credit),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
        })
    }
}

impl From<schemas::sales::Model> for SaleDetailResponse {
    fn from(model: schemas::sales::Model) -> Self {
        Self {
            id: model.id,
            customer_id: model.customer_id,
            user_id: model.user_id,
            invoice_no: model.invoice_no,
            date: get_current_timestamp_at_zone_mexico(model.date),
            subtotal: model.subtotal,
            tax_total: model.tax_total,
            discount_total: model.discount_total,
            total: model.total,
            status: model.status,
            is_credit: model.is_credit,
            created_at: get_current_timestamp_at_zone_mexico(model.created_at),
        }
    }
}

impl From<schemas::sales::ActiveModel> for SaleDetailResponse {
    fn from(model: schemas::sales::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            customer_id: model.customer_id.unwrap(),
            user_id: model.user_id.unwrap(),
            invoice_no: model.invoice_no.unwrap(),
            date: get_current_timestamp_at_zone_mexico(model.date.unwrap()),
            subtotal: model.subtotal.unwrap(),
            tax_total: model.tax_total.unwrap(),
            discount_total: model.discount_total.unwrap(),
            total: model.total.unwrap(),
            status: model.status.unwrap(),
            is_credit: model.is_credit.unwrap(),
            created_at: get_current_timestamp_at_zone_mexico(model.created_at.unwrap()),
        }
    }
}

impl From<schemas::sales::Model> for SaleIdResponse {
    fn from(model: schemas::sales::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::sales::ActiveModel> for SaleIdResponse {
    fn from(model: schemas::sales::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
