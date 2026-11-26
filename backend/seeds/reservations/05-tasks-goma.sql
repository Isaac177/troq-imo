INSERT INTO stay_operations_tasks (id, city_id, "propertyId", "physicalUnitId", "reservationItemId", "taskType", "assigneeUserId", state, priority, checklist, "completedAt", "createdAt")
VALUES
    ('seed-task-goma-checkin', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'seed-item-goma-confirmed', 'CHECK_IN', NULL, 'OPEN', 'HIGH', '["Confirm ID","Hand over keys"]', NULL, '2026-11-20T08:00:00Z'),
    ('seed-task-goma-cleaning', 'GOMA', 'seed-prop-goma-lakeside', 'seed-physical-goma-room-101', NULL, 'HOUSEKEEPING', NULL, 'DONE', 'NORMAL', '["Change linen"]', '2026-10-01T09:00:00Z', '2026-10-01T06:00:00Z'),
    ('seed-task-goma-urgent-maintenance', 'GOMA', 'seed-prop-goma-lakeside', NULL, NULL, 'MAINTENANCE', NULL, 'OPEN', 'URGENT', '["Fix the water pump in the garden","Test water pressure in every bathroom"]', NULL, '2026-11-24T07:00:00Z'),
    ('seed-task-goma-checkout', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'seed-item-goma-checkedin', 'CHECK_OUT', 'seed-user-goma-host', 'IN_PROGRESS', 'HIGH', '["Collect the keys","Inspect the rooms","Photograph any damage"]', NULL, '2026-11-25T08:00:00Z'),
    ('seed-task-goma-service-request', 'GOMA', 'seed-prop-goma-ready', NULL, NULL, 'SERVICE_REQUEST', NULL, 'OPEN', 'LOW', '["Prepare an extra bed"]', NULL, '2026-11-25T12:00:00Z'),
    ('seed-task-goma-cancelled', 'GOMA', 'seed-prop-goma-virunga', NULL, NULL, 'HOUSEKEEPING', NULL, 'CANCELLED', 'NORMAL', '[]', '2026-11-04T10:00:00Z', '2026-11-03T09:00:00Z'),
    ('seed-task-goma-maintenance-done', 'GOMA', 'seed-prop-goma-virunga', NULL, NULL, 'MAINTENANCE', 'seed-user-goma-host', 'DONE', 'HIGH', '["Replace the broken window latch"]', '2026-11-02T15:00:00Z', '2026-11-01T09:00:00Z')
ON CONFLICT (id) DO NOTHING;
