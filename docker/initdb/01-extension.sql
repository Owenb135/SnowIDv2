-- ==============================================================================
-- SnowID Docker Initialization Script
-- Automatically run when the container data volume is initialized for the first time
-- ==============================================================================

-- 1. Enable the SnowID native Rust extension
CREATE EXTENSION IF NOT EXISTS snowidv2;

-- 2. Create sample demonstration table with native snowidv2() primary key
CREATE TABLE IF NOT EXISTS demo_users (
    id BIGINT PRIMARY KEY DEFAULT snowidv2(),
    username TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Insert initial demonstration rows
INSERT INTO demo_users (username) VALUES
    ('alice_snowid'),
    ('bob_snowid')
ON CONFLICT (username) DO NOTHING;
