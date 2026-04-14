use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VwSalesWithPaymentsResponse {
    pub id: i64,
    pub invoice_no: Option<String>,
    pub date: Option<DateTimeWithTimeZone>,
    pub customer_id: Option<i64>,
    pub customer_name: Option<String>,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub subtotal: Option<Decimal>,
    pub tax_total: Option<Decimal>,
    pub discount_total: Option<Decimal>,
    pub total: Option<Decimal>,
    pub status: Option<String>,
    pub is_credit: Option<bool>,
    pub paid_amount: Option<Decimal>,
    pub allocated_amount: Option<Decimal>,
    pub outstanding: Option<Decimal>,
}

impl From<schemas::vw_t_sales_with_payments::Model> for VwSalesWithPaymentsResponse {
    fn from(m: schemas::vw_t_sales_with_payments::Model) -> Self {
        Self {
            id: m.id,
            invoice_no: m.invoice_no,
            date: m.date,
            customer_id: m.customer_id,
            customer_name: m.customer_name,
            user_id: m.user_id,
            user_name: m.user_name,
            subtotal: m.subtotal,
            tax_total: m.tax_total,
            discount_total: m.discount_total,
            total: m.total,
            status: m.status,
            is_credit: m.is_credit,
            paid_amount: m.paid_amount,
            allocated_amount: m.allocated_amount,
            outstanding: m.outstanding,
        }
    }
}
