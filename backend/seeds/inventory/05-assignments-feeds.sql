INSERT INTO stay_unit_assignments (id, city_id, "reservationItemId", "physicalUnitId", "startDate", "endDate", reason, "actorUserId", "releasedAt")
SELECT seed.id, seed.city_id, seed.item_id, seed.unit_id, seed.start_date, seed.end_date, seed.reason, seed.actor, seed.released_at
FROM (VALUES
    ('seed-assign-goma-confirmed-villa', 'GOMA', 'seed-item-goma-confirmed', 'seed-physical-goma-villa', DATE '2026-12-20', DATE '2026-12-22', 'ASSIGNED', 'seed-user-goma-host', NULL::timestamptz),
    ('seed-assign-goma-needs-room-first', 'GOMA', 'seed-item-goma-needs-room', 'seed-physical-goma-room-101', DATE '2026-12-26', DATE '2026-12-27', 'ASSIGNED', 'seed-user-goma-host', '2026-12-05 09:00:00+00'::timestamptz),
    ('seed-assign-goma-needs-room-moved', 'GOMA', 'seed-item-goma-needs-room', 'seed-physical-goma-room-103', DATE '2026-12-26', DATE '2026-12-27', 'MOVED', 'seed-user-goma-host', NULL),
    ('seed-assign-goma-checked-in', 'GOMA', 'seed-item-goma-arrives-today', 'seed-physical-goma-room-101', DATE '2026-11-06', DATE '2026-11-08', 'CHECK_IN', 'seed-user-goma-host', NULL),
    ('seed-assign-kin-confirmed', 'KIN', 'seed-item-kin-confirmed', 'seed-physical-kin-apartment', DATE '2026-12-10', DATE '2026-12-12', 'ASSIGNED', 'seed-user-kin-host', NULL)
) AS seed(id, city_id, item_id, unit_id, start_date, end_date, reason, actor, released_at)
WHERE EXISTS (SELECT 1 FROM stay_reservation_items i WHERE i.id = seed.item_id)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_calendar_feed_tokens (id, city_id, "propertyId", "secretHash", "createdByUserId", "revokedAt")
VALUES
    ('ftk-seed-goma-active', 'GOMA', 'seed-prop-goma-lakeside', '3138bb9bc78df27c473ecfd1410f7bd45ebac1f59cf3ff9cfe4db77aab7aedd3', 'seed-user-goma-host', NULL),
    ('ftk-seed-goma-revoked', 'GOMA', 'seed-prop-goma-lakeside', '4f2e8d65483c641648cdb374ae9d8abd368d269e4ddffe092a8237b8162cddd6', 'seed-user-goma-host', '2026-12-01 09:00:00+00'),
    ('ftk-seed-kin-active', 'KIN', 'seed-prop-kin-gombe', '79bd7d7fd684b399857c582b1b7172ddf277d4fe1b027ec52b28da3ae381e675', 'seed-user-kin-host', NULL)
ON CONFLICT (id) DO NOTHING;
