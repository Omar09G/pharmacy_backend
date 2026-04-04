use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::{
    get_current_timestamp_at_zone_mexico, get_current_timestamp_now,
};

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseRequest {
    pub id: i64,
    pub supplier_id: Option<i64>,
    pub invoice_no: Option<String>,
    pub date: DateTimeWithTimeZone,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseDetailResponse {
    pub id: i64,
    pub supplier_id: Option<i64>,
    pub invoice_no: Option<String>,
    pub date: DateTimeWithTimeZone,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
}

impl From<PurchaseRequest> for schemas::purchases::ActiveModel {
    fn from(request: PurchaseRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            supplier_id: ActiveValue::Set(request.supplier_id),
            invoice_no: ActiveValue::Set(request.invoice_no),
            date: ActiveValue::Set(request.date),
            subtotal: ActiveValue::Set(request.subtotal),
            tax_total: ActiveValue::Set(request.tax_total),
            total: ActiveValue::Set(request.total),
            status: ActiveValue::Set(request.status),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
            created_by: ActiveValue::Set(request.created_by),
        }
    }
}

impl From<schemas::purchases::Model> for PurchaseDetailResponse {
    fn from(model: schemas::purchases::Model) -> Self {
        Self {
            id: model.id,
            supplier_id: model.supplier_id,
            invoice_no: model.invoice_no,
            date: get_current_timestamp_at_zone_mexico(model.date),
            subtotal: model.subtotal,
            tax_total: model.tax_total,
            total: model.total,
            status: model.status,
            created_at: get_current_timestamp_at_zone_mexico(model.created_at),
            created_by: model.created_by,
        }
    }
}

impl From<schemas::purchases::ActiveModel> for PurchaseDetailResponse {
    fn from(model: schemas::purchases::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            supplier_id: model.supplier_id.unwrap(),
            invoice_no: model.invoice_no.unwrap(),
            date: get_current_timestamp_at_zone_mexico(model.date.unwrap()),
            subtotal: model.subtotal.unwrap(),
            tax_total: model.tax_total.unwrap(),
            total: model.total.unwrap(),
            status: model.status.unwrap(),
            created_at: get_current_timestamp_at_zone_mexico(model.created_at.unwrap()),
            created_by: model.created_by.unwrap(),
        }
    }
}

impl From<schemas::purchases::Model> for PurchaseIdResponse {
    fn from(model: schemas::purchases::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::purchases::ActiveModel> for PurchaseIdResponse {
    fn from(model: schemas::purchases::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
