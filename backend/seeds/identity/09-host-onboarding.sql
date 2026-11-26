INSERT INTO identity_users (id, home_city_id, status, role, email, phone, password_hash, first_name, last_name)
VALUES
    ('seed-user-goma-applicant', 'GOMA', 'ACTIVE', 'guest', 'applicant.goma@example.com', '+243 800 000 301', '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y', 'Moise', 'Bahati'),
    ('seed-user-kin-applicant', 'KIN', 'ACTIVE', 'host', 'applicant.kin@example.com', '+243 800 000 302', '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y', 'Sarah', 'Ilunga')
ON CONFLICT (id) DO NOTHING;

INSERT INTO hosts (id, city_id, display_name, kind, status)
VALUES ('seed-host-kin-ngaliema', 'KIN', 'Ngaliema Suites', 'BUSINESS', 'ACTIVE')
ON CONFLICT (id) DO NOTHING;

INSERT INTO host_applications (id, applicant_user_id, city_id, display_name, kind, status, reason_code, decided_by, decided_at, submitted_at, host_id, created_at, updated_at)
VALUES
    ('seed-application-goma-draft', 'seed-user-goma-applicant', 'GOMA', 'Virunga View Lodge', 'BUSINESS', 'DRAFT', NULL, NULL, NULL, NULL, NULL, now() - interval '2 days', now() - interval '2 days'),
    ('seed-application-goma-pending', 'seed-user-goma-email-fr', 'GOMA', 'Kivu Garden Rooms', 'INDIVIDUAL', 'PENDING_REVIEW', NULL, NULL, NULL, now() - interval '3 days', NULL, now() - interval '6 days', now() - interval '3 days'),
    ('seed-application-kin-approved', 'seed-user-kin-applicant', 'KIN', 'Ngaliema Suites', 'BUSINESS', 'APPROVED', 'DOCUMENTS_VERIFIED', 'seed-user-kin-host', now() - interval '10 days', now() - interval '14 days', 'seed-host-kin-ngaliema', now() - interval '20 days', now() - interval '10 days'),
    ('seed-application-goma-rejected', 'seed-user-goma-removed', 'GOMA', 'Riverside Flats', 'INDIVIDUAL', 'REJECTED', 'DOCUMENTS_INCOMPLETE', 'seed-user-goma-host', now() - interval '12 days', now() - interval '15 days', NULL, now() - interval '18 days', now() - interval '12 days')
ON CONFLICT (id) DO NOTHING;

INSERT INTO host_application_documents (id, application_id, kind, storage_key, file_name, mime_type, size_bytes, status, reviewed_by, reviewed_at, reason_code, created_at)
VALUES
    ('seed-hostdoc-pending-registration', 'seed-application-goma-pending', 'BUSINESS_REGISTRATION', 'goma/hosts/applications/seed-application-goma-pending/seed-hostdoc-pending-registration.pdf', 'registration.pdf', 'application/pdf', 182300, 'VERIFIED', 'seed-user-goma-host', now() - interval '2 days', NULL, now() - interval '5 days'),
    ('seed-hostdoc-pending-identity', 'seed-application-goma-pending', 'OWNER_IDENTITY', 'goma/hosts/applications/seed-application-goma-pending/seed-hostdoc-pending-identity.png', 'identity.png', 'image/png', 640210, 'PENDING', NULL, NULL, NULL, now() - interval '4 days'),
    ('seed-hostdoc-pending-proof', 'seed-application-goma-pending', 'PROPERTY_PROOF', 'goma/hosts/applications/seed-application-goma-pending/seed-hostdoc-pending-proof.pdf', 'property-proof.pdf', 'application/pdf', 921004, 'PENDING', NULL, NULL, NULL, now() - interval '3 days'),
    ('seed-hostdoc-approved-registration', 'seed-application-kin-approved', 'BUSINESS_REGISTRATION', 'kin/hosts/applications/seed-application-kin-approved/seed-hostdoc-approved-registration.pdf', 'rccm.pdf', 'application/pdf', 210344, 'VERIFIED', 'seed-user-kin-host', now() - interval '11 days', NULL, now() - interval '18 days'),
    ('seed-hostdoc-rejected-tax', 'seed-application-goma-rejected', 'TAX_CERTIFICATE', 'goma/hosts/applications/seed-application-goma-rejected/seed-hostdoc-rejected-tax.jpg', 'tax.jpg', 'image/jpeg', 301200, 'REJECTED', 'seed-user-goma-host', now() - interval '13 days', 'EXPIRED', now() - interval '16 days')
ON CONFLICT (id) DO NOTHING;

INSERT INTO host_members (id, host_id, user_id, is_owner, status, removed_at)
VALUES ('seed-member-kin-ngaliema-owner', 'seed-host-kin-ngaliema', 'seed-user-kin-applicant', true, 'ACTIVE', NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO host_status_history (id, host_id, status, reason_code, actor_user_id, created_at)
VALUES ('seed-hoststatus-kin-paused', 'seed-host-kin-paused', 'SUSPENDED', 'UNPAID_FEES', 'seed-user-kin-host', now() - interval '5 days')
ON CONFLICT (id) DO NOTHING;

UPDATE identity_users
SET password_hash = COALESCE(password_hash, '$2y$10$.y3oDCulL5iVh6uD0XadVu.aGlLc7SIN9JuV9FSHVqmm2fB1qg13y'),
    first_name = COALESCE(first_name, 'Chantal')
WHERE id = 'seed-user-goma-email-fr';
