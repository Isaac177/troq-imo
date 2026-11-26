INSERT INTO stay_cases (id, city_id, "operatorId", "reservationId", "caseType", parties, state, "deadlineAt", resolution, reason, "claimAmount", "closedAt", "assignedToUserId", "assignedAt", "escalatedAt", "escalatedByUserId", "escalationReason")
VALUES
    ('seed-case-goma-open', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-checkedout-reviewed', 'GUEST_COMPLAINT', '{"guestUserId":"seed-guest-goma-1","openedBy":"GUEST","openedByUserId":"seed-guest-goma-1"}', 'OPEN', '2026-12-20T18:00:00Z', NULL, 'The hot water stopped on the second night and nobody came to fix it.', NULL, NULL, NULL, NULL, NULL, NULL, NULL),
    ('seed-case-goma-awaiting', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-checkedout-pending', 'GUEST_COMPLAINT', '{"guestUserId":"seed-guest-goma-2","openedBy":"GUEST","openedByUserId":"seed-guest-goma-2"}', 'AWAITING_RESPONSE', '2026-11-09T18:00:00Z', NULL, 'The villa was advertised with a lake view but the room faced the parking lot.', NULL, NULL, 'seed-dev-admin', '2026-11-05T09:00:00Z', NULL, NULL, NULL),
    ('seed-case-goma-closed', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-checkedout-hidden', 'GENERAL', '{"guestUserId":"seed-guest-goma-3","openedBy":"GUEST","openedByUserId":"seed-guest-goma-3"}', 'CLOSED', '2026-10-05T18:00:00Z', '"A late checkout was agreed with the guest and the extra night was waived."', 'Question about the late checkout fee on the final invoice.', NULL, '2026-10-03T10:00:00Z', NULL, NULL, NULL, NULL, NULL),
    ('seed-case-goma-resolved', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-confirmed', 'GUEST_COMPLAINT', '{"guestUserId":"seed-guest-goma-1","openedBy":"GUEST","openedByUserId":"seed-guest-goma-1"}', 'RESOLVED', '2026-10-12T18:00:00Z', '"Refund of the cleaning fee was issued."', 'The villa was not cleaned before arrival.', NULL, '2026-10-10T09:00:00Z', NULL, NULL, NULL, NULL, NULL),
    ('seed-case-goma-operator', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-cancelled', 'OPERATOR_COMPLAINT', '{"guestUserId":"seed-guest-goma-3","openedBy":"OPERATOR","openedByUserId":"seed-user-goma-host"}', 'OPEN', '2026-11-25T18:00:00Z', NULL, 'The guest cancelled twice after the deadline and left damage on arrival.', NULL, NULL, 'seed-dev-admin', '2026-11-04T14:00:00Z', '2026-11-06T08:30:00Z', 'seed-dev-admin', 'The business has not answered after two reminders and the guest is asking for a refund.'),
    ('seed-case-goma-platform', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-noshow', 'GENERAL', '{"guestUserId":"seed-guest-goma-1","openedBy":"PLATFORM","openedByUserId":"seed-dev-admin"}', 'OPEN', '2026-12-02T18:00:00Z', NULL, 'Guest disputes the no-show charge and says the property gate was locked.', NULL, NULL, NULL, NULL, NULL, NULL, NULL),
    ('seed-claim-open-evidence', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-checkedout-reviewed', 'DAMAGE_CLAIM', '{"guestUserId":"seed-guest-goma-1","openedBy":"OPERATOR","openedByUserId":"seed-user-goma-host"}', 'OPEN', '2026-12-30T18:00:00Z', NULL, 'The bathroom mirror was broken and the sofa has burn marks.', 120000.00, NULL, NULL, NULL, NULL, NULL, NULL),
    ('seed-claim-open-bare', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-checkedout-pending', 'DAMAGE_CLAIM', '{"guestUserId":"seed-guest-goma-2","openedBy":"OPERATOR","openedByUserId":"seed-user-goma-host"}', 'AWAITING_RESPONSE', '2026-11-25T18:00:00Z', NULL, 'Two towels and a kettle are missing from the villa.', 85000.00, NULL, NULL, NULL, NULL, NULL, NULL),
    ('seed-claim-approved', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-checkedout-hidden', 'DAMAGE_CLAIM', '{"guestUserId":"seed-guest-goma-3","openedBy":"OPERATOR","openedByUserId":"seed-user-goma-host"}', 'RESOLVED', '2026-10-20T18:00:00Z', '{"decision":"APPROVED","approvedAmount":"60000.00","reasonCode":"DAMAGE_CONFIRMED"}', 'A bedside lamp was shattered during the stay.', 60000.00, '2026-10-14T10:00:00Z', NULL, NULL, NULL, NULL, NULL),
    ('seed-claim-partial-paid', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-owner-stay', 'DAMAGE_CLAIM', '{"guestUserId":"seed-user-goma-host","openedBy":"OPERATOR","openedByUserId":"seed-user-goma-host"}', 'RESOLVED', '2026-10-28T18:00:00Z', '{"decision":"PARTIAL","approvedAmount":"40000.00","reasonCode":"WEAR_AND_TEAR"}', 'The curtains were torn and the carpet is stained.', 100000.00, '2026-10-25T09:30:00Z', NULL, NULL, NULL, NULL, NULL),
    ('seed-claim-rejected', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-checkedout-pending', 'DAMAGE_CLAIM', '{"guestUserId":"seed-guest-goma-2","openedBy":"OPERATOR","openedByUserId":"seed-user-goma-host"}', 'RESOLVED', '2026-10-22T18:00:00Z', '{"decision":"REJECTED","approvedAmount":"0.00","reasonCode":"NO_EVIDENCE"}', 'A scratch on the dining table that the guest denies.', 50000.00, '2026-10-18T15:00:00Z', NULL, NULL, NULL, NULL, NULL),
    ('seed-claim-payout-failed', 'GOMA', 'seed-host-goma-lake', 'seed-res-goma-checkedout-reviewed', 'DAMAGE_CLAIM', '{"guestUserId":"seed-guest-goma-1","openedBy":"OPERATOR","openedByUserId":"seed-user-goma-host"}', 'RESOLVED', '2026-10-30T18:00:00Z', '{"decision":"APPROVED","approvedAmount":"30000.00","reasonCode":"EVIDENCE_SUFFICIENT"}', 'The shower curtain rail was pulled off the wall.', 30000.00, '2026-10-27T11:00:00Z', NULL, NULL, NULL, NULL, NULL)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_claim_decisions ("caseId", decision, "approvedAmount", "reasonCode", "decidedByUserId", "decidedAt", "payoutState", "payoutRef", "payoutUpdatedAt")
VALUES
    ('seed-claim-approved', 'APPROVED', 60000.00, 'DAMAGE_CONFIRMED', 'seed-dev-admin', '2026-10-14T10:00:00Z', 'PENDING', NULL, NULL),
    ('seed-claim-partial-paid', 'PARTIAL', 40000.00, 'WEAR_AND_TEAR', 'seed-dev-admin', '2026-10-25T09:30:00Z', 'PAID', 'seed-payout-0001', '2026-10-26T08:00:00Z'),
    ('seed-claim-rejected', 'REJECTED', 0.00, 'NO_EVIDENCE', 'seed-dev-admin', '2026-10-18T15:00:00Z', 'NOT_DUE', NULL, NULL),
    ('seed-claim-payout-failed', 'APPROVED', 30000.00, 'EVIDENCE_SUFFICIENT', 'seed-dev-admin', '2026-10-27T11:00:00Z', 'FAILED', NULL, '2026-10-28T09:00:00Z')
ON CONFLICT ("caseId") DO NOTHING;

INSERT INTO stay_claim_statements (id, "caseId", party, body, "recordedByUserId", "createdAt")
VALUES
    ('seed-claim-statement-open-host', 'seed-claim-open-evidence', 'HOST', 'We found the mirror in pieces on the morning of check-out and photographed it before cleaning.', 'seed-dev-admin', '2026-11-03T09:00:00Z'),
    ('seed-claim-statement-open-guest', 'seed-claim-open-evidence', 'GUEST', 'The mirror was already cracked when we arrived. The sofa marks were there too.', 'seed-dev-admin', '2026-11-04T14:20:00Z'),
    ('seed-claim-statement-rejected-host', 'seed-claim-rejected', 'HOST', 'The table has a deep scratch that was not there before.', 'seed-dev-admin', '2026-10-16T10:00:00Z'),
    ('seed-claim-statement-rejected-guest', 'seed-claim-rejected', 'GUEST', 'We did not touch the table. The scratch is visible in the listing photos.', 'seed-dev-admin', '2026-10-17T12:00:00Z')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_case_evidence (id, "caseId", "fileName", "mimeType", "sizeBytes", content, "addedByUserId")
VALUES
    ('seed-claim-evidence-mirror', 'seed-claim-open-evidence', 'mirror-photos.pdf', 'application/pdf', 15, decode('JVBERi0xLjQKJSVFT0YK', 'base64'), 'seed-user-goma-host'),
    ('seed-claim-evidence-invoice', 'seed-claim-open-evidence', 'repair-quote.pdf', 'application/pdf', 15, decode('JVBERi0xLjQKJSVFT0YK', 'base64'), 'seed-user-goma-host'),
    ('seed-claim-evidence-lamp', 'seed-claim-approved', 'lamp-receipt.pdf', 'application/pdf', 15, decode('JVBERi0xLjQKJSVFT0YK', 'base64'), 'seed-user-goma-host'),
    ('seed-case-evidence-hotwater', 'seed-case-goma-open', 'hot-water-photo.pdf', 'application/pdf', 15, decode('JVBERi0xLjQKJSVFT0YK', 'base64'), 'seed-guest-goma-1')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_reservation_messages (id, city_id, "reservationId", "senderRole", "senderUserId", body, "createdAt")
VALUES
    ('seed-msg-goma-confirmed-1', 'GOMA', 'seed-res-goma-confirmed', 'GUEST', 'seed-guest-goma-1', 'Could we arrange an early check-in on the 20th?', '2026-11-20T09:00:00Z'),
    ('seed-msg-goma-confirmed-2', 'GOMA', 'seed-res-goma-confirmed', 'OPERATOR', 'seed-user-goma-host', 'Early check-in from 12:00 is possible. We will confirm closer to the date.', '2026-11-20T10:30:00Z'),
    ('seed-msg-goma-checkedout-1', 'GOMA', 'seed-res-goma-checkedout-reviewed', 'GUEST', 'seed-guest-goma-1', 'The generator was off for an hour on the first night. Is that normal?', '2026-09-10T21:15:00Z'),
    ('seed-msg-goma-checkedout-2', 'GOMA', 'seed-res-goma-checkedout-reviewed', 'OPERATOR', 'seed-user-goma-host', 'Sorry about that. The generator was serviced the next morning and it will not recur.', '2026-09-11T07:45:00Z')
ON CONFLICT (id) DO NOTHING;
