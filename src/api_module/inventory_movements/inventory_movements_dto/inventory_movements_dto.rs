use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryMovementRequest {
    pub product_id: i64,
    pub lot_id: Option<i64>,
    pub location_id: Option<i64>,
    pub change_qty: Decimal,
    pub reason: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub cost: Option<Decimal>,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryMovementIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryMovementDetailResponse {
    pub id: i64,
    pub product_id: i64,
    pub lot_id: Option<i64>,
    pub location_id: Option<i64>,
    pub change_qty: Decimal,
    pub reason: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub cost: Option<Decimal>,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
}

impl TryFrom<InventoryMovementRequest> for schemas::inventory_movements::ActiveModel {
    type Error = String;

    fn try_from(request: InventoryMovementRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            product_id: ActiveValue::Set(request.product_id),
            lot_id: ActiveValue::Set(request.lot_id),
            location_id: ActiveValue::Set(request.location_id),
            change_qty: ActiveValue::Set(request.change_qty),
            reason: ActiveValue::Set(request.reason),
            reference_type: ActiveValue::Set(request.reference_type),
            reference_id: ActiveValue::Set(request.reference_id),
            cost: ActiveValue::Set(request.cost),
            created_at: ActiveValue::Set(request.created_at),
            created_by: ActiveValue::Set(request.created_by),
        })
    }
}

impl From<schemas::inventory_movements::Model> for InventoryMovementDetailResponse {
    fn from(model: schemas::inventory_movements::Model) -> Self {
        Self {
            id: model.id,
            product_id: model.product_id,
            lot_id: model.lot_id,
            location_id: model.location_id,
            change_qty: model.change_qty,
            reason: model.reason,
            reference_type: model.reference_type,
            reference_id: model.reference_id,
            cost: model.cost,
            created_at: model.created_at,
            created_by: model.created_by,
        }
    }
}

impl From<schemas::inventory_movements::ActiveModel> for InventoryMovementDetailResponse {
    fn from(model: schemas::inventory_movements::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            product_id: model.product_id.unwrap(),
            lot_id: model.lot_id.unwrap(),
            location_id: model.location_id.unwrap(),
            change_qty: model.change_qty.unwrap(),
            reason: model.reason.unwrap(),
            reference_type: model.reference_type.unwrap(),
            reference_id: model.reference_id.unwrap(),
            cost: model.cost.unwrap(),
            created_at: model.created_at.unwrap(),
            created_by: model.created_by.unwrap(),
        }
    }
}

impl From<schemas::inventory_movements::Model> for InventoryMovementIdResponse {
    fn from(model: schemas::inventory_movements::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::inventory_movements::ActiveModel> for InventoryMovementIdResponse {
    fn from(model: schemas::inventory_movements::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
