-- 1. Cleanup
DROP TABLE IF EXISTS knowledge_base; -- 🔥 Added cleanup
DROP TABLE IF EXISTS usage_logs;
DROP TABLE IF EXISTS api_keys;
DROP TABLE IF EXISTS device_flows;
DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS device_code_status;
DROP TYPE IF EXISTS user_role;

-- 2. Create Enums
CREATE TYPE user_role AS ENUM ('super_admin', 'admin', 'team', 'pro', 'free');
CREATE TYPE device_code_status AS ENUM ('pending', 'verified', 'expired');

-- 3. Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_address TEXT NOT NULL UNIQUE,
    role user_role NOT NULL DEFAULT 'free',
    credits INTEGER NOT NULL DEFAULT 50,
    team_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Device Flows
CREATE TABLE device_flows (
    device_code TEXT PRIMARY KEY,
    user_code TEXT NOT NULL UNIQUE,
    status device_code_status NOT NULL DEFAULT 'pending',
    wallet_address TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 5. API Keys
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 6. Usage Logs
CREATE TABLE usage_logs (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action TEXT NOT NULL,
    model_used TEXT NOT NULL,
    input_tokens INTEGER NOT NULL DEFAULT 0,
    output_tokens INTEGER NOT NULL DEFAULT 0,
    cost_usd DECIMAL(10, 6) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 7. 🔥 KNOWLEDGE BASE (For Scraped Documentation)
-- This stores the content read from sources.json URLs
CREATE TABLE knowledge_base (
    id BIGSERIAL PRIMARY KEY,
    topic TEXT NOT NULL,          -- e.g., "solana-pda-guide"
    source_url TEXT NOT NULL UNIQUE, -- Prevent duplicate scraping
    content TEXT NOT NULL,        -- The actual scraped text
    last_scraped_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 8. Indexes
CREATE INDEX idx_users_wallet ON users(wallet_address);
CREATE INDEX idx_logs_user_id ON usage_logs(user_id);
CREATE INDEX idx_api_keys_hash ON api_keys(key_hash);
CREATE INDEX idx_knowledge_topic ON knowledge_base(topic);