CREATE TABLE property_amenity_groups (
    id text PRIMARY KEY,
    code varchar(48) NOT NULL UNIQUE CHECK (code ~ '^[A-Z0-9_]+$'),
    source_locale text NOT NULL,
    sort_order integer NOT NULL DEFAULT 0,
    active boolean NOT NULL DEFAULT true,
    revision bigint NOT NULL DEFAULT 1,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE property_amenity_group_content (
    group_id text NOT NULL REFERENCES property_amenity_groups(id),
    locale text NOT NULL,
    name varchar(80) NOT NULL CHECK (char_length(trim(name)) > 0),
    PRIMARY KEY (group_id, locale)
);

CREATE TABLE property_amenities (
    id text PRIMARY KEY,
    code varchar(48) NOT NULL UNIQUE CHECK (code ~ '^[A-Z0-9_]+$'),
    group_id text NOT NULL REFERENCES property_amenity_groups(id),
    source_locale text NOT NULL,
    icon varchar(48) NOT NULL CHECK (icon ~ '^[A-Za-z0-9]+$'),
    sort_order integer NOT NULL DEFAULT 0,
    active boolean NOT NULL DEFAULT true,
    revision bigint NOT NULL DEFAULT 1,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE property_amenity_content (
    amenity_id text NOT NULL REFERENCES property_amenities(id),
    locale text NOT NULL,
    name varchar(80) NOT NULL CHECK (char_length(trim(name)) > 0),
    PRIMARY KEY (amenity_id, locale)
);

CREATE INDEX property_amenities_group_order_idx ON property_amenities (group_id, sort_order, code);
