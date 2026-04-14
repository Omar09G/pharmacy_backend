use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwCustomerInvoiceAgingResponse {
    pub invoice_id: i64,
    pub invoice_no: Option<String>,
    pub customer_id: Option<i64>,
    pub customer_name: Option<String>,
    pub invoice_date: Option<DateTimeWithTimeZone>,
    pub due_date: Option<Date>,
    pub paid_amount: Option<Decimal>,
    pub outstanding: Option<Decimal>,
    pub invoice_status: Option<String>,
    pub days_overdue: Option<i32>,
}

impl From<schemas::vw_t_customer_invoice_aging::Model> for VwCustomerInvoiceAgingResponse {
    fn from(m: schemas::vw_t_customer_invoice_aging::Model) -> Self {
        Self {
            invoice_id: m.invoice_id,
            invoice_no: m.invoice_no,
            customer_id: m.customer_id,
            customer_name: m.customer_name,
            invoice_date: m.invoice_date,
            due_date: m.due_date,
            paid_amount: m.paid_amount,
            outstanding: m.outstanding,
            invoice_status: m.invoice_status,
            days_overdue: m.days_overdue,
        }
    }
}
