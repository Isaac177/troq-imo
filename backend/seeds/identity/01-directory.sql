INSERT INTO identity_users (id, home_city_id, status)
VALUES
    ('seed-user-goma-host', 'GOMA', 'ACTIVE'),
    ('seed-user-kin-host', 'KIN', 'ACTIVE'),
    ('seed-user-goma-removed', 'GOMA', 'ACTIVE'),
    ('seed-user-kin-suspended', 'KIN', 'SUSPENDED')
ON CONFLICT (id) DO NOTHING;

INSERT INTO hosts (id, city_id, display_name, kind, status)
VALUES
    ('seed-host-goma-lake', 'GOMA', 'Lake Kivu Guesthouse', 'BUSINESS', 'ACTIVE'),
    ('seed-host-kin-river', 'KIN', 'River City Rooms', 'INDIVIDUAL', 'ACTIVE'),
    ('seed-host-kin-paused', 'KIN', 'Kinshasa Courtyard', 'BUSINESS', 'SUSPENDED')
ON CONFLICT (id) DO NOTHING;

INSERT INTO host_members (id, host_id, user_id, is_owner, status, removed_at)
VALUES
    ('seed-member-goma-owner', 'seed-host-goma-lake', 'seed-user-goma-host', true, 'ACTIVE', NULL),
    ('seed-member-kin-owner', 'seed-host-kin-river', 'seed-user-kin-host', true, 'ACTIVE', NULL),
    ('seed-member-goma-removed', 'seed-host-goma-lake', 'seed-user-goma-removed', false, 'REMOVED', '2026-01-01 00:00:00+00'),
    ('seed-member-kin-suspended', 'seed-host-kin-paused', 'seed-user-kin-host', true, 'ACTIVE', NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO identity_grants (id, user_id, permission, scope_type, scope_id, host_id, expires_at)
VALUES
    ('seed-grant-goma-properties', 'seed-user-goma-host', 'properties.read', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', NULL),
    ('seed-grant-kin-properties', 'seed-user-kin-host', 'properties.read', 'HOST', 'seed-host-kin-river', 'seed-host-kin-river', NULL),
    ('seed-grant-removed', 'seed-user-goma-removed', 'properties.read', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', NULL),
    ('seed-grant-expired', 'seed-user-goma-host', 'properties.write', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', '2026-01-01 00:00:00+00')
ON CONFLICT (id) DO NOTHING;
