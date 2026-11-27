INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, source, "bookingMode", currency, "totalAmount", "contractSnapshot")
SELECT 'seed-rev-res-goma-anonymous', 'STY-GOMA-RV09', city_id, "propertyId", "operatorId", "guestUserId", state, source, "bookingMode", currency, "totalAmount", "contractSnapshot"
FROM stay_reservations WHERE id = 'seed-rev-res-goma-approved-open'
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
SELECT 'seed-rev-item-goma-anonymous', 'seed-rev-res-goma-anonymous', "propertyId", "unitTypeId", "ratePlanId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount"
FROM stay_reservation_items WHERE id = 'seed-rev-item-goma-approved-open'
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_guests (id, "reservationItemId", "fullName", role, "consentBasis", "retainUntil", "anonymizedAt")
VALUES ('seed-rev-guest-goma-anonymous', 'seed-rev-item-goma-anonymous', 'Anonymous', 'PRIMARY', 'CONTRACT', '2026-01-01', now())
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reviews (id, city_id, "reservationId", "propertyId", "operatorId", "guestUserId", rating, comment, status)
SELECT 'seed-review-goma-anonymous', city_id, 'seed-rev-res-goma-anonymous', "propertyId", "operatorId", "guestUserId", rating, 'A quiet and comfortable stay.', 'APPROVED'
FROM stay_reviews WHERE id = 'seed-review-goma-approved-open'
ON CONFLICT (id) DO NOTHING;
