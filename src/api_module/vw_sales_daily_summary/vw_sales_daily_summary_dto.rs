use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwSalesDailySummaryResponse {
    pub day: DateTimeWithTimeZone,
    pub sales_count: Option<i64>,
    pub subtotal: Option<Decimal>,
    pub tax_total: Option<Decimal>,
    pub discount_total: Option<Decimal>,
    pub total: Option<Decimal>,
    pub total_credit: Option<Decimal>,
}

impl From<schemas::vw_t_sales_daily_summary::Model> for VwSalesDailySummaryResponse {
    fn from(m: schemas::vw_t_sales_daily_summary::Model) -> Self {
        Self {
            day: m.day,
            sales_count: m.sales_count,
            subtotal: m.subtotal,
            tax_total: m.tax_total,
            discount_total: m.discount_total,
            total: m.total,
            total_credit: m.total_credit,
        }
    }
}
