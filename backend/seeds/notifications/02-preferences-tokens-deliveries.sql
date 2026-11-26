INSERT INTO notification_event_preferences (user_id, event_key, in_app, push, email)
VALUES
    ('seed-user-goma-guest', 'category:stays', true, true, true),
    ('seed-user-goma-guest', 'stay.message.guest', true, false, false),
    ('seed-user-goma-host', 'stay.reservation_held.host', true, true, true),
    ('seed-user-goma-host', 'stay.review_submitted.host', true, false, false),
    ('seed-user-goma-admin', '*', true, false, false),
    ('seed-user-kin-host', 'stay.property_suspended.host', false, false, false)
ON CONFLICT (user_id, event_key) DO NOTHING;

INSERT INTO fcm_tokens (id, user_id, device_id, token, platform, app_version, is_active, last_used_at, invalidated_at)
VALUES
    ('seed-fcm-goma-guest', 'seed-user-goma-guest', 'seed-device-android-1', 'seed-token-android-goma-guest', 'ANDROID', '1.0.0', true, now() - interval '2 hours', NULL),
    ('seed-fcm-goma-host', 'seed-user-goma-host', 'seed-device-ios-1', 'seed-token-ios-goma-host', 'IOS', '1.0.0', true, now() - interval '1 hour', NULL),
    ('seed-fcm-kin-host-dead', 'seed-user-kin-host', 'seed-device-android-2', 'seed-token-android-kin-host', 'ANDROID', '0.9.0', false, now() - interval '30 days', now() - interval '20 days')
ON CONFLICT (id) DO NOTHING;

INSERT INTO notification_delivery_attempts (id, notification_id, platform, device_token, status, error_message, fcm_response, attempted_at, succeeded_at)
VALUES
    ('seed-attempt-01', 'seed-notif-goma-01', 'IOS', 'seed-token-ios-goma-host', 'SUCCESS', NULL, '{"name": "projects/seed/messages/1"}'::jsonb, now() - interval '35 minutes', now() - interval '35 minutes'),
    ('seed-attempt-02', 'seed-notif-kin-02', 'ANDROID', 'seed-token-android-kin-host', 'FAILED', 'UNREGISTERED', NULL, now() - interval '20 days', NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO notification_deliveries (id, city_id, dedupe_key, channel, user_id, notification_id, payload, state, attempts, last_error, next_attempt_at, done_at)
VALUES
    ('seed-delivery-push-done', 'GOMA', 'seed-event-01:PUSH:seed-user-goma-host', 'PUSH', 'seed-user-goma-host', 'seed-notif-goma-01', '{"titleEn": "Reservation to confirm", "titleFr": "Réservation à confirmer", "bodyEn": "Amani Kasereka requested Lake Kivu Guesthouse. Please respond.", "bodyFr": "Amani Kasereka a demandé Lake Kivu Guesthouse. Merci de répondre.", "audience": "host", "deepLink": "/reservations?reservation=seed-reservation-goma-1"}'::jsonb, 'DONE', 1, NULL, now() - interval '35 minutes', now() - interval '34 minutes'),
    ('seed-delivery-email-pending', 'GOMA', 'seed-event-02:EMAIL:seed-user-goma-guest', 'EMAIL', 'seed-user-goma-guest', 'seed-notif-goma-03', '{"titleEn": "Reservation accepted", "titleFr": "Réservation acceptée", "bodyEn": "Your stay at Lake Kivu Guesthouse was accepted.", "bodyFr": "Votre séjour à Maison d''hôtes du lac Kivu a été accepté.", "audience": "guest", "deepLink": "/trips?reservation=seed-reservation-goma-1"}'::jsonb, 'PENDING', 0, NULL, now(), NULL),
    ('seed-delivery-email-retry', 'GOMA', 'seed-event-03:EMAIL:seed-user-goma-guest', 'EMAIL', 'seed-user-goma-guest', 'seed-notif-goma-04', '{"titleEn": "Reservation cancelled", "titleFr": "Réservation annulée", "bodyEn": "Your stay was cancelled.", "bodyFr": "Votre séjour a été annulé.", "audience": "guest", "deepLink": "/trips?reservation=seed-reservation-goma-1"}'::jsonb, 'PENDING', 2, 'INTERNAL_ERROR', now() + interval '2 minutes', NULL),
    ('seed-delivery-email-failed', 'KIN', 'seed-event-04:EMAIL:seed-user-kin-host', 'EMAIL', 'seed-user-kin-host', 'seed-notif-kin-02', '{"titleEn": "Reservation to confirm", "titleFr": "Réservation à confirmer", "bodyEn": "A guest requested a stay.", "bodyFr": "Un voyageur a demandé un séjour.", "audience": "host", "deepLink": "/reservations?reservation=seed-reservation-kin-1"}'::jsonb, 'FAILED', 5, 'INTERNAL_ERROR', now() - interval '1 day', NULL),
    ('seed-delivery-email-skipped', 'GOMA', 'seed-event-05:EMAIL:seed-user-goma-admin', 'EMAIL', 'seed-user-goma-admin', 'seed-notif-goma-18', '{"titleEn": "New property", "titleFr": "Nouvelle propriété", "bodyEn": "A property was submitted.", "bodyFr": "Une propriété a été soumise.", "audience": "admin", "deepLink": "/audit"}'::jsonb, 'SKIPPED', 0, 'no contact', now() - interval '3 hours', now() - interval '3 hours'),
    ('seed-delivery-document-voucher', 'GOMA', 'seed-event-06:DOCUMENT:stay_voucher', 'DOCUMENT', NULL, NULL, '{"document": "stay_voucher", "snapshot": {"reservationId": "seed-reservation-goma-1", "reference": "STY-2026-000042", "state": "CONFIRMED", "hostId": "seed-host-goma-lake", "guestUserId": "seed-user-goma-guest", "currency": "CDF", "totalAmount": "450000.00", "checkIn": "2026-11-02", "checkOut": "2026-11-05", "propertyName": {"en": "Lake Kivu Guesthouse", "fr": "Maison d''hôtes du lac Kivu"}, "propertyAddress": "12 avenue de la Paix, Goma", "guest": {"fullName": "Amani Kasereka", "contactEmail": "amani@example.com", "contactPhone": "+243811111111"}, "language": "en"}}'::jsonb, 'DONE', 1, NULL, now() - interval '1 day', now() - interval '1 hour'),
    ('seed-delivery-document-cancellation', 'GOMA', 'seed-event-07:DOCUMENT:stay_cancellation', 'DOCUMENT', NULL, NULL, '{"document": "stay_cancellation", "snapshot": {"reservationId": "seed-reservation-goma-2", "reference": "STY-2026-000043", "state": "CANCELLED", "hostId": "seed-host-goma-lake", "guestUserId": "seed-user-goma-guest", "currency": "CDF", "totalAmount": "300000.00", "chargeDueAtProperty": "50000.00", "checkIn": "2026-12-02", "checkOut": "2026-12-04", "propertyName": {"en": "Lake Kivu Guesthouse", "fr": "Maison d''hôtes du lac Kivu"}, "propertyAddress": "12 avenue de la Paix, Goma", "guest": {"fullName": "Amani Kasereka", "contactEmail": null, "contactPhone": "+243811111111"}, "language": "fr"}}'::jsonb, 'DONE', 1, NULL, now() - interval '1 day', now() - interval '2 hours'),
    ('seed-delivery-document-damage', 'KIN', 'seed-event-08:DOCUMENT:stay_damage_claim', 'DOCUMENT', NULL, NULL, '{"document": "stay_damage_claim", "snapshot": {"reservationId": "seed-reservation-kin-1", "caseId": "seed-case-kin-1", "reference": "STY-2026-000044", "state": "CHECKED_OUT", "hostId": "seed-host-kin-river", "guestUserId": "seed-user-kin-guest", "currency": "CDF", "claimAmount": "25000.00", "claimReason": "Broken window in the living room.", "disputeDeadline": "2026-12-12T10:00:00+00:00", "checkOut": "2026-11-05", "propertyName": {"en": "River City Rooms", "fr": "Chambres River City"}, "propertyAddress": "4 avenue du Fleuve, Kinshasa", "guest": {"fullName": "Joel Mbala", "contactEmail": "joel@example.com", "contactPhone": null}, "language": "en"}}'::jsonb, 'FAILED', 5, 'INTERNAL_ERROR', now() - interval '1 day', NULL)
ON CONFLICT (id) DO NOTHING;
