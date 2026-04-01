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

/*Metodos ROLE_PERMISSIONS  */
const ROLE_PERMISSIONS: &str = route!("/role_permissions");
const ROLE_PERMISSIONS_BY_ID: &str = route!("/role_permissions/{:role_id}/{:permission_id}");
const ROLE_PERMISSIONS_LIST: &str = route!("/role_permissions/list");
const ROLE_PERMISSIONS_DELETE: &str = route!("/role_permissions/{:role_id}/{:permission_id}");
const ROLE_PERMISSIONS_UPDATE: &str = route!("/role_permissions/{:role_id}/{:permission_id}");

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
