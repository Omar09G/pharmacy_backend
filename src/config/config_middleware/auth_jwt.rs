use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use log::error;

use crate::config::config_jwt::validate_jwt::validate_token;

/// Middleware para validar el token JWT en el header `Authorization: Bearer <token>`.
///
/// Solo permite acceso público a la ruta de login.
pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();

    // Only login and token refresh are public
    if path == "/v1/api/auth/login" || path == "/v1/api/auth/refresh" {
        return Ok(next.run(req).await);
    }

    // Obtener header Authorization
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let token = match auth_header {
        Some(h) => {
            let h_lower = h.to_lowercase();
            if h_lower.starts_with("bearer ") {
                Some(h[7..].trim().to_string())
            } else {
                None
            }
        }
        None => None,
    };

    let token = match token {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

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
