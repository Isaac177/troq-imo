INSERT INTO hosts (id, city_id, display_name, kind, status)
VALUES
    ('seed-host-goma-nocredit', 'GOMA', 'Virunga Residences', 'BUSINESS', 'ACTIVE'),
    ('seed-host-goma-lowcredit', 'GOMA', 'Rive Gauche Lodges', 'BUSINESS', 'ACTIVE'),
    ('seed-host-goma-zerocredit', 'GOMA', 'Kivu Stays Collective', 'BUSINESS', 'ACTIVE')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_operator_commission_rates (id, city_id, "operatorId", "rateBps", "effectiveFrom", "reasonCode", note, "changedByUserId", "createdAt")
VALUES
    ('seed-comrate-goma-lake-1', 'GOMA', 'seed-host-goma-lake', 800, '2026-06-01T08:00:00Z', 'CONTRACT_AGREED', 'Launch contract signed with the owner.', 'seed-staff-billing', '2026-06-01T08:00:00Z'),
    ('seed-comrate-goma-lake-2', 'GOMA', 'seed-host-goma-lake', 1000, '2026-09-25T08:00:00Z', 'RENEGOTIATED', 'Rate raised after the first quarter review.', 'seed-staff-billing', '2026-09-25T08:00:00Z'),
    ('seed-comrate-goma-lake-3', 'GOMA', 'seed-host-goma-lake', 1250, '2026-10-24T08:00:00Z', 'PLATFORM_POLICY', NULL, 'seed-staff-billing', '2026-10-24T08:00:00Z'),
    ('seed-comrate-goma-lake-future', 'GOMA', 'seed-host-goma-lake', 1500, '2027-06-01T08:00:00Z', 'RENEGOTIATED', 'Scheduled increase, not yet effective.', 'seed-staff-billing', '2026-11-01T08:00:00Z'),
    ('seed-comrate-goma-nocredit', 'GOMA', 'seed-host-goma-nocredit', 1000, '2026-08-01T08:00:00Z', 'CONTRACT_AGREED', NULL, 'seed-staff-billing', '2026-08-01T08:00:00Z'),
    ('seed-comrate-goma-lowcredit', 'GOMA', 'seed-host-goma-lowcredit', 1000, '2026-08-01T08:00:00Z', 'CONTRACT_AGREED', NULL, 'seed-staff-billing', '2026-08-01T08:00:00Z'),
    ('seed-comrate-goma-zerocredit', 'GOMA', 'seed-host-goma-zerocredit', 800, '2026-08-01T08:00:00Z', 'LAUNCH_OFFER', NULL, 'seed-staff-billing', '2026-08-01T08:00:00Z'),
    ('seed-comrate-kin-river-1', 'KIN', 'seed-host-kin-river', 1000, '2026-08-01T08:00:00Z', 'CONTRACT_AGREED', NULL, 'seed-staff-billing', '2026-08-01T08:00:00Z'),
    ('seed-comrate-kin-paused', 'KIN', 'seed-host-kin-paused', 0, '2026-07-01T08:00:00Z', 'LAUNCH_OFFER', 'Commission-free launch period.', 'seed-staff-billing', '2026-07-01T08:00:00Z')
ON CONFLICT (id) DO NOTHING;
