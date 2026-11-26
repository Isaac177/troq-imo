INSERT INTO stay_rate_plans (id, city_id, "propertyId", "unitTypeId", "occupancyBasis", "mealPlan", "cancellationPolicyVersionId", "paymentPolicyVersionId", "bookingMode", status, "childNightlyAmount")
VALUES ('seed-rate-goma-villa-pay-property', 'GOMA', 'seed-prop-goma-lakeside', 'seed-unit-type-goma-villa', 'PER_UNIT', 'BREAKFAST', 'seed-policy-cancel-flex-v1', 'seed-policy-pay-at-property-v1', 'INSTANT', 'ACTIVE', 20000.00)
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_rate_rules (id, city_id, "ratePlanId", "startDate", "endDate", "weekdayMask", "adjustmentType", amount, priority)
VALUES ('seed-rule-goma-villa-pay-property', 'GOMA', 'seed-rate-goma-villa-pay-property', NULL, NULL, 127, 'OVERRIDE', 190000.00, 0)
ON CONFLICT (id) DO NOTHING;
