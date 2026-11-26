CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE stay_accommodation_categories (
    code varchar(48) PRIMARY KEY CHECK (code ~ '^[A-Z0-9_]+$'),
    "nameEn" varchar(80) NOT NULL,
    "nameFr" varchar(80) NOT NULL,
    icon varchar(48) NOT NULL CHECK (icon ~ '^[A-Za-z0-9]+$'),
    "sortOrder" integer NOT NULL DEFAULT 0,
    active boolean NOT NULL DEFAULT true,
    revision bigint NOT NULL DEFAULT 1,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    "updatedAt" timestamptz NOT NULL DEFAULT now()
);

INSERT INTO stay_accommodation_categories (code, "nameEn", "nameFr", icon, "sortOrder") VALUES
    ('HOTEL_ROOM', 'Hotel room', 'Chambre d''hôtel', 'BedDouble', 10),
    ('APARTMENT', 'Apartment', 'Appartement', 'Building2', 20),
    ('STUDIO', 'Studio', 'Studio', 'DoorOpen', 30),
    ('VILLA', 'Villa', 'Villa', 'House', 40),
    ('GUEST_HOUSE', 'Guest house', 'Maison d''hôtes', 'House', 50),
    ('BUNGALOW', 'Bungalow', 'Bungalow', 'Tent', 60),
    ('CHALET', 'Chalet', 'Chalet', 'Mountain', 70),
    ('LODGE', 'Lodge', 'Lodge', 'Trees', 80),
    ('HOSTEL_ROOM', 'Hostel room', 'Chambre d''auberge', 'Users', 90),
    ('DORMITORY_BED', 'Dormitory bed', 'Lit en dortoir', 'Bed', 100)
ON CONFLICT (code) DO NOTHING;

CREATE TABLE stay_properties (
    id text PRIMARY KEY,
    city_id text NOT NULL CHECK (city_id IN ('GOMA', 'KIN')),
    "operatorId" text NOT NULL REFERENCES hosts (id),
    timezone varchar(64) NOT NULL,
    status varchar(16) NOT NULL DEFAULT 'DRAFT'
        CHECK (status IN ('DRAFT', 'APPROVED', 'PUBLISHED', 'PAUSED', 'SUSPENDED', 'ARCHIVED')),
    "publishedRevision" integer NOT NULL DEFAULT 0 CHECK ("publishedRevision" >= 0),
    "nameEn" varchar(200) NOT NULL,
    "nameFr" varchar(200) NOT NULL,
    "descriptionEn" text NOT NULL DEFAULT '',
    "descriptionFr" text NOT NULL DEFAULT '',
    "addressLine" varchar(255) NOT NULL DEFAULT '',
    latitude numeric(9,6) CHECK (latitude IS NULL OR latitude BETWEEN -90 AND 90),
    longitude numeric(9,6) CHECK (longitude IS NULL OR longitude BETWEEN -180 AND 180),
    neighborhood varchar(120) NOT NULL DEFAULT '',
    "checkInFrom" varchar(5) CHECK ("checkInFrom" IS NULL OR "checkInFrom" ~ '^([01][0-9]|2[0-3]):[0-5][0-9]$'),
    "checkInUntil" varchar(5) CHECK ("checkInUntil" IS NULL OR "checkInUntil" ~ '^([01][0-9]|2[0-3]):[0-5][0-9]$'),
    "checkOutBy" varchar(5) CHECK ("checkOutBy" IS NULL OR "checkOutBy" ~ '^([01][0-9]|2[0-3]):[0-5][0-9]$'),
    "houseRulesEn" text NOT NULL DEFAULT '',
    "houseRulesFr" text NOT NULL DEFAULT '',
    "petsPolicy" varchar(16) CHECK ("petsPolicy" IS NULL OR "petsPolicy" IN ('NOT_ALLOWED', 'ALLOWED', 'ON_REQUEST')),
    "smokingPolicy" varchar(16) CHECK ("smokingPolicy" IS NULL OR "smokingPolicy" IN ('NOT_ALLOWED', 'ALLOWED')),
    "guestContactPhone" varchar(32) NOT NULL DEFAULT '',
    "guestContactEmail" varchar(254) NOT NULL DEFAULT '',
    amenities jsonb NOT NULL DEFAULT '[]'::jsonb,
    "verificationStatus" varchar(16) NOT NULL DEFAULT 'UNVERIFIED'
        CHECK ("verificationStatus" IN ('UNVERIFIED', 'PENDING', 'VERIFIED', 'REJECTED')),
    version integer NOT NULL DEFAULT 1 CHECK (version >= 1),
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    "updatedAt" timestamptz NOT NULL DEFAULT now(),
    CHECK ((latitude IS NULL) = (longitude IS NULL))
);

CREATE INDEX stay_properties_operator_idx ON stay_properties ("operatorId");
CREATE INDEX stay_properties_city_status_idx ON stay_properties (city_id, status);
CREATE INDEX stay_properties_geo_idx ON stay_properties
    USING GIST ((ST_SetSRID(ST_MakePoint(longitude::float8, latitude::float8), 4326)::geography));

CREATE TABLE stay_property_revisions (
    id text PRIMARY KEY,
    "propertyId" text NOT NULL REFERENCES stay_properties (id),
    "revisionNumber" integer NOT NULL,
    snapshot jsonb NOT NULL CHECK (jsonb_typeof(snapshot) = 'object'),
    "authorUserId" text NOT NULL,
    "reviewStatus" varchar(24) NOT NULL DEFAULT 'DRAFT'
        CHECK ("reviewStatus" IN ('DRAFT', 'SUBMITTED', 'CHANGES_REQUESTED', 'APPROVED', 'REJECTED')),
    "reviewReasons" jsonb NOT NULL DEFAULT '[]'::jsonb,
    "reviewGuidance" text,
    version integer NOT NULL DEFAULT 1 CHECK (version >= 1),
    "submittedAt" timestamptz,
    "submittedByUserId" text,
    "reviewedAt" timestamptz,
    "reviewedByUserId" text,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    "updatedAt" timestamptz NOT NULL DEFAULT now(),
    UNIQUE ("propertyId", "revisionNumber"),
    CHECK (("reviewStatus" IN ('CHANGES_REQUESTED', 'APPROVED', 'REJECTED')) = ("reviewedAt" IS NOT NULL)),
    CHECK (("reviewStatus" = 'DRAFT') = ("submittedAt" IS NULL))
);

CREATE UNIQUE INDEX stay_property_revisions_one_open_idx
    ON stay_property_revisions ("propertyId")
    WHERE "reviewStatus" IN ('DRAFT', 'SUBMITTED', 'CHANGES_REQUESTED');
CREATE INDEX stay_property_revisions_queue_idx ON stay_property_revisions ("reviewStatus", "submittedAt");

CREATE TABLE stay_property_media (
    id text PRIMARY KEY,
    "propertyId" text NOT NULL REFERENCES stay_properties (id),
    status varchar(16) NOT NULL CHECK (status IN ('READY', 'REJECTED', 'REMOVED')),
    "sortOrder" integer NOT NULL DEFAULT 0,
    "isCover" boolean NOT NULL DEFAULT false,
    "altEn" varchar(200) NOT NULL DEFAULT '',
    "altFr" varchar(200) NOT NULL DEFAULT '',
    "fileName" varchar(200) NOT NULL,
    "mimeType" varchar(32) NOT NULL,
    "sizeBytes" integer NOT NULL DEFAULT 0,
    "storageKey" text,
    "rejectionReason" varchar(200),
    "clientKey" varchar(100) NOT NULL,
    version integer NOT NULL DEFAULT 1,
    "addedByUserId" text NOT NULL,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    "updatedAt" timestamptz NOT NULL DEFAULT now(),
    UNIQUE ("propertyId", "clientKey"),
    CHECK (status <> 'READY' OR ("sizeBytes" > 0 AND "storageKey" IS NOT NULL)),
    CHECK (status <> 'REJECTED' OR "rejectionReason" IS NOT NULL),
    CHECK (NOT "isCover" OR status = 'READY')
);

CREATE UNIQUE INDEX stay_property_media_one_cover ON stay_property_media ("propertyId") WHERE "isCover";
CREATE INDEX stay_property_media_property_order ON stay_property_media ("propertyId", "sortOrder", "createdAt");

CREATE TABLE stay_accommodation_templates (
    id text PRIMARY KEY,
    city_id text NOT NULL CHECK (city_id IN ('GOMA', 'KIN')),
    "operatorId" text NOT NULL REFERENCES hosts (id),
    "categoryCode" varchar(48) NOT NULL REFERENCES stay_accommodation_categories (code),
    "inventoryModel" varchar(16) NOT NULL CHECK ("inventoryModel" IN ('WHOLE_UNIT', 'POOLED_ROOM')),
    "nameEn" varchar(200) NOT NULL,
    "nameFr" varchar(200) NOT NULL,
    "descriptionEn" text NOT NULL DEFAULT '',
    "descriptionFr" text NOT NULL DEFAULT '',
    "maxAdults" integer NOT NULL CHECK ("maxAdults" >= 1),
    "maxChildren" integer NOT NULL DEFAULT 0 CHECK ("maxChildren" >= 0),
    "bedroomCount" integer NOT NULL DEFAULT 0 CHECK ("bedroomCount" >= 0),
    "bedCount" integer NOT NULL DEFAULT 1 CHECK ("bedCount" >= 0),
    "bathroomCount" integer NOT NULL DEFAULT 1 CHECK ("bathroomCount" >= 0),
    accessibility jsonb NOT NULL DEFAULT '[]'::jsonb,
    "baseNightlyAmount" numeric(14,2) NOT NULL DEFAULT 0 CHECK ("baseNightlyAmount" >= 0 AND "baseNightlyAmount" <= 999999999),
    status varchar(16) NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'ARCHIVED')),
    version integer NOT NULL DEFAULT 1 CHECK (version >= 1),
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    "updatedAt" timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX stay_accommodation_templates_operator_idx
    ON stay_accommodation_templates (city_id, "operatorId", status, "createdAt");

CREATE TABLE stay_template_media (
    id text PRIMARY KEY,
    "templateId" text NOT NULL REFERENCES stay_accommodation_templates (id),
    status varchar(16) NOT NULL CHECK (status IN ('READY', 'REJECTED', 'REMOVED')),
    "sortOrder" integer NOT NULL DEFAULT 0,
    "isCover" boolean NOT NULL DEFAULT false,
    "altEn" varchar(200) NOT NULL DEFAULT '',
    "altFr" varchar(200) NOT NULL DEFAULT '',
    "fileName" varchar(200) NOT NULL,
    "mimeType" varchar(32) NOT NULL,
    "sizeBytes" integer NOT NULL DEFAULT 0,
    "storageKey" text,
    "rejectionReason" varchar(200),
    "clientKey" varchar(100) NOT NULL,
    version integer NOT NULL DEFAULT 1,
    "addedByUserId" text NOT NULL,
    "createdAt" timestamptz NOT NULL DEFAULT now(),
    "updatedAt" timestamptz NOT NULL DEFAULT now(),
    UNIQUE ("templateId", "clientKey"),
    CHECK (status <> 'READY' OR ("sizeBytes" > 0 AND "storageKey" IS NOT NULL)),
    CHECK (status <> 'REJECTED' OR "rejectionReason" IS NOT NULL),
    CHECK (NOT "isCover" OR status = 'READY')
);

CREATE UNIQUE INDEX stay_template_media_one_cover ON stay_template_media ("templateId") WHERE "isCover";
CREATE INDEX stay_template_media_template_order ON stay_template_media ("templateId", "sortOrder", "createdAt");
