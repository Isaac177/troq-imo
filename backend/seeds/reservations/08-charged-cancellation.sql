INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, source, "bookingMode", currency, "totalAmount", "contractSnapshot", "approvalDeadlineAt", "cancelledAt", "cancellationReason", "cancellationChargeAmount", "specialRequest", "specialRequestState", "commissionRateBps", "commissionAmount", version)
SELECT
    'seed-res-goma-charged',
    'STY-GOMA-CHARGED',
    r.city_id,
    r."propertyId",
    r."operatorId",
    r."guestUserId",
    'CONFIRMED',
    r.source,
    r."bookingMode",
    r.currency,
    r."totalAmount",
    jsonb_set(
        jsonb_set(r."contractSnapshot", '{cancellationDeadline}', to_jsonb('2026-11-20T00:00:00Z'::timestamptz)),
        '{policies,0,content}',
        '{"freeUntilHours": 0, "penaltyPercent": 50}'::jsonb
    ),
    NULL,
    NULL,
    NULL,
    0,
    NULL,
    'NONE',
    r."commissionRateBps",
    r."commissionAmount",
    1
FROM stay_reservations r
WHERE r.id = 'seed-res-goma-confirmed'
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
SELECT
    'seed-item-goma-charged',
    'seed-res-goma-charged',
    i."propertyId",
    i."unitTypeId",
    i."ratePlanId",
    i.quantity,
    i."adultCount",
    i."childCount",
    i."checkIn",
    i."checkOut",
    i."nightlyCharges",
    i."subtotalAmount"
FROM stay_reservation_items i
WHERE i.id = 'seed-item-goma-confirmed'
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_guests (id, "reservationItemId", "fullName", "contactPhone", "contactEmail", role, "consentBasis", "retainUntil", "anonymizedAt")
SELECT
    'seed-guest-goma-charged-primary',
    'seed-item-goma-charged',
    g."fullName",
    g."contactPhone",
    g."contactEmail",
    g.role,
    g."consentBasis",
    g."retainUntil",
    g."anonymizedAt"
FROM stay_reservation_guests g
WHERE g.id = 'seed-guest-goma-confirmed-primary'
ON CONFLICT (id) DO NOTHING;
