use axum::Router;
use axum::routing::get;

use crate::api_module::vw_best_sellers_30d::vw_best_sellers_30d_service::get_vw_best_sellers_30d;
use crate::api_module::vw_cash_journal_balance::vw_cash_journal_balance_service::get_vw_cash_journal_balance;
use crate::api_module::vw_customer_account_summary::vw_customer_account_summary_service::get_vw_customer_account_summary;
use crate::api_module::vw_customer_invoice_aging::vw_customer_invoice_aging_service::get_vw_customer_invoice_aging;
use crate::api_module::vw_daily_cash_cut::vw_daily_cash_cut_service::get_vw_daily_cash_cut;
use crate::api_module::vw_inventory_stock::vw_inventory_stock_service::get_vw_inventory_stock;
use crate::api_module::vw_sale_items_detail::vw_sale_items_detail_service::get_vw_sale_items_detail;
use crate::api_module::vw_sales_daily_summary::vw_sales_daily_summary_service::get_vw_sales_daily_summary;
use crate::api_module::vw_sales_with_payments::vw_sales_with_payments_service::get_vw_sales_with_payments;
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const VW_SALES_DAILY_SUMMARY: &str = route!("/vw_sales_daily_summary");
const VW_BEST_SELLERS_30D: &str = route!("/vw_best_sellers_30d");
const VW_INVENTORY_STOCK: &str = route!("/vw_inventory_stock");
const VW_CASH_JOURNAL_BALANCE: &str = route!("/vw_cash_journal_balance");
const VW_DAILY_CASH_CUT: &str = route!("/vw_daily_cash_cut");
const VW_CUSTOMER_ACCOUNT_SUMMARY: &str = route!("/vw_customer_account_summary");
const VW_CUSTOMER_INVOICE_AGING: &str = route!("/vw_customer_invoice_aging");
const VW_SALES_WITH_PAYMENTS: &str = route!("/vw_sales_with_payments");
const VW_SALE_ITEMS_DETAIL: &str = route!("/vw_sale_items_detail");

pub fn routes() -> Router<AppContext> {
    Router::new()
        .route(VW_SALES_DAILY_SUMMARY, get(get_vw_sales_daily_summary))
        .route(VW_BEST_SELLERS_30D, get(get_vw_best_sellers_30d))
        .route(VW_INVENTORY_STOCK, get(get_vw_inventory_stock))
        .route(VW_CASH_JOURNAL_BALANCE, get(get_vw_cash_journal_balance))
        .route(VW_DAILY_CASH_CUT, get(get_vw_daily_cash_cut))
        .route(
            VW_CUSTOMER_ACCOUNT_SUMMARY,
            get(get_vw_customer_account_summary),
        )
        .route(
            VW_CUSTOMER_INVOICE_AGING,
            get(get_vw_customer_invoice_aging),
        )
        .route(VW_SALES_WITH_PAYMENTS, get(get_vw_sales_with_payments))
        .route(VW_SALE_ITEMS_DETAIL, get(get_vw_sale_items_detail))
}
