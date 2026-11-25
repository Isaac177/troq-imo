INSERT INTO identity_member_overrides (id, user_id, permission, scope_type, scope_id, host_id, effect, created_by)
VALUES
    ('seed-override-goma-allow', 'seed-user-goma-host', 'analytics.read', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', 'ALLOW', 'seed-user-goma-host'),
    ('seed-override-goma-deny', 'seed-user-goma-host', 'properties.write', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', 'DENY', 'seed-user-goma-host')
ON CONFLICT (id) DO NOTHING;
