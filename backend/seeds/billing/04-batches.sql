INSERT INTO stay_commission_batches (id, city_id, reference, "operatorId", "periodFrom", "periodTo", "reservationCount", "commissionTotal", state, "paymentRef", "reasonCode", "createdByUserId", "stateChangedByUserId", version, "createdAt", "updatedAt")
VALUES
    ('seed-batch-goma-paid', 'GOMA', 'SEED-CB-GOMA-01', 'seed-host-goma-lake', '2026-09-01', '2026-09-15', 2, 53600.00, 'PAID', 'seed-bank-ref-batch-1', NULL, 'seed-staff-billing', 'seed-staff-billing', 3, '2026-09-16T08:00:00Z', '2026-09-20T08:00:00Z'),
    ('seed-batch-goma-approved', 'GOMA', 'SEED-CB-GOMA-02', 'seed-host-goma-lake', '2026-09-16', '2026-10-15', 2, 82000.00, 'APPROVED', NULL, NULL, 'seed-staff-billing', 'seed-staff-billing', 2, '2026-10-16T08:00:00Z', '2026-10-18T08:00:00Z'),
    ('seed-batch-goma-pending', 'GOMA', 'SEED-CB-GOMA-03', 'seed-host-goma-lake', '2026-10-16', '2026-10-31', 1, 41000.00, 'PENDING', NULL, NULL, 'seed-staff-billing', NULL, 1, '2026-11-01T08:00:00Z', '2026-11-01T08:00:00Z'),
    ('seed-batch-kin-failed', 'KIN', 'SEED-CB-KIN-01', 'seed-host-kin-river', '2026-10-01', '2026-10-31', 1, 30000.00, 'FAILED', NULL, 'PAYMENT_REJECTED', 'seed-staff-billing', 'seed-staff-billing', 3, '2026-11-01T08:00:00Z', '2026-11-05T08:00:00Z')
ON CONFLICT (id) DO NOTHING;

INSERT INTO stay_commission_batch_items (id, city_id, "batchId", "reservationId", "operatorId", "rateBps", "baseAmount", "commissionAmount", "earnedAt")
VALUES
    ('seed-batch-item-g1', 'GOMA', 'seed-batch-goma-paid', 'seed-res-bill-g1', 'seed-host-goma-lake', 800, 410000.00, 32800.00, '2026-09-10T12:00:00Z'),
    ('seed-batch-item-g2', 'GOMA', 'seed-batch-goma-paid', 'seed-res-bill-g2', 'seed-host-goma-lake', 800, 260000.00, 20800.00, '2026-09-16T12:00:00Z'),
    ('seed-batch-item-g3', 'GOMA', 'seed-batch-goma-approved', 'seed-res-bill-g3', 'seed-host-goma-lake', 1000, 520000.00, 52000.00, '2026-10-04T12:00:00Z'),
    ('seed-batch-item-g4', 'GOMA', 'seed-batch-goma-approved', 'seed-res-bill-g4', 'seed-host-goma-lake', 1000, 300000.00, 30000.00, '2026-10-12T12:00:00Z'),
    ('seed-batch-item-g5', 'GOMA', 'seed-batch-goma-pending', 'seed-res-bill-g5', 'seed-host-goma-lake', 1000, 410000.00, 41000.00, '2026-10-20T12:00:00Z'),
    ('seed-batch-item-k3', 'KIN', 'seed-batch-kin-failed', 'seed-res-bill-k3', 'seed-host-kin-river', 1000, 300000.00, 30000.00, '2026-10-15T12:00:00Z')
ON CONFLICT (id) DO NOTHING;
