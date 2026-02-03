use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::app::AppState;
use crate::users::model::CreateUserRequest;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<uuid::Uuid>,
}

#[derive(Debug, Serialize)]
pub struct RegisterError {
    pub error: String,
    pub code: String,
}

impl RegisterError {
    fn email_exists() -> Self {
        Self {
            error: "Email already registered".to_string(),
            code: "EMAIL_EXISTS".to_string(),
        }
    }

    fn username_exists() -> Self {
        Self {
            error: "Username already taken".to_string(),
            code: "USERNAME_EXISTS".to_string(),
        }
    }

    fn weak_password() -> Self {
        Self {
            error: "Password must be at least 8 characters".to_string(),
            code: "WEAK_PASSWORD".to_string(),
        }
    }

    fn invalid_email() -> Self {
        Self {
            error: "Invalid email format".to_string(),
            code: "INVALID_EMAIL".to_string(),
        }
    }

    fn internal_error() -> Self {
        Self {
            error: "Internal server error".to_string(),
            code: "INTERNAL_ERROR".to_string(),
        }
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<RegisterError>)> {
    if !req.email.contains('@') || !req.email.contains('.') {
        return Err((StatusCode::BAD_REQUEST, Json(RegisterError::invalid_email())));
    }

    if req.password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, Json(RegisterError::weak_password())));
    }

    if let Ok(Some(_)) = state.storage.get_user_by_email(&req.email).await {
        return Err((StatusCode::CONFLICT, Json(RegisterError::email_exists())));
    }

    if let Ok(Some(_)) = state.storage.get_user_by_username(&req.username).await {
        return Err((StatusCode::CONFLICT, Json(RegisterError::username_exists())));
    }

    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterError::internal_error())))?;

    let create_req = CreateUserRequest {
        email: req.email,
        password: req.password.clone(),
        username: req.username,
        first_name: req.first_name,
        last_name: req.last_name,
    };

    let user = state
        .storage
        .create_user(&create_req, &password_hash)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(RegisterError::internal_error())))?;

    let _ = state.storage.create_account(user.id).await;

    Ok(Json(RegisterResponse {
        success: true,
        message: "Registration successful. Please check your email to verify your account.".to_string(),
        user_id: Some(user.id),
    }))
}
