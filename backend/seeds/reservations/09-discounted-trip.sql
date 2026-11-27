INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, source, "bookingMode", currency, "totalAmount", "contractSnapshot", "specialRequestState", "commissionRateBps", "commissionAmount", version)
SELECT 'seed-res-goma-discounted', 'STY-GOMA-DISCOUNT', r.city_id, r."propertyId", r."operatorId", 'seed-user-goma-invitee', 'CHECKED_OUT', r.source, r."bookingMode", r.currency, 575000.00,
    jsonb_set(jsonb_set(jsonb_set(r."contractSnapshot", '{discount}', '{"promotionId":"seed-promo-goma-villa-limited","amount":"40000.00"}'::jsonb), '{commission}', '{"rateBps":0,"amount":"0.00","net":"575000.00"}'::jsonb), '{cancellationDeadline}', to_jsonb('2026-11-08T23:00:00Z'::timestamptz)),
    'NONE', 0, 0, 1
FROM stay_reservations r WHERE r.id = 'seed-res-goma-confirmed'
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
SELECT 'seed-item-goma-discounted', 'seed-res-goma-discounted', i."propertyId", i."unitTypeId", i."ratePlanId", 1, 2, 0, '2026-11-10', '2026-11-13',
    '[{"date":"2026-11-10","amount":"205000.00"},{"date":"2026-11-11","amount":"205000.00"},{"date":"2026-11-12","amount":"205000.00"}]'::jsonb, 615000.00
FROM stay_reservation_items i WHERE i.id = 'seed-item-goma-confirmed'
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_guests (id, "reservationItemId", "fullName", "contactPhone", "contactEmail", role, "consentBasis", "retainUntil")
VALUES ('seed-guest-goma-discounted-primary', 'seed-item-goma-discounted', 'Amani Kasereka', '+243810000042', 'amani.kasereka@example.test', 'PRIMARY', 'CONTRACT', '2028-02-04')
ON CONFLICT (id) DO NOTHING;
