-- Habo workout tables
-- Migration 002

-- Device bindings (move from 001 concept to full table)
CREATE TABLE IF NOT EXISTS device_bindings (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id),
    device_type     VARCHAR(20) NOT NULL,
    device_name     VARCHAR(100) DEFAULT '',
    is_active       BOOLEAN DEFAULT true,
    config_json     JSONB DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_device_bindings_user ON device_bindings(user_id);

-- Workout records
CREATE TABLE IF NOT EXISTS workouts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id),
    device_id       UUID REFERENCES device_bindings(id) ON DELETE SET NULL,
    sport_type      VARCHAR(20) NOT NULL DEFAULT 'running',
    status          VARCHAR(20) NOT NULL DEFAULT 'completed',
    started_at      TIMESTAMPTZ NOT NULL,
    ended_at        TIMESTAMPTZ,
    duration_secs   INT,
    distance_meters DECIMAL(10,2),
    avg_heart_rate  SMALLINT,
    max_heart_rate  SMALLINT,
    avg_pace_km     DECIMAL(5,2),
    calories_kcal   INT,
    elevation_gain_m DECIMAL(8,2),
    route_data      JSONB DEFAULT '[]',
    splits_data     JSONB DEFAULT '[]',
    sensor_data     JSONB DEFAULT '{}',
    source          VARCHAR(20) DEFAULT 'habo_app',
    notes           TEXT DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_workouts_user_id ON workouts(user_id);
CREATE INDEX idx_workouts_started_at ON workouts(started_at);
CREATE INDEX idx_workouts_user_sport ON workouts(user_id, sport_type);

-- AI analysis results
CREATE TABLE IF NOT EXISTS workout_analysis (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workout_id      UUID NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users(id),
    model_used      VARCHAR(50) DEFAULT '',
    summary         TEXT DEFAULT '',
    score           SMALLINT CHECK (score BETWEEN 1 AND 100),
    highlights      JSONB DEFAULT '[]',
    suggestions     JSONB DEFAULT '[]',
    training_tips   TEXT DEFAULT '',
    raw_response    JSONB DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(workout_id)
);

-- AI conversations
CREATE TABLE IF NOT EXISTS ai_conversations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id),
    workout_id      UUID REFERENCES workouts(id) ON DELETE SET NULL,
    session_type    VARCHAR(20) DEFAULT 'post_workout',
    messages        JSONB NOT NULL DEFAULT '[]',
    model_used      VARCHAR(50) DEFAULT '',
    tokens_used     INT DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_ai_conversations_user ON ai_conversations(user_id);
