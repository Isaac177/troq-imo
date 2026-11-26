INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, source, "bookingMode", currency, "totalAmount", "contractSnapshot") VALUES
    ('seed-rev-res-kin-approved-replied', 'STY-KIN-RV01', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-1', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 410000.00, '{"cancellation":"seed-policy-cancel-flex-v1","payment":"seed-policy-pay-prepay-v1"}'),
    ('seed-rev-res-kin-approved-open', 'STY-KIN-RV02', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-2', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 410000.00, '{"cancellation":"seed-policy-cancel-flex-v1","payment":"seed-policy-pay-prepay-v1"}'),
    ('seed-rev-res-kin-pending', 'STY-KIN-RV03', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-3', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 410000.00, '{"cancellation":"seed-policy-cancel-flex-v1","payment":"seed-policy-pay-prepay-v1"}'),
    ('seed-rev-res-kin-hidden', 'STY-KIN-RV04', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-4', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 410000.00, '{"cancellation":"seed-policy-cancel-flex-v1","payment":"seed-policy-pay-prepay-v1"}'),
    ('seed-rev-res-kin-awaiting-review', 'STY-KIN-RV05', 'KIN', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-5', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 410000.00, '{"cancellation":"seed-policy-cancel-flex-v1","payment":"seed-policy-pay-prepay-v1"}')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", "physicalUnitId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount") VALUES
    ('seed-rev-item-kin-approved-replied', 'seed-rev-res-kin-approved-replied', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', NULL, 1, 2, 0, '2026-09-05', '2026-09-06', '[{"date":"2026-09-05","amount":"410000.00"}]', 410000.00),
    ('seed-rev-item-kin-approved-open', 'seed-rev-res-kin-approved-open', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', NULL, 1, 2, 0, '2026-09-12', '2026-09-13', '[{"date":"2026-09-12","amount":"410000.00"}]', 410000.00),
    ('seed-rev-item-kin-pending', 'seed-rev-res-kin-pending', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', NULL, 1, 2, 0, '2026-09-19', '2026-09-20', '[{"date":"2026-09-19","amount":"410000.00"}]', 410000.00),
    ('seed-rev-item-kin-hidden', 'seed-rev-res-kin-hidden', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', NULL, 1, 2, 0, '2026-09-26', '2026-09-27', '[{"date":"2026-09-26","amount":"410000.00"}]', 410000.00),
    ('seed-rev-item-kin-awaiting-review', 'seed-rev-res-kin-awaiting-review', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'seed-rate-kin-apartment-instant', NULL, 1, 2, 0, '2026-10-08', '2026-10-09', '[{"date":"2026-10-08","amount":"410000.00"}]', 410000.00)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_guests (id, "reservationItemId", "fullName", "contactPhone", "contactEmail", role, "consentBasis", "retainUntil") VALUES
    ('seed-rev-guest-kin-approved-replied', 'seed-rev-item-kin-approved-replied', 'Marc Ilunga', '+243820000001', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-rev-guest-kin-approved-open', 'seed-rev-item-kin-approved-open', 'Sandrine Mafuta', '+243820000002', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-rev-guest-kin-pending', 'seed-rev-item-kin-pending', 'Olivier Kabongo', '+243820000003', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-rev-guest-kin-hidden', 'seed-rev-item-kin-hidden', 'Gloria Tshimanga', '+243820000004', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31'),
    ('seed-rev-guest-kin-awaiting-review', 'seed-rev-item-kin-awaiting-review', 'Paul Mukendi', '+243820000005', NULL, 'PRIMARY', 'CONTRACT', '2027-12-31')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reviews (id, city_id, "reservationId", "propertyId", "operatorId", "guestUserId", rating, comment, status, "operatorReply", "operatorRepliedAt", "moderatedBy", "moderatedAt", "createdAt", "updatedAt") VALUES
    ('seed-review-kin-approved-replied', 'KIN', 'seed-rev-res-kin-approved-replied', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-1', 5, 'Apartment right on the boulevard, quiet at night and very clean.', 'APPROVED', 'Merci Marc, a bientot a Gombe.', '2026-09-09T08:30:00Z', 'seed-platform-reviewer', '2026-09-08T10:00:00Z', '2026-09-08T09:00:00Z', '2026-09-08T10:00:00Z'),
    ('seed-review-kin-approved-open', 'KIN', 'seed-rev-res-kin-approved-open', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-2', 4, 'Bon emplacement, le wifi etait lent le soir.', 'APPROVED', NULL, NULL, 'seed-platform-reviewer', '2026-09-16T10:00:00Z', '2026-09-16T09:00:00Z', '2026-09-16T10:00:00Z'),
    ('seed-review-kin-pending', 'KIN', 'seed-rev-res-kin-pending', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-3', 2, 'The air conditioning was noisy and the hot water ran out.', 'PENDING', NULL, NULL, NULL, NULL, '2026-09-22T09:00:00Z', '2026-09-22T09:00:00Z'),
    ('seed-review-kin-hidden', 'KIN', 'seed-rev-res-kin-hidden', 'seed-prop-kin-gombe', 'seed-host-kin-river', 'seed-guest-kin-4', 1, 'Contains a private phone number: +243 000 000 000.', 'HIDDEN', NULL, NULL, 'seed-platform-reviewer', '2026-09-30T10:00:00Z', '2026-09-29T09:00:00Z', '2026-09-30T10:00:00Z')
ON CONFLICT (id) DO NOTHING;
