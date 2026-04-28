use crate::api_module::login::dto::login_dto::{
    LoginRequest, LoginResponseDTO, LogoutRequest, RefreshRequest,
};
use crate::api_utils::api_const::{JWT_TYPE_ACCESS, JWT_TYPE_REFRESH};
use crate::api_utils::extractors::AuthClaims;
use crate::config::config_jwt::token_revocation::revoke_token;
use crate::config::config_jwt::validate_jwt::{generate_jwt, validate_token_refresh};
use crate::{
    api_utils::api_response::ApiResponse, config::config_database::config_db_context::AppContext,
};

use crate::api_utils::api_error::ApiError;

use crate::config::config_pass::config_password::verify_password;
use axum::body::Bytes;
use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Response};
use axum::{Json, extract::State};
use log::{error, info, warn};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use std::env;
use validator::Validate;

/// Returns `"; Secure"` unless `COOKIE_SECURE=false` is explicitly set.
/// Defaults to secure to prevent accidental insecure deployments.
fn secure_flag() -> &'static str {
    if env::var("COOKIE_SECURE")
        .unwrap_or_else(|_| "true".to_string())
        .to_lowercase()
        == "false"
    {
        ""
    } else {
        "; Secure"
    }
}

/// Build a `Set-Cookie` header value for an HttpOnly cookie.
fn build_cookie(name: &str, value: &str, path: &str, max_age_secs: i64) -> String {
    format!(
        "{}={}; HttpOnly; SameSite=Strict; Path={}{secure}; Max-Age={}",
        name,
        value,
        path,
        max_age_secs,
        secure = secure_flag()
    )
}

/// Build an expired `Set-Cookie` header to clear a cookie.
fn build_expired_cookie(name: &str, path: &str) -> String {
    format!(
        "{}=; HttpOnly; SameSite=Strict; Path={}{secure}; Max-Age=0",
        name,
        path,
        secure = secure_flag()
    )
}

/// Extract a named cookie value from the `Cookie` header.
fn extract_cookie(headers: &axum::http::HeaderMap, name: &str) -> Option<String> {
    headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|c| {
                let c = c.trim();
                if let Some(val) = c.strip_prefix(&format!("{}=", name)) {
                    Some(val.to_string())
                } else {
                    None
                }
            })
        })
}

/// Returns true when the request comes from a Capacitor native client.
fn is_native_client(headers: &axum::http::HeaderMap) -> bool {
    headers
        .get("x-client-platform")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("native"))
        .unwrap_or(false)
}

pub async fn get_login(
    headers: axum::http::HeaderMap,
    State(app_ctx): State<AppContext>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response, ApiError> {
    info!("get_login called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    // Verify user exists, is active, and not soft-deleted
    let user = schemas::users::Entity::find()
        .filter(schemas::users::Column::Username.eq(payload.username.clone()))
        .filter(schemas::users::Column::Status.eq("ACTIVE"))
        .filter(schemas::users::Column::DeletedAt.is_null())
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or(ApiError::Unauthorized)?;

    let pwd = payload.password.clone();
    let user_hash = user.password_hash.clone();
    let is_valid_password = tokio::task::spawn_blocking(move || verify_password(&pwd, &user_hash))
        .await
        .map_err(|e| {
            error!("Join error verifying password: {}", e);
            ApiError::Unauthorized
        })?
        .map_err(|e| {
            error!("Error verifying password: {}", e);
            ApiError::Unauthorized
        })?;

    if !is_valid_password {
        return Err(ApiError::Unauthorized);
    }

    let roles = user
        .find_related(schemas::roles::Entity)
        .all(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let role = roles.into_iter().next().ok_or(ApiError::Unauthorized)?;

    // Fetch permissions for this role — try Redis cache first (cached per role)
    let cache_key = format!("role_permissions:{}", role.name.clone());
    let permissions: Vec<String> =
        match crate::config::config_redis::get_json::<Vec<String>>(&cache_key).await {
            Ok(Some(cached)) => cached,
            _ => {
                let permission_models = role
                    .find_related(schemas::permissions::Entity)
                    .all(&app_ctx.conn)
                    .await
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
                let permissions_vec: Vec<String> =
                    permission_models.into_iter().map(|p| p.name).collect();
                // Best-effort cache for 1 hour
                let _ =
                    crate::config::config_redis::set_json(&cache_key, &permissions_vec, 3600).await;
                permissions_vec
            }
        };

    let full_name = user.full_name.clone().unwrap_or_default();

    let access_token = generate_jwt(
        user.username.clone(),
        role.name.clone(),
        JWT_TYPE_ACCESS.to_string(),
        user.id,
        full_name.clone(),
        permissions.clone(),
    )
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let refresh_token = generate_jwt(
        user.username.clone(),
        role.name.clone(),
        JWT_TYPE_REFRESH.to_string(),
        user.id,
        full_name.clone(),
        permissions,
    )
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let response_body = ApiResponse::success(
        {
            let dto = LoginResponseDTO::new(user.id, full_name, user.username, role.name.clone());
            // For native clients (Capacitor/Android), also include tokens in the body.
            // Web clients receive tokens exclusively via HttpOnly cookies.
            if is_native_client(&headers) {
                dto.with_tokens(access_token.clone(), refresh_token.clone())
            } else {
                dto
            }
        },
        "Login successful".to_string(),
        1,
    );
    let access_cookie = build_cookie("access_token", &access_token, "/v1/api", 86_400); // 1 day
    let refresh_cookie = build_cookie("refresh_token", &refresh_token, "/v1/api/auth", 604_800); // 7 days

    let mut response = Json(response_body).into_response();
    // B-1 fix: use map_err instead of unwrap() to avoid panics on malformed cookie strings
    response.headers_mut().append(
        SET_COOKIE,
        access_cookie.parse().map_err(|_| {
            ApiError::Unexpected(Box::new(std::io::Error::other(
                "invalid access cookie header",
            )))
        })?,
    );
    response.headers_mut().append(
        SET_COOKIE,
        refresh_cookie.parse().map_err(|_| {
            ApiError::Unexpected(Box::new(std::io::Error::other(
                "invalid refresh cookie header",
            )))
        })?,
    );

    Ok(response)
}

/// Endpoint: POST /v1/api/auth/refresh
/// Reads the refresh_token from the HttpOnly cookie (web) or request body (native) and
/// returns a new token pair as cookies plus body tokens for native clients.
pub async fn refresh_token(
    headers: axum::http::HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    info!("refresh_token called");

    // Try body first (native clients send refresh_token in JSON body)
    let refresh_from_body: Option<String> = if !body.is_empty() {
        serde_json::from_slice::<RefreshRequest>(&body)
            .ok()
            .and_then(|r| r.refresh_token)
    } else {
        None
    };

    let refresh_tok = refresh_from_body
        .or_else(|| extract_cookie(&headers, "refresh_token"))
        .ok_or(ApiError::Unauthorized)?;

    let claims = validate_token_refresh(&refresh_tok)
        .await
        .map_err(|_| ApiError::Unauthorized)?;

    let new_access = generate_jwt(
        claims.user_name.clone(),
        claims.role.clone(),
        JWT_TYPE_ACCESS.to_string(),
        claims.id,
        claims.name.clone(),
        claims.permissions.clone(),
    )
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_refresh = generate_jwt(
        claims.user_name.clone(),
        claims.role.clone(),
        JWT_TYPE_REFRESH.to_string(),
        claims.id,
        claims.name.clone(),
        claims.permissions,
    )
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let access_cookie = build_cookie("access_token", &new_access, "/v1/api", 86_400);
    let refresh_cookie = build_cookie("refresh_token", &new_refresh, "/v1/api/auth", 604_800);

    let response_dto = {
        let dto = LoginResponseDTO::new(claims.id, claims.name, claims.user_name, claims.role);
        if is_native_client(&headers) {
            dto.with_tokens(new_access.clone(), new_refresh.clone())
        } else {
            dto
        }
    };

    let body = ApiResponse::success(response_dto, "Token refreshed successfully".to_string(), 1);

    let mut response = Json(body).into_response();
    response.headers_mut().append(
        SET_COOKIE,
        access_cookie.parse().map_err(|_| {
            ApiError::Unexpected(Box::new(std::io::Error::other(
                "invalid access cookie header",
            )))
        })?,
    );
    response.headers_mut().append(
        SET_COOKIE,
        refresh_cookie.parse().map_err(|_| {
            ApiError::Unexpected(Box::new(std::io::Error::other(
                "invalid refresh cookie header",
            )))
        })?,
    );

    Ok(response)
}

/// Endpoint: POST /v1/api/auth/logout
/// Revokes the current refresh token JTI and clears both HttpOnly cookies.
/// Native clients may send `{ "refreshToken": "<token>" }` in the request body.
pub async fn logout(headers: axum::http::HeaderMap, body: Bytes) -> Response {
    info!("logout called");

    // Native clients send the refresh token in the body; web clients use the cookie.
    let refresh_from_body: Option<String> = if !body.is_empty() {
        serde_json::from_slice::<LogoutRequest>(&body)
            .ok()
            .and_then(|r| r.refresh_token)
    } else {
        None
    };

    let refresh_tok_opt = refresh_from_body.or_else(|| extract_cookie(&headers, "refresh_token"));

    // Attempt to revoke the refresh token so it can no longer be used even if stolen
    if let Some(refresh_tok) = refresh_tok_opt {
        match validate_token_refresh(&refresh_tok).await {
            Ok(claims) => {
                if let Some(jti) = claims.jti {
                    revoke_token(&jti);
                    info!("Refresh token revoked: jti={}", jti);
                }
            }
            Err(e) => {
                // Token may be already expired or invalid — still clear cookies
                warn!("Could not validate refresh token on logout: {}", e);
            }
        }
    }

    let access_cookie = build_expired_cookie("access_token", "/v1/api");
    let refresh_cookie = build_expired_cookie("refresh_token", "/v1/api/auth");

    let body = ApiResponse::success((), "Logged out successfully".to_string(), 0);
    let mut response = Json(body).into_response();
    // Safe: expired cookie strings are always valid header values
    if let Ok(val) = access_cookie.parse() {
        response.headers_mut().append(SET_COOKIE, val);
    }
    if let Ok(val) = refresh_cookie.parse() {
        response.headers_mut().append(SET_COOKIE, val);
    }

    response
}

pub async fn get_profile(
    AuthClaims(claims): AuthClaims,
) -> Result<Json<ApiResponse<LoginResponseDTO>>, ApiError> {
    info!("get_profile called");

    let token_validate =
        LoginResponseDTO::new(claims.id, claims.name, claims.user_name, claims.role);

    Ok(Json(ApiResponse::success(
        token_validate,
        "Token Valid OK".to_string(),
        1,
    )))
}
