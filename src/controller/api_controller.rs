use axum::routing::{delete, get, patch, post, put};
use axum::{Router, middleware::from_fn};
use log::info;

use crate::api_module::login::service::login_service::{get_login, get_profile};
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

use crate::api_module::user::service::user_service::{
    change_user_password, change_user_status, create_user, delete_user, get_all_users,
    get_user_by_id, update_user,
};
use crate::config::config_database::config_db_context::AppContext;
//use crate::config::config_middleware::auth_jwt::auth_middleware;
use crate::config::config_middleware::cors::cors_middleware;

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
const USER_LIST: &str = route!("/users");
const USER_DELETE: &str = route!("/user");
const USER_UPDATE: &str = route!("/user");

/*Metodos PERMISSION  */
const PERMISSION: &str = route!("/permission");
const PERMISSION_BY_ID: &str = route!("/permission/{:id}");
const PERMISSION_LIST: &str = route!("/permissions");
const PERMISSION_DELETE: &str = route!("/permission");
const PERMISSION_UPDATE: &str = route!("/permission");
const PERMISSION_BY_NAME: &str = route!("/permission/name");

/*Metodos ROLE  */
const ROLE: &str = route!("/role");
const ROLE_BY_ID: &str = route!("/role/{:id}");
const ROLE_LIST: &str = route!("/roles");
const ROLE_DELETE: &str = route!("/role");
const ROLE_UPDATE: &str = route!("/role");
const ROLE_BY_NAME: &str = route!("/role/name");

/*Metodos PRODUCT  */
const PRODUCT: &str = route!("/product");
const PRODUCT_BY_ID: &str = route!("/product/{:id}");
const PRODUCTS_LIST: &str = route!("/products");
const PRODUCT_DELETE: &str = route!("/product");
const PRODUCT_UPDATE: &str = route!("/product");
const PRODUCT_BY_NAME: &str = route!("/product/name");

/*Metodos CATEGORY  */
const CATEGORY: &str = route!("/category");
const CATEGORY_BY_ID: &str = route!("/category/{:id}");
const CATEGORY_LIST: &str = route!("/categories");
const CATEGORY_DELETE: &str = route!("/category");
const CATEGORY_UPDATE: &str = route!("/category");

/*Metodos CUSTOMER  */
const CUSTOMER: &str = route!("/customer");
const CUSTOMER_BY_ID: &str = route!("/customer/{:id}");
const CUSTOMER_LIST: &str = route!("/customers");
const CUSTOMER_DELETE: &str = route!("/customer");
const CUSTOMER_UPDATE: &str = route!("/customer");

/*Metodos SUPPLIER  */
const SUPPLIER: &str = route!("/supplier");
const SUPPLIER_BY_ID: &str = route!("/supplier/{:id}");
const SUPPLIER_LIST: &str = route!("/suppliers");
const SUPPLIER_DELETE: &str = route!("/supplier");
const SUPPLIER_UPDATE: &str = route!("/supplier");

/*Metodos ROLE_PERMISSIONS  */
const ROLE_PERMISSIONS: &str = route!("/role_permissions");
const ROLE_PERMISSIONS_BY_ID: &str = route!("/role_permissions/{:role_id}/{:permission_id}");
const ROLE_PERMISSIONS_LIST: &str = route!("/role_permissions/list");
const ROLE_PERMISSIONS_DELETE: &str = route!("/role_permissions/{:role_id}/{:permission_id}");
const ROLE_PERMISSIONS_UPDATE: &str = route!("/role_permissions/{:role_id}/{:permission_id}");

/*Metodos PURCHASE  */
const PURCHASE: &str = route!("/purchase");
const PURCHASE_BY_ID: &str = route!("/purchase/{:id}");
const PURCHASES_LIST: &str = route!("/purchases");
const PURCHASE_DELETE: &str = route!("/purchase");
const PURCHASE_UPDATE: &str = route!("/purchase");

/*Metodos PURCHASE_ITEM  */
const PURCHASE_ITEM: &str = route!("/purchase_item");
const PURCHASE_ITEM_BY_ID: &str = route!("/purchase_item/{:id}");
const PURCHASE_ITEM_DELETE: &str = route!("/purchase_item");

/*Metodos PURCHASE_PAYMENT  */
const PURCHASE_PAYMENT: &str = route!("/purchase_payment");
const PURCHASE_PAYMENT_BY_ID: &str = route!("/purchase_payment/{:id}");
const PURCHASE_PAYMENT_DELETE: &str = route!("/purchase_payment");

/*Metodos SALE  */
const SALE: &str = route!("/sale");
const SALE_BY_ID: &str = route!("/sale/{:id}");
const SALES_LIST: &str = route!("/sales");
const SALE_DELETE: &str = route!("/sale");
const SALE_UPDATE: &str = route!("/sale");

/*Metodos SALE_ITEM  */
const SALE_ITEM: &str = route!("/sale_item");
const SALE_ITEM_BY_ID: &str = route!("/sale_item/{:id}");
const SALE_ITEM_DELETE: &str = route!("/sale_item");

/*Metodos SALE_PAYMENT  */
const SALE_PAYMENT: &str = route!("/sale_payment");
const SALE_PAYMENT_BY_ID: &str = route!("/sale_payment/{:id}");
const SALE_PAYMENTS_LIST: &str = route!("/sale_payments");
const SALE_PAYMENT_DELETE: &str = route!("/sale_payment");

/*Metodos INVENTORY_MOVEMENT  */
const INVENTORY_MOVEMENT: &str = route!("/inventory_movement");
const INVENTORY_MOVEMENT_BY_ID: &str = route!("/inventory_movement/{:id}");
const INVENTORY_MOVEMENTS_LIST: &str = route!("/inventory_movements");
const INVENTORY_MOVEMENT_DELETE: &str = route!("/inventory_movement");

/*Metodos PRODUCT_LOT  */
const PRODUCT_LOT: &str = route!("/product_lot");
const PRODUCT_LOT_BY_ID: &str = route!("/product_lot/{:id}");
const PRODUCT_LOT_DELETE: &str = route!("/product_lot");

/*Metodos PRODUCT_BARCODE  */
const PRODUCT_BARCODE: &str = route!("/product_barcode");
const PRODUCT_BARCODE_BY_ID: &str = route!("/product_barcode/{:id}");
const PRODUCT_BARCODE_DELETE: &str = route!("/product_barcode");

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .route(LOGIN, post(get_login))
        .route(PROFILE, get(get_profile))
        // User routes
        .route(USER, put(create_user))
        .route(USER_BY_ID, get(get_user_by_id))
        .route(USER_CHANGE_PASSWORD, put(change_user_password))
        .route(USER_CHANGE_STATUS, put(change_user_status))
        .route(USER_LIST, get(get_all_users))
        .route(USER_DELETE, delete(delete_user))
        .route(USER_UPDATE, patch(update_user))
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
        .with_state(app_ctx.clone())
        // CORS middleware must be the outermost layer so it runs before auth
        //.layer(from_fn(auth_middleware))
        .layer(from_fn(cors_middleware));

    Ok(router)
}
