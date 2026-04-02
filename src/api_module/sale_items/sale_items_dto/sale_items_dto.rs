use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaleItemRequest {
    pub id: i64,
    pub sale_id: i64,
    pub product_id: i64,
    pub lot_id: Option<i64>,
    pub qty: Decimal,
    pub unit_price: Decimal,
    pub discount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub line_total: Decimal,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaleItemIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SaleItemDetailResponse {
    pub id: i64,
    pub sale_id: i64,
    pub product_id: i64,
    pub lot_id: Option<i64>,
    pub qty: Decimal,
    pub unit_price: Decimal,
    pub discount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub line_total: Decimal,
}

impl From<SaleItemRequest> for schemas::sale_items::ActiveModel {
    fn from(request: SaleItemRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            sale_id: ActiveValue::Set(request.sale_id),
            product_id: ActiveValue::Set(request.product_id),
            lot_id: ActiveValue::Set(request.lot_id),
            qty: ActiveValue::Set(request.qty),
            unit_price: ActiveValue::Set(request.unit_price),
            discount: ActiveValue::Set(request.discount),
            tax_amount: ActiveValue::Set(request.tax_amount),
            line_total: ActiveValue::Set(request.line_total),
        }
    }
}

impl From<schemas::sale_items::Model> for SaleItemDetailResponse {
    fn from(model: schemas::sale_items::Model) -> Self {
        Self {
            id: model.id,
            sale_id: model.sale_id,
            product_id: model.product_id,
            lot_id: model.lot_id,
            qty: model.qty,
            unit_price: model.unit_price,
            discount: model.discount,
            tax_amount: model.tax_amount,
            line_total: model.line_total,
        }
    }
}

impl From<schemas::sale_items::ActiveModel> for SaleItemDetailResponse {
    fn from(model: schemas::sale_items::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            sale_id: model.sale_id.unwrap(),
            product_id: model.product_id.unwrap(),
            lot_id: model.lot_id.unwrap(),
            qty: model.qty.unwrap(),
            unit_price: model.unit_price.unwrap(),
            discount: model.discount.unwrap(),
            tax_amount: model.tax_amount.unwrap(),
            line_total: model.line_total.unwrap(),
        }
    }
}

impl From<schemas::sale_items::Model> for SaleItemIdResponse {
    fn from(model: schemas::sale_items::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::sale_items::ActiveModel> for SaleItemIdResponse {
    fn from(model: schemas::sale_items::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
