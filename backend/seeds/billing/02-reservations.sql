INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, source, "bookingMode", currency, "totalAmount", "contractSnapshot", "commissionRateBps", "commissionAmount", "cancelledAt", "cancellationReason", "cancellationChargeAmount")
VALUES
    ('seed-res-bill-g1', 'STY-BILL-0001', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 410000.00, '{}', 800, 32800.00, NULL, NULL, 0),
    ('seed-res-bill-g2', 'STY-BILL-0002', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 260000.00, '{}', 800, 20800.00, NULL, NULL, 0),
    ('seed-res-bill-g3', 'STY-BILL-0003', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 520000.00, '{}', 1000, 52000.00, NULL, NULL, 0),
    ('seed-res-bill-g4', 'STY-BILL-0004', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 300000.00, '{}', 1000, 30000.00, NULL, NULL, 0),
    ('seed-res-bill-g5', 'STY-BILL-0005', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 410000.00, '{}', 1000, 41000.00, NULL, NULL, 0),
    ('seed-res-bill-g6', 'STY-BILL-0006', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'CONFIRMED', 'DIRECT', 'INSTANT', 'CDF', 360000.00, '{}', 1250, 45000.00, NULL, NULL, 0),
    ('seed-res-bill-g7', 'STY-BILL-0007', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'CANCELLED', 'DIRECT', 'INSTANT', 'CDF', 280000.00, '{}', 1000, 28000.00, '2026-10-13T07:03:40Z', 'GUEST_REQUEST', 0),
    ('seed-res-bill-g8', 'STY-BILL-0008', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'NO_SHOW', 'DIRECT', 'INSTANT', 'CDF', 200000.00, '{}', 1000, 20000.00, NULL, NULL, 100000.00),
    ('seed-res-bill-g9', 'STY-BILL-0009', 'GOMA', 'seed-prop-bill-goma', 'seed-host-goma-lake', 'seed-guest-bill-1', 'CHECKED_IN', 'DIRECT', 'INSTANT', 'CDF', 150000.00, '{}', 1250, 18750.00, NULL, NULL, 0),
    ('seed-res-bill-k1', 'STY-BILL-0101', 'KIN', 'seed-prop-bill-kin', 'seed-host-kin-paused', 'seed-guest-bill-2', 'CONFIRMED', 'DIRECT', 'INSTANT', 'CDF', 180000.00, '{}', 0, 0, NULL, NULL, 0),
    ('seed-res-bill-k2', 'STY-BILL-0102', 'KIN', 'seed-prop-bill-kin', 'seed-host-kin-river', 'seed-guest-bill-2', 'CONFIRMED', 'DIRECT', 'INSTANT', 'CDF', 240000.00, '{}', 1000, 24000.00, NULL, NULL, 0),
    ('seed-res-bill-k3', 'STY-BILL-0103', 'KIN', 'seed-prop-bill-kin', 'seed-host-kin-river', 'seed-guest-bill-2', 'CHECKED_OUT', 'DIRECT', 'INSTANT', 'CDF', 300000.00, '{}', 1000, 30000.00, NULL, NULL, 0)
ON CONFLICT (id) DO NOTHING;
