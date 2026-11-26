UPDATE identity_users
SET email = COALESCE(email, 'goma.host@example.com'),
    phone = COALESCE(phone, '+243 800 000 101')
WHERE id = 'seed-user-goma-host';

UPDATE identity_users
SET email = COALESCE(email, 'kin.host@example.com'),
    phone = COALESCE(phone, '+243 800 000 102')
WHERE id = 'seed-user-kin-host';

INSERT INTO identity_users (id, home_city_id, status, email, preferred_language)
VALUES ('seed-user-goma-email-fr', 'GOMA', 'ACTIVE', 'guest.fr@example.com', 'fr')
ON CONFLICT (id) DO NOTHING;
