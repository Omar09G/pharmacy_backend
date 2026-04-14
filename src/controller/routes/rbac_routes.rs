use axum::routing::{delete, get, patch, post};
use axum::Router;

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
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const PERMISSION: &str = route!("/permission");
const PERMISSION_BY_ID: &str = route!("/permission/{:id}");
const PERMISSION_LIST: &str = route!("/permission");
const PERMISSION_DELETE: &str = route!("/permission/{:id}");
const PERMISSION_UPDATE: &str = route!("/permission/{:id}");
const PERMISSION_BY_NAME: &str = route!("/permission/name");

const ROLE: &str = route!("/role");
const ROLE_BY_ID: &str = route!("/role/{:id}");
const ROLE_LIST: &str = route!("/role");
const ROLE_DELETE: &str = route!("/role/{:id}");
const ROLE_UPDATE: &str = route!("/role/{:id}");
const ROLE_BY_NAME: &str = route!("/role/name");

const ROLE_PERMISSIONS: &str = route!("/role_permissions");
const ROLE_PERMISSIONS_BY_ID: &str = route!("/role_permissions/{:role_id}");
const ROLE_PERMISSIONS_LIST: &str = route!("/role_permissions/list");
const ROLE_PERMISSIONS_DELETE: &str = route!("/role_permissions/{:role_id}/{:permission_id}");
const ROLE_PERMISSIONS_UPDATE: &str = route!("/role_permissions/{:role_id}/{:permission_id}");

pub fn routes() -> Router<AppContext> {
    Router::new()
        // Permission routes
        .route(PERMISSION, post(create_permission))
        .route(PERMISSION_BY_ID, get(get_permission_by_id))
        .route(PERMISSION_LIST, get(get_permissions))
        .route(PERMISSION_DELETE, delete(delete_permission))
        .route(PERMISSION_UPDATE, patch(update_permission))
        .route(PERMISSION_BY_NAME, get(get_permissions_by_name))
        // Role routes
        .route(ROLE, post(create_role))
        .route(ROLE_BY_ID, get(get_role_by_id))
        .route(ROLE_LIST, get(get_roles))
        .route(ROLE_DELETE, delete(delete_role))
        .route(ROLE_UPDATE, patch(update_role))
        .route(ROLE_BY_NAME, get(get_roles_by_name))
        // Role Permissions routes
        .route(ROLE_PERMISSIONS, post(create_role_permissions))
        .route(ROLE_PERMISSIONS_BY_ID, get(get_role_permissions_by_id))
        .route(ROLE_PERMISSIONS_LIST, get(get_role_permissions))
        .route(ROLE_PERMISSIONS_DELETE, delete(delete_role_permissions))
        .route(ROLE_PERMISSIONS_UPDATE, patch(update_role_permissions))
}
