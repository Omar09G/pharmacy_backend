use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwCashJournalBalanceResponse {
    pub cash_journal_id: i64,
    pub name: Option<String>,
    pub opening_amount: Option<Decimal>,
    pub opened_at: Option<DateTimeWithTimeZone>,
    pub closed_at: Option<DateTimeWithTimeZone>,
    pub inflow: Option<Decimal>,
    pub outflow: Option<Decimal>,
    pub balance: Option<Decimal>,
}

impl From<schemas::vw_t_cash_journal_balance::Model> for VwCashJournalBalanceResponse {
    fn from(m: schemas::vw_t_cash_journal_balance::Model) -> Self {
        Self {
            cash_journal_id: m.cash_journal_id,
            name: m.name,
            opening_amount: m.opening_amount,
            opened_at: m.opened_at,
            closed_at: m.closed_at,
            inflow: m.inflow,
            outflow: m.outflow,
            balance: m.balance,
        }
    }
}
