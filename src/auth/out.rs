use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    Json,
};
use serde::Serialize;

use crate::app::AppState;
use crate::auth::tokens::TokenService;

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LogoutError {
    pub error: String,
    pub code: String,
}

impl LogoutError {
    fn missing_token() -> Self {
        Self {
            error: "Missing or invalid authorization header".to_string(),
            code: "MISSING_TOKEN".to_string(),
        }
    }

    fn invalid_token() -> Self {
        Self {
            error: "Invalid token".to_string(),
            code: "INVALID_TOKEN".to_string(),
        }
    }
}

/// Logout endpoint - revokes the current token
/// POST /auth/logout
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<LogoutResponse>, (StatusCode, Json<LogoutError>)> {
    // 1. Extract token from Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(LogoutError::missing_token())))?;

    let token = TokenService::extract_bearer_token(auth_header)
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(LogoutError::missing_token())))?;

    // 2. Verify the token is valid (to get user_id)
    let claims = state
        .token_service
        .verify_access_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(LogoutError::invalid_token())))?;

    // 3. Add token to blacklist in ValidationStore
    state.validation.blacklist_jti(claims.jti);

    // 4. Revoke all tokens for this user in the database (optional - more aggressive)
    // For now, just revoke the refresh tokens
    let _ = state.storage.revoke_all_user_tokens(claims.sub).await;

    Ok(Json(LogoutResponse {
        success: true,
        message: "Successfully logged out".to_string(),
    }))
}

/// Logout from all devices - revokes all tokens for the user
/// POST /auth/logout-all
pub async fn logout_all(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<LogoutResponse>, (StatusCode, Json<LogoutError>)> {
    // 1. Extract token from Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(LogoutError::missing_token())))?;

    let token = TokenService::extract_bearer_token(auth_header)
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(LogoutError::missing_token())))?;

    // 2. Verify the token is valid
    let claims = state
        .token_service
        .verify_access_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(LogoutError::invalid_token())))?;

    // 3. Revoke all tokens for this user
    let _ = state.storage.revoke_all_user_tokens(claims.sub).await;

    // 4. Blacklist current token
    state.validation.blacklist_jti(claims.jti);

    Ok(Json(LogoutResponse {
        success: true,
        message: "Successfully logged out from all devices".to_string(),
    }))
}
