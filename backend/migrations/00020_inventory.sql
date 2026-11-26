CREATE EXTENSION IF NOT EXISTS btree_gist;

CREATE TABLE stay_unit_types (
    id text NOT NULL,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "inventoryModel" character varying(16) NOT NULL,
    "categoryCode" character varying(48) NOT NULL DEFAULT 'HOTEL_ROOM',
    "templateId" text,
    "nameEn" character varying(200) NOT NULL,
    "nameFr" character varying(200) NOT NULL,
    "descriptionEn" text NOT NULL DEFAULT '',
    "descriptionFr" text NOT NULL DEFAULT '',
    "maxAdults" integer NOT NULL,
    "maxChildren" integer NOT NULL DEFAULT 0,
    "bedroomCount" integer NOT NULL DEFAULT 0,
    "bedCount" integer NOT NULL DEFAULT 1,
    "bathroomCount" integer NOT NULL DEFAULT 1,
    accessibility jsonb NOT NULL DEFAULT '[]'::jsonb,
    status character varying(16) NOT NULL DEFAULT 'ACTIVE',
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_unit_types_pkey PRIMARY KEY (id),
    CONSTRAINT stay_unit_types_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_unit_types_model_check CHECK ("inventoryModel" IN ('WHOLE_UNIT', 'POOLED_ROOM')),
    CONSTRAINT stay_unit_types_status_check CHECK (status IN ('ACTIVE', 'INACTIVE', 'ARCHIVED')),
    CONSTRAINT stay_unit_types_capacity_check CHECK ("maxAdults" >= 1 AND "maxChildren" >= 0),
    CONSTRAINT stay_unit_types_counts_check CHECK ("bedroomCount" >= 0 AND "bedCount" >= 0 AND "bathroomCount" >= 0),
    CONSTRAINT stay_unit_types_version_check CHECK (version >= 1)
);

CREATE UNIQUE INDEX stay_unit_types_id_property_unique_idx ON stay_unit_types (id, "propertyId");
CREATE INDEX stay_unit_types_property_active_idx ON stay_unit_types ("propertyId") WHERE status = 'ACTIVE';
CREATE INDEX stay_unit_types_accessibility_idx ON stay_unit_types USING gin (accessibility);
CREATE INDEX stay_unit_types_category_idx ON stay_unit_types ("categoryCode");
CREATE INDEX stay_unit_types_city_property_idx ON stay_unit_types (city_id, "propertyId");

CREATE TABLE stay_physical_units (
    id text NOT NULL,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "unitTypeId" text NOT NULL,
    "localCode" character varying(64) NOT NULL,
    status character varying(16) NOT NULL DEFAULT 'ACTIVE',
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "retiredAt" timestamp with time zone,
    CONSTRAINT stay_physical_units_pkey PRIMARY KEY (id),
    CONSTRAINT stay_physical_units_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_physical_units_unit_type_fkey FOREIGN KEY ("unitTypeId", "propertyId") REFERENCES stay_unit_types (id, "propertyId"),
    CONSTRAINT stay_physical_units_status_check CHECK (status IN ('ACTIVE', 'OUT_OF_SERVICE', 'RETIRED')),
    CONSTRAINT stay_physical_units_retired_check CHECK ((status = 'RETIRED') = ("retiredAt" IS NOT NULL)),
    CONSTRAINT stay_physical_units_local_code_unique UNIQUE ("propertyId", "localCode")
);

CREATE INDEX stay_physical_units_unit_type_idx ON stay_physical_units ("unitTypeId");

CREATE TABLE stay_inventory_pools (
    id text NOT NULL,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "unitTypeId" text NOT NULL,
    "sellableQuantity" integer NOT NULL,
    "defaultNightlyCapacity" boolean NOT NULL DEFAULT true,
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_inventory_pools_pkey PRIMARY KEY (id),
    CONSTRAINT stay_inventory_pools_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_inventory_pools_unit_type_fkey FOREIGN KEY ("unitTypeId", "propertyId") REFERENCES stay_unit_types (id, "propertyId"),
    CONSTRAINT stay_inventory_pools_quantity_check CHECK ("sellableQuantity" >= 0),
    CONSTRAINT stay_inventory_pools_unit_type_unique UNIQUE ("unitTypeId")
);

CREATE TABLE stay_pool_physical_units (
    "poolId" text NOT NULL,
    "physicalUnitId" text NOT NULL,
    CONSTRAINT stay_pool_physical_units_pkey PRIMARY KEY ("poolId", "physicalUnitId"),
    CONSTRAINT stay_pool_physical_units_pool_fkey FOREIGN KEY ("poolId") REFERENCES stay_inventory_pools (id),
    CONSTRAINT stay_pool_physical_units_unit_fkey FOREIGN KEY ("physicalUnitId") REFERENCES stay_physical_units (id),
    CONSTRAINT stay_pool_physical_units_unit_unique UNIQUE ("physicalUnitId")
);

CREATE TABLE stay_night_inventory (
    "poolId" text NOT NULL,
    "stayDate" date NOT NULL,
    "sellableCapacity" integer NOT NULL,
    "allocatedCount" integer NOT NULL DEFAULT 0,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT stay_night_inventory_pkey PRIMARY KEY ("poolId", "stayDate"),
    CONSTRAINT stay_night_inventory_pool_fkey FOREIGN KEY ("poolId") REFERENCES stay_inventory_pools (id),
    CONSTRAINT stay_night_inventory_capacity_check CHECK ("sellableCapacity" >= 0),
    CONSTRAINT stay_night_inventory_allocated_check CHECK ("allocatedCount" >= 0 AND "allocatedCount" <= "sellableCapacity")
);

CREATE TABLE stay_inventory_allocations (
    id text NOT NULL,
    city_id text NOT NULL,
    "poolId" text NOT NULL,
    "physicalUnitId" text,
    "referenceType" character varying(16) NOT NULL,
    "referenceId" text NOT NULL,
    "startDate" date NOT NULL,
    "endDate" date NOT NULL,
    quantity integer NOT NULL,
    state character varying(16) NOT NULL DEFAULT 'ACTIVE',
    "expiresAt" timestamp with time zone,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "releasedAt" timestamp with time zone,
    CONSTRAINT stay_inventory_allocations_pkey PRIMARY KEY (id),
    CONSTRAINT stay_inventory_allocations_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_inventory_allocations_pool_fkey FOREIGN KEY ("poolId") REFERENCES stay_inventory_pools (id),
    CONSTRAINT stay_inventory_allocations_unit_fkey FOREIGN KEY ("physicalUnitId") REFERENCES stay_physical_units (id),
    CONSTRAINT stay_inventory_allocations_reference_check CHECK ("referenceType" IN ('HOLD', 'RESERVATION', 'BLOCK')),
    CONSTRAINT stay_inventory_allocations_state_check CHECK (state IN ('ACTIVE', 'RELEASED', 'EXPIRED')),
    CONSTRAINT stay_inventory_allocations_range_check CHECK ("endDate" > "startDate"),
    CONSTRAINT stay_inventory_allocations_quantity_check CHECK (quantity > 0),
    CONSTRAINT stay_inventory_allocations_released_check CHECK ((state = 'ACTIVE') = ("releasedAt" IS NULL)),
    CONSTRAINT stay_inventory_allocations_hold_expiry_check CHECK ("referenceType" <> 'HOLD' OR "expiresAt" IS NOT NULL)
);

CREATE INDEX stay_inventory_allocations_pool_range_idx
    ON stay_inventory_allocations ("poolId", "startDate", "endDate") WHERE state = 'ACTIVE';
CREATE INDEX stay_inventory_allocations_reference_idx
    ON stay_inventory_allocations ("referenceType", "referenceId");

CREATE TABLE stay_availability_blocks (
    id text NOT NULL,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "unitTypeId" text,
    "physicalUnitId" text,
    "startDate" date NOT NULL,
    "endDate" date NOT NULL,
    reason character varying(16) NOT NULL,
    origin character varying(16) NOT NULL,
    "actorUserId" text NOT NULL,
    "conflictDisposition" character varying(16) NOT NULL DEFAULT 'NONE',
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "removedAt" timestamp with time zone,
    CONSTRAINT stay_availability_blocks_pkey PRIMARY KEY (id),
    CONSTRAINT stay_availability_blocks_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_availability_blocks_unit_type_fkey FOREIGN KEY ("unitTypeId", "propertyId") REFERENCES stay_unit_types (id, "propertyId"),
    CONSTRAINT stay_availability_blocks_unit_fkey FOREIGN KEY ("physicalUnitId") REFERENCES stay_physical_units (id),
    CONSTRAINT stay_availability_blocks_range_check CHECK ("endDate" > "startDate"),
    CONSTRAINT stay_availability_blocks_scope_check CHECK ("unitTypeId" IS NOT NULL OR "physicalUnitId" IS NOT NULL),
    CONSTRAINT stay_availability_blocks_reason_check CHECK (reason IN ('MAINTENANCE', 'OWNER_USE', 'OPERATIONS', 'OTHER', 'EXTERNAL_BOOKING', 'MANUAL_CLOSURE', 'PLATFORM')),
    CONSTRAINT stay_availability_blocks_origin_check CHECK (origin IN ('OPERATOR', 'PLATFORM', 'INTEGRATION')),
    CONSTRAINT stay_availability_blocks_conflict_check CHECK ("conflictDisposition" IN ('NONE', 'OVERRIDDEN', 'REJECTED')),
    CONSTRAINT stay_availability_blocks_version_check CHECK (version >= 1)
);

CREATE INDEX stay_availability_blocks_property_range_idx
    ON stay_availability_blocks ("propertyId", "startDate", "endDate") WHERE "removedAt" IS NULL;

CREATE TABLE stay_restrictions (
    id text NOT NULL,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "unitTypeId" text,
    kind character varying(32) NOT NULL,
    basis character varying(16) NOT NULL DEFAULT 'STAY_THROUGH',
    "startDate" date,
    "endDate" date,
    value integer,
    "actorUserId" text NOT NULL,
    version integer NOT NULL DEFAULT 1,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "removedAt" timestamp with time zone,
    CONSTRAINT stay_restrictions_pkey PRIMARY KEY (id),
    CONSTRAINT stay_restrictions_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_restrictions_unit_type_fkey FOREIGN KEY ("unitTypeId", "propertyId") REFERENCES stay_unit_types (id, "propertyId"),
    CONSTRAINT stay_restrictions_kind_check CHECK (kind IN ('MIN_NIGHTS', 'MAX_NIGHTS', 'CLOSED_TO_ARRIVAL', 'CLOSED_TO_DEPARTURE', 'ADVANCE_NOTICE_DAYS', 'MAX_HORIZON_DAYS', 'TURNOVER_BUFFER_NIGHTS')),
    CONSTRAINT stay_restrictions_basis_check CHECK (basis IN ('STAY_THROUGH', 'ARRIVAL')),
    CONSTRAINT stay_restrictions_basis_scope_check CHECK (basis = 'STAY_THROUGH' OR kind = 'MIN_NIGHTS'),
    CONSTRAINT stay_restrictions_value_check CHECK (
        (kind IN ('MIN_NIGHTS', 'MAX_NIGHTS', 'ADVANCE_NOTICE_DAYS', 'MAX_HORIZON_DAYS', 'TURNOVER_BUFFER_NIGHTS')) = (value IS NOT NULL)
        AND (value IS NULL OR (value >= 0 AND value <= 365))
    ),
    CONSTRAINT stay_restrictions_range_check CHECK ("startDate" IS NULL OR "endDate" IS NULL OR "endDate" > "startDate"),
    CONSTRAINT stay_restrictions_version_check CHECK (version >= 1)
);

CREATE INDEX stay_restrictions_property_idx ON stay_restrictions ("propertyId", kind) WHERE "removedAt" IS NULL;

CREATE TABLE stay_unit_assignments (
    id text NOT NULL,
    city_id text NOT NULL,
    "reservationItemId" text NOT NULL,
    "physicalUnitId" text NOT NULL,
    "startDate" date NOT NULL,
    "endDate" date NOT NULL,
    reason character varying(16) NOT NULL,
    "actorUserId" text NOT NULL,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "releasedAt" timestamp with time zone,
    CONSTRAINT stay_unit_assignments_pkey PRIMARY KEY (id),
    CONSTRAINT stay_unit_assignments_city_check CHECK (city_id IN ('GOMA', 'KIN')),
    CONSTRAINT stay_unit_assignments_item_fkey FOREIGN KEY ("reservationItemId") REFERENCES stay_reservation_items (id),
    CONSTRAINT stay_unit_assignments_unit_fkey FOREIGN KEY ("physicalUnitId") REFERENCES stay_physical_units (id),
    CONSTRAINT stay_unit_assignments_range_check CHECK ("endDate" > "startDate"),
    CONSTRAINT stay_unit_assignments_reason_check CHECK (reason IN ('ASSIGNED', 'CHECK_IN', 'MOVED')),
    CONSTRAINT stay_unit_assignments_no_overlap EXCLUDE USING gist (
        "physicalUnitId" WITH =,
        daterange("startDate", "endDate", '[)') WITH &&
    ) WHERE ("releasedAt" IS NULL)
);

CREATE INDEX stay_unit_assignments_item_idx ON stay_unit_assignments ("reservationItemId", "createdAt");

CREATE TABLE stay_calendar_feed_tokens (
    id text NOT NULL,
    city_id text NOT NULL,
    "propertyId" text NOT NULL,
    "secretHash" character varying(64) NOT NULL,
    "createdByUserId" text NOT NULL,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "revokedAt" timestamp with time zone,
    CONSTRAINT stay_calendar_feed_tokens_pkey PRIMARY KEY (id),
    CONSTRAINT stay_calendar_feed_tokens_city_check CHECK (city_id IN ('GOMA', 'KIN'))
);

CREATE INDEX stay_calendar_feed_tokens_property_idx
    ON stay_calendar_feed_tokens ("propertyId") WHERE "revokedAt" IS NULL;

CREATE FUNCTION stay_unallocated_physical_blocks(pool_id text, night date)
RETURNS bigint
LANGUAGE sql
STABLE
AS $$
    SELECT COUNT(DISTINCT b."physicalUnitId")
    FROM stay_availability_blocks b
    JOIN stay_pool_physical_units pu ON pu."physicalUnitId" = b."physicalUnitId"
    WHERE pu."poolId" = pool_id AND b."removedAt" IS NULL
      AND b."startDate" <= night AND b."endDate" > night
      AND NOT EXISTS (
          SELECT 1 FROM stay_availability_blocks other
          JOIN stay_inventory_allocations a ON a."referenceType" = 'BLOCK' AND a."referenceId" = other.id
          WHERE other."physicalUnitId" = b."physicalUnitId" AND other."removedAt" IS NULL
            AND a."poolId" = pool_id AND a.state = 'ACTIVE'
            AND a."startDate" <= night AND a."endDate" > night
      );
$$;

CREATE FUNCTION stay_room_available(unit_type text, from_date date, to_date date, exclude_reservation text)
RETURNS boolean
LANGUAGE sql
STABLE
AS $$
    SELECT NOT EXISTS (
        SELECT 1 FROM stay_physical_units u
        JOIN stay_pool_physical_units pu ON pu."physicalUnitId" = u.id
        JOIN stay_inventory_pools p ON p.id = pu."poolId" AND p."unitTypeId" = u."unitTypeId"
        WHERE u."unitTypeId" = unit_type AND u.status = 'ACTIVE'
    ) OR EXISTS (
        SELECT 1 FROM stay_physical_units u
        JOIN stay_pool_physical_units pu ON pu."physicalUnitId" = u.id
        JOIN stay_inventory_pools p ON p.id = pu."poolId" AND p."unitTypeId" = u."unitTypeId"
        CROSS JOIN LATERAL (
            SELECT COALESCE(
                (SELECT MAX(r.value) FROM stay_restrictions r
                 WHERE r."propertyId" = u."propertyId" AND r.kind = 'TURNOVER_BUFFER_NIGHTS' AND r."removedAt" IS NULL
                   AND r."unitTypeId" = u."unitTypeId"
                   AND (r."startDate" IS NULL OR r."startDate" < to_date) AND (r."endDate" IS NULL OR r."endDate" > from_date)),
                (SELECT MAX(r.value) FROM stay_restrictions r
                 WHERE r."propertyId" = u."propertyId" AND r.kind = 'TURNOVER_BUFFER_NIGHTS' AND r."removedAt" IS NULL
                   AND r."unitTypeId" IS NULL
                   AND (r."startDate" IS NULL OR r."startDate" < to_date) AND (r."endDate" IS NULL OR r."endDate" > from_date)),
                0) AS nights
        ) buffer
        WHERE u."unitTypeId" = unit_type AND u.status = 'ACTIVE'
          AND NOT EXISTS (
              SELECT 1 FROM stay_unit_assignments a
              JOIN stay_reservation_items i ON i.id = a."reservationItemId"
              WHERE a."physicalUnitId" = u.id AND a."releasedAt" IS NULL
                AND i."reservationId" <> exclude_reservation
                AND a."startDate" < to_date AND a."endDate" > from_date)
          AND NOT EXISTS (
              SELECT 1 FROM stay_unit_assignments a
              JOIN stay_reservation_items i ON i.id = a."reservationItemId"
              WHERE buffer.nights > 0 AND a."physicalUnitId" = u.id AND a."releasedAt" IS NULL
                AND i."reservationId" <> exclude_reservation
                AND a."startDate" < to_date + buffer.nights AND a."endDate" > from_date - buffer.nights)
          AND NOT EXISTS (
              SELECT 1 FROM stay_availability_blocks b
              WHERE b."physicalUnitId" = u.id AND b."removedAt" IS NULL
                AND b."startDate" < to_date AND b."endDate" > from_date)
          AND NOT EXISTS (
              SELECT 1 FROM stay_reservation_items i
              JOIN stay_reservations r ON r.id = i."reservationId"
              WHERE i."physicalUnitId" = u.id AND i."reservationId" <> exclude_reservation
                AND r.state IN ('CONFIRMED', 'CHECKED_IN')
                AND i."checkIn" < to_date AND i."checkOut" > from_date
                AND NOT EXISTS (SELECT 1 FROM stay_unit_assignments a2
                                WHERE a2."reservationItemId" = i.id AND a2."releasedAt" IS NULL))
    );
$$;
