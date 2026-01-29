-- User accounts table (stores account level, status, and capabilities)

CREATE TABLE user_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID UNIQUE NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_level account_level NOT NULL DEFAULT 'free',
    account_status account_status NOT NULL DEFAULT 'pending',
    capabilities TEXT[] NOT NULL DEFAULT '{}',
    status_reason TEXT,
    status_changed_at TIMESTAMPTZ,
    status_changed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for user lookup
CREATE INDEX idx_user_accounts_user_id ON user_accounts(user_id);
CREATE INDEX idx_user_accounts_status ON user_accounts(account_status);

-- Trigger for updated_at
CREATE TRIGGER update_user_accounts_updated_at
    BEFORE UPDATE ON user_accounts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
