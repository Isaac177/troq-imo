INSERT INTO stay_cases (id, city_id, "operatorId", "reservationId", "caseType", parties, state, "deadlineAt", resolution, reason, "claimAmount", "closedAt")
VALUES
    ('seed-case-kin-open', 'KIN', 'seed-host-kin-river', 'seed-res-kin-checkedin', 'GUEST_COMPLAINT', '{"guestUserId":"seed-guest-kin-3","openedBy":"GUEST","openedByUserId":"seed-guest-kin-3"}', 'OPEN', '2026-12-10T18:00:00Z', NULL, 'The air conditioning in the apartment is not working.', NULL, NULL),
    ('seed-case-kin-closed', 'KIN', 'seed-host-kin-river', 'seed-res-kin-checkedout', 'GENERAL', '{"guestUserId":"seed-guest-kin-4","openedBy":"GUEST","openedByUserId":"seed-guest-kin-4"}', 'CLOSED', '2026-10-30T18:00:00Z', '"The deposit question was answered by phone."', 'Question about the cleaning fee.', NULL, '2026-10-28T09:00:00Z'),
    ('seed-claim-kin-open', 'KIN', 'seed-host-kin-river', 'seed-res-kin-checkedout', 'DAMAGE_CLAIM', '{"guestUserId":"seed-guest-kin-4","openedBy":"OPERATOR","openedByUserId":"seed-user-kin-host"}', 'OPEN', '2026-12-05T18:00:00Z', NULL, 'The balcony door handle was broken.', 45000.00, NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_claim_statements (id, "caseId", party, body, "recordedByUserId", "createdAt")
VALUES
    ('seed-claim-statement-kin-host', 'seed-claim-kin-open', 'HOST', 'The handle was intact before the stay and the door was locked for the whole month.', 'seed-dev-admin', '2026-11-20T10:00:00Z')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_messages (id, city_id, "reservationId", "senderRole", "senderUserId", body, "createdAt")
VALUES
    ('seed-msg-kin-confirmed-1', 'KIN', 'seed-res-kin-confirmed', 'GUEST', 'seed-guest-kin-1', 'Is there a ground floor apartment available for the 10th?', '2026-11-22T08:00:00Z'),
    ('seed-msg-kin-confirmed-2', 'KIN', 'seed-res-kin-confirmed', 'OPERATOR', 'seed-user-kin-host', 'Yes, the apartment is on the ground floor with a ramp at the entrance.', '2026-11-22T09:10:00Z')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_operations_tasks (id, city_id, "propertyId", "physicalUnitId", "reservationItemId", "taskType", "assigneeUserId", state, priority, checklist, "completedAt", "createdAt")
VALUES
    ('seed-task-kin-checkin', 'KIN', 'seed-prop-kin-gombe', NULL, 'seed-item-kin-confirmed', 'CHECK_IN', NULL, 'OPEN', 'HIGH', '["Confirm ID","Hand over keys","Show the ground floor access"]', NULL, '2026-11-25T08:00:00Z'),
    ('seed-task-kin-cleaning', 'KIN', 'seed-prop-kin-gombe', 'seed-physical-kin-apartment', NULL, 'HOUSEKEEPING', 'seed-user-kin-host', 'IN_PROGRESS', 'NORMAL', '["Change linen","Restock water"]', NULL, '2026-11-25T09:00:00Z'),
    ('seed-task-kin-maintenance', 'KIN', 'seed-prop-kin-gombe', 'seed-physical-kin-apartment', NULL, 'MAINTENANCE', NULL, 'OPEN', 'URGENT', '["Repair the balcony door handle"]', NULL, '2026-11-24T07:30:00Z'),
    ('seed-task-kin-service', 'KIN', 'seed-prop-kin-gombe', NULL, 'seed-item-kin-checkedin', 'SERVICE_REQUEST', NULL, 'DONE', 'LOW', '["Bring an extra blanket"]', '2026-11-25T18:00:00Z', '2026-11-25T12:00:00Z'),
    ('seed-task-kin-checkout', 'KIN', 'seed-prop-kin-gombe', NULL, 'seed-item-kin-checkedout', 'CHECK_OUT', NULL, 'CANCELLED', 'NORMAL', '[]', '2026-10-22T10:00:00Z', '2026-10-20T10:00:00Z')
ON CONFLICT (id) DO NOTHING;
