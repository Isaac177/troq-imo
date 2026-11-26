ALTER TABLE identity_users
    ADD COLUMN first_name varchar(80),
    ADD COLUMN last_name varchar(80),
    ADD COLUMN email_verified_at timestamptz;

CREATE UNIQUE INDEX identity_users_email_unique_idx
    ON identity_users (lower(email)) WHERE email IS NOT NULL;
CREATE UNIQUE INDEX identity_users_phone_unique_idx
    ON identity_users (regexp_replace(phone, '\D', '', 'g')) WHERE phone IS NOT NULL AND phone <> '';

CREATE TABLE identity_invitations (
    id text PRIMARY KEY,
    host_id text NOT NULL REFERENCES hosts (id),
    email text NOT NULL,
    permissions text[] NOT NULL,
    invited_by text NOT NULL REFERENCES identity_users (id),
    status text NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'ACCEPTED', 'REVOKED', 'EXPIRED')),
    expires_at timestamptz NOT NULL,
    accepted_by text REFERENCES identity_users (id),
    resolved_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    CHECK ((status = 'PENDING') = (resolved_at IS NULL)),
    CHECK ((status = 'ACCEPTED') = (accepted_by IS NOT NULL))
);

CREATE UNIQUE INDEX identity_invitations_pending_unique_idx
    ON identity_invitations (host_id, lower(email)) WHERE status = 'PENDING';
CREATE INDEX identity_invitations_email_idx
    ON identity_invitations (lower(email), status);
CREATE INDEX identity_invitations_host_idx
    ON identity_invitations (host_id, status);

CREATE TABLE host_applications (
    id text PRIMARY KEY,
    applicant_user_id text NOT NULL REFERENCES identity_users (id),
    city_id text NOT NULL,
    display_name varchar(160) NOT NULL,
    kind text NOT NULL CHECK (kind IN ('INDIVIDUAL', 'BUSINESS')),
    status text NOT NULL DEFAULT 'DRAFT' CHECK (status IN ('DRAFT', 'PENDING_REVIEW', 'APPROVED', 'REJECTED', 'WITHDRAWN')),
    reason_code text,
    decided_by text REFERENCES identity_users (id),
    decided_at timestamptz,
    submitted_at timestamptz,
    host_id text REFERENCES hosts (id),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CHECK (status IN ('DRAFT', 'WITHDRAWN') OR submitted_at IS NOT NULL),
    CHECK ((status IN ('APPROVED', 'REJECTED')) = (decided_at IS NOT NULL))
);

CREATE INDEX host_applications_applicant_idx
    ON host_applications (applicant_user_id, status);
CREATE INDEX host_applications_city_status_idx
    ON host_applications (city_id, status, created_at DESC);

CREATE TABLE host_application_documents (
    id text PRIMARY KEY,
    application_id text NOT NULL REFERENCES host_applications (id),
    kind text NOT NULL CHECK (kind IN ('BUSINESS_REGISTRATION', 'TAX_CERTIFICATE', 'OWNER_IDENTITY', 'PROPERTY_PROOF', 'INSURANCE', 'SAFETY_CERTIFICATE')),
    storage_key text NOT NULL,
    file_name varchar(255) NOT NULL,
    mime_type varchar(64) NOT NULL,
    size_bytes integer NOT NULL CHECK (size_bytes > 0),
    status text NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'VERIFIED', 'REJECTED', 'EXPIRED')),
    reviewed_by text REFERENCES identity_users (id),
    reviewed_at timestamptz,
    reason_code text,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX host_application_documents_application_idx
    ON host_application_documents (application_id, created_at);

CREATE TABLE host_status_history (
    id text PRIMARY KEY,
    host_id text NOT NULL REFERENCES hosts (id),
    status text NOT NULL CHECK (status IN ('ACTIVE', 'SUSPENDED')),
    reason_code text NOT NULL,
    actor_user_id text NOT NULL REFERENCES identity_users (id),
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX host_status_history_host_idx
    ON host_status_history (host_id, created_at DESC);
