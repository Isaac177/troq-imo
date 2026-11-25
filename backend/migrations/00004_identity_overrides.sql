CREATE TABLE identity_member_overrides (
    id text PRIMARY KEY,
    user_id text NOT NULL REFERENCES identity_users (id),
    permission text NOT NULL,
    scope_type text NOT NULL CHECK (scope_type IN ('PLATFORM', 'HOST', 'PROPERTY')),
    scope_id text,
    host_id text REFERENCES hosts (id),
    effect text NOT NULL CHECK (effect IN ('ALLOW', 'DENY')),
    created_by text NOT NULL REFERENCES identity_users (id),
    created_at timestamptz NOT NULL DEFAULT now(),
    CHECK (
        (scope_type = 'PLATFORM' AND scope_id IS NULL AND host_id IS NULL)
        OR (scope_type = 'HOST' AND scope_id IS NOT NULL AND scope_id = host_id AND host_id IS NOT NULL)
        OR (scope_type = 'PROPERTY' AND scope_id IS NOT NULL AND host_id IS NOT NULL)
    ),
    UNIQUE NULLS NOT DISTINCT (user_id, permission, scope_type, scope_id)
);

CREATE INDEX identity_member_overrides_user_idx ON identity_member_overrides (user_id);
