INSERT INTO identity_users (id, home_city_id, status, role, email, phone, password_hash, first_name, last_name)
VALUES ('seed-user-goma-invitee', 'GOMA', 'ACTIVE', 'guest', 'invitee.desk@example.com', '+243 800 000 403', '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y', 'Chantal', 'Mugisha')
ON CONFLICT DO NOTHING;

INSERT INTO identity_invitations (id, host_id, email, permissions, invited_by, status, expires_at, accepted_by, resolved_at, created_at)
VALUES ('seed-invite-kin-other-city', 'seed-host-kin-river', 'invitee.desk@example.com', ARRAY['reservations.read'], 'seed-user-kin-host', 'PENDING', now() + interval '7 days', NULL, NULL, now())
ON CONFLICT (id) DO NOTHING;
