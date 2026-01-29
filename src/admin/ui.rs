use askama::Template;
use serde::Deserialize;

pub const AUTH_COOKIE_NAME: &str = "admin_token";

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "admin/dashboard.html")]
pub struct DashboardTemplate {
    pub user_email: String,
    pub admin_role: String,
    pub account_level: String,
    pub total_users: i64,
    pub active_users: i64,
    pub total_admins: i64,
    pub system_healthy: bool,
}

#[derive(Template)]
#[template(path = "admin/users.html")]
pub struct UsersTemplate {
    pub user_email: String,
    pub users: Vec<UserRow>,
    pub message: Option<String>,
    pub current_page: i32,
    pub total_pages: i32,
}

pub struct UserRow {
    pub id: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub account_level: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
}
