INSERT INTO stay_rate_plans (id, city_id, "propertyId", "unitTypeId", "occupancyBasis", "mealPlan", "cancellationPolicyVersionId", "paymentPolicyVersionId", "bookingMode", status, "childNightlyAmount")
VALUES
    ('seed-rate-goma-villa-instant', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 'PER_UNIT', 'ROOM_ONLY', 'seed-policy-cancel-flex-v1', 'seed-policy-pay-prepay-v1', 'INSTANT', 'ACTIVE', 20000.00),
    ('seed-rate-goma-room-request', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-room', 'PER_GUEST', 'BREAKFAST', 'seed-policy-cancel-strict-v1', 'seed-policy-pay-prepay-v1', 'REQUEST', 'ACTIVE', 20000.00),
    ('seed-rate-goma-triple-instant', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-triple', 'PER_UNIT', 'ROOM_ONLY', 'seed-policy-cancel-flex-v1', 'seed-policy-pay-prepay-v1', 'INSTANT', 'ACTIVE', 20000.00),
    ('seed-rate-goma-villa-zz-gap', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 'PER_UNIT', 'ROOM_ONLY', 'seed-policy-cancel-flex-v1', 'seed-policy-pay-prepay-v1', 'INSTANT', 'ACTIVE', 0.00),
    ('seed-rate-goma-villa-draft', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 'PER_UNIT', 'HALF_BOARD', 'seed-policy-cancel-moderate-v1', 'seed-policy-pay-at-property-v1', 'INSTANT', 'DRAFT', 0.00),
    ('seed-rate-goma-villa-paused', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 'PER_UNIT', 'FULL_BOARD', 'seed-policy-cancel-strict-v1', 'seed-policy-pay-prepay-v1', 'REQUEST', 'PAUSED', 25000.00),
    ('seed-rate-goma-villa-archived', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 'PER_UNIT', 'ROOM_ONLY', 'seed-policy-cancel-flex-v1', 'seed-policy-pay-prepay-v1', 'INSTANT', 'ARCHIVED', 0.00),
    ('seed-rate-goma-ready-instant', 'GOMA', 'seed-prop-goma-ready', 'seed-unit-type-goma-ready-house', 'PER_UNIT', 'ROOM_ONLY', 'seed-policy-cancel-moderate-v1', 'seed-policy-pay-at-property-v1', 'INSTANT', 'ACTIVE', 0.00),
    ('seed-rate-goma-virunga-suite', 'GOMA', 'seed-prop-goma-virunga', 'seed-unit-type-goma-virunga-suite', 'PER_UNIT', 'ROOM_ONLY', 'seed-policy-cancel-flex-v1', 'seed-policy-pay-prepay-v1', 'INSTANT', 'ACTIVE', 0.00)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_rate_rules (id, city_id, "ratePlanId", "startDate", "endDate", "weekdayMask", "adjustmentType", amount, priority)
VALUES
    ('seed-rule-goma-villa-base', 'GOMA', 'seed-rate-goma-villa-instant', NULL, NULL, 127, 'OVERRIDE', 180000.00, 0),
    ('seed-rule-goma-villa-december', 'GOMA', 'seed-rate-goma-villa-instant', '2026-12-15', '2027-01-05', 127, 'AMOUNT', 25000.00, 10),
    ('seed-rule-goma-villa-weekend', 'GOMA', 'seed-rate-goma-villa-instant', NULL, NULL, 48, 'AMOUNT', 15000.00, 5),
    ('seed-rule-goma-villa-holiday-pct', 'GOMA', 'seed-rate-goma-villa-instant', '2026-12-24', '2026-12-26', 127, 'PERCENTAGE', 10.00, 12),
    ('seed-rule-goma-room-base', 'GOMA', 'seed-rate-goma-room-request', NULL, NULL, 127, 'OVERRIDE', 130000.00, 0),
    ('seed-rule-goma-triple-base', 'GOMA', 'seed-rate-goma-triple-instant', NULL, NULL, 127, 'OVERRIDE', 90000.00, 0),
    ('seed-rule-goma-villa-zz-gap-march', 'GOMA', 'seed-rate-goma-villa-zz-gap', '2027-03-01', '2027-03-31', 127, 'OVERRIDE', 160000.00, 0),
    ('seed-rule-goma-villa-paused-base', 'GOMA', 'seed-rate-goma-villa-paused', NULL, NULL, 127, 'OVERRIDE', 210000.00, 0),
    ('seed-rule-goma-villa-archived-base', 'GOMA', 'seed-rate-goma-villa-archived', NULL, NULL, 127, 'OVERRIDE', 150000.00, 0),
    ('seed-rule-goma-ready-base', 'GOMA', 'seed-rate-goma-ready-instant', NULL, NULL, 127, 'OVERRIDE', 90000.00, 0),
    ('seed-rule-goma-virunga-base', 'GOMA', 'seed-rate-goma-virunga-suite', NULL, NULL, 127, 'OVERRIDE', 150000.00, 0)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_promotions (id, city_id, "propertyId", "ratePlanId", kind, "minNights", "discountPercent", "capAmount", "startDate", "endDate", priority, status, "redemptionLimit")
VALUES
    ('seed-promo-goma-villa-week', 'GOMA', 'seed-prop-goma-lakeside', 'seed-rate-goma-villa-instant', 'LENGTH_OF_STAY', 7, 10.00, 50000.00, NULL, NULL, 10, 'ACTIVE', NULL),
    ('seed-promo-goma-room-paused', 'GOMA', 'seed-prop-goma-lakeside', NULL, 'LENGTH_OF_STAY', 3, 5.00, NULL, '2026-12-01', '2027-02-28', 5, 'PAUSED', NULL),
    ('seed-promo-goma-villa-limited', 'GOMA', 'seed-prop-goma-lakeside', 'seed-rate-goma-villa-instant', 'LENGTH_OF_STAY', 3, 15.00, 40000.00, '2026-11-01', '2027-03-31', 8, 'ACTIVE', 20)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_promotion_funding (city_id, "promotionId", "fundedBy", "updatedByUserId")
VALUES
    ('GOMA', 'seed-promo-goma-villa-week', 'PLATFORM', 'seed-user-goma-host'),
    ('GOMA', 'seed-promo-goma-room-paused', 'OPERATOR', 'seed-user-goma-host')
ON CONFLICT ("promotionId") DO NOTHING;
