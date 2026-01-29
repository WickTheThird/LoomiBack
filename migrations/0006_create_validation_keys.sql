-- Validation keys table (for email verification, password reset, etc.)

CREATE TABLE validation_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    key_type validation_type NOT NULL,
    key_value VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used BOOLEAN NOT NULL DEFAULT false,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_validation_keys_user_id ON validation_keys(user_id);
CREATE INDEX idx_validation_keys_value ON validation_keys(key_value);
CREATE INDEX idx_validation_keys_type ON validation_keys(key_type);
CREATE INDEX idx_validation_keys_expires ON validation_keys(expires_at);
