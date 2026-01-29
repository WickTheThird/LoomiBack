use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use uuid::Uuid;

use crate::users::model::User;
use crate::auth::model::{Claims, UserAccount, UserRole};
use crate::admin::model::{Admin, AdminRole};
use crate::validation::model::{AuthToken, TokenType};

#[derive(Debug)]
pub enum TokenError {
    EncodingFailed(String),
    DecodingFailed(String),
    Expired,
    Invalid,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::EncodingFailed(msg) => write!(f, "Token encoding failed: {}", msg),
            TokenError::DecodingFailed(msg) => write!(f, "Token decoding failed: {}", msg),
            TokenError::Expired => write!(f, "Token has expired"),
            TokenError::Invalid => write!(f, "Invalid token"),
        }
    }
}

impl std::error::Error for TokenError {}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_in: i64,
    pub refresh_expires_in: i64,
}

pub struct TokenService {
    jwt_secret: String,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
}

impl TokenService {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret,
            access_token_ttl: Duration::minutes(15),
            refresh_token_ttl: Duration::days(7),
        }
    }

    pub fn with_ttl(jwt_secret: String, access_ttl_minutes: i64, refresh_ttl_days: i64) -> Self {
        Self {
            jwt_secret,
            access_token_ttl: Duration::minutes(access_ttl_minutes),
            refresh_token_ttl: Duration::days(refresh_ttl_days),
        }
    }

    /// Generate tokens for a regular user
    pub fn generate_user_tokens(
        &self,
        user: &User,
        account: &UserAccount,
    ) -> Result<TokenPair, TokenError> {
        self.generate_tokens_internal(user, account, false, None)
    }

    /// Generate tokens for an admin user
    pub fn generate_admin_tokens(
        &self,
        user: &User,
        account: &UserAccount,
        admin: &Admin,
    ) -> Result<TokenPair, TokenError> {
        self.generate_tokens_internal(user, account, true, Some(&admin.role))
    }

    fn generate_tokens_internal(
        &self,
        user: &User,
        account: &UserAccount,
        is_admin: bool,
        admin_role: Option<&AdminRole>,
    ) -> Result<TokenPair, TokenError> {
        let now = Utc::now();
        let access_exp = now + self.access_token_ttl;
        let refresh_exp = now + self.refresh_token_ttl;

        // Generate access token
        let access_jti = Uuid::new_v4();
        let access_claims = Claims {
            sub: user.id,
            jti: access_jti,
            email: user.email.clone(),
            account_level: account.account_level.clone(),
            account_status: account.account_status.clone(),
            capabilities: account.capabilities.clone(),
            role: if is_admin { UserRole::Admin } else { UserRole::User },
            is_admin,
            admin_role: admin_role.cloned(),
            iat: now.timestamp() as usize,
            exp: access_exp.timestamp() as usize,
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| TokenError::EncodingFailed(e.to_string()))?;

        // Generate refresh token (simpler claims, just user id and jti)
        let refresh_jti = Uuid::new_v4();
        let refresh_claims = RefreshClaims {
            sub: user.id,
            jti: refresh_jti,
            is_admin,
            iat: now.timestamp() as usize,
            exp: refresh_exp.timestamp() as usize,
        };

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| TokenError::EncodingFailed(e.to_string()))?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            access_expires_in: self.access_token_ttl.num_seconds(),
            refresh_expires_in: self.refresh_token_ttl.num_seconds(),
        })
    }

    /// Verify and decode an access token
    pub fn verify_access_token(&self, token: &str) -> Result<Claims, TokenError> {
        let key = DecodingKey::from_secret(self.jwt_secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(token, &key, &validation)
            .map(|data| data.claims)
            .map_err(|e| {
                if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                    TokenError::Expired
                } else {
                    TokenError::DecodingFailed(e.to_string())
                }
            })
    }

    /// Verify and decode a refresh token
    pub fn verify_refresh_token(&self, token: &str) -> Result<RefreshClaims, TokenError> {
        let key = DecodingKey::from_secret(self.jwt_secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        decode::<RefreshClaims>(token, &key, &validation)
            .map(|data| data.claims)
            .map_err(|e| {
                if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                    TokenError::Expired
                } else {
                    TokenError::DecodingFailed(e.to_string())
                }
            })
    }

    /// Create an AuthToken record for storing in the database
    pub fn create_token_record(
        &self,
        user_id: Uuid,
        token: &str,
        is_admin: bool,
        is_refresh: bool,
        device_info: Option<String>,
    ) -> AuthToken {
        let now = Utc::now();
        let ttl = if is_refresh {
            self.refresh_token_ttl
        } else {
            self.access_token_ttl
        };

        let token_type = match (is_admin, is_refresh) {
            (true, true) => TokenType::AdminRefresh,
            (true, false) => TokenType::AdminAccess,
            (false, true) => TokenType::Refresh,
            (false, false) => TokenType::Access,
        };

        AuthToken {
            id: Uuid::new_v4(),
            user_id,
            token_hash: hash_token(token),
            token_type,
            expires_at: now + ttl,
            created_at: now,
            revoked_at: None,
            device_info,
        }
    }

    /// Extract bearer token from Authorization header
    pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }
}

/// Simplified claims for refresh tokens
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefreshClaims {
    pub sub: Uuid,
    pub jti: Uuid,
    pub is_admin: bool,
    pub iat: usize,
    pub exp: usize,
}

/// Hash a token for storage (we don't store raw tokens)
fn hash_token(token: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
