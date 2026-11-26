CREATE TABLE stay_wishlists (
    id text NOT NULL,
    city_id text NOT NULL,
    "guestUserId" text NOT NULL,
    name character varying(80) NOT NULL,
    visibility character varying(16) NOT NULL DEFAULT 'PRIVATE',
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    "updatedAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_wishlists_pkey PRIMARY KEY (id),
    CONSTRAINT stay_wishlists_visibility_check CHECK (visibility IN ('PRIVATE', 'SHARED')),
    CONSTRAINT stay_wishlists_name_check CHECK (char_length(btrim(name)) > 0)
);

CREATE INDEX stay_wishlists_guest_idx ON stay_wishlists (city_id, "guestUserId", "createdAt");

CREATE TABLE stay_wishlist_items (
    "wishlistId" text NOT NULL,
    "propertyId" text NOT NULL,
    "addedAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_wishlist_items_pkey PRIMARY KEY ("wishlistId", "propertyId"),
    CONSTRAINT stay_wishlist_items_wishlist_fkey FOREIGN KEY ("wishlistId") REFERENCES stay_wishlists (id) ON DELETE CASCADE
);

CREATE INDEX stay_wishlist_items_property_idx ON stay_wishlist_items ("propertyId");

CREATE TABLE stay_saved_searches (
    id text NOT NULL,
    city_id text NOT NULL,
    "guestUserId" text NOT NULL,
    parameters jsonb NOT NULL,
    "alertConsent" boolean NOT NULL DEFAULT false,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_saved_searches_pkey PRIMARY KEY (id),
    CONSTRAINT stay_saved_searches_parameters_check CHECK (jsonb_typeof(parameters) = 'object')
);

CREATE INDEX stay_saved_searches_guest_idx ON stay_saved_searches (city_id, "guestUserId", "createdAt");

CREATE TABLE stay_search_events (
    id text NOT NULL,
    city_id text NOT NULL,
    fingerprint character varying(32) NOT NULL,
    engine_version integer NOT NULL,
    query jsonb NOT NULL,
    result_count integer NOT NULL,
    candidate_plans integer NOT NULL,
    priced_plans integer NOT NULL,
    duration_ms integer NOT NULL,
    "createdAt" timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT stay_search_events_pkey PRIMARY KEY (id),
    CONSTRAINT stay_search_events_counts_check CHECK (result_count >= 0 AND candidate_plans >= priced_plans AND priced_plans >= 0),
    CONSTRAINT stay_search_events_query_check CHECK (jsonb_typeof(query) = 'object')
);

CREATE INDEX stay_search_events_fingerprint_idx ON stay_search_events (fingerprint, "createdAt");
CREATE INDEX stay_search_events_created_idx ON stay_search_events ("createdAt");
