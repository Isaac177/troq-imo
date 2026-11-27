INSERT INTO stay_message_host_reads (reservation_id, user_id, message_id, message_created_at)
SELECT m."reservationId", 'seed-user-goma-host', m.id, m."createdAt"
FROM stay_reservation_messages m
WHERE m.id = 'seed-msg-goma-checkedout-2'
ON CONFLICT (reservation_id, user_id) DO NOTHING;
