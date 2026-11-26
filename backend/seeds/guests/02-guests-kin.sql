INSERT INTO stay_wishlists (id, city_id, "guestUserId", name, visibility) VALUES
    ('seed-wishlist-kin-city', 'KIN', 'seed-guest-kin-1', 'City break', 'PRIVATE'),
    ('seed-wishlist-kin-shared', 'KIN', 'seed-guest-kin-1', 'Business trips', 'SHARED'),
    ('seed-wishlist-kin-draft', 'KIN', 'seed-guest-kin-2', 'Not yet published', 'PRIVATE')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_wishlist_items ("wishlistId", "propertyId") VALUES
    ('seed-wishlist-kin-city', 'seed-prop-kin-gombe'),
    ('seed-wishlist-kin-shared', 'seed-prop-kin-gombe'),
    ('seed-wishlist-kin-draft', 'seed-prop-kin-draft')
ON CONFLICT ("wishlistId", "propertyId") DO NOTHING;

INSERT INTO stay_saved_searches (id, city_id, "guestUserId", parameters, "alertConsent") VALUES
    ('seed-saved-search-kin-apartment', 'KIN', 'seed-guest-kin-1', '{"adults": 2, "children": 0, "propertyTypes": ["ENTIRE_PLACE"]}', true),
    ('seed-saved-search-kin-flexible', 'KIN', 'seed-guest-kin-1', '{"adults": 1, "minFreeCancellationHours": 48, "maxPenaltyPercent": 0}', false)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_search_events (id, city_id, fingerprint, engine_version, query, result_count, candidate_plans, priced_plans, duration_ms, "createdAt") VALUES
    ('seed-sev-kin-recent', 'KIN', 'seed0000kin00001', 3, '{"adults": 2, "children": 0, "childAges": [], "sort": "price", "page": 1, "pageSize": 20}', 1, 1, 1, 37, now() - interval '1 day'),
    ('seed-sev-kin-expired', 'KIN', 'seed0000kin00002', 2, '{"adults": 3, "children": 0, "childAges": [], "sort": "relevance", "page": 1, "pageSize": 20}', 0, 0, 0, 11, now() - interval '240 days')
ON CONFLICT (id) DO NOTHING;
