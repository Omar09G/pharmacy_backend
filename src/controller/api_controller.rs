use axum::routing::{delete, get, patch, post, put};
use axum::{Router, middleware::from_fn};
use log::info;

use crate::api_module::add_product::add_product_service::add_product_service::{add_product, get_product_by_bar_code, get_products_with_details};
use crate::api_module::add_sale::add_sale_service::add_sale_service::{cancel_add_sale,create_add_sale, get_add_sale_by_id, get_add_sales_with_details};
use crate::api_module::inventory_locations::inventory_location_service::inventory_location_service::{create_inventory_location, delete_inventory_location, get_inventory_location_by_id, get_inventory_locations, update_inventory_location};
use crate::api_module::login::service::login_service::{get_login, get_profile};
use crate::api_module::payment_methods::payment_methods_service::payment_methods_service::{
    create_payment_method, delete_payment_method, get_payment_method_by_id, get_payment_methods,
    update_payment_method,
};
use crate::api_module::permissions::permissions_service::permissions_service::{
    create_permission, delete_permission, get_permission_by_id, get_permissions,
    get_permissions_by_name, update_permission,
};

use crate::api_module::role::role_service::role_service::{
    create_role, delete_role, get_role_by_id, get_roles, get_roles_by_name, update_role,
};
use crate::api_module::role_permissions::role_permissions_service::role_permissions_service::{
    create_role_permissions, delete_role_permissions, get_role_permissions,
    get_role_permissions_by_id, update_role_permissions,
};

use crate::api_module::categories::categories_service::categories_service::{
    create_category, delete_category, get_categories, get_category_by_id,
    update_category,
};
use crate::api_module::customers::customers_service::customers_service::{
    create_customer, delete_customer, get_customer_by_id, get_customers,
    update_customer,
};
use crate::api_module::customer_credit_accounts::customer_credit_accounts_service::customer_credit_accounts_service::{
    create_customer_credit_account, delete_customer_credit_account, get_customer_credit_account_by_id,
    get_customer_credit_accounts, update_customer_credit_account,
};
use crate::api_module::product_barcodes::product_barcodes_service::product_barcodes_service::{
    create_product_barcode, delete_product_barcode, get_product_barcode_by_id,
    get_product_barcodes, get_product_barcodes_by_barcode, update_product_barcode,
};
use crate::api_module::product_lots::product_lots_service::product_lots_service::{
    create_product_lot, delete_product_lot, get_product_lot_by_id, get_product_lots,
    update_product_lot,
};
use crate::api_module::product_prices::product_prices_service::product_prices_service::{
    create_product_price, delete_product_price, get_product_price_by_id, get_product_prices,
    update_product_price,
};
use crate::api_module::products::products_service::products_service::{
    create_product, delete_product, get_product_by_id, get_products, get_products_by_name,
    update_product,
};
use crate::api_module::inventory_movements::inventory_movements_service::inventory_movements_service::{
    create_inventory_movement, delete_inventory_movement, get_inventory_movement_by_id,
    get_inventory_movements, update_inventory_movement,
};
use crate::api_module::purchase_items::purchase_items_service::purchase_items_service::{
    create_purchase_item, delete_purchase_item, get_purchase_item_by_id, get_purchase_items,
    update_purchase_item,
};
use crate::api_module::purchase_payments::purchase_payments_service::purchase_payments_service::{
    create_purchase_payment, delete_purchase_payment, get_purchase_payment_by_id,
    get_purchase_payments, update_purchase_payment,
};
use crate::api_module::cash_entries::cash_entries_service::cash_entries_service::{
    create_cash_entry, delete_cash_entry, get_cash_entry_by_id, get_cash_entries, update_cash_entry,
};
use crate::api_module::cash_journals::cash_journals_service::cash_journals_service::{
    create_cash_journal, delete_cash_journal, get_cash_journal_by_id, get_cash_journals,
    update_cash_journal,
};
use crate::api_module::audit_log::audit_log_service::audit_log_service::{
    get_audit_logs,
};
use crate::api_module::purchases::purchases_service::purchases_service::{
    create_purchase, delete_purchase, get_purchase_by_id, get_purchases, update_purchase,
};
use crate::api_module::sale_items::sale_items_service::sale_items_service::{
    create_sale_item, delete_sale_item, get_sale_item_by_id, get_sale_items, update_sale_item,
};
use crate::api_module::sale_payments::sale_payments_service::sale_payments_service::{
    create_sale_payment, delete_sale_payment, get_sale_payment_by_id, get_sale_payments,
    update_sale_payment,
};
use crate::api_module::sales::sales_service::sales_service::{
    create_sale, delete_sale, get_sale_by_id, get_sales, update_sale,
};
use crate::api_module::discounts::discounts_service::discounts_service::{
    create_discount, delete_discount, get_discount_by_id, get_discounts, update_discount,
};
use crate::api_module::suppliers::suppliers_service::suppliers_service::{
    create_supplier, delete_supplier, get_supplier_by_id, get_suppliers,
    update_supplier,
};

use crate::api_module::units::units_service::units_service::{
    create_unit, delete_unit, get_unit_by_id, list_units, update_unit,
};
use crate::api_module::user::service::user_service::{
    change_user_password, change_user_status, create_user, delete_user, get_all_users,
    get_user_by_id, update_user,
};
use crate::api_module::user_role::user_role_service::user_role_service::{
    create_user_role, delete_user_role, get_user_role_by_user_id, get_user_roles, update_user_role,
};
use crate::config::config_database::config_db_context::AppContext;
use crate::config::config_middleware::auth_jwt::auth_middleware;
use crate::config::config_middleware::content_type::content_type_middleware;
use crate::config::config_middleware::cors::cors_middleware;

use crate::api_module::tax_profiles::tax_profiles_service::tax_profiles_service::{
    create_tax_profile, delete_tax_profile, get_tax_profile_by_id, list_tax_profiles,
    update_tax_profile,
};
use crate::config::config_middleware::rate_limit::rate_limit_middleware;

// API route constants
// Base API prefix and helper macro to compose routes at compile time.

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const LOGIN: &str = route!("/auth/login");
const PROFILE: &str = route!("/auth/profile");

/*Metodos USER  */
const USER: &str = route!("/user");
const USER_BY_ID: &str = route!("/user/{:id}");
const USER_CHANGE_PASSWORD: &str = route!("/user/password");
const USER_CHANGE_STATUS: &str = route!("/user/status");
const USER_LIST: &str = route!("/user");
const USER_DELETE: &str = route!("/user/{:id}");
const USER_UPDATE: &str = route!("/user/{:id}");

/*Metodos USER_ROLE  */
const USER_ROLE: &str = route!("/user_role");
const USER_ROLE_BY_ID: &str = route!("/user_role/{:user_id}/{:role_id}");
const USER_ROLE_LIST: &str = route!("/user_role");
const USER_ROLE_DELETE: &str = route!("/user_role/{:user_id}/{:role_id}");
const USER_ROLE_UPDATE: &str = route!("/user_role/{:user_id}/{:role_id}");

/*Metodos PERMISSION  */
const PERMISSION: &str = route!("/permission");
const PERMISSION_BY_ID: &str = route!("/permission/{:id}");
const PERMISSION_LIST: &str = route!("/permission");
const PERMISSION_DELETE: &str = route!("/permission/{:id}");
const PERMISSION_UPDATE: &str = route!("/permission/{:id}");
const PERMISSION_BY_NAME: &str = route!("/permission/name");

/*Metodos ROLE  */
const ROLE: &str = route!("/role");
const ROLE_BY_ID: &str = route!("/role/{:id}");
const ROLE_LIST: &str = route!("/role");
const ROLE_DELETE: &str = route!("/role/{:id}");
const ROLE_UPDATE: &str = route!("/role/{:id}");
const ROLE_BY_NAME: &str = route!("/role/name");

/*Metodos PRODUCT  */
const PRODUCT: &str = route!("/product");
const PRODUCT_BY_ID: &str = route!("/product/{:id}");
const PRODUCTS_LIST: &str = route!("/product");
const PRODUCT_DELETE: &str = route!("/product/{:id}");
const PRODUCT_UPDATE: &str = route!("/product/{:id}");
const PRODUCT_BY_NAME: &str = route!("/product/name");

/*Metodos CATEGORY  */
const CATEGORY: &str = route!("/category");
const CATEGORY_BY_ID: &str = route!("/category/{:id}");
const CATEGORY_LIST: &str = route!("/category");
const CATEGORY_DELETE: &str = route!("/category/{:id}");
const CATEGORY_UPDATE: &str = route!("/category/{:id}");

/*Metodos CUSTOMER  */
const CUSTOMER: &str = route!("/customer");
const CUSTOMER_BY_ID: &str = route!("/customer/{:id}");
const CUSTOMER_LIST: &str = route!("/customer");
const CUSTOMER_DELETE: &str = route!("/customer/{:id}");
const CUSTOMER_UPDATE: &str = route!("/customer/{:id}");
/*Metodos CUSTOMER CREDIT ACCOUNT */
const CUSTOMER_CREDIT_ACCOUNT: &str = route!("/customer_credit_account");
const CUSTOMER_CREDIT_ACCOUNT_BY_ID: &str = route!("/customer_credit_account/{:id}");
const CUSTOMER_CREDIT_ACCOUNTS_LIST: &str = route!("/customer_credit_account");
const CUSTOMER_CREDIT_ACCOUNT_DELETE: &str = route!("/customer_credit_account/{:id}");
const CUSTOMER_CREDIT_ACCOUNT_UPDATE: &str = route!("/customer_credit_account/{:id}");

/*Metodos SUPPLIER  */
const SUPPLIER: &str = route!("/supplier");
const SUPPLIER_BY_ID: &str = route!("/supplier/{:id}");
const SUPPLIER_LIST: &str = route!("/supplier");
const SUPPLIER_DELETE: &str = route!("/supplier/{:id}");
const SUPPLIER_UPDATE: &str = route!("/supplier/{:id}");

/*Metodos ROLE_PERMISSIONS  */
const ROLE_PERMISSIONS: &str = route!("/role_permissions");
const ROLE_PERMISSIONS_BY_ID: &str = route!("/role_permissions/{:role_id}");
const ROLE_PERMISSIONS_LIST: &str = route!("/role_permissions/list");
const ROLE_PERMISSIONS_DELETE: &str = route!("/role_permissions/{:role_id}/{:permission_id}");
const ROLE_PERMISSIONS_UPDATE: &str = route!("/role_permissions/{:role_id}/{:permission_id}");

/*Metodos PURCHASE  */
const PURCHASE: &str = route!("/purchase");
const PURCHASE_BY_ID: &str = route!("/purchase/{:id}");
const PURCHASES_LIST: &str = route!("/purchase");
const PURCHASE_DELETE: &str = route!("/purchase/{:id}");
const PURCHASE_UPDATE: &str = route!("/purchase/{:id}");

/*Metodos SALE  */
const SALE: &str = route!("/sale");
const SALE_BY_ID: &str = route!("/sale/{:id}");
const SALES_LIST: &str = route!("/sale");
const SALE_DELETE: &str = route!("/sale/{:id}");
const SALE_UPDATE: &str = route!("/sale/{:id}");

/*Metodos PRODUCT_LOT  */
const PRODUCT_LOT: &str = route!("/product_lot");
const PRODUCT_LOT_BY_ID: &str = route!("/product_lot/{:id}");
const PRODUCT_LOT_DELETE: &str = route!("/product_lot/{:id}");
const PRODUCT_LOT_UPDATE: &str = route!("/product_lot/{:id}");
/*Metodos PURCHASE_ITEM */
const PURCHASE_ITEM: &str = route!("/purchase_item");
const PURCHASE_ITEM_BY_ID: &str = route!("/purchase_item/{:id}");
const PURCHASE_ITEM_DELETE: &str = route!("/purchase_item/{:id}");
const PURCHASE_ITEM_UPDATE: &str = route!("/purchase_item/{:id}");
/*Metodos SALE_ITEM */
const SALE_ITEM: &str = route!("/sale_item");
const SALE_ITEM_BY_ID: &str = route!("/sale_item/{:id}");
const SALE_ITEMS_LIST: &str = route!("/sale_item");
const SALE_ITEM_DELETE: &str = route!("/sale_item/{:id}");
const SALE_ITEM_UPDATE: &str = route!("/sale_item/{:id}");
/*Metodos PURCHASE_PAYMENT  */
const PURCHASE_PAYMENT: &str = route!("/purchase_payment");
const PURCHASE_PAYMENT_BY_ID: &str = route!("/purchase_payment/{:id}");
const PURCHASE_PAYMENTS_LIST: &str = route!("/purchase_payment");
const PURCHASE_PAYMENT_DELETE: &str = route!("/purchase_payment/{:id}");
const PURCHASE_PAYMENT_UPDATE: &str = route!("/purchase_payment/{:id}");

/*Metodos CASH_ENTRY */
const CASH_ENTRY: &str = route!("/cash_entry");
const CASH_ENTRY_BY_ID: &str = route!("/cash_entry/{:id}");
const CASH_ENTRIES_LIST: &str = route!("/cash_entry");
const CASH_ENTRY_DELETE: &str = route!("/cash_entry/{:id}");
const CASH_ENTRY_UPDATE: &str = route!("/cash_entry/{:id}");

/*Metodos CASH_JOURNAL */
const CASH_JOURNAL: &str = route!("/cash_journal");
const CASH_JOURNAL_BY_ID: &str = route!("/cash_journal/{:id}");
const CASH_JOURNALS_LIST: &str = route!("/cash_journal");
const CASH_JOURNAL_DELETE: &str = route!("/cash_journal/{:id}");
const CASH_JOURNAL_UPDATE: &str = route!("/cash_journal/{:id}");

/*Metodos AUDIT_LOG */
const AUDIT_LOGS_LIST: &str = route!("/audit_log");

/*Metodos SALE_PAYMENT  */
const SALE_PAYMENT: &str = route!("/sale_payment");
const SALE_PAYMENT_BY_ID: &str = route!("/sale_payment/{:id}");
const SALE_PAYMENTS_LIST: &str = route!("/sale_payment");
const SALE_PAYMENT_DELETE: &str = route!("/sale_payment/{:id}");
const SALE_PAYMENT_UPDATE: &str = route!("/sale_payment/{:id}");

/*Metodos DISCOUNT */
const DISCOUNT: &str = route!("/discount");
const DISCOUNT_BY_ID: &str = route!("/discount/{:id}");
const DISCOUNTS_LIST: &str = route!("/discount");
const DISCOUNT_DELETE: &str = route!("/discount/{:id}");
const DISCOUNT_UPDATE: &str = route!("/discount/{:id}");

/*Metodos INVENTORY_MOVEMENT */
const INVENTORY_MOVEMENT: &str = route!("/inventory_movement");
const INVENTORY_MOVEMENT_BY_ID: &str = route!("/inventory_movement/{:id}");
const INVENTORY_MOVEMENTS_LIST: &str = route!("/inventory_movement");
const INVENTORY_MOVEMENT_DELETE: &str = route!("/inventory_movement/{:id}");
const INVENTORY_MOVEMENT_UPDATE: &str = route!("/inventory_movement/{:id}");

/*Metodos PRODUCT_BARCODE  */
const PRODUCT_BARCODE: &str = route!("/product_barcode");
const PRODUCT_BARCODE_BY_ID: &str = route!("/product_barcode/{:id}");
const PRODUCT_BARCODE_DELETE: &str = route!("/product_barcode/{:id}");
const PRODUCT_BARCODE_BY_BARCODE: &str = route!("/product_barcode/barcode");
const PRODUCT_BARCODE_UPDATE: &str = route!("/product_barcode/{:id}");

/*Metodos PRODUCT_PRICE */
const PRODUCT_PRICE: &str = route!("/product_price");
const PRODUCT_PRICE_BY_ID: &str = route!("/product_price/{:id}");
const PRODUCT_PRICE_DELETE: &str = route!("/product_price/{:id}");

/*Metodos PAYMENT_METHODS */
const PAYMENT_METHODS: &str = route!("/payment_methods");
const PAYMENT_METHODS_BY_ID: &str = route!("/payment_methods/{:id}");
const PAYMENT_METHODS_DELETE: &str = route!("/payment_methods/{:id}");
const PAYMENT_METHODS_UPDATE: &str = route!("/payment_methods/{:id}");

/* INVENTORY LOCATIONS */
const INVENTORY_LOCATIONS: &str = route!("/inventory_locations");
const INVENTORY_LOCATIONS_BY_ID: &str = route!("/inventory_locations/{:id}");
const INVENTORY_LOCATIONS_DELETE: &str = route!("/inventory_locations/{:id}");
const INVENTORY_LOCATIONS_UPDATE: &str = route!("/inventory_locations/{:id}");

/* UNITS */
const UNITS: &str = route!("/units");
const UNITS_BY_ID: &str = route!("/units/{:id}");
const UNITS_DELETE: &str = route!("/units/{:id}");
const UNITS_UPDATE: &str = route!("/units/{:id}");

/*TAX_PROFILES */
const TAX_PROFILES: &str = route!("/tax_profiles");
const TAX_PROFILES_BY_ID: &str = route!("/tax_profiles/{:id}");
const TAX_PROFILES_DELETE: &str = route!("/tax_profiles/{:id}");
const TAX_PROFILES_UPDATE: &str = route!("/tax_profiles/{:id}");

const ADD_PRODUCT: &str = route!("/add_product");
const ADD_PRODUCT_BY_DETAIL: &str = route!("/add_product");
const ADD_PRODUCT_BY_BARCODE: &str = route!("/add_product/{:barcode}");

const ADD_SALE: &str = route!("/add_sale");
const ADD_SALE_BY_ID: &str = route!("/add_sale/{:id}");

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .route(LOGIN, post(get_login))
        .route(PROFILE, get(get_profile))
        //ADD PRODUCT routes
        .route(ADD_PRODUCT, put(add_product))
        .route(ADD_PRODUCT_BY_DETAIL, get(get_products_with_details))
        .route(ADD_PRODUCT_BY_BARCODE, get(get_product_by_bar_code))
        // User routes
        .route(USER, put(create_user))
        .route(USER_BY_ID, get(get_user_by_id))
        .route(USER_CHANGE_PASSWORD, put(change_user_password))
        .route(USER_CHANGE_STATUS, put(change_user_status))
        .route(USER_LIST, get(get_all_users))
        .route(USER_DELETE, delete(delete_user))
        .route(USER_UPDATE, patch(update_user))
        // User Role routes
        .route(USER_ROLE, put(create_user_role))
        .route(USER_ROLE_BY_ID, get(get_user_role_by_user_id))
        .route(USER_ROLE_LIST, get(get_user_roles))
        .route(USER_ROLE_DELETE, delete(delete_user_role))
        .route(USER_ROLE_UPDATE, patch(update_user_role))
        // Permission routes
        .route(PERMISSION, put(create_permission))
        .route(PERMISSION_BY_ID, get(get_permission_by_id))
        .route(PERMISSION_LIST, get(get_permissions))
        .route(PERMISSION_DELETE, delete(delete_permission))
        .route(PERMISSION_UPDATE, patch(update_permission))
        .route(PERMISSION_BY_NAME, get(get_permissions_by_name))
        // Role routes
        .route(ROLE, put(create_role))
        .route(ROLE_BY_ID, get(get_role_by_id))
        .route(ROLE_LIST, get(get_roles))
        .route(ROLE_DELETE, delete(delete_role))
        .route(ROLE_UPDATE, patch(update_role))
        .route(ROLE_BY_NAME, get(get_roles_by_name))
        // Product routes
        // Role Permissions routes
        .route(ROLE_PERMISSIONS, put(create_role_permissions))
        .route(ROLE_PERMISSIONS_BY_ID, get(get_role_permissions_by_id))
        .route(ROLE_PERMISSIONS_LIST, get(get_role_permissions))
        .route(ROLE_PERMISSIONS_DELETE, delete(delete_role_permissions))
        .route(ROLE_PERMISSIONS_UPDATE, patch(update_role_permissions))
        // Product routes
        .route(PRODUCT, put(create_product))
        .route(PRODUCT_BY_ID, get(get_product_by_id))
        .route(PRODUCTS_LIST, get(get_products))
        .route(PRODUCT_DELETE, delete(delete_product))
        .route(PRODUCT_UPDATE, patch(update_product))
        .route(PRODUCT_BY_NAME, get(get_products_by_name))
        // Category routes
        .route(CATEGORY, put(create_category))
        .route(CATEGORY_BY_ID, get(get_category_by_id))
        .route(CATEGORY_LIST, get(get_categories))
        .route(CATEGORY_DELETE, delete(delete_category))
        .route(CATEGORY_UPDATE, patch(update_category))
        // Customer routes
        .route(CUSTOMER, put(create_customer))
        .route(CUSTOMER_BY_ID, get(get_customer_by_id))
        .route(CUSTOMER_LIST, get(get_customers))
        .route(CUSTOMER_DELETE, delete(delete_customer))
        .route(CUSTOMER_UPDATE, patch(update_customer))
        // Customer Credit Account routes
        .route(CUSTOMER_CREDIT_ACCOUNT, put(create_customer_credit_account))
        .route(
            CUSTOMER_CREDIT_ACCOUNT_BY_ID,
            get(get_customer_credit_account_by_id),
        )
        .route(
            CUSTOMER_CREDIT_ACCOUNTS_LIST,
            get(get_customer_credit_accounts),
        )
        .route(
            CUSTOMER_CREDIT_ACCOUNT_DELETE,
            delete(delete_customer_credit_account),
        )
        .route(
            CUSTOMER_CREDIT_ACCOUNT_UPDATE,
            patch(update_customer_credit_account),
        )
        // Supplier routes
        .route(SUPPLIER, put(create_supplier))
        .route(SUPPLIER_BY_ID, get(get_supplier_by_id))
        .route(SUPPLIER_LIST, get(get_suppliers))
        .route(SUPPLIER_DELETE, delete(delete_supplier))
        .route(SUPPLIER_UPDATE, patch(update_supplier))
        // Product Barcode routes
        .route(PRODUCT_BARCODE, put(create_product_barcode))
        .route(PRODUCT_BARCODE_BY_ID, get(get_product_barcode_by_id))
        .route(PRODUCT_BARCODE_DELETE, delete(delete_product_barcode))
        .route(PRODUCT_BARCODE, get(get_product_barcodes))
        .route(PRODUCT_BARCODE_UPDATE, patch(update_product_barcode))
        .route(
            PRODUCT_BARCODE_BY_BARCODE,
            get(get_product_barcodes_by_barcode),
        )
        // Product Price routes
        .route(PRODUCT_PRICE, put(create_product_price))
        .route(PRODUCT_PRICE_BY_ID, get(get_product_price_by_id))
        .route(PRODUCT_PRICE_DELETE, delete(delete_product_price))
        .route(PRODUCT_PRICE, get(get_product_prices))
        .route(PRODUCT_PRICE_DELETE, patch(update_product_price))
        // Product Lot routes
        .route(PRODUCT_LOT, put(create_product_lot))
        .route(PRODUCT_LOT_BY_ID, get(get_product_lot_by_id))
        .route(PRODUCT_LOT_DELETE, delete(delete_product_lot))
        .route(PRODUCT_LOT, get(get_product_lots))
        .route(PRODUCT_LOT_UPDATE, patch(update_product_lot))
        // Purchase Item routes
        .route(PURCHASE_ITEM, put(create_purchase_item))
        .route(PURCHASE_ITEM_BY_ID, get(get_purchase_item_by_id))
        .route(PURCHASE_ITEM_DELETE, delete(delete_purchase_item))
        .route(PURCHASE_ITEM, get(get_purchase_items))
        .route(PURCHASE_ITEM_UPDATE, patch(update_purchase_item))
        // Purchase Payment routes
        .route(PURCHASE_PAYMENT, put(create_purchase_payment))
        .route(PURCHASE_PAYMENT_BY_ID, get(get_purchase_payment_by_id))
        .route(PURCHASE_PAYMENTS_LIST, get(get_purchase_payments))
        .route(PURCHASE_PAYMENT_DELETE, delete(delete_purchase_payment))
        .route(PURCHASE_PAYMENT_UPDATE, patch(update_purchase_payment))
        // Cash Entry routes
        .route(CASH_ENTRY, put(create_cash_entry))
        .route(CASH_ENTRY_BY_ID, get(get_cash_entry_by_id))
        .route(CASH_ENTRIES_LIST, get(get_cash_entries))
        .route(CASH_ENTRY_DELETE, delete(delete_cash_entry))
        .route(CASH_ENTRY_UPDATE, patch(update_cash_entry))
        // Cash Journal routes
        .route(CASH_JOURNAL, put(create_cash_journal))
        .route(CASH_JOURNAL_BY_ID, get(get_cash_journal_by_id))
        .route(CASH_JOURNALS_LIST, get(get_cash_journals))
        .route(CASH_JOURNAL_DELETE, delete(delete_cash_journal))
        .route(CASH_JOURNAL_UPDATE, patch(update_cash_journal))
        // Audit Log routes
        .route(AUDIT_LOGS_LIST, get(get_audit_logs))
        // Purchase routes
        .route(PURCHASE, put(create_purchase))
        .route(PURCHASE_BY_ID, get(get_purchase_by_id))
        .route(PURCHASES_LIST, get(get_purchases))
        .route(PURCHASE_DELETE, delete(delete_purchase))
        .route(PURCHASE_UPDATE, patch(update_purchase))
        // Sale routes
        .route(SALE, put(create_sale))
        .route(SALE_BY_ID, get(get_sale_by_id))
        .route(SALES_LIST, get(get_sales))
        .route(SALE_DELETE, delete(delete_sale))
        .route(SALE_UPDATE, patch(update_sale))
        // Sale Payment routes
        .route(SALE_PAYMENT, put(create_sale_payment))
        .route(SALE_PAYMENT_BY_ID, get(get_sale_payment_by_id))
        .route(SALE_PAYMENTS_LIST, get(get_sale_payments))
        .route(SALE_PAYMENT_DELETE, delete(delete_sale_payment))
        .route(SALE_PAYMENT_UPDATE, patch(update_sale_payment))
        // Discount routes
        .route(DISCOUNT, put(create_discount))
        .route(DISCOUNT_BY_ID, get(get_discount_by_id))
        .route(DISCOUNTS_LIST, get(get_discounts))
        .route(DISCOUNT_DELETE, delete(delete_discount))
        .route(DISCOUNT_UPDATE, patch(update_discount))
        // Inventory Movement routes
        .route(INVENTORY_MOVEMENT, put(create_inventory_movement))
        .route(INVENTORY_MOVEMENT_BY_ID, get(get_inventory_movement_by_id))
        .route(INVENTORY_MOVEMENTS_LIST, get(get_inventory_movements))
        .route(INVENTORY_MOVEMENT_DELETE, delete(delete_inventory_movement))
        .route(INVENTORY_MOVEMENT_UPDATE, patch(update_inventory_movement))
        // Sale Item routes
        .route(SALE_ITEM, put(create_sale_item))
        .route(SALE_ITEM_BY_ID, get(get_sale_item_by_id))
        .route(SALE_ITEMS_LIST, get(get_sale_items))
        .route(SALE_ITEM_DELETE, delete(delete_sale_item))
        .route(SALE_ITEM_UPDATE, patch(update_sale_item))
        // Payment Methods routes
        .route(PAYMENT_METHODS, put(create_payment_method))
        .route(PAYMENT_METHODS_BY_ID, get(get_payment_method_by_id))
        .route(PAYMENT_METHODS, get(get_payment_methods))
        .route(PAYMENT_METHODS_DELETE, delete(delete_payment_method))
        .route(PAYMENT_METHODS_UPDATE, patch(update_payment_method))
        // Inventory Locations routes
        .route(INVENTORY_LOCATIONS, put(create_inventory_location))
        .route(INVENTORY_LOCATIONS_BY_ID, get(get_inventory_location_by_id))
        .route(INVENTORY_LOCATIONS, get(get_inventory_locations))
        .route(
            INVENTORY_LOCATIONS_DELETE,
            delete(delete_inventory_location),
        )
        .route(INVENTORY_LOCATIONS_UPDATE, patch(update_inventory_location))
        // Units routes
        .route(UNITS, put(create_unit))
        .route(UNITS_BY_ID, get(get_unit_by_id))
        .route(UNITS, get(list_units))
        .route(UNITS_DELETE, delete(delete_unit))
        .route(UNITS_UPDATE, patch(update_unit))
        // Tax Profiles routes
        .route(TAX_PROFILES, put(create_tax_profile))
        .route(TAX_PROFILES_BY_ID, get(get_tax_profile_by_id))
        .route(TAX_PROFILES, get(list_tax_profiles))
        .route(TAX_PROFILES_DELETE, delete(delete_tax_profile))
        .route(TAX_PROFILES_UPDATE, patch(update_tax_profile))
        // Add SALE routes
        .route(ADD_SALE, put(create_add_sale))
        .route(ADD_SALE_BY_ID, get(get_add_sale_by_id))
        .route(ADD_SALE, get(get_add_sales_with_details))
        .route(ADD_SALE_BY_ID, patch(cancel_add_sale))
        .with_state(app_ctx.clone())
        // CORS middleware must be the outermost layer so it runs before auth
        // Order: auth -> content_type -> rate_limit -> cors (cors outermost)
        .layer(from_fn(auth_middleware))
        .layer(from_fn(content_type_middleware))
        .layer(from_fn(rate_limit_middleware))
        .layer(from_fn(cors_middleware));

    Ok(router)
}
