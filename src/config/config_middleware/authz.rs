use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use log::warn;

use crate::api_utils::api_response::{ApiResponse, ErrorType};
use crate::config::config_jwt::dto_jwt::Claims;

/// Central RBAC enforcement.
///
/// Runs after `auth_middleware` (claims are in request extensions) and maps
/// `(method, path-prefix)` to the permission or role required for write
/// operations. Reads stay open to any authenticated user; management of
/// users/roles requires the ADMIN role. Permissions come from
/// `role_permissions` in the database and travel inside the JWT.
enum Access {
    /// Any authenticated user.
    Any,
    /// One of these permissions must be present in the JWT claims.
    Permission(&'static [&'static str]),
    /// Only the ADMIN role.
    Admin,
}

fn is_write(method: &Method) -> bool {
    method == Method::POST || method == Method::PUT || method == Method::PATCH
        || method == Method::DELETE
}

fn strip_api_prefix(path: &str) -> &str {
    path.strip_prefix("/v1/api").unwrap_or(path)
}

/// Decide what a request needs based on its method and path prefix.
fn required_access(method: &Method, path: &str) -> Access {
    let p = strip_api_prefix(path);

    // Administration surfaces: manage or even list users/roles/permissions.
    if p.starts_with("/user")
        || p.starts_with("/role")
        || p.starts_with("/permission")
    {
        return Access::Admin;
    }

    if !is_write(method) {
        return Access::Any;
    }

    // Catalog / inventory / purchasing writes.
    if p.starts_with("/product")
        || p.starts_with("/category")
        || p.starts_with("/unit")
        || p.starts_with("/tax_profile")
        || p.starts_with("/supplier")
        || p.starts_with("/purchase")
        || p.starts_with("/discount")
        || p.starts_with("/inventory_movement")
    {
        return Access::Permission(&["PRODUCT_MANAGEMENT"]);
    }

    // Sales / cash / customer credit writes.
    if p.starts_with("/sale")
        || p.starts_with("/add_sale")
        || p.starts_with("/cash_")
        || p.starts_with("/customer")
    {
        return Access::Permission(&["SALES_MANAGER"]);
    }

    Access::Any
}

pub async fn authz_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Public paths already passed through auth_middleware without claims.
    let Some(claims) = req.extensions().get::<Claims>().cloned() else {
        return Ok(next.run(req).await);
    };

    let access = required_access(req.method(), req.uri().path());

    let allowed = match access {
        Access::Any => true,
        Access::Admin => claims.role.eq_ignore_ascii_case("ADMIN"),
        Access::Permission(perms) => {
            perms.iter().any(|p| claims.permissions.iter().any(|c| c == p))
        }
    };

    if !allowed {
        warn!(
            "Forbidden: user={} role={} {} {}",
            claims.user_name, claims.role, req.method(), req.uri().path()
        );
        return Ok((
            StatusCode::FORBIDDEN,
            axum::Json(ApiResponse::with_error_type(
                (),
                "No tienes permisos para realizar esta acción.".to_string(),
                403,
                ErrorType::Auth,
            )),
        )
            .into_response());
    }

    Ok(next.run(req).await)
}
