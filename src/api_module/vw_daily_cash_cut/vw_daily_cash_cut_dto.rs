use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwDailyCashCutResponse {
    pub day: DateTimeWithTimeZone,
    pub sales_cash: Option<Decimal>,
    pub sales_non_cash: Option<Decimal>,
    pub cash_entries_in: Option<Decimal>,
    pub cash_entries_out: Option<Decimal>,
    pub net_cash: Option<Decimal>,
}

impl From<schemas::vw_t_daily_cash_cut::Model> for VwDailyCashCutResponse {
    fn from(m: schemas::vw_t_daily_cash_cut::Model) -> Self {
        Self {
            day: m.day,
            sales_cash: m.sales_cash,
            sales_non_cash: m.sales_non_cash,
            cash_entries_in: m.cash_entries_in,
            cash_entries_out: m.cash_entries_out,
            net_cash: m.net_cash,
        }
    }
}
