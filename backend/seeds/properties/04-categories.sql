INSERT INTO stay_accommodation_categories (code, "nameEn", "nameFr", icon, "sortOrder", active) VALUES
    ('SEED_TREEHOUSE', 'Treehouse', 'Cabane dans les arbres', 'Trees', 110, true),
    ('SEED_RETIRED_CABIN', 'Retired cabin', 'Cabane retirée', 'Tent', 120, false)
ON CONFLICT (code) DO NOTHING;
