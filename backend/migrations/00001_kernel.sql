CREATE TABLE kernel_audit_records (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    actor_user_id text NOT NULL,
    actor_scope jsonb NOT NULL,
    action varchar(64) NOT NULL,
    resource_type varchar(32) NOT NULL,
    resource_id text NOT NULL,
    reason text,
    correlation_id text NOT NULL,
    change_record jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX kernel_audit_resource_idx
    ON kernel_audit_records (resource_type, resource_id, created_at DESC);
CREATE INDEX kernel_audit_actor_idx
    ON kernel_audit_records (actor_user_id, created_at DESC);

CREATE TABLE kernel_outbox (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    aggregate_type varchar(32) NOT NULL,
    aggregate_id text NOT NULL,
    event_type varchar(64) NOT NULL,
    event_version integer NOT NULL DEFAULT 1 CHECK (event_version >= 1),
    payload jsonb NOT NULL,
    state varchar(16) NOT NULL DEFAULT 'PENDING' CHECK (state IN ('PENDING', 'PUBLISHED', 'FAILED')),
    attempts integer NOT NULL DEFAULT 0,
    last_error text,
    created_at timestamptz NOT NULL DEFAULT now(),
    published_at timestamptz,
    CHECK ((state = 'PUBLISHED') = (published_at IS NOT NULL))
);

CREATE INDEX kernel_outbox_pending_idx
    ON kernel_outbox (created_at) WHERE state IN ('PENDING', 'FAILED');

CREATE TABLE kernel_idempotency_keys (
    city_id text NOT NULL,
    scope varchar(64) NOT NULL,
    idempotency_key varchar(128) NOT NULL,
    resource_id text NOT NULL,
    response jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (city_id, scope, idempotency_key)
);

CREATE TABLE kernel_inbox (
    consumer_name varchar(64) NOT NULL,
    event_id text NOT NULL,
    event_type varchar(64) NOT NULL,
    processed_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (consumer_name, event_id)
);
