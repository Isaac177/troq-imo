INSERT INTO stay_property_revisions
    (id, "propertyId", "revisionNumber", snapshot, "authorUserId", "reviewStatus", "submittedAt", "submittedByUserId", "reviewedAt", "reviewedByUserId", "reviewReasons", "reviewGuidance")
SELECT v.id, p.id, 1,
    jsonb_build_object(
        'nameEn', p."nameEn", 'nameFr', p."nameFr",
        'descriptionEn', p."descriptionEn", 'descriptionFr', p."descriptionFr",
        'neighborhood', p.neighborhood, 'addressLine', p."addressLine",
        'latitude', p.latitude, 'longitude', p.longitude,
        'checkInFrom', p."checkInFrom", 'checkInUntil', p."checkInUntil", 'checkOutBy', p."checkOutBy",
        'houseRulesEn', p."houseRulesEn", 'houseRulesFr', p."houseRulesFr",
        'petsPolicy', p."petsPolicy", 'smokingPolicy', p."smokingPolicy",
        'guestContactPhone', p."guestContactPhone", 'guestContactEmail', p."guestContactEmail", 'amenities', p.amenities
    ), v.author_id, v.status,
    CASE WHEN v.status = 'DRAFT' THEN NULL ELSE now() END,
    CASE WHEN v.status = 'DRAFT' THEN NULL ELSE v.author_id END,
    CASE WHEN v.status IN ('CHANGES_REQUESTED', 'APPROVED', 'REJECTED') THEN now() ELSE NULL END,
    CASE WHEN v.status IN ('CHANGES_REQUESTED', 'APPROVED', 'REJECTED') THEN 'seed-platform-reviewer' ELSE NULL END,
    CASE WHEN v.status = 'CHANGES_REQUESTED' THEN '["MEDIA_UNSUITABLE"]'::jsonb ELSE '[]'::jsonb END,
    CASE WHEN v.status = 'CHANGES_REQUESTED' THEN 'Add a clearer cover photo.' ELSE NULL END
FROM (VALUES
    ('seed-revision-kin-draft', 'seed-prop-kin-draft', 'seed-user-kin-host', 'DRAFT'),
    ('seed-revision-goma-submitted', 'seed-prop-goma-ready', 'seed-user-goma-host', 'SUBMITTED'),
    ('seed-revision-goma-changes', 'seed-prop-goma-virunga', 'seed-user-goma-host', 'CHANGES_REQUESTED'),
    ('seed-revision-goma-approved', 'seed-prop-goma-lakeside', 'seed-user-goma-host', 'APPROVED'),
    ('seed-revision-goma-rejected', 'seed-prop-goma-archived', 'seed-user-goma-host', 'REJECTED')
) AS v(id, property_id, author_id, status)
JOIN stay_properties p ON p.id = v.property_id
ON CONFLICT DO NOTHING;
