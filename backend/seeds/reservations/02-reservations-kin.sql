INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, source, "bookingMode", currency, "totalAmount", "contractSnapshot", "approvalDeadlineAt", "cancelledAt", "cancellationReason", "cancellationChargeAmount", "specialRequest", "specialRequestState", "commissionRateBps", "commissionAmount", version)
VALUES
    ('seed-res-kin-confirmed', 'STY-KIN-0001', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-1', 'CONFIRMED', 'DIRECT', 'INSTANT', 'CDF', 300000.00,
     '{"policies":[{"id":"seed-policy-cancel-flex-v1","kind":"CANCELLATION","content":{"freeUntilHours":48,"penaltyPercent":0}}],"cancellationDeadline":"2026-12-08T00:00:00Z"}',
     NULL, NULL, NULL, 0, 'Ground floor if possible, one guest uses a wheelchair.', 'REQUESTED', 1000, 30000.00, 1),
    ('seed-res-kin-requested', 'STY-KIN-0002', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-2', 'REQUESTED', 'DIRECT', 'REQUEST', 'CDF', 150000.00,
     '{"policies":[{"id":"seed-policy-cancel-flex-v1","kind":"CANCELLATION","content":{"freeUntilHours":48,"penaltyPercent":0}}],"cancellationDeadline":"2027-02-10T00:00:00Z"}',
     '2027-01-25T18:00:00Z', NULL, NULL, 0, NULL, 'NONE', 1000, 15000.00, 1),
    ('seed-res-kin-checkedin', 'STY-KIN-0003', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-3', 'CHECKED_IN', 'DIRECT', 'INSTANT', 'CDF', 450000.00,
     '{"policies":[{"id":"seed-policy-cancel-flex-v1","kind":"CANCELLATION","content":{"freeUntilHours":48,"penaltyPercent":0}}],"cancellationDeadline":"2026-11-20T00:00:00Z"}',
     NULL, NULL, NULL, 0, NULL, 'NONE', 1000, 45000.00, 3),
    ('seed-res-kin-checkedout', 'STY-KIN-0004', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-4', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 300000.00,
     '{"policies":[{"id":"seed-policy-cancel-flex-v1","kind":"CANCELLATION","content":{"freeUntilHours":48,"penaltyPercent":0}}],"cancellationDeadline":"2026-10-18T00:00:00Z"}',
     NULL, NULL, NULL, 0, NULL, 'NONE', 1000, 30000.00, 4),
    ('seed-res-kin-cancelled', 'STY-KIN-0005', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-5', 'CANCELLED', 'DIRECT', 'INSTANT', 'CDF', 300000.00,
     '{"policies":[{"id":"seed-policy-cancel-flex-v1","kind":"CANCELLATION","content":{"freeUntilHours":48,"penaltyPercent":0}}],"cancellationDeadline":"2026-11-08T00:00:00Z"}',
     NULL, '2026-11-03T10:00:00Z', 'OPERATOR_CANCELLED', 0, NULL, 'NONE', 1000, 30000.00, 2),
    ('seed-res-kin-declined', 'STY-KIN-0006', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-2', 'DECLINED', 'DIRECT', 'REQUEST', 'CDF', 150000.00,
     '{"policies":[{"id":"seed-policy-cancel-flex-v1","kind":"CANCELLATION","content":{"freeUntilHours":48,"penaltyPercent":0}}],"cancellationDeadline":"2026-12-01T00:00:00Z"}',
     NULL, NULL, NULL, 0, NULL, 'NONE', 1000, 15000.00, 2),
    ('seed-res-kin-expired', 'STY-KIN-0007', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-3', 'EXPIRED', 'DIRECT', 'REQUEST', 'CDF', 150000.00,
     '{"policies":[{"id":"seed-policy-cancel-flex-v1","kind":"CANCELLATION","content":{"freeUntilHours":48,"penaltyPercent":0}}],"cancellationDeadline":"2026-11-10T00:00:00Z"}',
     NULL, NULL, NULL, 0, NULL, 'NONE', 1000, 15000.00, 2),
    ('seed-res-kin-noshow', 'STY-KIN-0008', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-4', 'NO_SHOW', 'DIRECT', 'INSTANT', 'CDF', 300000.00,
     '{"policies":[{"id":"seed-policy-cancel-moderate-v1","kind":"CANCELLATION","content":{"freeUntilHours":72,"penaltyPercent":50}}],"cancellationDeadline":"2026-11-05T00:00:00Z"}',
     NULL, NULL, NULL, 150000.00, NULL, 'NONE', 1000, 30000.00, 2),
    ('seed-res-kin-pendingpay', 'STY-KIN-0009', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-5', 'PENDING_PAYMENT', 'DIRECT', 'INSTANT', 'CDF', 300000.00,
     '{"policies":[{"id":"seed-policy-cancel-flex-v1","kind":"CANCELLATION","content":{"freeUntilHours":48,"penaltyPercent":0}}],"cancellationDeadline":"2027-02-20T00:00:00Z"}',
     NULL, NULL, NULL, 0, NULL, 'NONE', 1000, 30000.00, 1)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
VALUES
    ('seed-item-kin-confirmed', 'seed-res-kin-confirmed', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 3, 1, '2026-12-10', '2026-12-12', '[{"date":"2026-12-10","amount":"150000.00"},{"date":"2026-12-11","amount":"150000.00"}]', 300000.00),
    ('seed-item-kin-requested', 'seed-res-kin-requested', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 2, 0, '2027-02-12', '2027-02-13', '[{"date":"2027-02-12","amount":"150000.00"}]', 150000.00),
    ('seed-item-kin-checkedin', 'seed-res-kin-checkedin', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 2, 0, '2026-11-24', '2026-11-27', '[{"date":"2026-11-24","amount":"150000.00"},{"date":"2026-11-25","amount":"150000.00"},{"date":"2026-11-26","amount":"150000.00"}]', 450000.00),
    ('seed-item-kin-checkedout', 'seed-res-kin-checkedout', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 2, 0, '2026-10-20', '2026-10-22', '[{"date":"2026-10-20","amount":"150000.00"},{"date":"2026-10-21","amount":"150000.00"}]', 300000.00),
    ('seed-item-kin-cancelled', 'seed-res-kin-cancelled', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 2, 0, '2026-11-10', '2026-11-12', '[{"date":"2026-11-10","amount":"150000.00"},{"date":"2026-11-11","amount":"150000.00"}]', 300000.00),
    ('seed-item-kin-declined', 'seed-res-kin-declined', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 2, 0, '2026-12-03', '2026-12-04', '[{"date":"2026-12-03","amount":"150000.00"}]', 150000.00),
    ('seed-item-kin-expired', 'seed-res-kin-expired', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 1, 0, '2026-11-12', '2026-11-13', '[{"date":"2026-11-12","amount":"150000.00"}]', 150000.00),
    ('seed-item-kin-noshow', 'seed-res-kin-noshow', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 2, 0, '2026-11-07', '2026-11-09', '[{"date":"2026-11-07","amount":"150000.00"},{"date":"2026-11-08","amount":"150000.00"}]', 300000.00),
    ('seed-item-kin-pendingpay', 'seed-res-kin-pendingpay', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', 1, 2, 1, '2027-02-22', '2027-02-24', '[{"date":"2027-02-22","amount":"150000.00"},{"date":"2027-02-23","amount":"150000.00"}]', 300000.00)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_guests (id, "reservationItemId", "fullName", "contactPhone", "contactEmail", role, "consentBasis", "retainUntil")
VALUES
    ('seed-guest-kin-confirmed-primary', 'seed-item-kin-confirmed', 'Esther Ngalula', '+243990000001', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-guest-kin-requested-primary', 'seed-item-kin-requested', 'Noel Tshibangu', '+243990000002', 'noel.t@example.test', 'PRIMARY', 'CONTRACT', '2028-12-31'),
    ('seed-guest-kin-checkedin-primary', 'seed-item-kin-checkedin', 'Marthe Kasongo', '+243990000003', NULL, 'PRIMARY', 'CONTRACT', '2028-12-31'),
    ('seed-guest-kin-checkedout-primary', 'seed-item-kin-checkedout', 'Joel Mbuyi', '+243990000004', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-guest-kin-cancelled-primary', 'seed-item-kin-cancelled', 'Rachel Kalala', '+243990000005', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-guest-kin-declined-primary', 'seed-item-kin-declined', 'Noel Tshibangu', '+243990000002', 'noel.t@example.test', 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-guest-kin-expired-primary', 'seed-item-kin-expired', 'Marthe Kasongo', '+243990000003', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-guest-kin-noshow-primary', 'seed-item-kin-noshow', 'Joel Mbuyi', '+243990000004', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-guest-kin-pendingpay-primary', 'seed-item-kin-pendingpay', 'Rachel Kalala', '+243990000005', NULL, 'PRIMARY', 'CONTRACT', '2028-12-31')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_records (id, "reservationItemId", "physicalUnitId", "actualArrivalAt", "actualDepartureAt", "authorizedByUserId")
VALUES
    ('seed-record-kin-checkedin', 'seed-item-kin-checkedin', 'seed-physical-kin-apartment', '2026-11-24T15:10:00Z', NULL, 'seed-user-kin-host'),
    ('seed-record-kin-checkedout', 'seed-item-kin-checkedout', 'seed-physical-kin-apartment', '2026-10-20T14:30:00Z', '2026-10-22T09:45:00Z', 'seed-user-kin-host')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_amendments (id, city_id, "reservationId", "originalVersion", "proposedContract", "deltaAmount", state, "expiresAt", "createdAt", "respondedAt")
VALUES
    ('seed-amend-kin-proposed', 'KIN', 'seed-res-kin-confirmed', 1,
     '{"checkIn":"2026-12-10","checkOut":"2026-12-13","nights":[{"date":"2026-12-10","amount":"150000.00"},{"date":"2026-12-11","amount":"150000.00"},{"date":"2026-12-12","amount":"150000.00"}],"total":"450000.00","subtotal":"450000.00","discount":null,"cancellationDeadline":"2026-12-08T00:00:00Z"}',
     150000.00, 'PROPOSED', '2026-12-05T18:00:00Z', '2026-11-25T09:00:00Z', NULL)
ON CONFLICT (id) DO NOTHING;
