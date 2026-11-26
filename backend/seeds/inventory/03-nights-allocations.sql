INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount")
VALUES
    ('seed-pool-goma-villa', '2026-12-20', 1, 1),
    ('seed-pool-goma-villa', '2026-12-21', 1, 1),
    ('seed-pool-goma-villa', '2026-12-22', 1, 0),
    ('seed-pool-goma-villa', '2026-12-23', 1, 0),
    ('seed-pool-goma-villa', '2026-12-24', 1, 0),
    ('seed-pool-goma-villa', '2026-12-28', 1, 1),
    ('seed-pool-goma-villa', '2026-12-29', 1, 1),
    ('seed-pool-goma-room', '2026-12-20', 2, 1),
    ('seed-pool-goma-room', '2026-12-21', 2, 0),
    ('seed-pool-goma-room', '2026-12-23', 2, 0),
    ('seed-pool-goma-room', '2026-12-26', 2, 0),
    ('seed-pool-goma-triple', '2026-12-20', 3, 0),
    ('seed-pool-goma-triple', '2026-12-21', 3, 0),
    ('seed-pool-goma-triple', '2026-12-22', 3, 1),
    ('seed-pool-goma-triple', '2026-12-23', 3, 1),
    ('seed-pool-goma-triple', '2026-12-24', 3, 0),
    ('seed-pool-goma-dorm', '2026-12-20', 6, 0),
    ('seed-pool-goma-dorm', '2026-12-21', 6, 2),
    ('seed-pool-goma-dorm', '2026-12-22', 0, 0),
    ('seed-pool-kin-apartment', '2026-12-10', 1, 1),
    ('seed-pool-kin-apartment', '2026-12-11', 1, 1),
    ('seed-pool-kin-apartment', '2026-12-12', 1, 0),
    ('seed-pool-kin-suite', '2026-12-10', 2, 1),
    ('seed-pool-kin-suite', '2026-12-11', 2, 0)
ON CONFLICT ("poolId", "stayDate") DO NOTHING;

INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount")
SELECT pool_id, night::date, quantity, 0
FROM (VALUES
    ('seed-pool-goma-ready-house', 1), ('seed-pool-goma-virunga-suite', 1)
) AS pools(pool_id, quantity)
CROSS JOIN generate_series(DATE '2026-10-01', DATE '2027-03-31', INTERVAL '1 day') AS night
ON CONFLICT ("poolId", "stayDate") DO NOTHING;

INSERT INTO stay_inventory_allocations (id, city_id, "poolId", "referenceType", "referenceId", "startDate", "endDate", quantity, state, "expiresAt", "releasedAt")
VALUES
    ('seed-alloc-goma-confirmed-n1', 'GOMA', 'seed-pool-goma-villa', 'RESERVATION', 'seed-res-goma-confirmed', '2026-12-20', '2026-12-21', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-goma-confirmed-n2', 'GOMA', 'seed-pool-goma-villa', 'RESERVATION', 'seed-res-goma-confirmed', '2026-12-21', '2026-12-22', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-goma-block-n1', 'GOMA', 'seed-pool-goma-villa', 'BLOCK', 'seed-block-goma-villa-maintenance', '2026-12-28', '2026-12-29', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-goma-block-n2', 'GOMA', 'seed-pool-goma-villa', 'BLOCK', 'seed-block-goma-villa-maintenance', '2026-12-29', '2026-12-30', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-goma-cancelled-n1', 'GOMA', 'seed-pool-goma-villa', 'RESERVATION', 'seed-res-goma-cancelled', '2026-12-23', '2026-12-24', 1, 'RELEASED', NULL, '2026-12-01 09:00:00+00'),
    ('seed-alloc-goma-expired-n1', 'GOMA', 'seed-pool-goma-villa', 'HOLD', 'seed-res-goma-expired', '2026-12-24', '2026-12-25', 1, 'EXPIRED', '2026-12-02 10:00:00+00', '2026-12-02 10:05:00+00'),
    ('seed-alloc-goma-requested', 'GOMA', 'seed-pool-goma-room', 'HOLD', 'seed-res-goma-requested', '2026-12-20', '2026-12-21', 1, 'ACTIVE', '2026-12-10 18:00:00+00', NULL),
    ('seed-alloc-goma-triple-external-n1', 'GOMA', 'seed-pool-goma-triple', 'BLOCK', 'seed-block-goma-triple-external', '2026-12-22', '2026-12-23', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-goma-triple-owner-n1', 'GOMA', 'seed-pool-goma-triple', 'BLOCK', 'seed-block-goma-triple-owner', '2026-12-23', '2026-12-24', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-goma-dorm-n1', 'GOMA', 'seed-pool-goma-dorm', 'RESERVATION', 'seed-res-goma-dorm-a', '2026-12-21', '2026-12-22', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-goma-dorm-n2', 'GOMA', 'seed-pool-goma-dorm', 'RESERVATION', 'seed-res-goma-dorm-b', '2026-12-21', '2026-12-22', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-kin-confirmed-n1', 'KIN', 'seed-pool-kin-apartment', 'RESERVATION', 'seed-res-kin-confirmed', '2026-12-10', '2026-12-11', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-kin-confirmed-n2', 'KIN', 'seed-pool-kin-apartment', 'RESERVATION', 'seed-res-kin-confirmed', '2026-12-11', '2026-12-12', 1, 'ACTIVE', NULL, NULL),
    ('seed-alloc-kin-suite-hold', 'KIN', 'seed-pool-kin-suite', 'HOLD', 'seed-res-kin-suite-hold', '2026-12-10', '2026-12-11', 1, 'ACTIVE', '2026-12-09 12:00:00+00', NULL)
ON CONFLICT (id) DO NOTHING;
