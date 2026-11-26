INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, source, "bookingMode", currency, "totalAmount", "contractSnapshot", "approvalDeadlineAt", "cancelledAt", "cancellationReason", "cancellationChargeAmount", "specialRequest", "specialRequestState", "commissionRateBps", "commissionAmount", version)
SELECT seed.id, seed.reference, r.city_id, r."propertyId", r."operatorId", 'seed-user-goma-invitee', r.state, r.source, r."bookingMode", r.currency, r."totalAmount", r."contractSnapshot", r."approvalDeadlineAt", r."cancelledAt", r."cancellationReason", r."cancellationChargeAmount", r."specialRequest", r."specialRequestState", r."commissionRateBps", r."commissionAmount", r.version
FROM (VALUES
    ('seed-res-invitee-confirmed', 'STY-DEMO-1001', 'seed-res-goma-confirmed'),
    ('seed-res-invitee-requested', 'STY-DEMO-1002', 'seed-res-goma-requested'),
    ('seed-res-invitee-checkedin', 'STY-DEMO-1003', 'seed-res-goma-checkedin'),
    ('seed-res-invitee-checkedout', 'STY-DEMO-1004', 'seed-res-goma-checkedout-reviewed'),
    ('seed-res-invitee-cancelled', 'STY-DEMO-1005', 'seed-res-goma-cancelled')
) AS seed(id, reference, source_id)
JOIN stay_reservations r ON r.id = seed.source_id
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
SELECT seed.id, seed.reservation_id, i."propertyId", i."unitTypeId", i."ratePlanId", i.quantity, i."adultCount", i."childCount", i."checkIn", i."checkOut", i."nightlyCharges", i."subtotalAmount"
FROM (VALUES
    ('seed-item-invitee-confirmed', 'seed-res-invitee-confirmed', 'seed-item-goma-confirmed'),
    ('seed-item-invitee-requested', 'seed-res-invitee-requested', 'seed-item-goma-requested'),
    ('seed-item-invitee-checkedin', 'seed-res-invitee-checkedin', 'seed-item-goma-checkedin'),
    ('seed-item-invitee-checkedout', 'seed-res-invitee-checkedout', 'seed-item-goma-checkedout-reviewed'),
    ('seed-item-invitee-cancelled', 'seed-res-invitee-cancelled', 'seed-item-goma-cancelled')
) AS seed(id, reservation_id, source_id)
JOIN stay_reservation_items i ON i.id = seed.source_id
ON CONFLICT (id) DO NOTHING;
