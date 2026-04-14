use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::api_module::customers::customers_service::customers_service::{
    create_customer, delete_customer, get_customer_by_id, get_customers, update_customer,
};
use crate::api_module::customer_credit_accounts::customer_credit_accounts_service::customer_credit_accounts_service::{
    create_customer_credit_account, delete_customer_credit_account,
    get_customer_credit_account_by_id, get_customer_credit_accounts,
    update_customer_credit_account,
};
use crate::api_module::suppliers::suppliers_service::suppliers_service::{
    create_supplier, delete_supplier, get_supplier_by_id, get_suppliers, update_supplier,
};
use crate::api_module::tax_profiles::tax_profiles_service::tax_profiles_service::{
    create_tax_profile, delete_tax_profile, get_tax_profile_by_id, list_tax_profiles,
    update_tax_profile,
};
use crate::api_module::units::units_service::units_service::{
    create_unit, delete_unit, get_unit_by_id, list_units, update_unit,
};
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const CUSTOMER: &str = route!("/customer");
const CUSTOMER_BY_ID: &str = route!("/customer/{:id}");
const CUSTOMER_LIST: &str = route!("/customer");
const CUSTOMER_DELETE: &str = route!("/customer/{:id}");
const CUSTOMER_UPDATE: &str = route!("/customer/{:id}");

const CUSTOMER_CREDIT_ACCOUNT: &str = route!("/customer_credit_account");
const CUSTOMER_CREDIT_ACCOUNT_BY_ID: &str = route!("/customer_credit_account/{:id}");
const CUSTOMER_CREDIT_ACCOUNTS_LIST: &str = route!("/customer_credit_account");
const CUSTOMER_CREDIT_ACCOUNT_DELETE: &str = route!("/customer_credit_account/{:id}");
const CUSTOMER_CREDIT_ACCOUNT_UPDATE: &str = route!("/customer_credit_account/{:id}");

const SUPPLIER: &str = route!("/supplier");
const SUPPLIER_BY_ID: &str = route!("/supplier/{:id}");
const SUPPLIER_LIST: &str = route!("/supplier");
const SUPPLIER_DELETE: &str = route!("/supplier/{:id}");
const SUPPLIER_UPDATE: &str = route!("/supplier/{:id}");

const UNITS: &str = route!("/units");
const UNITS_BY_ID: &str = route!("/units/{:id}");
const UNITS_DELETE: &str = route!("/units/{:id}");
const UNITS_UPDATE: &str = route!("/units/{:id}");

const TAX_PROFILES: &str = route!("/tax_profiles");
const TAX_PROFILES_BY_ID: &str = route!("/tax_profiles/{:id}");
const TAX_PROFILES_DELETE: &str = route!("/tax_profiles/{:id}");
const TAX_PROFILES_UPDATE: &str = route!("/tax_profiles/{:id}");

pub fn routes() -> Router<AppContext> {
    Router::new()
        // Customer routes
        .route(CUSTOMER, post(create_customer))
        .route(CUSTOMER_BY_ID, get(get_customer_by_id))
        .route(CUSTOMER_LIST, get(get_customers))
        .route(CUSTOMER_DELETE, delete(delete_customer))
        .route(CUSTOMER_UPDATE, patch(update_customer))
        // Customer Credit Account routes
        .route(CUSTOMER_CREDIT_ACCOUNT, post(create_customer_credit_account))
        .route(CUSTOMER_CREDIT_ACCOUNT_BY_ID, get(get_customer_credit_account_by_id))
        .route(CUSTOMER_CREDIT_ACCOUNTS_LIST, get(get_customer_credit_accounts))
        .route(CUSTOMER_CREDIT_ACCOUNT_DELETE, delete(delete_customer_credit_account))
        .route(CUSTOMER_CREDIT_ACCOUNT_UPDATE, patch(update_customer_credit_account))
        // Supplier routes
        .route(SUPPLIER, post(create_supplier))
        .route(SUPPLIER_BY_ID, get(get_supplier_by_id))
        .route(SUPPLIER_LIST, get(get_suppliers))
        .route(SUPPLIER_DELETE, delete(delete_supplier))
        .route(SUPPLIER_UPDATE, patch(update_supplier))
        // Units routes
        .route(UNITS, post(create_unit))
        .route(UNITS_BY_ID, get(get_unit_by_id))
        .route(UNITS, get(list_units))
        .route(UNITS_DELETE, delete(delete_unit))
        .route(UNITS_UPDATE, patch(update_unit))
        // Tax Profiles routes
        .route(TAX_PROFILES, post(create_tax_profile))
        .route(TAX_PROFILES_BY_ID, get(get_tax_profile_by_id))
        .route(TAX_PROFILES, get(list_tax_profiles))
        .route(TAX_PROFILES_DELETE, delete(delete_tax_profile))
        .route(TAX_PROFILES_UPDATE, patch(update_tax_profile))
}
