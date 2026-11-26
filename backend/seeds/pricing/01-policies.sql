INSERT INTO stay_policy_versions (id, kind, "versionNumber", content, "effectiveFrom")
VALUES
    ('seed-policy-cancel-flex-v1', 'CANCELLATION', 1, '{"freeUntilHours": 48, "penaltyPercent": 0}', '2026-01-01T00:00:00Z'),
    ('seed-policy-cancel-strict-v1', 'CANCELLATION', 2, '{"freeUntilHours": 168, "penaltyPercent": 50}', '2026-01-01T00:00:00Z'),
    ('seed-policy-cancel-moderate-v1', 'CANCELLATION', 3, '{"freeUntilHours": 72, "penaltyPercent": 25}', '2026-01-01T00:00:00Z'),
    ('seed-policy-pay-prepay-v1', 'PAYMENT', 1, '{"collection": "PREPAY"}', '2026-01-01T00:00:00Z'),
    ('seed-policy-pay-at-property-v1', 'PAYMENT', 2, '{"collection": "PAY_AT_PROPERTY"}', '2026-01-01T00:00:00Z')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_policy_versions (id, kind, "versionNumber", content, "effectiveFrom", "tombstonedAt")
VALUES
    ('seed-policy-cancel-retired-v0', 'CANCELLATION', 4, '{"freeUntilHours": 0, "penaltyPercent": 100}', '2025-01-01T00:00:00Z', '2026-01-01T00:00:00Z')
ON CONFLICT (id) DO NOTHING;
