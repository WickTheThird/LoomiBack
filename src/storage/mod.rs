pub mod postgres;
pub mod memory;
pub mod error;

pub use error::DbError;
pub use memory::MemoryStorage;

use async_trait::async_trait;
use uuid::Uuid;

use crate::users::model::{User, CreateUserRequest};
use crate::auth::model::UserAccount;
use crate::admin::model::Admin;
use crate::validation::model::AuthToken;

#[async_trait]
pub trait StorageLayer: Send + Sync {
    async fn health_check(&self) -> bool;

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, DbError>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, DbError>;
    async fn create_user(&self, req: &CreateUserRequest, password_hash: &str) -> Result<User, DbError>;
    async fn update_user_last_login(&self, user_id: Uuid) -> Result<(), DbError>;

    async fn get_account_by_user_id(&self, user_id: Uuid) -> Result<Option<UserAccount>, DbError>;
    async fn create_account(&self, user_id: Uuid) -> Result<UserAccount, DbError>;

    async fn get_admin_by_user_id(&self, user_id: Uuid) -> Result<Option<Admin>, DbError>;
    async fn is_admin(&self, user_id: Uuid) -> Result<bool, DbError>;

    async fn store_token(&self, token: &AuthToken) -> Result<(), DbError>;
    async fn get_token_by_hash(&self, token_hash: &str) -> Result<Option<AuthToken>, DbError>;
    async fn revoke_token(&self, token_hash: &str) -> Result<(), DbError>;
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<(), DbError>;
}
