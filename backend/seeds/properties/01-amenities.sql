INSERT INTO property_amenity_groups (id, code, source_locale, sort_order, active) VALUES
    ('seed-amenity-group-essentials', 'SEED_ESSENTIALS', 'en', 10, true),
    ('seed-amenity-group-outdoor', 'SEED_OUTDOOR', 'en', 20, true),
    ('seed-amenity-group-retired', 'SEED_RETIRED', 'en', 30, false)
ON CONFLICT DO NOTHING;

INSERT INTO property_amenity_group_content (group_id, locale, name)
SELECT v.group_id, v.locale, v.name FROM (VALUES
    ('seed-amenity-group-essentials', 'en', 'Essentials'),
    ('seed-amenity-group-essentials', 'fr', 'Essentiels'),
    ('seed-amenity-group-outdoor', 'en', 'Outdoors and views'),
    ('seed-amenity-group-outdoor', 'fr', 'Extérieur et vues'),
    ('seed-amenity-group-retired', 'en', 'Retired facilities'),
    ('seed-amenity-group-retired', 'fr', 'Équipements retirés')
) AS v(group_id, locale, name) JOIN property_amenity_groups g ON g.id = v.group_id
ON CONFLICT DO NOTHING;

INSERT INTO property_amenities (id, code, group_id, source_locale, icon, sort_order, active)
SELECT v.id, v.code, v.group_id, v.source_locale, v.icon, v.sort_order, v.active FROM (VALUES
    ('seed-amenity-wifi', 'SEED_WIFI', 'seed-amenity-group-essentials', 'en', 'Wifi', 10, true),
    ('seed-amenity-water', 'SEED_HOT_WATER', 'seed-amenity-group-essentials', 'en', 'ShowerHead', 20, true),
    ('seed-amenity-lake', 'SEED_LAKE_VIEW', 'seed-amenity-group-outdoor', 'en', 'Sailboat', 10, true),
    ('seed-amenity-retired', 'SEED_RETIRED_FAN', 'seed-amenity-group-retired', 'en', 'Fan', 10, false),
    ('seed-amenity-retired-active-group', 'SEED_RETIRED_TV', 'seed-amenity-group-essentials', 'en', 'Tv', 30, false)
) AS v(id, code, group_id, source_locale, icon, sort_order, active)
JOIN property_amenity_groups g ON g.id = v.group_id
ON CONFLICT DO NOTHING;

INSERT INTO property_amenity_content (amenity_id, locale, name)
SELECT v.amenity_id, v.locale, v.name FROM (VALUES
    ('seed-amenity-wifi', 'en', 'Wi-Fi'), ('seed-amenity-wifi', 'fr', 'Wi-Fi'),
    ('seed-amenity-water', 'en', 'Hot water'), ('seed-amenity-water', 'fr', 'Eau chaude'),
    ('seed-amenity-lake', 'en', 'Lake view'), ('seed-amenity-lake', 'fr', 'Vue sur le lac'),
    ('seed-amenity-retired', 'en', 'Ceiling fan'), ('seed-amenity-retired', 'fr', 'Ventilateur de plafond'),
    ('seed-amenity-retired-active-group', 'en', 'Television'), ('seed-amenity-retired-active-group', 'fr', 'Télévision')
) AS v(amenity_id, locale, name) JOIN property_amenities a ON a.id = v.amenity_id
ON CONFLICT DO NOTHING;
