INSERT INTO stay_properties
    (id, city_id, "operatorId", timezone, status, "publishedRevision", "nameEn", "nameFr", "descriptionEn", "descriptionFr", "addressLine", latitude, longitude, "verificationStatus")
VALUES
    ('seed-prop-goma-lakeside', 'GOMA', 'seed-host-goma-lake', 'Africa/Lubumbashi', 'PUBLISHED', 1, 'Lakeside Villa', 'Villa du Lac', 'Entire villa on the lake shore.', 'Villa entière au bord du lac.', 'Avenue du Lac, Goma', -1.6792, 29.2228, 'VERIFIED'),
    ('seed-prop-goma-ready', 'GOMA', 'seed-host-goma-lake', 'Africa/Lubumbashi', 'APPROVED', 0, 'Kivu Garden House', 'Maison Jardin Kivu', 'A guest house with a shared terrace.', 'Une maison d''hôtes avec terrasse commune.', 'Quartier Himbi, Goma', -1.6700, 29.2250, 'VERIFIED'),
    ('seed-prop-goma-virunga', 'GOMA', 'seed-host-goma-lake', 'Africa/Lubumbashi', 'PUBLISHED', 1, 'Virunga Suites', 'Suites Virunga', 'Suites facing the volcanoes.', 'Suites face aux volcans.', 'Route de Rutshuru, Goma', -1.6450, 29.2300, 'VERIFIED'),
    ('seed-prop-goma-paused', 'GOMA', 'seed-host-goma-lake', 'Africa/Lubumbashi', 'PAUSED', 1, 'Old Town Guesthouse', 'Maison d''Hôte Centre', 'Closed for renovation.', 'Fermée pour rénovation.', 'Quartier Himbi, Goma', NULL, NULL, 'VERIFIED'),
    ('seed-prop-goma-archived', 'GOMA', 'seed-host-goma-lake', 'Africa/Lubumbashi', 'ARCHIVED', 1, 'Retired Lodge', 'Ancien Lodge', 'No longer listed.', 'Plus proposé.', 'Ndosho, Goma', NULL, NULL, 'REJECTED'),
    ('seed-prop-kin-gombe', 'KIN', 'seed-host-kin-river', 'Africa/Kinshasa', 'PUBLISHED', 1, 'Gombe River Apartments', 'Appartements Fleuve Gombe', 'Apartments near the river.', 'Appartements près du fleuve.', 'Avenue du Fleuve, Kinshasa', -4.3110, 15.2860, 'VERIFIED'),
    ('seed-prop-kin-draft', 'KIN', 'seed-host-kin-river', 'Africa/Kinshasa', 'DRAFT', 0, 'Gombe Courtyard', 'Cour de Gombe', '', '', 'Gombe, Kinshasa', NULL, NULL, 'UNVERIFIED'),
    ('seed-prop-bill-goma', 'GOMA', 'seed-host-goma-lake', 'Africa/Lubumbashi', 'PUBLISHED', 1, 'Kivu Business Rooms', 'Chambres Affaires Kivu', 'Rooms for business travellers.', 'Chambres pour les voyageurs d''affaires.', 'Goma', NULL, NULL, 'VERIFIED'),
    ('seed-prop-bill-kin', 'KIN', 'seed-host-kin-river', 'Africa/Kinshasa', 'PUBLISHED', 1, 'River City Rooms', 'Chambres de la Cité du Fleuve', 'Rooms near the city centre.', 'Chambres près du centre-ville.', 'Gombe, Kinshasa', NULL, NULL, 'VERIFIED'),
    ('seed-prop-bill-kin-paused', 'KIN', 'seed-host-kin-paused', 'Africa/Kinshasa', 'SUSPENDED', 1, 'Kinshasa Courtyard', 'Cour de Kinshasa', 'Temporarily unavailable.', 'Temporairement indisponible.', 'Kinshasa', NULL, NULL, 'REJECTED')
ON CONFLICT (id) DO NOTHING;
