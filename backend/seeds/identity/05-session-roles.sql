UPDATE identity_users SET role = 'host' WHERE id IN ('seed-user-goma-host', 'seed-user-kin-host') AND role IS NULL;
UPDATE identity_users SET role = 'guest' WHERE id IN ('seed-user-goma-removed', 'seed-user-goma-email-fr') AND role IS NULL;
UPDATE identity_users SET role = 'host' WHERE id = 'seed-user-kin-suspended' AND role IS NULL;

INSERT INTO identity_refresh_sessions (id, user_id, token_hash, expires_at, revoked_at)
VALUES
    ('seed-session-expired', 'seed-user-goma-host', decode(repeat('01', 32), 'hex'), '2025-01-01 00:00:00+00', NULL),
    ('seed-session-revoked', 'seed-user-kin-host', decode(repeat('02', 32), 'hex'), '2025-01-01 00:00:00+00', '2024-12-31 00:00:00+00')
ON CONFLICT DO NOTHING;
