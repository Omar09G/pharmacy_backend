use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwSaleItemsDetailResponse {
    pub sale_item_id: i64,
    pub sale_id: Option<i64>,
    pub product_id: Option<i64>,
    pub product_name: Option<String>,
    pub lot_id: Option<i64>,
    pub qty: Option<Decimal>,
    pub unit_price: Option<Decimal>,
    pub discount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub line_total: Option<Decimal>,
}

impl From<schemas::vw_t_sale_items_detail::Model> for VwSaleItemsDetailResponse {
    fn from(m: schemas::vw_t_sale_items_detail::Model) -> Self {
        Self {
            sale_item_id: m.sale_item_id,
            sale_id: m.sale_id,
            product_id: m.product_id,
            product_name: m.product_name,
            lot_id: m.lot_id,
            qty: m.qty,
            unit_price: m.unit_price,
            discount: m.discount,
            tax_amount: m.tax_amount,
            line_total: m.line_total,
        }
    }
}
