ALTER TABLE identity_users
    ADD COLUMN password_hash text,
    ADD COLUMN last_login_at timestamptz;

CREATE TABLE identity_login_attempts (
    id text PRIMARY KEY,
    user_id text NOT NULL REFERENCES identity_users(id),
    success boolean NOT NULL,
    attempted_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX identity_login_attempts_failed_idx
    ON identity_login_attempts (user_id, attempted_at DESC)
    WHERE success = false;
