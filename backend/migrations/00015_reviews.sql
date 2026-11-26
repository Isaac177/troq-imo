CREATE TABLE stay_reviews (
    id text NOT NULL,
    city_id text NOT NULL,
    "reservationId" text NOT NULL,
    "propertyId" text NOT NULL,
    "operatorId" text NOT NULL,
    "guestUserId" text NOT NULL,
    rating smallint NOT NULL,
    comment character varying(2000),
    status character varying(16) NOT NULL DEFAULT 'PENDING',
    "operatorReply" character varying(2000),
    "operatorRepliedAt" timestamp with time zone,
    "moderatedBy" text,
    "moderatedAt" timestamp with time zone,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_reviews_pkey PRIMARY KEY (id),
    CONSTRAINT stay_reviews_reservation_unique UNIQUE ("reservationId"),
    CONSTRAINT stay_reviews_reservation_fkey FOREIGN KEY ("reservationId") REFERENCES stay_reservations (id) ON DELETE CASCADE,
    CONSTRAINT stay_reviews_operator_fkey FOREIGN KEY ("operatorId") REFERENCES hosts (id),
    CONSTRAINT stay_reviews_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_reviews_rating_check CHECK (rating BETWEEN 1 AND 5),
    CONSTRAINT stay_reviews_status_check CHECK (status IN ('PENDING', 'APPROVED', 'HIDDEN')),
    CONSTRAINT stay_reviews_comment_check CHECK (comment IS NULL OR char_length(comment) <= 2000),
    CONSTRAINT stay_reviews_reply_check CHECK ("operatorReply" IS NULL OR char_length("operatorReply") <= 2000)
);

CREATE INDEX stay_reviews_property_status_idx ON stay_reviews (city_id, "propertyId", status);
CREATE INDEX stay_reviews_status_created_idx ON stay_reviews (city_id, status, "createdAt");
