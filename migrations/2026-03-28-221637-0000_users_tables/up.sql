-- Your SQL goes here
CREATE TABLE users (
    id TEXT PRIMARY KEY CHECK (char_length(id) = 26),
    email TEXT NOT NULL UNIQUE,
    verified_at TIMESTAMPTZ NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email_lower ON users (LOWER(email));

CREATE TABLE user_sessions (
    id TEXT PRIMARY KEY CHECK (char_length(id) = 26),
    user_id TEXT NOT NULL REFERENCES users(id),
    session_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    ip_address TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    device_info TEXT NOT NULL,
    location TEXT NOT NULL,
    country TEXT NOT NULL,
    city TEXT NOT NULL,
    region TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ NULL

);

CREATE INDEX idx_user_sessions_user_id ON user_sessions (user_id);

CREATE INDEX idx_user_sessions_session_token ON user_sessions (session_token);