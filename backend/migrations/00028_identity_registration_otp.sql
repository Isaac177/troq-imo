CREATE TABLE identity_registration_otp (
    id text PRIMARY KEY,
    channel text NOT NULL CHECK (channel IN ('email', 'whatsapp')),
    destination text NOT NULL,
    code_digest bytea NOT NULL,
    attempts integer NOT NULL DEFAULT 0 CHECK (attempts BETWEEN 0 AND 5),
    issued_at timestamptz NOT NULL DEFAULT now(),
    expires_at timestamptz NOT NULL,
    proof_digest bytea,
    verified_until timestamptz,
    UNIQUE (channel, destination),
    CHECK ((proof_digest IS NULL) = (verified_until IS NULL))
);
