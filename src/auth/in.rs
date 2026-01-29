use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::app::AppState;
use crate::auth::model::{LoginRequest, LoginResponse, AccountInfo, AccountStatus};
use crate::auth::account_levels::get_all_capabilities;
use crate::users::model::UserProfile;

#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    pub email: String,
    pub password: String,
    pub device_info: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct AuthError {
    pub error: String,
    pub code: String,
}

impl AuthError {
    fn invalid_credentials() -> Self {
        Self {
            error: "Invalid email or password".to_string(),
            code: "INVALID_CREDENTIALS".to_string(),
        }
    }

    fn account_inactive() -> Self {
        Self {
            error: "Account is not active".to_string(),
            code: "ACCOUNT_INACTIVE".to_string(),
        }
    }

    fn not_admin() -> Self {
        Self {
            error: "User is not an admin".to_string(),
            code: "NOT_ADMIN".to_string(),
        }
    }

    fn internal_error() -> Self {
        Self {
            error: "Internal server error".to_string(),
            code: "INTERNAL_ERROR".to_string(),
        }
    }
}

/// Admin login endpoint
/// POST /auth/admin/login
pub async fn admin_login(
    State(state): State<AppState>,
    Json(req): Json<AdminLoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<AuthError>)> {
    // 1. Find user by email
    let user = state
        .storage
        .get_user_by_email(&req.email)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(AuthError::invalid_credentials())))?;

    // 2. Verify password
    let password_valid = bcrypt::verify(&req.password, &user.password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?;

    if !password_valid {
        return Err((StatusCode::UNAUTHORIZED, Json(AuthError::invalid_credentials())));
    }

    // 3. Check if user is active
    if !user.is_active {
        return Err((StatusCode::FORBIDDEN, Json(AuthError::account_inactive())));
    }

    // 4. Check if user is admin
    let admin = state
        .storage
        .get_admin_by_user_id(user.id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?
        .ok_or_else(|| (StatusCode::FORBIDDEN, Json(AuthError::not_admin())))?;

    // 5. Get user account
    let account = state
        .storage
        .get_account_by_user_id(user.id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?;

    // 6. Check account status
    if account.account_status != AccountStatus::Active {
        return Err((StatusCode::FORBIDDEN, Json(AuthError::account_inactive())));
    }

    // 7. Generate tokens
    let token_pair = state
        .token_service
        .generate_admin_tokens(&user, &account, &admin)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?;

    // 8. Store refresh token in database
    let refresh_record = state.token_service.create_token_record(
        user.id,
        &token_pair.refresh_token,
        true, // is_admin
        true, // is_refresh
        req.device_info,
    );

    let _ = state.storage.store_token(&refresh_record).await;

    // 9. Update last login
    let _ = state.storage.update_user_last_login(user.id).await;

    // 10. Build response
    let user_profile = UserProfile::from(&user);
    let all_capabilities = get_all_capabilities(&account);

    Ok(Json(LoginResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
        expires_in: token_pair.access_expires_in,
        user_profile,
        account_info: AccountInfo {
            level: account.account_level,
            status: account.account_status,
            capabilities: all_capabilities,
        },
    }))
}

/// Regular user login endpoint
/// POST /auth/login
pub async fn user_login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<AuthError>)> {
    // 1. Find user by email
    let user = state
        .storage
        .get_user_by_email(&req.email)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(AuthError::invalid_credentials())))?;

    // 2. Verify password
    let password_valid = bcrypt::verify(&req.password, &user.password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?;

    if !password_valid {
        return Err((StatusCode::UNAUTHORIZED, Json(AuthError::invalid_credentials())));
    }

    // 3. Check if user is active
    if !user.is_active {
        return Err((StatusCode::FORBIDDEN, Json(AuthError::account_inactive())));
    }

    // 4. Get user account
    let account = state
        .storage
        .get_account_by_user_id(user.id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?;

    // 5. Check account status
    if account.account_status != AccountStatus::Active {
        return Err((StatusCode::FORBIDDEN, Json(AuthError::account_inactive())));
    }

    // 6. Generate tokens
    let token_pair = state
        .token_service
        .generate_user_tokens(&user, &account)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(AuthError::internal_error())))?;

    // 7. Store refresh token
    let refresh_record = state.token_service.create_token_record(
        user.id,
        &token_pair.refresh_token,
        false, // is_admin
        true,  // is_refresh
        None,
    );

    let _ = state.storage.store_token(&refresh_record).await;

    // 8. Update last login
    let _ = state.storage.update_user_last_login(user.id).await;

    // 9. Build response
    let user_profile = UserProfile::from(&user);
    let all_capabilities = get_all_capabilities(&account);

    Ok(Json(LoginResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
        expires_in: token_pair.access_expires_in,
        user_profile,
        account_info: AccountInfo {
            level: account.account_level,
            status: account.account_status,
            capabilities: all_capabilities,
        },
    }))
}
