use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseItemRequest {
    pub id: i64,
    pub purchase_id: i64,
    pub product_id: i64,
    pub lot_id: Option<i64>,
    pub qty: Decimal,
    pub unit_cost: Decimal,
    pub discount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub line_total: Decimal,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseItemIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseItemDetailResponse {
    pub id: i64,
    pub purchase_id: i64,
    pub product_id: i64,
    pub lot_id: Option<i64>,
    pub qty: Decimal,
    pub unit_cost: Decimal,
    pub discount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub line_total: Decimal,
}

impl TryFrom<PurchaseItemRequest> for schemas::purchase_items::ActiveModel {
    type Error = String;

    fn try_from(request: PurchaseItemRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            purchase_id: ActiveValue::Set(request.purchase_id),
            product_id: ActiveValue::Set(request.product_id),
            lot_id: ActiveValue::Set(request.lot_id),
            qty: ActiveValue::Set(request.qty),
            unit_cost: ActiveValue::Set(request.unit_cost),
            discount: ActiveValue::Set(request.discount),
            tax_amount: ActiveValue::Set(request.tax_amount),
            line_total: ActiveValue::Set(request.line_total),
        })
    }
}

impl From<schemas::purchase_items::Model> for PurchaseItemDetailResponse {
    fn from(model: schemas::purchase_items::Model) -> Self {
        Self {
            id: model.id,
            purchase_id: model.purchase_id,
            product_id: model.product_id,
            lot_id: model.lot_id,
            qty: model.qty,
            unit_cost: model.unit_cost,
            discount: model.discount,
            tax_amount: model.tax_amount,
            line_total: model.line_total,
        }
    }
}

impl From<schemas::purchase_items::ActiveModel> for PurchaseItemDetailResponse {
    fn from(model: schemas::purchase_items::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            purchase_id: model.purchase_id.unwrap(),
            product_id: model.product_id.unwrap(),
            lot_id: model.lot_id.unwrap(),
            qty: model.qty.unwrap(),
            unit_cost: model.unit_cost.unwrap(),
            discount: model.discount.unwrap(),
            tax_amount: model.tax_amount.unwrap(),
            line_total: model.line_total.unwrap(),
        }
    }
}

impl From<schemas::purchase_items::Model> for PurchaseItemIdResponse {
    fn from(model: schemas::purchase_items::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::purchase_items::ActiveModel> for PurchaseItemIdResponse {
    fn from(model: schemas::purchase_items::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
