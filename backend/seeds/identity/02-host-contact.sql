UPDATE hosts
SET guest_contact_phone = CASE WHEN guest_contact_phone = '' THEN '+243 800 000 101' ELSE guest_contact_phone END,
    guest_contact_email = CASE WHEN guest_contact_email = '' THEN 'lake@example.com' ELSE guest_contact_email END
WHERE id = 'seed-host-goma-lake';

UPDATE hosts
SET guest_contact_phone = CASE WHEN guest_contact_phone = '' THEN '+243 800 000 102' ELSE guest_contact_phone END,
    guest_contact_email = CASE WHEN guest_contact_email = '' THEN 'river@example.com' ELSE guest_contact_email END
WHERE id = 'seed-host-kin-river';
