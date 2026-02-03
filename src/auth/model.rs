use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_level: AccountLevel,
    pub account_status: AccountStatus,
    pub capabilities: Vec<String>,
    pub status_reason: Option<String>,
    pub status_changed_at: Option<DateTime<Utc>>,
    pub status_changed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_level", rename_all = "lowercase")]
pub enum AccountLevel {
    Free,
    Premium,
    Enterprise,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Pending,
    Suspended,
    Banned,
    Deactivated,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub required_level: AccountLevel,
}

pub mod capabilities {
    pub const CREATE_WEBSITE: &str = "create_website";
    pub const MANAGE_COMPONENTS: &str = "manage_components";
    pub const SEND_EMAILS: &str = "send_emails";
    pub const ACCESS_ANALYTICS: &str = "access_analytics";
    pub const API_ACCESS: &str = "api_access";
    pub const PRIORITY_SUPPORT: &str = "priority_support";
}


#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user_profile: crate::users::model::UserProfile,
    pub account_info: AccountInfo,
}

#[derive(Debug, Serialize)]
pub struct AccountInfo {
    pub level: AccountLevel,
    pub status: AccountStatus,
    pub capabilities: Vec<String>,
}


use crate::admin::model::AdminRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub jti: Uuid,
    pub email: String,
    pub account_level: AccountLevel,
    pub account_status: AccountStatus,
    pub capabilities: Vec<String>,
    pub role: UserRole,
    pub is_admin: bool,
    pub admin_role: Option<AdminRole>,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Admin,
}
