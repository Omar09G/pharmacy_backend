use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwInventoryStockResponse {
    pub product_id: i64,
    pub sku: Option<String>,
    pub product_name: Option<String>,
    pub qty_on_hand: Option<Decimal>,
    pub max_expiry_date: Option<Date>,
    pub last_movement_at: Option<DateTimeWithTimeZone>,
}

impl From<schemas::vw_t_inventory_stock::Model> for VwInventoryStockResponse {
    fn from(m: schemas::vw_t_inventory_stock::Model) -> Self {
        Self {
            product_id: m.product_id,
            sku: m.sku,
            product_name: m.product_name,
            qty_on_hand: m.qty_on_hand,
            max_expiry_date: m.max_expiry_date,
            last_movement_at: m.last_movement_at,
        }
    }
}
