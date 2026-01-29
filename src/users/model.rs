use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    pub users: Vec<UserProfile>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

// Conversions
impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        UserProfile {
            id: user.id,
            email: user.email,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

impl From<&User> for UserProfile {
    fn from(user: &User) -> Self {
        UserProfile {
            id: user.id,
            email: user.email.clone(),
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}
