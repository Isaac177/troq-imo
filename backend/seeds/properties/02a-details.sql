UPDATE stay_properties p SET
    neighborhood = d.neighborhood, "checkInFrom" = d.check_in_from, "checkInUntil" = d.check_in_until,
    "checkOutBy" = d.check_out_by, "houseRulesEn" = d.rules_en, "houseRulesFr" = d.rules_fr,
    "petsPolicy" = d.pets, "smokingPolicy" = d.smoking, "guestContactPhone" = d.phone,
    "guestContactEmail" = d.email, amenities = d.amenities::jsonb
FROM (VALUES
    ('seed-prop-goma-lakeside', 'Lakeshore, Goma', '15:00', '20:00', '11:00', 'No parties. Quiet hours from 22:00.', 'Pas de fêtes. Silence à partir de 22h.', 'NOT_ALLOWED', 'NOT_ALLOWED', '+243 800 000 001', 'stays@lac-kivu.example', '["WIFI", "PARKING", "GARDEN", "LAKE_VIEW", "GENERATOR"]'),
    ('seed-prop-goma-ready', 'Himbi, Goma', '14:00', '21:00', '10:00', 'Quiet hours from 22:00.', 'Silence à partir de 22h.', 'NOT_ALLOWED', 'NOT_ALLOWED', '+243 800 000 005', 'book@kivu-guest-house.example', '["WIFI", "AIR_CONDITIONING", "KITCHEN", "SECURITY"]'),
    ('seed-prop-goma-virunga', 'Katindo, Goma', '14:00', NULL, '11:00', 'No smoking indoors.', 'Interdiction de fumer à l''intérieur.', 'ON_REQUEST', 'NOT_ALLOWED', '+243 800 000 006', 'suites@virunga.example', '["WIFI", "HOT_WATER", "TV"]'),
    ('seed-prop-goma-paused', 'Himbi, Goma', '14:00', NULL, '10:00', '', '', 'ON_REQUEST', 'NOT_ALLOWED', '', '', '["WIFI", "FAN"]'),
    ('seed-prop-goma-archived', '', NULL, NULL, NULL, '', '', NULL, NULL, '', '', '[]'),
    ('seed-prop-kin-gombe', 'Gombe, Kinshasa', '14:00', '22:00', '10:00', 'No parties.', 'Pas de fêtes.', 'NOT_ALLOWED', 'NOT_ALLOWED', '+243 810 000 001', 'hello@gombe-river.example', '["WIFI", "AIR_CONDITIONING", "SECURITY"]'),
    ('seed-prop-kin-draft', '', NULL, NULL, NULL, '', '', NULL, NULL, '', '', '[]'),
    ('seed-prop-bill-goma', 'Himbi, Goma', '14:00', NULL, '11:00', '', '', 'NOT_ALLOWED', 'NOT_ALLOWED', '', '', '["WIFI", "WORKSPACE"]'),
    ('seed-prop-bill-kin', 'Gombe, Kinshasa', '15:00', NULL, '11:00', '', '', 'NOT_ALLOWED', 'ALLOWED', '', '', '["WIFI", "AIR_CONDITIONING"]'),
    ('seed-prop-bill-kin-paused', 'Gombe, Kinshasa', '15:00', NULL, '11:00', '', '', 'NOT_ALLOWED', 'NOT_ALLOWED', '', '', '[]')
) AS d(id, neighborhood, check_in_from, check_in_until, check_out_by, rules_en, rules_fr, pets, smoking, phone, email, amenities)
WHERE p.id = d.id AND p.neighborhood = '' AND p."checkInFrom" IS NULL AND p.amenities = '[]'::jsonb;
