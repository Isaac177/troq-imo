CREATE TABLE stay_reservations (
    id text NOT NULL,
    reference character varying(24) NOT NULL,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "operatorId" text NOT NULL,
    "guestUserId" text NOT NULL,
    state character varying(24) NOT NULL,
    source character varying(16) NOT NULL DEFAULT 'DIRECT',
    "bookingMode" character varying(16) NOT NULL,
    "quoteId" text,
    currency character varying(3) NOT NULL DEFAULT 'CDF',
    "totalAmount" numeric(14,2) NOT NULL,
    "contractSnapshot" jsonb NOT NULL,
    "paymentRef" text,
    "approvalDeadlineAt" timestamp with time zone,
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT now(),
    "cancelledAt" timestamp with time zone,
    "cancellationReason" character varying(32),
    "cancellationChargeAmount" numeric(12,2) NOT NULL DEFAULT 0,
    "specialRequest" text,
    "specialRequestState" character varying(16) NOT NULL DEFAULT 'NONE',
    "commissionRateBps" integer NOT NULL DEFAULT 0,
    "commissionAmount" numeric(14,2) NOT NULL DEFAULT 0,
    "operatorNetAmount" numeric(14,2),
    "commissionChargedAmount" numeric(14,2) NOT NULL DEFAULT 0,
    CONSTRAINT stay_reservations_pkey PRIMARY KEY (id),
    CONSTRAINT stay_reservations_id_property_unique UNIQUE (id, "propertyId"),
    CONSTRAINT stay_reservations_reference_unique UNIQUE (reference),
    CONSTRAINT stay_reservations_operator_fkey FOREIGN KEY ("operatorId") REFERENCES hosts (id),
    CONSTRAINT stay_reservations_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_reservations_state_check CHECK (state IN (
        'PENDING_PAYMENT', 'REQUESTED', 'CONFIRMED', 'DECLINED', 'CANCELLED',
        'EXPIRED', 'CHECKED_IN', 'CHECKED_OUT', 'NO_SHOW'
    )),
    CONSTRAINT stay_reservations_source_check CHECK (source IN ('DIRECT', 'CHANNEL', 'CORPORATE', 'AFFILIATE')),
    CONSTRAINT stay_reservations_booking_mode_check CHECK ("bookingMode" IN ('INSTANT', 'REQUEST')),
    CONSTRAINT stay_reservations_currency_check CHECK (currency = 'CDF'),
    CONSTRAINT stay_reservations_total_check CHECK ("totalAmount" >= 0),
    CONSTRAINT stay_reservations_approval_check CHECK ("approvalDeadlineAt" IS NULL OR "bookingMode" = 'REQUEST'),
    CONSTRAINT stay_reservations_version_check CHECK (version >= 1),
    CONSTRAINT stay_reservations_cancellation_charge_check CHECK ("cancellationChargeAmount" >= 0),
    CONSTRAINT stay_reservations_cancelled_at_check CHECK (state <> 'CANCELLED' OR "cancelledAt" IS NOT NULL),
    CONSTRAINT stay_reservations_special_request_state_check CHECK ("specialRequestState" IN ('NONE', 'REQUESTED', 'ACCEPTED', 'DECLINED')),
    CONSTRAINT stay_reservations_special_request_presence_check CHECK (("specialRequest" IS NULL) = ("specialRequestState" = 'NONE')),
    CONSTRAINT stay_reservations_special_request_length_check CHECK ("specialRequest" IS NULL OR char_length("specialRequest") <= 500),
    CONSTRAINT stay_reservations_commission_rate_check CHECK ("commissionRateBps" BETWEEN 0 AND 10000),
    CONSTRAINT stay_reservations_commission_amount_check CHECK ("commissionAmount" >= 0 AND "commissionAmount" <= "totalAmount"),
    CONSTRAINT stay_reservations_net_amount_check CHECK ("operatorNetAmount" = "totalAmount" - "commissionAmount"),
    CONSTRAINT stay_reservations_commission_charged_check CHECK ("commissionChargedAmount" >= 0)
);

CREATE INDEX stay_reservations_city_property_state_idx ON stay_reservations (city_id, "propertyId", state);
CREATE INDEX stay_reservations_guest_idx ON stay_reservations ("guestUserId", "createdAt" DESC);
CREATE INDEX stay_reservations_operator_idx ON stay_reservations ("operatorId", state);
CREATE INDEX stay_reservations_pending_deadline_idx ON stay_reservations ("approvalDeadlineAt") WHERE state = 'REQUESTED';
CREATE UNIQUE INDEX stay_reservations_quote_unique_idx ON stay_reservations ("quoteId") WHERE "quoteId" IS NOT NULL;

CREATE FUNCTION stay_reservation_net_default() RETURNS trigger AS $$
BEGIN
    IF NEW."operatorNetAmount" IS NULL THEN
        NEW."operatorNetAmount" := NEW."totalAmount" - NEW."commissionAmount";
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER stay_reservation_net_default_trigger
    BEFORE INSERT ON stay_reservations
    FOR EACH ROW EXECUTE FUNCTION stay_reservation_net_default();

CREATE TABLE stay_reservation_items (
    id text NOT NULL,
    "reservationId" text NOT NULL,
    "propertyId" text NOT NULL,
    "unitTypeId" text NOT NULL,
    "ratePlanId" text NOT NULL,
    "physicalUnitId" text,
    quantity integer NOT NULL DEFAULT 1,
    "adultCount" integer NOT NULL,
    "childCount" integer NOT NULL DEFAULT 0,
    "checkIn" date NOT NULL,
    "checkOut" date NOT NULL,
    "nightlyCharges" jsonb NOT NULL,
    "subtotalAmount" numeric(14,2) NOT NULL,
    CONSTRAINT stay_reservation_items_pkey PRIMARY KEY (id),
    CONSTRAINT stay_reservation_items_reservation_fkey FOREIGN KEY ("reservationId", "propertyId") REFERENCES stay_reservations (id, "propertyId"),
    CONSTRAINT stay_reservation_items_range_check CHECK ("checkOut" > "checkIn"),
    CONSTRAINT stay_reservation_items_quantity_check CHECK (quantity >= 1),
    CONSTRAINT stay_reservation_items_guests_check CHECK ("adultCount" >= 1 AND "childCount" >= 0),
    CONSTRAINT stay_reservation_items_amount_check CHECK ("subtotalAmount" >= 0)
);

CREATE INDEX stay_reservation_items_reservation_idx ON stay_reservation_items ("reservationId");
CREATE INDEX stay_reservation_items_unit_range_idx ON stay_reservation_items ("unitTypeId", "checkIn", "checkOut");

CREATE TABLE stay_reservation_guests (
    id text NOT NULL,
    "reservationItemId" text NOT NULL,
    "fullName" character varying(200) NOT NULL,
    "contactPhone" character varying(32),
    "contactEmail" character varying(254),
    role character varying(16) NOT NULL DEFAULT 'ADDITIONAL',
    "consentBasis" character varying(32) NOT NULL,
    "retainUntil" date NOT NULL,
    "anonymizedAt" timestamp with time zone,
    CONSTRAINT stay_reservation_guests_pkey PRIMARY KEY (id),
    CONSTRAINT stay_reservation_guests_item_fkey FOREIGN KEY ("reservationItemId") REFERENCES stay_reservation_items (id),
    CONSTRAINT stay_reservation_guests_role_check CHECK (role IN ('PRIMARY', 'ADDITIONAL')),
    CONSTRAINT stay_reservation_guests_contact_check CHECK (
        "contactPhone" IS NOT NULL OR "contactEmail" IS NOT NULL OR role = 'ADDITIONAL' OR "anonymizedAt" IS NOT NULL
    )
);

CREATE UNIQUE INDEX stay_reservation_guests_primary_unique_idx ON stay_reservation_guests ("reservationItemId") WHERE role = 'PRIMARY';
CREATE INDEX stay_reservation_guests_retention_idx ON stay_reservation_guests ("retainUntil") WHERE "anonymizedAt" IS NULL;

CREATE TABLE stay_amendments (
    id text NOT NULL,
    city_id text NOT NULL,
    "reservationId" text NOT NULL,
    "originalVersion" integer NOT NULL,
    "proposedContract" jsonb NOT NULL,
    "deltaAmount" numeric(14,2) NOT NULL,
    state character varying(16) NOT NULL DEFAULT 'PROPOSED',
    "expiresAt" timestamp with time zone NOT NULL,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "respondedAt" timestamp with time zone,
    CONSTRAINT stay_amendments_pkey PRIMARY KEY (id),
    CONSTRAINT stay_amendments_reservation_fkey FOREIGN KEY ("reservationId") REFERENCES stay_reservations (id),
    CONSTRAINT stay_amendments_state_check CHECK (state IN ('PROPOSED', 'ACCEPTED', 'DECLINED', 'EXPIRED', 'SUPERSEDED')),
    CONSTRAINT stay_amendments_responded_check CHECK ((state = 'PROPOSED') = ("respondedAt" IS NULL))
);

CREATE UNIQUE INDEX stay_amendments_open_unique_idx ON stay_amendments ("reservationId") WHERE state = 'PROPOSED';

CREATE TABLE stay_records (
    id text NOT NULL,
    "reservationItemId" text NOT NULL,
    "physicalUnitId" text NOT NULL,
    "actualArrivalAt" timestamp with time zone,
    "actualDepartureAt" timestamp with time zone,
    exceptions jsonb NOT NULL DEFAULT '[]'::jsonb,
    "authorizedByUserId" text,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_records_pkey PRIMARY KEY (id),
    CONSTRAINT stay_records_item_fkey FOREIGN KEY ("reservationItemId") REFERENCES stay_reservation_items (id),
    CONSTRAINT stay_records_item_unique UNIQUE ("reservationItemId"),
    CONSTRAINT stay_records_departure_check CHECK ("actualDepartureAt" IS NULL OR "actualArrivalAt" IS NOT NULL)
);

CREATE TABLE stay_reservation_messages (
    id text NOT NULL,
    city_id text NOT NULL,
    "reservationId" text NOT NULL,
    "senderRole" character varying(16) NOT NULL,
    "senderUserId" text NOT NULL,
    body character varying(2000) NOT NULL,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_reservation_messages_pkey PRIMARY KEY (id),
    CONSTRAINT stay_reservation_messages_reservation_fkey FOREIGN KEY ("reservationId") REFERENCES stay_reservations (id) ON DELETE CASCADE,
    CONSTRAINT stay_reservation_messages_role_check CHECK ("senderRole" IN ('GUEST', 'OPERATOR')),
    CONSTRAINT stay_reservation_messages_body_check CHECK (char_length(body) BETWEEN 1 AND 2000)
);

CREATE INDEX stay_reservation_messages_reservation_idx ON stay_reservation_messages ("reservationId", "createdAt");
