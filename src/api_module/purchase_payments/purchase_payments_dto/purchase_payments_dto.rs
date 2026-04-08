use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::{
    get_current_timestamp_at_zone_mexico, get_current_timestamp_now,
};

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchasePaymentRequest {
    pub purchase_id: i64,
    pub amount: Decimal,
    pub method_id: Option<i64>,
    pub paid_at: DateTimeWithTimeZone,
    pub reference: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchasePaymentIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchasePaymentDetailResponse {
    pub id: i64,
    pub purchase_id: i64,
    pub amount: Decimal,
    pub method_id: Option<i64>,
    pub paid_at: DateTimeWithTimeZone,
    pub reference: Option<String>,
}

impl TryFrom<PurchasePaymentRequest> for schemas::purchase_payments::ActiveModel {
    type Error = String;

    fn try_from(request: PurchasePaymentRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            purchase_id: ActiveValue::Set(request.purchase_id),
            amount: ActiveValue::Set(request.amount),
            method_id: ActiveValue::Set(request.method_id),
            paid_at: ActiveValue::Set(get_current_timestamp_now()),
            reference: ActiveValue::Set(request.reference),
        })
    }
}

impl From<schemas::purchase_payments::Model> for PurchasePaymentDetailResponse {
    fn from(model: schemas::purchase_payments::Model) -> Self {
        Self {
            id: model.id,
            purchase_id: model.purchase_id,
            amount: model.amount,
            method_id: model.method_id,
            paid_at: model.paid_at,
            reference: model.reference,
        }
    }
}

impl From<schemas::purchase_payments::ActiveModel> for PurchasePaymentDetailResponse {
    fn from(model: schemas::purchase_payments::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            purchase_id: model.purchase_id.unwrap(),
            amount: model.amount.unwrap(),
            method_id: model.method_id.unwrap(),
            paid_at: get_current_timestamp_at_zone_mexico(model.paid_at.unwrap()),
            reference: model.reference.unwrap(),
        }
    }
}

impl From<schemas::purchase_payments::Model> for PurchasePaymentIdResponse {
    fn from(model: schemas::purchase_payments::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::purchase_payments::ActiveModel> for PurchasePaymentIdResponse {
    fn from(model: schemas::purchase_payments::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
