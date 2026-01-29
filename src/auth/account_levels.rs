use crate::auth::model::{AccountLevel, UserAccount, capabilities};

impl AccountLevel {
    /// Get the default capabilities for this account level
    pub fn default_capabilities(&self) -> Vec<String> {
        match self {
            AccountLevel::Free => vec![
                capabilities::CREATE_WEBSITE.to_string(),
            ],
            AccountLevel::Premium => vec![
                capabilities::CREATE_WEBSITE.to_string(),
                capabilities::MANAGE_COMPONENTS.to_string(),
                capabilities::SEND_EMAILS.to_string(),
                capabilities::ACCESS_ANALYTICS.to_string(),
            ],
            AccountLevel::Enterprise => vec![
                capabilities::CREATE_WEBSITE.to_string(),
                capabilities::MANAGE_COMPONENTS.to_string(),
                capabilities::SEND_EMAILS.to_string(),
                capabilities::ACCESS_ANALYTICS.to_string(),
                capabilities::API_ACCESS.to_string(),
                capabilities::PRIORITY_SUPPORT.to_string(),
            ],
        }
    }

    /// Check if this account level has access to a specific capability
    pub fn has_capability(&self, capability: &str) -> bool {
        self.default_capabilities().contains(&capability.to_string())
    }

    /// Get the display name for this account level
    pub fn display_name(&self) -> &'static str {
        match self {
            AccountLevel::Free => "Free",
            AccountLevel::Premium => "Premium",
            AccountLevel::Enterprise => "Enterprise",
        }
    }

    /// Get the maximum number of websites allowed for this level
    pub fn max_websites(&self) -> usize {
        match self {
            AccountLevel::Free => 1,
            AccountLevel::Premium => 5,
            AccountLevel::Enterprise => usize::MAX,
        }
    }

    /// Get the maximum storage in MB for this level
    pub fn max_storage_mb(&self) -> usize {
        match self {
            AccountLevel::Free => 100,
            AccountLevel::Premium => 1000,
            AccountLevel::Enterprise => 10000,
        }
    }
}

/// Check if a user account has a specific capability
/// This checks both the account level defaults AND any custom capabilities
pub fn check_capability(account: &UserAccount, capability: &str) -> bool {
    // First check if the capability is in the account's custom capabilities
    if account.capabilities.contains(&capability.to_string()) {
        return true;
    }

    // Fall back to the account level's default capabilities
    account.account_level.has_capability(capability)
}

/// Check if an account can perform an action based on its status
pub fn is_account_active(account: &UserAccount) -> bool {
    use crate::auth::model::AccountStatus;
    matches!(account.account_status, AccountStatus::Active)
}

/// Get all capabilities for an account (combining level defaults and custom)
pub fn get_all_capabilities(account: &UserAccount) -> Vec<String> {
    let mut caps = account.account_level.default_capabilities();

    // Add any custom capabilities that aren't already in the list
    for cap in &account.capabilities {
        if !caps.contains(cap) {
            caps.push(cap.clone());
        }
    }

    caps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_tier_capabilities() {
        let caps = AccountLevel::Free.default_capabilities();
        assert!(caps.contains(&capabilities::CREATE_WEBSITE.to_string()));
        assert!(!caps.contains(&capabilities::API_ACCESS.to_string()));
    }

    #[test]
    fn test_enterprise_has_all_capabilities() {
        let caps = AccountLevel::Enterprise.default_capabilities();
        assert!(caps.contains(&capabilities::CREATE_WEBSITE.to_string()));
        assert!(caps.contains(&capabilities::API_ACCESS.to_string()));
        assert!(caps.contains(&capabilities::PRIORITY_SUPPORT.to_string()));
    }
}
