CREATE TABLE identity_users (
    id text PRIMARY KEY,
    home_city_id text NOT NULL,
    status text NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'SUSPENDED')),
    revoked_before bigint,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE hosts (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    display_name varchar(160) NOT NULL,
    kind text NOT NULL CHECK (kind IN ('INDIVIDUAL', 'BUSINESS')),
    status text NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'SUSPENDED')),
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE host_members (
    id text PRIMARY KEY,
    host_id text NOT NULL REFERENCES hosts (id),
    user_id text NOT NULL REFERENCES identity_users (id),
    is_owner boolean NOT NULL DEFAULT false,
    status text NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'REMOVED')),
    created_at timestamptz NOT NULL DEFAULT now(),
    removed_at timestamptz,
    CHECK ((status = 'REMOVED') = (removed_at IS NOT NULL))
);

CREATE UNIQUE INDEX host_members_active_unique_idx
    ON host_members (host_id, user_id) WHERE status = 'ACTIVE';
CREATE INDEX host_members_user_active_idx
    ON host_members (user_id, host_id) WHERE status = 'ACTIVE';

CREATE TABLE identity_grants (
    id text PRIMARY KEY,
    user_id text NOT NULL REFERENCES identity_users (id),
    permission text NOT NULL,
    scope_type text NOT NULL CHECK (scope_type IN ('PLATFORM', 'HOST', 'PROPERTY')),
    scope_id text,
    host_id text REFERENCES hosts (id),
    expires_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    CHECK (
        (scope_type = 'PLATFORM' AND scope_id IS NULL AND host_id IS NULL)
        OR (scope_type = 'HOST' AND scope_id IS NOT NULL AND scope_id = host_id AND host_id IS NOT NULL)
        OR (scope_type = 'PROPERTY' AND scope_id IS NOT NULL AND host_id IS NOT NULL)
    )
);

CREATE INDEX identity_grants_user_idx ON identity_grants (user_id);
