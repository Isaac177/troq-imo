ALTER TABLE identity_users
    ADD COLUMN role text CHECK (role IN ('guest', 'host', 'staff'));

CREATE TABLE identity_refresh_sessions (
    id text PRIMARY KEY,
    user_id text NOT NULL REFERENCES identity_users(id),
    token_hash bytea NOT NULL UNIQUE,
    expires_at timestamptz NOT NULL,
    revoked_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX identity_refresh_sessions_user_idx ON identity_refresh_sessions (user_id, expires_at DESC);
