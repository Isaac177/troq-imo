ALTER TABLE property_amenity_groups
    ADD COLUMN "nameEn" varchar(80) NOT NULL DEFAULT '',
    ADD COLUMN "nameFr" varchar(80) NOT NULL DEFAULT '';

ALTER TABLE property_amenities
    ADD COLUMN "nameEn" varchar(80) NOT NULL DEFAULT '',
    ADD COLUMN "nameFr" varchar(80) NOT NULL DEFAULT '';

UPDATE property_amenity_groups g SET
    "nameEn" = COALESCE((SELECT c.name FROM property_amenity_group_content c WHERE c.group_id = g.id AND c.locale = 'en'), ''),
    "nameFr" = COALESCE((SELECT c.name FROM property_amenity_group_content c WHERE c.group_id = g.id AND c.locale = 'fr'), '');

UPDATE property_amenities a SET
    "nameEn" = COALESCE((SELECT c.name FROM property_amenity_content c WHERE c.amenity_id = a.id AND c.locale = 'en'), ''),
    "nameFr" = COALESCE((SELECT c.name FROM property_amenity_content c WHERE c.amenity_id = a.id AND c.locale = 'fr'), '');
