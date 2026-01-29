use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use super::{DbError, StorageLayer};
use crate::users::model::{User, CreateUserRequest};
use crate::auth::model::{UserAccount, AccountLevel, AccountStatus};
use crate::admin::model::{Admin, AdminRole};
use crate::validation::model::AuthToken;

/// In-memory storage for development and testing
pub struct MemoryStorage {
    users: RwLock<HashMap<Uuid, User>>,
    accounts: RwLock<HashMap<Uuid, UserAccount>>,
    admins: RwLock<HashMap<Uuid, Admin>>,
    tokens: RwLock<HashMap<String, AuthToken>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            accounts: RwLock::new(HashMap::new()),
            admins: RwLock::new(HashMap::new()),
            tokens: RwLock::new(HashMap::new()),
        }
    }

    /// Create a memory storage with a default admin user for testing
    pub fn with_default_admin(email: &str, password_hash: &str) -> Self {
        let storage = Self::new();

        let user_id = Uuid::new_v4();
        let now = Utc::now();

        // Create user
        let user = User {
            id: user_id,
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            username: "admin".to_string(),
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
            last_login: None,
        };
        storage.users.write().unwrap().insert(user_id, user);

        // Create account
        let account = UserAccount {
            id: Uuid::new_v4(),
            user_id,
            account_level: AccountLevel::Enterprise,
            account_status: AccountStatus::Active,
            capabilities: vec![
                "create_website".to_string(),
                "manage_components".to_string(),
                "send_emails".to_string(),
                "access_analytics".to_string(),
                "api_access".to_string(),
                "priority_support".to_string(),
            ],
            status_reason: None,
            status_changed_at: None,
            status_changed_by: None,
            created_at: now,
            updated_at: now,
        };
        storage.accounts.write().unwrap().insert(user_id, account);

        // Create admin
        let admin = Admin {
            id: Uuid::new_v4(),
            user_id,
            role: AdminRole::SuperAdmin,
            permissions: vec!["*".to_string()],
            created_at: now,
            updated_at: now,
            created_by: None,
        };
        storage.admins.write().unwrap().insert(user_id, admin);

        storage
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageLayer for MemoryStorage {
    async fn health_check(&self) -> bool {
        true
    }

    // Users
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DbError> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|u| u.email == email).cloned())
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, DbError> {
        let users = self.users.read().unwrap();
        Ok(users.get(&id).cloned())
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, DbError> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|u| u.username == username).cloned())
    }

    async fn create_user(&self, req: &CreateUserRequest, password_hash: &str) -> Result<User, DbError> {
        let mut users = self.users.write().unwrap();

        // Check for duplicate email
        if users.values().any(|u| u.email == req.email) {
            return Err(DbError::Duplicate("email".to_string()));
        }

        // Check for duplicate username
        if users.values().any(|u| u.username == req.username) {
            return Err(DbError::Duplicate("username".to_string()));
        }

        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            email: req.email.clone(),
            password_hash: password_hash.to_string(),
            username: req.username.clone(),
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
            is_active: true,
            created_at: now,
            updated_at: now,
            last_login: None,
        };

        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn update_user_last_login(&self, user_id: Uuid) -> Result<(), DbError> {
        let mut users = self.users.write().unwrap();
        if let Some(user) = users.get_mut(&user_id) {
            user.last_login = Some(Utc::now());
            user.updated_at = Utc::now();
        }
        Ok(())
    }

    // User Accounts
    async fn get_account_by_user_id(&self, user_id: Uuid) -> Result<Option<UserAccount>, DbError> {
        let accounts = self.accounts.read().unwrap();
        Ok(accounts.get(&user_id).cloned())
    }

    async fn create_account(&self, user_id: Uuid) -> Result<UserAccount, DbError> {
        let mut accounts = self.accounts.write().unwrap();

        let now = Utc::now();
        let account = UserAccount {
            id: Uuid::new_v4(),
            user_id,
            account_level: AccountLevel::Free,
            account_status: AccountStatus::Pending,
            capabilities: vec![],
            status_reason: None,
            status_changed_at: None,
            status_changed_by: None,
            created_at: now,
            updated_at: now,
        };

        accounts.insert(user_id, account.clone());
        Ok(account)
    }

    // Admins
    async fn get_admin_by_user_id(&self, user_id: Uuid) -> Result<Option<Admin>, DbError> {
        let admins = self.admins.read().unwrap();
        Ok(admins.get(&user_id).cloned())
    }

    async fn is_admin(&self, user_id: Uuid) -> Result<bool, DbError> {
        let admins = self.admins.read().unwrap();
        Ok(admins.contains_key(&user_id))
    }

    // Auth Tokens
    async fn store_token(&self, token: &AuthToken) -> Result<(), DbError> {
        let mut tokens = self.tokens.write().unwrap();
        tokens.insert(token.token_hash.clone(), token.clone());
        Ok(())
    }

    async fn get_token_by_hash(&self, token_hash: &str) -> Result<Option<AuthToken>, DbError> {
        let tokens = self.tokens.read().unwrap();
        Ok(tokens.get(token_hash).cloned())
    }

    async fn revoke_token(&self, token_hash: &str) -> Result<(), DbError> {
        let mut tokens = self.tokens.write().unwrap();
        if let Some(token) = tokens.get_mut(token_hash) {
            token.revoked_at = Some(Utc::now());
        }
        Ok(())
    }

    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<(), DbError> {
        let mut tokens = self.tokens.write().unwrap();
        let now = Utc::now();
        for token in tokens.values_mut() {
            if token.user_id == user_id && token.revoked_at.is_none() {
                token.revoked_at = Some(now);
            }
        }
        Ok(())
    }
}
