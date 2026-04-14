use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwBestSellers30dResponse {
    pub product_id: i64,
    pub sku: Option<String>,
    pub product_name: Option<String>,
    pub qty_sold: Option<Decimal>,
    pub revenue: Option<Decimal>,
    pub sales_count: Option<i64>,
}

impl From<schemas::vw_t_best_sellers_30d::Model> for VwBestSellers30dResponse {
    fn from(m: schemas::vw_t_best_sellers_30d::Model) -> Self {
        Self {
            product_id: m.product_id,
            sku: m.sku,
            product_name: m.product_name,
            qty_sold: m.qty_sold,
            revenue: m.revenue,
            sales_count: m.sales_count,
        }
    }
}
