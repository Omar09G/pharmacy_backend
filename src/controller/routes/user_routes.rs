use axum::routing::{delete, get, patch, post, put};
use axum::Router;

use crate::api_module::user::service::user_service::{
    change_user_password, change_user_status, create_user, delete_user, get_all_users,
    get_user_by_id, update_user,
};
use crate::api_module::user_role::user_role_service::user_role_service::{
    create_user_role, delete_user_role, get_user_role_by_user_id, get_user_roles, update_user_role,
};
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const USER: &str = route!("/user");
const USER_BY_ID: &str = route!("/user/{:id}");
const USER_CHANGE_PASSWORD: &str = route!("/user/password");
const USER_CHANGE_STATUS: &str = route!("/user/status");
const USER_LIST: &str = route!("/user");
const USER_DELETE: &str = route!("/user/{:id}");
const USER_UPDATE: &str = route!("/user/{:id}");

const USER_ROLE: &str = route!("/user_role");
const USER_ROLE_BY_ID: &str = route!("/user_role/{:user_id}/{:role_id}");
const USER_ROLE_LIST: &str = route!("/user_role");
const USER_ROLE_DELETE: &str = route!("/user_role/{:user_id}/{:role_id}");
const USER_ROLE_UPDATE: &str = route!("/user_role/{:user_id}/{:role_id}");

pub fn routes() -> Router<AppContext> {
    Router::new()
        // User routes
        .route(USER, post(create_user))
        .route(USER_BY_ID, get(get_user_by_id))
        .route(USER_CHANGE_PASSWORD, put(change_user_password))
        .route(USER_CHANGE_STATUS, put(change_user_status))
        .route(USER_LIST, get(get_all_users))
        .route(USER_DELETE, delete(delete_user))
        .route(USER_UPDATE, patch(update_user))
        // User Role routes
        .route(USER_ROLE, post(create_user_role))
        .route(USER_ROLE_BY_ID, get(get_user_role_by_user_id))
        .route(USER_ROLE_LIST, get(get_user_roles))
        .route(USER_ROLE_DELETE, delete(delete_user_role))
        .route(USER_ROLE_UPDATE, patch(update_user_role))
}
