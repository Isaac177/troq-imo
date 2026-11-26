CREATE TABLE stay_policy_versions (
    id text PRIMARY KEY,
    kind varchar(24) NOT NULL,
    "versionNumber" integer NOT NULL,
    content jsonb NOT NULL,
    "effectiveFrom" timestamptz NOT NULL,
    "tombstonedAt" timestamptz,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT stay_policy_versions_kind_check CHECK (kind IN ('CANCELLATION', 'PAYMENT')),
    CONSTRAINT stay_policy_versions_number_check CHECK ("versionNumber" >= 1),
    CONSTRAINT stay_policy_versions_kind_number_unique UNIQUE (kind, "versionNumber")
);

CREATE TABLE stay_rate_plans (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "unitTypeId" text NOT NULL,
    "occupancyBasis" varchar(16) NOT NULL,
    "mealPlan" varchar(16) NOT NULL DEFAULT 'ROOM_ONLY',
    "cancellationPolicyVersionId" text NOT NULL REFERENCES stay_policy_versions (id),
    "paymentPolicyVersionId" text NOT NULL REFERENCES stay_policy_versions (id),
    "bookingMode" varchar(16) NOT NULL DEFAULT 'INSTANT',
    status varchar(16) NOT NULL DEFAULT 'DRAFT',
    version integer NOT NULL DEFAULT 1,
    "childNightlyAmount" numeric(12,2) NOT NULL DEFAULT 0,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    "updatedAt" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT stay_rate_plans_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_rate_plans_occupancy_check CHECK ("occupancyBasis" IN ('PER_UNIT', 'PER_GUEST')),
    CONSTRAINT stay_rate_plans_meal_check CHECK ("mealPlan" IN ('ROOM_ONLY', 'BREAKFAST', 'HALF_BOARD', 'FULL_BOARD')),
    CONSTRAINT stay_rate_plans_booking_mode_check CHECK ("bookingMode" IN ('INSTANT', 'REQUEST')),
    CONSTRAINT stay_rate_plans_status_check CHECK (status IN ('DRAFT', 'ACTIVE', 'PAUSED', 'ARCHIVED')),
    CONSTRAINT stay_rate_plans_version_check CHECK (version >= 1),
    CONSTRAINT stay_rate_plans_child_amount_check CHECK ("childNightlyAmount" >= 0)
);

CREATE INDEX stay_rate_plans_property_idx ON stay_rate_plans (city_id, "propertyId");
CREATE INDEX stay_rate_plans_unit_active_idx ON stay_rate_plans ("unitTypeId") WHERE status = 'ACTIVE';

CREATE TABLE stay_rate_rules (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    "ratePlanId" text NOT NULL REFERENCES stay_rate_plans (id),
    "startDate" date,
    "endDate" date,
    "weekdayMask" smallint NOT NULL DEFAULT 127,
    "adjustmentType" varchar(16) NOT NULL,
    amount numeric(12,2) NOT NULL,
    priority integer NOT NULL DEFAULT 0,
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT stay_rate_rules_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_rate_rules_adjustment_check CHECK ("adjustmentType" IN ('AMOUNT', 'PERCENTAGE', 'OVERRIDE')),
    CONSTRAINT stay_rate_rules_weekday_check CHECK ("weekdayMask" BETWEEN 1 AND 127),
    CONSTRAINT stay_rate_rules_range_check CHECK ("startDate" IS NULL OR "endDate" IS NULL OR "endDate" > "startDate"),
    CONSTRAINT stay_rate_rules_amount_check CHECK (amount >= 0 AND ("adjustmentType" <> 'PERCENTAGE' OR amount <= 1000)),
    CONSTRAINT stay_rate_rules_version_check CHECK (version >= 1)
);

CREATE INDEX stay_rate_rules_plan_priority_idx ON stay_rate_rules ("ratePlanId", priority DESC);

CREATE TABLE stay_promotions (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "ratePlanId" text REFERENCES stay_rate_plans (id),
    kind varchar(24) NOT NULL,
    "minNights" integer NOT NULL,
    "discountPercent" numeric(5,2) NOT NULL,
    "capAmount" numeric(12,2),
    "startDate" date,
    "endDate" date,
    priority integer NOT NULL DEFAULT 0,
    status varchar(16) NOT NULL DEFAULT 'ACTIVE',
    "redemptionLimit" integer,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT stay_promotions_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_promotions_kind_check CHECK (kind = 'LENGTH_OF_STAY'),
    CONSTRAINT stay_promotions_min_nights_check CHECK ("minNights" >= 1),
    CONSTRAINT stay_promotions_percent_check CHECK ("discountPercent" > 0 AND "discountPercent" <= 100),
    CONSTRAINT stay_promotions_cap_check CHECK ("capAmount" IS NULL OR "capAmount" >= 0),
    CONSTRAINT stay_promotions_range_check CHECK ("startDate" IS NULL OR "endDate" IS NULL OR "endDate" > "startDate"),
    CONSTRAINT stay_promotions_status_check CHECK (status IN ('ACTIVE', 'PAUSED')),
    CONSTRAINT stay_promotions_redemption_limit_check CHECK ("redemptionLimit" IS NULL OR "redemptionLimit" >= 1)
);

CREATE INDEX stay_promotions_property_active_idx ON stay_promotions (city_id, "propertyId") WHERE status = 'ACTIVE';

CREATE TABLE stay_promotion_funding (
    city_id text NOT NULL,
    "promotionId" text PRIMARY KEY REFERENCES stay_promotions (id),
    "fundedBy" varchar(16) NOT NULL DEFAULT 'OPERATOR',
    "updatedByUserId" text NOT NULL,
    "updatedAt" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT stay_promotion_funding_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_promotion_funding_funded_by_check CHECK ("fundedBy" IN ('OPERATOR', 'PLATFORM'))
);

CREATE TABLE stay_platform_settings (
    city_id text NOT NULL,
    key varchar(48) NOT NULL,
    "valueInt" integer NOT NULL,
    "updatedByUserId" text,
    "updatedAt" timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (city_id, key),
    CONSTRAINT stay_platform_settings_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_platform_settings_key_check CHECK (key IN (
        'DEFAULT_COMMISSION_BPS', 'QUOTE_TTL_MINUTES', 'APPROVAL_WINDOW_HOURS',
        'AMENDMENT_WINDOW_HOURS', 'CLAIM_WINDOW_DAYS', 'CASE_RESPONSE_DAYS',
        'MIN_CREDIT_CDF', 'LOW_CREDIT_CDF'
    )),
    CONSTRAINT stay_platform_settings_commission_check CHECK (key <> 'DEFAULT_COMMISSION_BPS' OR "valueInt" BETWEEN 0 AND 10000),
    CONSTRAINT stay_platform_settings_quote_check CHECK (key <> 'QUOTE_TTL_MINUTES' OR "valueInt" BETWEEN 5 AND 120),
    CONSTRAINT stay_platform_settings_approval_check CHECK (key <> 'APPROVAL_WINDOW_HOURS' OR "valueInt" BETWEEN 1 AND 168),
    CONSTRAINT stay_platform_settings_amendment_check CHECK (key <> 'AMENDMENT_WINDOW_HOURS' OR "valueInt" BETWEEN 1 AND 168),
    CONSTRAINT stay_platform_settings_claim_check CHECK (key <> 'CLAIM_WINDOW_DAYS' OR "valueInt" BETWEEN 1 AND 60),
    CONSTRAINT stay_platform_settings_case_check CHECK (key <> 'CASE_RESPONSE_DAYS' OR "valueInt" BETWEEN 1 AND 60),
    CONSTRAINT stay_platform_settings_min_credit_check CHECK (key <> 'MIN_CREDIT_CDF' OR "valueInt" BETWEEN 0 AND 100000000),
    CONSTRAINT stay_platform_settings_low_credit_check CHECK (key <> 'LOW_CREDIT_CDF' OR "valueInt" BETWEEN 0 AND 100000000)
);

CREATE TABLE stay_quotes (
    id text PRIMARY KEY,
    city_id text NOT NULL,
    "sessionRef" text NOT NULL,
    "propertyId" text NOT NULL,
    "checkIn" date NOT NULL,
    "checkOut" date NOT NULL,
    "adultCount" integer NOT NULL,
    "childCount" integer NOT NULL DEFAULT 0,
    currency varchar(3) NOT NULL DEFAULT 'CDF',
    "totalAmount" numeric(14,2) NOT NULL,
    breakdown jsonb NOT NULL,
    "policyVersionIds" jsonb NOT NULL,
    "expiresAt" timestamptz NOT NULL,
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT stay_quotes_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_quotes_currency_check CHECK (currency = 'CDF'),
    CONSTRAINT stay_quotes_range_check CHECK ("checkOut" > "checkIn"),
    CONSTRAINT stay_quotes_guests_check CHECK ("adultCount" >= 1 AND "childCount" >= 0),
    CONSTRAINT stay_quotes_total_check CHECK ("totalAmount" >= 0)
);

CREATE INDEX stay_quotes_session_idx ON stay_quotes ("sessionRef", "createdAt" DESC);
