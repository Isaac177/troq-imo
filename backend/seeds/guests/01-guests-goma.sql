INSERT INTO stay_wishlists (id, city_id, "guestUserId", name, visibility) VALUES
    ('seed-wishlist-goma-lake', 'GOMA', 'seed-guest-goma-1', 'Lake weekend', 'PRIVATE'),
    ('seed-wishlist-goma-family', 'GOMA', 'seed-guest-goma-1', 'Family trip', 'SHARED'),
    ('seed-wishlist-goma-empty', 'GOMA', 'seed-guest-goma-1', 'Ideas for later', 'PRIVATE'),
    ('seed-wishlist-goma-unlisted', 'GOMA', 'seed-guest-goma-2', 'Places that closed', 'PRIVATE')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_wishlist_items ("wishlistId", "propertyId") VALUES
    ('seed-wishlist-goma-lake', 'seed-prop-goma-lakeside'),
    ('seed-wishlist-goma-family', 'seed-prop-goma-lakeside'),
    ('seed-wishlist-goma-unlisted', 'seed-prop-goma-lakeside'),
    ('seed-wishlist-goma-unlisted', 'seed-prop-goma-paused'),
    ('seed-wishlist-goma-unlisted', 'seed-prop-goma-archived')
ON CONFLICT ("wishlistId", "propertyId") DO NOTHING;

INSERT INTO stay_saved_searches (id, city_id, "guestUserId", parameters, "alertConsent") VALUES
    ('seed-saved-search-goma-weekend', 'GOMA', 'seed-guest-goma-1', '{"adults": 2, "children": 0, "sort": "price"}', false),
    ('seed-saved-search-goma-alerts', 'GOMA', 'seed-guest-goma-1', '{"adults": 4, "children": 2, "childAges": [4, 9], "amenities": ["WIFI", "LAKE_VIEW"], "instantBooking": true}', true),
    ('seed-saved-search-goma-nearby', 'GOMA', 'seed-guest-goma-2', '{"latitude": -1.68, "longitude": 29.22, "maxDistanceKm": 5, "sort": "distance"}', false)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_search_events (id, city_id, fingerprint, engine_version, query, result_count, candidate_plans, priced_plans, duration_ms, "createdAt") VALUES
    ('seed-sev-goma-recent', 'GOMA', 'seed0000goma0001', 3, '{"adults": 2, "children": 0, "childAges": [], "sort": "relevance", "page": 1, "pageSize": 20}', 1, 3, 3, 42, now() - interval '2 days'),
    ('seed-sev-goma-undated', 'GOMA', 'seed0000goma0002', 3, '{"adults": 1, "children": 0, "childAges": [], "sort": "name", "page": 1, "pageSize": 20}', 2, 4, 0, 18, now() - interval '30 days'),
    ('seed-sev-goma-expired', 'GOMA', 'seed0000goma0003', 2, '{"adults": 2, "children": 1, "childAges": [], "sort": "relevance", "page": 1, "pageSize": 20}', 0, 0, 0, 9, now() - interval '200 days')
ON CONFLICT (id) DO NOTHING;
