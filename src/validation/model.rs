use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::RwLock;


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub token_type: TokenType,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub device_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "token_type", rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
    AdminAccess,
    AdminRefresh,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationKey {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub key_type: ValidationType,
    pub key_value: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    EmailVerification,
    PasswordReset,
    TwoFactorAuth,
    AdminInvite,
    AccountActivation,
}


#[derive(Debug)]
pub struct TokenValidation {
    pub user_id: Uuid,
    pub token_type: TokenType,
    pub is_valid: bool,
    pub expires_at: DateTime<Utc>,
}
