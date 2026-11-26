UPDATE identity_users
SET password_hash = '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y'
WHERE id IN ('seed-user-goma-host', 'seed-user-kin-host') AND password_hash IS NULL;

INSERT INTO identity_login_attempts (id, user_id, success, attempted_at)
VALUES
    ('seed-login-goma-failed', 'seed-user-goma-host', false, now() - interval '1 day'),
    ('seed-login-kin-success', 'seed-user-kin-host', true, now() - interval '1 day')
ON CONFLICT (id) DO NOTHING;

INSERT INTO identity_users (id, home_city_id, status, role, email, password_hash)
VALUES ('seed-user-locked-login', 'GOMA', 'ACTIVE', 'guest', 'locked.guest@example.com', '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y')
ON CONFLICT (id) DO NOTHING;

INSERT INTO identity_login_attempts (id, user_id, success, attempted_at)
SELECT 'seed-login-locked-' || n, 'seed-user-locked-login', false, now() - n * interval '1 minute'
FROM generate_series(1, 5) AS n
ON CONFLICT (id) DO NOTHING;
