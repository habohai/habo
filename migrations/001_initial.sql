-- Habo initial schema
-- Migration 001

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone           VARCHAR(20) UNIQUE NOT NULL,
    nickname        VARCHAR(100) DEFAULT '',
    avatar_url      TEXT DEFAULT '',
    apple_id        VARCHAR(255) UNIQUE DEFAULT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Verification codes
CREATE TABLE IF NOT EXISTS verification_codes (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone           VARCHAR(20) NOT NULL,
    code            VARCHAR(6) NOT NULL,
    used            BOOLEAN NOT NULL DEFAULT false,
    expired_at      TIMESTAMPTZ NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_verification_codes_phone ON verification_codes(phone);
CREATE INDEX idx_verification_codes_phone_code ON verification_codes(phone, code);

-- User profiles (extended info)
CREATE TABLE IF NOT EXISTS user_profiles (
    user_id         UUID PRIMARY KEY REFERENCES users(id),
    gender          SMALLINT NOT NULL DEFAULT 0,
    height_cm       INT DEFAULT NULL,
    weight_kg       DECIMAL(5,1) DEFAULT NULL,
    running_goal    TEXT DEFAULT '',
    weekly_goal_km  DECIMAL(6,2) DEFAULT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- User settings table
CREATE TABLE IF NOT EXISTS user_settings (
    user_id         UUID PRIMARY KEY REFERENCES users(id),
    language        VARCHAR(10) NOT NULL DEFAULT 'zh-CN',
    distance_unit   VARCHAR(5) NOT NULL DEFAULT 'km',
    notification_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
