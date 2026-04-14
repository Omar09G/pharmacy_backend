use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwCustomerAccountSummaryResponse {
    pub customer_id: i64,
    pub customer_name: Option<String>,
    pub total_invoiced: Option<Decimal>,
    pub total_paid: Option<Decimal>,
    pub balance: Option<Decimal>,
}

impl From<schemas::vw_t_customer_account_summary::Model> for VwCustomerAccountSummaryResponse {
    fn from(m: schemas::vw_t_customer_account_summary::Model) -> Self {
        Self {
            customer_id: m.customer_id,
            customer_name: m.customer_name,
            total_invoiced: m.total_invoiced,
            total_paid: m.total_paid,
            balance: m.balance,
        }
    }
}
