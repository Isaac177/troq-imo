INSERT INTO stay_wishlists (id, city_id, "guestUserId", name, visibility) VALUES
    ('seed-wishlist-invitee-goma', 'GOMA', 'seed-user-goma-invitee', 'Lake weekend', 'PRIVATE'),
    ('seed-wishlist-invitee-empty', 'GOMA', 'seed-user-goma-invitee', 'Family trip', 'PRIVATE')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_wishlist_items ("wishlistId", "propertyId") VALUES
    ('seed-wishlist-invitee-goma', 'seed-prop-goma-lakeside')
ON CONFLICT ("wishlistId", "propertyId") DO NOTHING;
