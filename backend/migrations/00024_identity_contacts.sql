ALTER TABLE identity_users
    ADD COLUMN email text,
    ADD COLUMN phone text,
    ADD COLUMN preferred_language text NOT NULL DEFAULT 'en';
