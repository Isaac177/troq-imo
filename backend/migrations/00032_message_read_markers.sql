CREATE TABLE stay_message_host_reads (
    reservation_id text NOT NULL REFERENCES stay_reservations(id) ON DELETE CASCADE,
    user_id text NOT NULL REFERENCES identity_users(id) ON DELETE CASCADE,
    message_id text NOT NULL REFERENCES stay_reservation_messages(id) ON DELETE CASCADE,
    message_created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (reservation_id, user_id)
);

CREATE INDEX stay_reservation_messages_guest_unread_idx
    ON stay_reservation_messages ("reservationId", "createdAt", id)
    WHERE "senderRole" = 'GUEST';
