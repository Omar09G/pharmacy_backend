use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::api_module::cash_entries::cash_entries_service::cash_entries_service::{
    create_cash_entry, delete_cash_entry, get_cash_entries, get_cash_entry_by_id, update_cash_entry,
};
use crate::api_module::cash_journals::cash_journals_service::cash_journals_service::{
    create_cash_journal, delete_cash_journal, get_cash_journal_by_id, get_cash_journals,
    update_cash_journal,
};
use crate::api_module::payment_methods::payment_methods_service::payment_methods_service::{
    create_payment_method, delete_payment_method, get_payment_method_by_id, get_payment_methods,
    update_payment_method,
};
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const CASH_ENTRY: &str = route!("/cash_entry");
const CASH_ENTRY_BY_ID: &str = route!("/cash_entry/{:id}");
const CASH_ENTRIES_LIST: &str = route!("/cash_entry");
const CASH_ENTRY_DELETE: &str = route!("/cash_entry/{:id}");
const CASH_ENTRY_UPDATE: &str = route!("/cash_entry/{:id}");

const CASH_JOURNAL: &str = route!("/cash_journal");
const CASH_JOURNAL_BY_ID: &str = route!("/cash_journal/{:id}");
const CASH_JOURNALS_LIST: &str = route!("/cash_journal");
const CASH_JOURNAL_DELETE: &str = route!("/cash_journal/{:id}");
const CASH_JOURNAL_UPDATE: &str = route!("/cash_journal/{:id}");

const PAYMENT_METHODS: &str = route!("/payment_methods");
const PAYMENT_METHODS_BY_ID: &str = route!("/payment_methods/{:id}");
const PAYMENT_METHODS_DELETE: &str = route!("/payment_methods/{:id}");
const PAYMENT_METHODS_UPDATE: &str = route!("/payment_methods/{:id}");

pub fn routes() -> Router<AppContext> {
    Router::new()
        // Cash Entry routes
        .route(CASH_ENTRY, post(create_cash_entry))
        .route(CASH_ENTRY_BY_ID, get(get_cash_entry_by_id))
        .route(CASH_ENTRIES_LIST, get(get_cash_entries))
        .route(CASH_ENTRY_DELETE, delete(delete_cash_entry))
        .route(CASH_ENTRY_UPDATE, patch(update_cash_entry))
        // Cash Journal routes
        .route(CASH_JOURNAL, post(create_cash_journal))
        .route(CASH_JOURNAL_BY_ID, get(get_cash_journal_by_id))
        .route(CASH_JOURNALS_LIST, get(get_cash_journals))
        .route(CASH_JOURNAL_DELETE, delete(delete_cash_journal))
        .route(CASH_JOURNAL_UPDATE, patch(update_cash_journal))
        // Payment Methods routes
        .route(PAYMENT_METHODS, post(create_payment_method))
        .route(PAYMENT_METHODS_BY_ID, get(get_payment_method_by_id))
        .route(PAYMENT_METHODS, get(get_payment_methods))
        .route(PAYMENT_METHODS_DELETE, delete(delete_payment_method))
        .route(PAYMENT_METHODS_UPDATE, patch(update_payment_method))
}
