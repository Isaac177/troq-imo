INSERT INTO stay_property_media
    (id, "propertyId", status, "sortOrder", "isCover", "altEn", "altFr", "fileName", "mimeType", "sizeBytes", "storageKey", "rejectionReason", "clientKey", "addedByUserId")
VALUES
    ('seed-media-goma-terrace', 'seed-prop-goma-lakeside', 'READY', 1, TRUE, 'Lakeside villa terrace', 'Terrasse de la villa au bord du lac', 'terrace.png', 'image/png', 68, 'goma/stays/properties/seed-media-goma-terrace.png', NULL, 'seed-media-key-goma-terrace', 'seed-user-goma-host'),
    ('seed-media-goma-living', 'seed-prop-goma-lakeside', 'READY', 2, FALSE, 'Living room with lake view', 'Salon avec vue sur le lac', 'living.png', 'image/png', 68, 'goma/stays/properties/seed-media-goma-living.png', NULL, 'seed-media-key-goma-living', 'seed-user-goma-host'),
    ('seed-media-goma-rejected', 'seed-prop-goma-lakeside', 'REJECTED', 3, FALSE, '', '', 'brochure.pdf', 'application/pdf', 0, NULL, 'MEDIA_UNSUPPORTED_TYPE', 'seed-media-key-goma-rejected', 'seed-user-goma-host'),
    ('seed-media-goma-removed', 'seed-prop-goma-lakeside', 'REMOVED', 4, FALSE, '', '', 'old-photo.jpg', 'image/jpeg', 0, NULL, NULL, 'seed-media-key-goma-removed', 'seed-user-goma-host'),
    ('seed-media-goma-ready-cover', 'seed-prop-goma-ready', 'READY', 1, TRUE, 'Guest house terrace', 'Terrasse de la maison d''hôtes', 'terrace.png', 'image/png', 68, 'goma/stays/properties/seed-media-goma-ready-cover.png', NULL, 'seed-media-key-goma-ready-cover', 'seed-user-goma-host'),
    ('seed-media-goma-virunga-cover', 'seed-prop-goma-virunga', 'READY', 1, TRUE, 'Virunga suite view', 'Vue de la suite Virunga', 'suite.jpg', 'image/jpeg', 68, 'goma/stays/properties/seed-media-goma-virunga-cover.jpg', NULL, 'seed-media-key-goma-virunga-cover', 'seed-user-goma-host'),
    ('seed-media-kin-gombe-cover', 'seed-prop-kin-gombe', 'READY', 1, TRUE, 'River view from the apartment', 'Vue sur le fleuve depuis l''appartement', 'river.png', 'image/png', 68, 'kin/stays/properties/seed-media-kin-gombe-cover.png', NULL, 'seed-media-key-kin-gombe-cover', 'seed-user-kin-host'),
    ('seed-media-kin-gombe-living', 'seed-prop-kin-gombe', 'READY', 2, FALSE, 'Living room', 'Salon', 'living.png', 'image/png', 68, 'kin/stays/properties/seed-media-kin-gombe-living.png', NULL, 'seed-media-key-kin-gombe-living', 'seed-user-kin-host')
ON CONFLICT (id) DO NOTHING;
