use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use serde::Serialize;

use crate::app::AppState;
use crate::auth::model::Claims;
use crate::auth::tokens::TokenService;

#[derive(Debug, Serialize)]
pub struct AuthMiddlewareError {
    pub error: String,
    pub code: String,
}

impl AuthMiddlewareError {
    fn missing_token() -> Self {
        Self {
            error: "Missing authorization header".to_string(),
            code: "MISSING_TOKEN".to_string(),
        }
    }

    fn invalid_token() -> Self {
        Self {
            error: "Invalid or expired token".to_string(),
            code: "INVALID_TOKEN".to_string(),
        }
    }

    fn token_revoked() -> Self {
        Self {
            error: "Token has been revoked".to_string(),
            code: "TOKEN_REVOKED".to_string(),
        }
    }

    fn not_admin() -> Self {
        Self {
            error: "Admin access required".to_string(),
            code: "NOT_ADMIN".to_string(),
        }
    }
}

/// Extension to store authenticated user claims in request
#[derive(Clone)]
pub struct AuthenticatedUser {
    pub claims: Claims,
}

/// Middleware that requires a valid JWT token
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthMiddlewareError>)> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::missing_token())))?;

    // Extract bearer token
    let token = TokenService::extract_bearer_token(auth_header)
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::missing_token())))?;

    // Verify token
    let claims = state
        .token_service
        .verify_access_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::invalid_token())))?;

    // Check if token is blacklisted
    if state.validation.is_jti_blacklisted(&claims.jti) {
        return Err((StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::token_revoked())));
    }

    // Store claims in request extensions for handlers to access
    request.extensions_mut().insert(AuthenticatedUser { claims });

    Ok(next.run(request).await)
}

/// Middleware that requires admin access
pub async fn require_admin(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthMiddlewareError>)> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::missing_token())))?;

    // Extract bearer token
    let token = TokenService::extract_bearer_token(auth_header)
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::missing_token())))?;

    // Verify token
    let claims = state
        .token_service
        .verify_access_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::invalid_token())))?;

    // Check if token is blacklisted
    if state.validation.is_jti_blacklisted(&claims.jti) {
        return Err((StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::token_revoked())));
    }

    // Check if user is admin
    if !claims.is_admin {
        return Err((StatusCode::FORBIDDEN, Json(AuthMiddlewareError::not_admin())));
    }

    // Store claims in request extensions
    request.extensions_mut().insert(AuthenticatedUser { claims });

    Ok(next.run(request).await)
}

/// Helper to extract authenticated user from request extensions
pub fn get_authenticated_user(request: &Request) -> Option<&AuthenticatedUser> {
    request.extensions().get::<AuthenticatedUser>()
}

/// Middleware that checks for a specific capability
pub fn require_capability(
    capability: &'static str,
) -> impl Fn(
    State<AppState>,
    Request,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, (StatusCode, Json<AuthMiddlewareError>)>> + Send>>
       + Clone
       + Send {
    move |State(state): State<AppState>, mut request: Request, next: Next| {
        Box::pin(async move {
            // Extract Authorization header
            let auth_header = request
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::missing_token())))?;

            // Extract bearer token
            let token = TokenService::extract_bearer_token(auth_header)
                .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::missing_token())))?;

            // Verify token
            let claims = state
                .token_service
                .verify_access_token(token)
                .map_err(|_| (StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::invalid_token())))?;

            // Check if token is blacklisted
            if state.validation.is_jti_blacklisted(&claims.jti) {
                return Err((StatusCode::UNAUTHORIZED, Json(AuthMiddlewareError::token_revoked())));
            }

            // Check if user has the required capability
            if !claims.capabilities.contains(&capability.to_string()) {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(AuthMiddlewareError {
                        error: format!("Missing required capability: {}", capability),
                        code: "MISSING_CAPABILITY".to_string(),
                    }),
                ));
            }

            // Store claims in request extensions
            request.extensions_mut().insert(AuthenticatedUser { claims });

            Ok(next.run(request).await)
        })
    }
}
