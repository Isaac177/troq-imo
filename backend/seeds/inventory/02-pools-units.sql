INSERT INTO stay_inventory_pools (id, city_id, "propertyId", "unitTypeId", "sellableQuantity", "defaultNightlyCapacity")
VALUES
    ('seed-pool-goma-villa', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 1, true),
    ('seed-pool-goma-room', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-room', 2, true),
    ('seed-pool-goma-triple', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', 3, true),
    ('seed-pool-goma-dorm', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-dorm', 6, false),
    ('seed-pool-goma-inactive', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-inactive', 1, true),
    ('seed-pool-goma-archived', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-archived', 0, true),
    ('seed-pool-goma-ready-house', 'GOMA', 'seed-prop-goma-ready', 'seed-unit-type-goma-ready-house', 1, true),
    ('seed-pool-goma-virunga-suite', 'GOMA', 'seed-prop-goma-virunga', 'seed-unit-type-goma-virunga-suite', 1, true),
    ('seed-pool-kin-apartment', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 1, true),
    ('seed-pool-kin-suite', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-suite', 2, true)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_physical_units (id, city_id, "propertyId", "unitTypeId", "localCode", status, "retiredAt")
VALUES
    ('seed-physical-goma-villa', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 'VILLA', 'ACTIVE', NULL),
    ('seed-physical-goma-room-101', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-room', 'R101', 'ACTIVE', NULL),
    ('seed-physical-goma-room-102', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-room', 'R102', 'OUT_OF_SERVICE', NULL),
    ('seed-physical-goma-room-103', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-room', 'R103', 'ACTIVE', NULL),
    ('seed-physical-goma-room-104', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-room', 'R104', 'RETIRED', '2026-09-01 08:00:00+00'),
    ('seed-physical-goma-triple-201', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', 'T201', 'ACTIVE', NULL),
    ('seed-physical-goma-triple-202', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', 'T202', 'ACTIVE', NULL),
    ('seed-physical-goma-triple-203', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', 'T203', 'ACTIVE', NULL),
    ('seed-physical-goma-triple-204', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', 'T204', 'OUT_OF_SERVICE', NULL),
    ('seed-physical-goma-dorm-1', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-dorm', 'D1', 'ACTIVE', NULL),
    ('seed-physical-goma-dorm-2', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-dorm', 'D2', 'ACTIVE', NULL),
    ('seed-physical-goma-ready-house', 'GOMA', 'seed-prop-goma-ready', 'seed-unit-type-goma-ready-house', 'GH1', 'ACTIVE', NULL),
    ('seed-physical-goma-virunga-suite', 'GOMA', 'seed-prop-goma-virunga', 'seed-unit-type-goma-virunga-suite', 'VS1', 'ACTIVE', NULL),
    ('seed-physical-kin-apartment', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'APT-1', 'ACTIVE', NULL),
    ('seed-physical-kin-suite-1', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-suite', 'S1', 'ACTIVE', NULL),
    ('seed-physical-kin-suite-2', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-suite', 'S2', 'ACTIVE', NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_pool_physical_units ("poolId", "physicalUnitId")
VALUES
    ('seed-pool-goma-villa', 'seed-physical-goma-villa'),
    ('seed-pool-goma-room', 'seed-physical-goma-room-101'),
    ('seed-pool-goma-room', 'seed-physical-goma-room-103'),
    ('seed-pool-goma-triple', 'seed-physical-goma-triple-201'),
    ('seed-pool-goma-triple', 'seed-physical-goma-triple-202'),
    ('seed-pool-goma-triple', 'seed-physical-goma-triple-203'),
    ('seed-pool-goma-dorm', 'seed-physical-goma-dorm-1'),
    ('seed-pool-goma-dorm', 'seed-physical-goma-dorm-2'),
    ('seed-pool-goma-ready-house', 'seed-physical-goma-ready-house'),
    ('seed-pool-goma-virunga-suite', 'seed-physical-goma-virunga-suite'),
    ('seed-pool-kin-apartment', 'seed-physical-kin-apartment'),
    ('seed-pool-kin-suite', 'seed-physical-kin-suite-1'),
    ('seed-pool-kin-suite', 'seed-physical-kin-suite-2')
ON CONFLICT DO NOTHING;
