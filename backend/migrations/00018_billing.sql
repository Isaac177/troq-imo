CREATE TABLE stay_operator_commission_rates (
    id text NOT NULL,
    city_id text NOT NULL,
    "operatorId" text NOT NULL,
    "rateBps" integer NOT NULL,
    "effectiveFrom" timestamp with time zone NOT NULL DEFAULT now(),
    "reasonCode" character varying(32) NOT NULL,
    note text,
    "changedByUserId" text NOT NULL,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_operator_commission_rates_pkey PRIMARY KEY (id),
    CONSTRAINT stay_operator_commission_rates_operator_fkey FOREIGN KEY ("operatorId") REFERENCES hosts (id),
    CONSTRAINT stay_operator_commission_rates_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_operator_commission_rates_rate_check CHECK ("rateBps" BETWEEN 0 AND 10000),
    CONSTRAINT stay_operator_commission_rates_note_check CHECK (note IS NULL OR char_length(note) <= 500)
);

CREATE INDEX stay_operator_commission_rates_current_idx
    ON stay_operator_commission_rates ("operatorId", "effectiveFrom" DESC, "createdAt" DESC);

CREATE TABLE stay_commission_batches (
    id text NOT NULL,
    city_id text NOT NULL,
    reference character varying(24) NOT NULL,
    "operatorId" text NOT NULL,
    "periodFrom" date NOT NULL,
    "periodTo" date NOT NULL,
    "reservationCount" integer NOT NULL,
    "commissionTotal" numeric(14,2) NOT NULL,
    state character varying(16) NOT NULL DEFAULT 'PENDING',
    "paymentRef" text,
    "reasonCode" character varying(32),
    "createdByUserId" text NOT NULL,
    "stateChangedByUserId" text,
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_commission_batches_pkey PRIMARY KEY (id),
    CONSTRAINT stay_commission_batches_reference_unique UNIQUE (reference),
    CONSTRAINT stay_commission_batches_operator_fkey FOREIGN KEY ("operatorId") REFERENCES hosts (id),
    CONSTRAINT stay_commission_batches_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_commission_batches_state_check CHECK (state IN ('PENDING', 'APPROVED', 'PAID', 'FAILED')),
    CONSTRAINT stay_commission_batches_period_check CHECK ("periodTo" >= "periodFrom"),
    CONSTRAINT stay_commission_batches_amount_check CHECK ("commissionTotal" >= 0 AND "reservationCount" >= 1),
    CONSTRAINT stay_commission_batches_paid_check CHECK (state <> 'PAID' OR "paymentRef" IS NOT NULL)
);

CREATE INDEX stay_commission_batches_operator_state_idx
    ON stay_commission_batches ("operatorId", state, "createdAt" DESC);

CREATE TABLE stay_commission_batch_items (
    id text NOT NULL,
    city_id text NOT NULL,
    "batchId" text NOT NULL,
    "reservationId" text NOT NULL,
    "operatorId" text NOT NULL,
    "rateBps" integer NOT NULL,
    "baseAmount" numeric(14,2) NOT NULL,
    "commissionAmount" numeric(14,2) NOT NULL,
    "earnedAt" timestamp with time zone NOT NULL,
    CONSTRAINT stay_commission_batch_items_pkey PRIMARY KEY (id),
    CONSTRAINT stay_commission_batch_items_batch_fkey FOREIGN KEY ("batchId") REFERENCES stay_commission_batches (id),
    CONSTRAINT stay_commission_batch_items_reservation_fkey FOREIGN KEY ("reservationId") REFERENCES stay_reservations (id),
    CONSTRAINT stay_commission_batch_items_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_commission_batch_items_reservation_unique UNIQUE ("reservationId")
);

CREATE INDEX stay_commission_batch_items_batch_idx
    ON stay_commission_batch_items ("batchId");

CREATE TABLE stay_credit_accounts (
    "operatorId" text NOT NULL,
    city_id text NOT NULL,
    balance numeric(14,2) NOT NULL DEFAULT 0,
    version integer NOT NULL DEFAULT 1,
    "updatedAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_credit_accounts_pkey PRIMARY KEY ("operatorId"),
    CONSTRAINT stay_credit_accounts_operator_fkey FOREIGN KEY ("operatorId") REFERENCES hosts (id),
    CONSTRAINT stay_credit_accounts_city_check CHECK (city_id IN ('GOMA', 'KIN'))
);

CREATE TABLE stay_credit_entries (
    id text NOT NULL,
    city_id text NOT NULL,
    "operatorId" text NOT NULL,
    "entryType" character varying(16) NOT NULL,
    amount numeric(14,2) NOT NULL,
    "balanceAfter" numeric(14,2) NOT NULL,
    "reservationId" text,
    "reasonCode" character varying(32),
    "paymentRef" text,
    note text,
    "createdByUserId" text,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_credit_entries_pkey PRIMARY KEY (id),
    CONSTRAINT stay_credit_entries_operator_fkey FOREIGN KEY ("operatorId") REFERENCES hosts (id),
    CONSTRAINT stay_credit_entries_reservation_fkey FOREIGN KEY ("reservationId") REFERENCES stay_reservations (id),
    CONSTRAINT stay_credit_entries_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_credit_entries_type_check CHECK ("entryType" IN ('TOPUP', 'COMMISSION', 'REFUND', 'ADJUSTMENT')),
    CONSTRAINT stay_credit_entries_amount_check CHECK (amount <> 0),
    CONSTRAINT stay_credit_entries_sign_check CHECK (
        ("entryType" = 'TOPUP' AND amount > 0)
        OR ("entryType" = 'COMMISSION' AND amount < 0)
        OR ("entryType" = 'REFUND' AND amount > 0)
        OR "entryType" = 'ADJUSTMENT'
    ),
    CONSTRAINT stay_credit_entries_reservation_check CHECK (("entryType" IN ('COMMISSION', 'REFUND')) = ("reservationId" IS NOT NULL)),
    CONSTRAINT stay_credit_entries_note_check CHECK (note IS NULL OR char_length(note) <= 500)
);

CREATE UNIQUE INDEX stay_credit_entries_commission_unique_idx
    ON stay_credit_entries ("reservationId") WHERE "entryType" = 'COMMISSION';

CREATE UNIQUE INDEX stay_credit_entries_refund_unique_idx
    ON stay_credit_entries ("reservationId") WHERE "entryType" = 'REFUND';

CREATE INDEX stay_credit_entries_operator_idx
    ON stay_credit_entries ("operatorId", "createdAt" DESC, id DESC);

CREATE INDEX stay_credit_entries_city_created_idx
    ON stay_credit_entries (city_id, "createdAt" DESC);
