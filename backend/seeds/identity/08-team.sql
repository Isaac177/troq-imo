INSERT INTO identity_users (id, home_city_id, status, role, email, phone, password_hash, first_name, last_name)
VALUES
    ('seed-user-goma-manager', 'GOMA', 'ACTIVE', 'host', 'manager.lake@example.com', '+243 800 000 201', '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y', 'Aline', 'Kambale')
ON CONFLICT (id) DO NOTHING;

INSERT INTO host_members (id, host_id, user_id, is_owner, status, removed_at)
VALUES ('seed-member-goma-manager', 'seed-host-goma-lake', 'seed-user-goma-manager', false, 'ACTIVE', NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO identity_grants (id, user_id, permission, scope_type, scope_id, host_id, expires_at)
VALUES
    ('seed-grant-manager-reservations', 'seed-user-goma-manager', 'reservations.read', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', NULL),
    ('seed-grant-manager-messages', 'seed-user-goma-manager', 'messages.write', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', NULL),
    ('seed-grant-manager-checkin', 'seed-user-goma-manager', 'operations.checkin', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO identity_member_overrides (id, user_id, permission, scope_type, scope_id, host_id, effect, created_by)
VALUES ('seed-override-manager-deny-messages', 'seed-user-goma-manager', 'messages.write', 'HOST', 'seed-host-goma-lake', 'seed-host-goma-lake', 'DENY', 'seed-user-goma-host')
ON CONFLICT (id) DO NOTHING;

INSERT INTO identity_users (id, home_city_id, status, role, email, phone, password_hash, first_name, last_name)
VALUES
    ('seed-user-platform-admin', 'GOMA', 'ACTIVE', 'staff', 'admin.platform@example.com', '+243 800 000 401', '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y', 'Amina', 'Nkosi'),
    ('seed-user-platform-kin', 'KIN', 'ACTIVE', 'staff', 'admin.kin@example.com', '+243 800 000 402', '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y', 'Patrick', 'Mbuyi')
ON CONFLICT (id) DO NOTHING;

INSERT INTO identity_grants (id, user_id, permission, scope_type, scope_id, host_id, expires_at)
VALUES
    ('seed-grant-admin-properties-read', 'seed-user-platform-admin', 'properties.read', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-admin-properties-moderate', 'seed-user-platform-admin', 'properties.moderate', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-admin-hosts-review', 'seed-user-platform-admin', 'hosts.review_applications', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-admin-reviews-moderate', 'seed-user-platform-admin', 'reviews.moderate', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-admin-reservations-read', 'seed-user-platform-admin', 'reservations.read', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-admin-billing-read', 'seed-user-platform-admin', 'billing.read', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-admin-audit-read', 'seed-user-platform-admin', 'audit.read', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-admin-analytics-read', 'seed-user-platform-admin', 'analytics.read', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-kin-hosts-review', 'seed-user-platform-kin', 'hosts.review_applications', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-kin-properties-read', 'seed-user-platform-kin', 'properties.read', 'PLATFORM', NULL, NULL, NULL),
    ('seed-grant-kin-reservations-read', 'seed-user-platform-kin', 'reservations.read', 'PLATFORM', NULL, NULL, NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO identity_invitations (id, host_id, email, permissions, invited_by, status, expires_at, accepted_by, resolved_at, created_at)
VALUES
    ('seed-invite-goma-pending', 'seed-host-goma-lake', 'invitee.desk@example.com', ARRAY['reservations.read', 'reservations.write', 'messages.write'], 'seed-user-goma-host', 'PENDING', now() + interval '7 days', NULL, NULL, now() - interval '1 day'),
    ('seed-invite-goma-accepted', 'seed-host-goma-lake', 'manager.lake@example.com', ARRAY['reservations.read'], 'seed-user-goma-host', 'ACCEPTED', now() - interval '20 days', 'seed-user-goma-manager', now() - interval '21 days', now() - interval '28 days'),
    ('seed-invite-kin-expired', 'seed-host-kin-river', 'former.desk@example.com', ARRAY['reservations.read'], 'seed-user-kin-host', 'EXPIRED', now() - interval '2 days', NULL, now() - interval '2 days', now() - interval '9 days'),
    ('seed-invite-kin-revoked', 'seed-host-kin-river', 'revoked.desk@example.com', ARRAY['analytics.read'], 'seed-user-kin-host', 'REVOKED', now() + interval '3 days', NULL, now() - interval '1 day', now() - interval '4 days')
ON CONFLICT (id) DO NOTHING;
