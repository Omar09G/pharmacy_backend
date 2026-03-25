use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use log::{error, info};

use crate::config::config_jwt::validate_jwt::{validate_token, validate_token_refresh};

/// Middleware para validar el token JWT en el header `Authorization: Bearer <token>`.
///
/// Permite pasar sin token a las rutas de login y creación de usuario.
pub async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();
    let method = req.method();

    // Permitir acceso público a login y creación de usuario
    if path == "/v1/api/login" || (path == "/v1/api/user" && method == Method::PUT) {
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

    // Validar token usando la implementación existente
    match validate_token(&token).await {
        Ok(_) => Ok(next.run(req).await),
        Err(e) => {
            error!(
                "Error validando token de acceso: {}. Intentando validar como refresh token...",
                e
            );

            match validate_token_refresh(&token).await {
                Ok(claims) => {
                    info!("Refresh token validado para usuario: {}", claims.sub);
                    Ok(next.run(req).await)
                }
                Err(_) => Err({
                    error!("Error validando refresh token: {}", e);
                    StatusCode::UNAUTHORIZED
                }),
            }
        }
    }
}
