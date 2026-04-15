use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use log::error;

use crate::config::config_jwt::validate_jwt::validate_token;

/// Middleware para validar el token JWT.
///
/// Checks (in order):
/// 1. `Authorization: Bearer <token>` header
/// 2. `access_token` HttpOnly cookie
///
/// Solo permite acceso público a login, refresh y logout.
pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();

    // Public endpoints: login, refresh, and logout
    if path == "/v1/api/auth/login"
        || path == "/v1/api/auth/refresh"
        || path == "/v1/api/auth/logout"
    {
        return Ok(next.run(req).await);
    }

    // 1. Try Authorization header
    let token_from_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|h| {
            if h.to_lowercase().starts_with("bearer ") {
                Some(h[7..].trim().to_string())
            } else {
                None
            }
        });

    // 2. Fallback to access_token cookie
    let token_from_cookie = req
        .headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|c| {
                let c = c.trim();
                c.strip_prefix("access_token=").map(|v| v.to_string())
            })
        });

    let token = token_from_header
        .or(token_from_cookie)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validar token: solo access tokens son válidos aquí.
    // Refresh tokens deben usarse exclusivamente en /auth/refresh.
    match validate_token(&token).await {
        Ok(claims) => {
            // Inject claims into request extensions so handlers can access them
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(e) => {
            error!("Invalid access token: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
