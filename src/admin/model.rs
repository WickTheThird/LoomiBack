use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Admin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: AdminRole,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "admin_role", rename_all = "lowercase")]
pub enum AdminRole {
    SuperAdmin,
    Admin,
    Moderator,
}

#[derive(Debug, Serialize)]
pub struct AdminProfile {
    pub id: Uuid,
    pub user: crate::users::model::UserProfile,
    pub role: AdminRole,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}
