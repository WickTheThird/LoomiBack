use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;

use super::model::{AuthToken, ValidationKey, TokenValidation, TokenType};

pub struct ValidationStore {
    keys: RwLock<HashMap<Uuid, ValidationKey>>,
    tokens: RwLock<HashMap<String, AuthToken>>,
    blacklisted_jtis: RwLock<HashSet<Uuid>>,
}

impl Default for ValidationStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationStore {
    pub fn new() -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
            tokens: RwLock::new(HashMap::new()),
            blacklisted_jtis: RwLock::new(HashSet::new()),
        }
    }

    pub fn blacklist_jti(&self, jti: Uuid) {
        let mut blacklist = self.blacklisted_jtis.write().unwrap();
        blacklist.insert(jti);
    }

    pub fn is_jti_blacklisted(&self, jti: &Uuid) -> bool {
        let blacklist = self.blacklisted_jtis.read().unwrap();
        blacklist.contains(jti)
    }

    pub fn cleanup_blacklist(&self) {
    }

    pub fn store_key(&self, key: ValidationKey) {
        let mut keys = self.keys.write().unwrap();
        keys.insert(key.id, key);
    }

    pub fn get_key(&self, key_value: &str) -> Option<ValidationKey> {
        let keys = self.keys.read().unwrap();
        keys.values()
            .find(|key| key.key_value == key_value && !key.used && key.expires_at > Utc::now())
            .cloned()
    }

    pub fn use_key(&self, key_value: &str) -> Option<ValidationKey> {
        let mut keys = self.keys.write().unwrap();

        let key_id = keys.iter()
            .find(|(_, key)| key.key_value == key_value && !key.used && key.expires_at > Utc::now())
            .map(|(id, _)| *id)?;

        if let Some(mut key) = keys.get(&key_id).cloned() {
            key.used = true;
            keys.insert(key_id, key.clone());
            Some(key)
        } else {
            None
        }
    }

    pub fn store_token(&self, token: AuthToken) {
        let mut tokens = self.tokens.write().unwrap();
        tokens.insert(token.token_hash.clone(), token);
    }

    pub fn validate_token(&self, token_hash: &str) -> Option<TokenValidation> {
        let tokens = self.tokens.read().unwrap();
        
        if let Some(token) = tokens.get(token_hash) {
            if token.expires_at > Utc::now() && token.revoked_at.is_none() {
                Some(TokenValidation {
                    user_id: token.user_id,
                    token_type: token.token_type.clone(),
                    is_valid: true,
                    expires_at: token.expires_at,
                })
            } else {
                Some(TokenValidation {
                    user_id: token.user_id,
                    token_type: token.token_type.clone(),
                    is_valid: false,
                    expires_at: token.expires_at,
                })
            }
        } else {
            None
        }
    }

        pub fn revoke_token(&self, token_hash: &str) -> bool {
        let mut tokens = self.tokens.write().unwrap();
        if let Some(mut token) = tokens.get(token_hash).cloned() {
            token.revoked_at = Some(Utc::now());
            tokens.insert(token_hash.to_string(), token);
            true
        } else {
            false
        }
    }

    pub fn is_admin_token(&self, token_hash: &str) -> bool {
        let tokens = self.tokens.read().unwrap();
        if let Some(token) = tokens.get(token_hash) {
            matches!(token.token_type, TokenType::AdminAccess | TokenType::AdminRefresh)
        } else {
            false
        }
    }

    pub fn cleanup_expired(&self) {
        let now = Utc::now();

        let mut keys = self.keys.write().unwrap();
        keys.retain(|_, key| key.expires_at > now);

        let mut tokens = self.tokens.write().unwrap();
        tokens.retain(|_, token| token.expires_at > now);
    }

    pub fn revoke_all_user_tokens(&self, user_id: Uuid) {
        let mut tokens = self.tokens.write().unwrap();
        for token in tokens.values_mut() {
            if token.user_id == user_id {
                token.revoked_at = Some(Utc::now());
            }
        }
    }
}

pub struct JwtValidator {
    secret: String,
}

impl JwtValidator {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn validate_jwt(&self, token: &str) -> Result<crate::auth::model::Claims, String> {
        use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
        
        let key = DecodingKey::from_secret(self.secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);
        
        match decode::<crate::auth::model::Claims>(token, &key, &validation) {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => Err(format!("Invalid JWT: {}", e)),
        }
    }

    pub fn extract_token_from_header(auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }
}
