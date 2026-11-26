INSERT INTO stay_availability_blocks (id, city_id, "propertyId", "unitTypeId", "physicalUnitId", "startDate", "endDate", reason, origin, "actorUserId", "conflictDisposition", version, "removedAt")
VALUES
    ('seed-block-goma-villa-maintenance', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', NULL, '2026-12-28', '2026-12-30', 'MAINTENANCE', 'OPERATOR', 'seed-user-goma-host', 'NONE', 1, NULL),
    ('seed-block-goma-triple-external', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', NULL, '2026-12-22', '2026-12-23', 'EXTERNAL_BOOKING', 'INTEGRATION', 'seed-user-goma-host', 'NONE', 1, NULL),
    ('seed-block-goma-triple-owner', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', NULL, '2026-12-23', '2026-12-24', 'OWNER_USE', 'OPERATOR', 'seed-user-goma-host', 'NONE', 1, NULL),
    ('seed-block-goma-room-103-physical', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'seed-physical-goma-room-103', '2026-12-26', '2026-12-27', 'OPERATIONS', 'OPERATOR', 'seed-user-goma-host', 'NONE', 1, NULL),
    ('seed-block-goma-platform', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-room', NULL, '2026-12-31', '2027-01-02', 'PLATFORM', 'OPERATOR', 'seed-user-goma-host', 'OVERRIDDEN', 1, NULL),
    ('seed-block-goma-manual-closure', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-dorm', NULL, '2026-12-24', '2026-12-26', 'MANUAL_CLOSURE', 'OPERATOR', 'seed-user-goma-host', 'REJECTED', 1, NULL),
    ('seed-block-goma-other-removed', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', NULL, '2027-01-05', '2027-01-07', 'OTHER', 'OPERATOR', 'seed-user-goma-host', 'NONE', 2, '2026-12-01 09:00:00+00'),
    ('seed-block-kin-maintenance', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', NULL, '2026-12-20', '2026-12-22', 'MAINTENANCE', 'OPERATOR', 'seed-user-kin-host', 'NONE', 1, NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_restrictions (id, city_id, "propertyId", "unitTypeId", kind, basis, "startDate", "endDate", value, "actorUserId", version, "removedAt")
VALUES
    ('seed-restr-goma-min-two-nights', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'MIN_NIGHTS', 'STAY_THROUGH', NULL, NULL, 2, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-min-arrival-holidays', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'MIN_NIGHTS', 'ARRIVAL', '2026-12-20', '2027-01-03', 4, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-closed-arrival-christmas', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'CLOSED_TO_ARRIVAL', 'STAY_THROUGH', '2026-12-25', '2026-12-26', NULL, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-closed-departure-newyear', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'CLOSED_TO_DEPARTURE', 'STAY_THROUGH', '2027-01-01', '2027-01-02', NULL, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-triple-max-five', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', 'MAX_NIGHTS', 'STAY_THROUGH', NULL, NULL, 5, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-notice-one-day', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'ADVANCE_NOTICE_DAYS', 'STAY_THROUGH', NULL, NULL, 1, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-horizon-one-year', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'MAX_HORIZON_DAYS', 'STAY_THROUGH', NULL, NULL, 365, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-villa-turnover-one', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 'TURNOVER_BUFFER_NIGHTS', 'STAY_THROUGH', NULL, NULL, 1, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-triple-turnover-one', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', 'TURNOVER_BUFFER_NIGHTS', 'STAY_THROUGH', NULL, NULL, 1, 'seed-user-goma-host', 1, NULL),
    ('seed-restr-goma-removed-max-two', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'MAX_NIGHTS', 'STAY_THROUGH', NULL, NULL, 2, 'seed-user-goma-host', 2, '2026-12-01 09:00:00+00'),
    ('seed-restr-kin-min-two-nights', 'KIN', 'seed-prop-kin-gombe', NULL, 'MIN_NIGHTS', 'STAY_THROUGH', NULL, NULL, 2, 'seed-user-kin-host', 1, NULL),
    ('seed-restr-kin-closed-arrival', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'CLOSED_TO_ARRIVAL', 'STAY_THROUGH', '2026-12-24', '2026-12-25', NULL, 'seed-user-kin-host', 1, NULL),
    ('seed-restr-kin-turnover-two', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'TURNOVER_BUFFER_NIGHTS', 'STAY_THROUGH', NULL, NULL, 2, 'seed-user-kin-host', 1, NULL)
ON CONFLICT (id) DO NOTHING;
