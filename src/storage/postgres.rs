use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use super::{DbError, StorageLayer};
use crate::users::model::{User, CreateUserRequest};
use crate::auth::model::{UserAccount, AccountLevel, AccountStatus};
use crate::admin::model::Admin;
use crate::validation::model::AuthToken;

pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn from_url(database_url: &str) -> Result<Self, DbError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| DbError::Connection(e.to_string()))?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl StorageLayer for PostgresStorage {
    async fn health_check(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }

    // Users
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DbError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, username, first_name, last_name,
                    is_active, created_at, updated_at, last_login
             FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, DbError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, username, first_name, last_name,
                    is_active, created_at, updated_at, last_login
             FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, DbError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, username, first_name, last_name,
                    is_active, created_at, updated_at, last_login
             FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create_user(&self, req: &CreateUserRequest, password_hash: &str) -> Result<User, DbError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash, username, first_name, last_name)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, email, password_hash, username, first_name, last_name,
                       is_active, created_at, updated_at, last_login"
        )
        .bind(&req.email)
        .bind(password_hash)
        .bind(&req.username)
        .bind(&req.first_name)
        .bind(&req.last_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_user_last_login(&self, user_id: Uuid) -> Result<(), DbError> {
        sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // User Accounts
    async fn get_account_by_user_id(&self, user_id: Uuid) -> Result<Option<UserAccount>, DbError> {
        let account = sqlx::query_as::<_, UserAccount>(
            "SELECT id, user_id, account_level, account_status, capabilities,
                    status_reason, status_changed_at, status_changed_by, created_at, updated_at
             FROM user_accounts WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }

    async fn create_account(&self, user_id: Uuid) -> Result<UserAccount, DbError> {
        let account = sqlx::query_as::<_, UserAccount>(
            "INSERT INTO user_accounts (user_id, account_level, account_status, capabilities)
             VALUES ($1, $2, $3, $4)
             RETURNING id, user_id, account_level, account_status, capabilities,
                       status_reason, status_changed_at, status_changed_by, created_at, updated_at"
        )
        .bind(user_id)
        .bind(AccountLevel::Free)
        .bind(AccountStatus::Pending)
        .bind(Vec::<String>::new())
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }

    // Admins
    async fn get_admin_by_user_id(&self, user_id: Uuid) -> Result<Option<Admin>, DbError> {
        let admin = sqlx::query_as::<_, Admin>(
            "SELECT id, user_id, role, permissions, created_at, updated_at, created_by
             FROM admins WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(admin)
    }

    async fn is_admin(&self, user_id: Uuid) -> Result<bool, DbError> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM admins WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    // Auth Tokens
    async fn store_token(&self, token: &AuthToken) -> Result<(), DbError> {
        sqlx::query(
            "INSERT INTO auth_tokens (id, user_id, token_hash, token_type, expires_at, device_info)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(token.id)
        .bind(token.user_id)
        .bind(&token.token_hash)
        .bind(&token.token_type)
        .bind(token.expires_at)
        .bind(&token.device_info)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_token_by_hash(&self, token_hash: &str) -> Result<Option<AuthToken>, DbError> {
        let token = sqlx::query_as::<_, AuthToken>(
            "SELECT id, user_id, token_hash, token_type, expires_at, created_at, revoked_at, device_info
             FROM auth_tokens WHERE token_hash = $1"
        )
        .bind(token_hash)
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    async fn revoke_token(&self, token_hash: &str) -> Result<(), DbError> {
        sqlx::query("UPDATE auth_tokens SET revoked_at = NOW() WHERE token_hash = $1")
            .bind(token_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<(), DbError> {
        sqlx::query("UPDATE auth_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
