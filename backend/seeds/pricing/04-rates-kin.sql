INSERT INTO stay_rate_plans (id, city_id, "propertyId", "unitTypeId", "occupancyBasis", "mealPlan", "cancellationPolicyVersionId", "paymentPolicyVersionId", "bookingMode", status, "childNightlyAmount")
VALUES
    ('seed-rate-kin-apartment-instant', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'PER_UNIT', 'ROOM_ONLY', 'seed-policy-cancel-flex-v1', 'seed-policy-pay-prepay-v1', 'INSTANT', 'ACTIVE', 15000.00),
    ('seed-rate-kin-apartment-guest', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'PER_GUEST', 'BREAKFAST', 'seed-policy-cancel-moderate-v1', 'seed-policy-pay-at-property-v1', 'REQUEST', 'ACTIVE', 10000.00),
    ('seed-rate-kin-apartment-draft', 'KIN', 'seed-prop-kin-gombe', 'seed-unit-type-kin-apartment', 'PER_UNIT', 'ROOM_ONLY', 'seed-policy-cancel-strict-v1', 'seed-policy-pay-prepay-v1', 'INSTANT', 'DRAFT', 0.00)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_rate_rules (id, city_id, "ratePlanId", "startDate", "endDate", "weekdayMask", "adjustmentType", amount, priority)
VALUES
    ('seed-rule-kin-apartment-base', 'KIN', 'seed-rate-kin-apartment-instant', NULL, NULL, 127, 'OVERRIDE', 150000.00, 0),
    ('seed-rule-kin-apartment-weekend-pct', 'KIN', 'seed-rate-kin-apartment-instant', NULL, NULL, 96, 'PERCENTAGE', 12.50, 4),
    ('seed-rule-kin-apartment-guest-base', 'KIN', 'seed-rate-kin-apartment-guest', NULL, NULL, 127, 'OVERRIDE', 60000.00, 0),
    ('seed-rule-kin-apartment-guest-season', 'KIN', 'seed-rate-kin-apartment-guest', '2026-12-20', '2027-01-03', 127, 'OVERRIDE', 80000.00, 6)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_promotions (id, city_id, "propertyId", "ratePlanId", kind, "minNights", "discountPercent", "capAmount", "startDate", "endDate", priority, status, "redemptionLimit")
VALUES
    ('seed-promo-kin-apartment-long', 'KIN', 'seed-prop-kin-gombe', 'seed-rate-kin-apartment-instant', 'LENGTH_OF_STAY', 3, 5.00, 30000.00, NULL, NULL, 10, 'ACTIVE', NULL),
    ('seed-promo-kin-apartment-month', 'KIN', 'seed-prop-kin-gombe', NULL, 'LENGTH_OF_STAY', 28, 20.00, NULL, NULL, '2026-12-31', 3, 'PAUSED', 5)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_promotion_funding (city_id, "promotionId", "fundedBy", "updatedByUserId")
VALUES
    ('KIN', 'seed-promo-kin-apartment-long', 'PLATFORM', 'seed-user-kin-host')
ON CONFLICT ("promotionId") DO NOTHING;
