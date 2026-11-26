CREATE TABLE notifications (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    user_id text NOT NULL,
    type text NOT NULL CHECK (type IN (
        'MESSAGE', 'TICKET_UPDATE', 'TICKET_ASSIGNED', 'TICKET_COMMENT', 'TICKET_RESOLVED',
        'SYSTEM_UPDATE', 'SECURITY_ALERT', 'PAYMENT_UPDATE', 'ORDER_UPDATE', 'REVIEW_RECEIVED',
        'ANNOUNCEMENT', 'REMINDER', 'PROMOTION', 'INFO', 'ALERT'
    )),
    title text NOT NULL,
    title_en text NOT NULL,
    title_fr text NOT NULL,
    body text NOT NULL,
    body_en text NOT NULL,
    body_fr text NOT NULL,
    data jsonb,
    status text NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'SENT', 'FAILED', 'CANCELLED', 'READ', 'EXPIRED')),
    priority text NOT NULL DEFAULT 'NORMAL' CHECK (priority IN ('LOW', 'NORMAL', 'HIGH', 'URGENT')),
    icon text,
    image text,
    deep_link text,
    event_key text,
    category text,
    audience text,
    params jsonb,
    entity_type text,
    entity_id text,
    actor_id text,
    dedupe_key text,
    source_event_id text,
    read_at timestamptz,
    seen_at timestamptz,
    expires_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz
);

CREATE INDEX notifications_user_created_idx
    ON notifications (user_id, created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX notifications_user_unread_idx
    ON notifications (user_id, created_at DESC) WHERE read_at IS NULL AND deleted_at IS NULL;
CREATE INDEX notifications_user_category_idx
    ON notifications (user_id, category, created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX notifications_user_dedupe_idx
    ON notifications (user_id, dedupe_key, created_at DESC) WHERE dedupe_key IS NOT NULL;
CREATE INDEX notifications_entity_idx
    ON notifications (entity_type, entity_id) WHERE entity_id IS NOT NULL;
CREATE UNIQUE INDEX notifications_source_event_idx
    ON notifications (user_id, event_key, source_event_id) WHERE source_event_id IS NOT NULL;

CREATE TABLE notification_event_preferences (
    user_id text NOT NULL,
    event_key text NOT NULL,
    in_app boolean NOT NULL DEFAULT true,
    push boolean NOT NULL DEFAULT true,
    email boolean NOT NULL DEFAULT false,
    updated_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, event_key)
);

CREATE TABLE fcm_tokens (
    id text PRIMARY KEY,
    user_id text NOT NULL,
    device_id text NOT NULL,
    token text NOT NULL UNIQUE,
    platform text NOT NULL CHECK (platform IN ('ANDROID', 'IOS', 'WEB', 'DESKTOP')),
    app_version text,
    device_info jsonb,
    is_active boolean NOT NULL DEFAULT true,
    last_used_at timestamptz,
    invalidated_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX fcm_tokens_user_active_idx ON fcm_tokens (user_id) WHERE is_active;

CREATE TABLE notification_delivery_attempts (
    id text PRIMARY KEY,
    notification_id text NOT NULL REFERENCES notifications (id),
    platform text NOT NULL CHECK (platform IN ('ANDROID', 'IOS', 'WEB', 'DESKTOP')),
    device_token text NOT NULL,
    status text NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'SUCCESS', 'FAILED', 'RETRY')),
    error_message text,
    fcm_response jsonb,
    attempted_at timestamptz NOT NULL DEFAULT now(),
    succeeded_at timestamptz
);

CREATE INDEX notification_delivery_attempts_notification_idx
    ON notification_delivery_attempts (notification_id);

CREATE TABLE notification_deliveries (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    dedupe_key text NOT NULL UNIQUE,
    channel text NOT NULL CHECK (channel IN ('PUSH', 'EMAIL', 'WHATSAPP', 'DOCUMENT')),
    user_id text,
    notification_id text REFERENCES notifications (id),
    payload jsonb NOT NULL,
    state text NOT NULL DEFAULT 'PENDING' CHECK (state IN ('PENDING', 'DONE', 'FAILED', 'SKIPPED')),
    attempts integer NOT NULL DEFAULT 0,
    last_error text,
    next_attempt_at timestamptz NOT NULL DEFAULT now(),
    created_at timestamptz NOT NULL DEFAULT now(),
    done_at timestamptz
);

CREATE INDEX notification_deliveries_due_idx
    ON notification_deliveries (next_attempt_at, created_at, id) WHERE state = 'PENDING';

CREATE TABLE messaging_document (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    kind text NOT NULL CHECK (kind IN ('stay_voucher', 'stay_cancellation', 'stay_damage_claim')),
    locale text NOT NULL CHECK (locale IN ('en', 'fr')),
    entity_type text NOT NULL,
    entity_id text NOT NULL,
    document_number text,
    storage_key text NOT NULL,
    file_name text NOT NULL,
    sha256 text NOT NULL CHECK (sha256 ~ '^[0-9a-f]{64}$'),
    page_count integer NOT NULL CHECK (page_count > 0),
    bytes integer NOT NULL CHECK (bytes > 0),
    supersedes_id text REFERENCES messaging_document (id),
    guest_user_id text,
    host_id text,
    created_by text,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX messaging_document_entity_idx
    ON messaging_document (entity_type, entity_id, created_at DESC);
CREATE INDEX messaging_document_guest_idx
    ON messaging_document (guest_user_id, created_at DESC) WHERE guest_user_id IS NOT NULL;
CREATE INDEX messaging_document_host_idx
    ON messaging_document (host_id, created_at DESC) WHERE host_id IS NOT NULL;
CREATE UNIQUE INDEX messaging_document_stay_voucher_per_reservation_idx
    ON messaging_document (entity_type, entity_id) WHERE kind = 'stay_voucher';
CREATE UNIQUE INDEX messaging_document_stay_cancellation_per_reservation_idx
    ON messaging_document (entity_type, entity_id) WHERE kind = 'stay_cancellation';
CREATE UNIQUE INDEX messaging_document_stay_damage_claim_per_case_idx
    ON messaging_document (entity_type, entity_id) WHERE kind = 'stay_damage_claim';

CREATE FUNCTION messaging_document_immutable() RETURNS trigger
LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'messaging_document rows are immutable';
END;
$$;

CREATE TRIGGER messaging_document_no_update
    BEFORE UPDATE OR DELETE ON messaging_document
    FOR EACH ROW EXECUTE FUNCTION messaging_document_immutable();

CREATE TABLE messaging_document_event (
    id text PRIMARY KEY,
    document_id text NOT NULL REFERENCES messaging_document (id),
    event_type text NOT NULL CHECK (event_type IN ('generated', 'downloaded')),
    actor_id text,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX messaging_document_event_document_idx
    ON messaging_document_event (document_id, created_at);

CREATE TRIGGER messaging_document_event_no_update
    BEFORE UPDATE OR DELETE ON messaging_document_event
    FOR EACH ROW EXECUTE FUNCTION messaging_document_immutable();
