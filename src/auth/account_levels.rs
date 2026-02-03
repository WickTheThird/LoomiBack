use crate::auth::model::{AccountLevel, UserAccount, capabilities};

impl AccountLevel {
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

    pub fn has_capability(&self, capability: &str) -> bool {
        self.default_capabilities().contains(&capability.to_string())
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AccountLevel::Free => "Free",
            AccountLevel::Premium => "Premium",
            AccountLevel::Enterprise => "Enterprise",
        }
    }

    pub fn max_websites(&self) -> usize {
        match self {
            AccountLevel::Free => 1,
            AccountLevel::Premium => 5,
            AccountLevel::Enterprise => usize::MAX,
        }
    }

    pub fn max_storage_mb(&self) -> usize {
        match self {
            AccountLevel::Free => 100,
            AccountLevel::Premium => 1000,
            AccountLevel::Enterprise => 10000,
        }
    }
}

pub fn check_capability(account: &UserAccount, capability: &str) -> bool {
    if account.capabilities.contains(&capability.to_string()) {
        return true;
    }
    account.account_level.has_capability(capability)
}

pub fn is_account_active(account: &UserAccount) -> bool {
    use crate::auth::model::AccountStatus;
    matches!(account.account_status, AccountStatus::Active)
}

pub fn get_all_capabilities(account: &UserAccount) -> Vec<String> {
    let mut caps = account.account_level.default_capabilities();
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
