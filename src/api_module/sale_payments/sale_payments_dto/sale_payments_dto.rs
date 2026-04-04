use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalePaymentRequest {
    pub id: i64,
    pub sale_id: Option<i64>,
    pub amount: Decimal,
    pub method_id: Option<i64>,
    pub paid_at: DateTimeWithTimeZone,
    pub reference: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalePaymentIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalePaymentDetailResponse {
    pub id: i64,
    pub sale_id: Option<i64>,
    pub amount: Decimal,
    pub method_id: Option<i64>,
    pub paid_at: DateTimeWithTimeZone,
    pub reference: Option<String>,
}

impl TryFrom<SalePaymentRequest> for schemas::sale_payments::ActiveModel {
    type Error = String;

    fn try_from(request: SalePaymentRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            sale_id: ActiveValue::Set(request.sale_id),
            amount: ActiveValue::Set(request.amount),
            method_id: ActiveValue::Set(request.method_id),
            paid_at: ActiveValue::Set(request.paid_at),
            reference: ActiveValue::Set(request.reference),
        })
    }
}

impl From<schemas::sale_payments::Model> for SalePaymentDetailResponse {
    fn from(model: schemas::sale_payments::Model) -> Self {
        Self {
            id: model.id,
            sale_id: model.sale_id,
            amount: model.amount,
            method_id: model.method_id,
            paid_at: model.paid_at,
            reference: model.reference,
        }
    }
}

impl From<schemas::sale_payments::ActiveModel> for SalePaymentDetailResponse {
    fn from(model: schemas::sale_payments::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            sale_id: model.sale_id.unwrap(),
            amount: model.amount.unwrap(),
            method_id: model.method_id.unwrap(),
            paid_at: model.paid_at.unwrap(),
            reference: model.reference.unwrap(),
        }
    }
}

impl From<schemas::sale_payments::Model> for SalePaymentIdResponse {
    fn from(model: schemas::sale_payments::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::sale_payments::ActiveModel> for SalePaymentIdResponse {
    fn from(model: schemas::sale_payments::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
