use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalePaymentAllocationRequest {
    pub id: i64,
    pub payment_id: i64,
    pub credit_invoice_id: Option<i64>,
    pub amount: Decimal,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalePaymentAllocationIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalePaymentAllocationDetailResponse {
    pub id: i64,
    pub payment_id: i64,
    pub credit_invoice_id: Option<i64>,
    pub amount: Decimal,
}

impl From<SalePaymentAllocationRequest> for schemas::sale_payment_allocations::ActiveModel {
    fn from(request: SalePaymentAllocationRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            payment_id: ActiveValue::Set(request.payment_id),
            credit_invoice_id: ActiveValue::Set(request.credit_invoice_id),
            amount: ActiveValue::Set(request.amount),
        }
    }
}

impl From<schemas::sale_payment_allocations::Model> for SalePaymentAllocationDetailResponse {
    fn from(model: schemas::sale_payment_allocations::Model) -> Self {
        Self {
            id: model.id,
            payment_id: model.payment_id,
            credit_invoice_id: model.credit_invoice_id,
            amount: model.amount,
        }
    }
}

impl From<schemas::sale_payment_allocations::ActiveModel> for SalePaymentAllocationDetailResponse {
    fn from(model: schemas::sale_payment_allocations::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            payment_id: model.payment_id.unwrap(),
            credit_invoice_id: model.credit_invoice_id.unwrap(),
            amount: model.amount.unwrap(),
        }
    }
}

impl From<schemas::sale_payment_allocations::Model> for SalePaymentAllocationIdResponse {
    fn from(model: schemas::sale_payment_allocations::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::sale_payment_allocations::ActiveModel> for SalePaymentAllocationIdResponse {
    fn from(model: schemas::sale_payment_allocations::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
