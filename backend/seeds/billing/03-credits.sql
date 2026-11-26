WITH entries AS (
    SELECT * FROM (VALUES
        ('seed-credit-goma-topup-1', 'GOMA', 'seed-host-goma-lake', 'TOPUP', 400000.00, NULL::text, '2026-06-02T09:00:00Z'::timestamptz, 'CASH_RECEIVED'::text, 'seed-cash-receipt-0602'::text, 'Opening credit paid in cash at the office.'::text, 'seed-staff-billing'::text),
        ('seed-credit-goma-promo-1', 'GOMA', 'seed-host-goma-lake', 'TOPUP', 100000.00, NULL, '2026-09-01T09:00:00Z', 'PROMOTIONAL_CREDIT', NULL, 'Launch promotion for the first quarter.', 'seed-staff-billing'),
        ('seed-credit-goma-topup-2', 'GOMA', 'seed-host-goma-lake', 'TOPUP', 300000.00, NULL, '2026-09-20T10:30:00Z', 'MOBILE_MONEY_RECEIVED', 'seed-momo-ref-0920', NULL, 'seed-staff-billing'),
        ('seed-credit-goma-topup-3', 'GOMA', 'seed-host-goma-lake', 'TOPUP', 250000.00, NULL, '2026-10-28T08:15:00Z', 'BANK_TRANSFER', 'seed-bank-ref-1028', NULL, 'seed-staff-billing'),
        ('seed-credit-goma-adjust-debit', 'GOMA', 'seed-host-goma-lake', 'ADJUSTMENT', -5000.00, NULL, '2026-10-30T11:00:00Z', 'ERROR_CORRECTED', NULL, 'Duplicate top-up entry removed.', 'seed-staff-billing'),
        ('seed-credit-goma-adjust-credit', 'GOMA', 'seed-host-goma-lake', 'ADJUSTMENT', 10000.00, NULL, '2026-10-31T11:00:00Z', 'GOODWILL', NULL, 'Compensation for a platform outage.', 'seed-staff-billing'),
        ('seed-credit-goma-amendment', 'GOMA', 'seed-host-goma-lake', 'ADJUSTMENT', -2500.00, NULL, '2026-11-04T09:00:00Z', 'AMENDMENT', NULL, 'Stay amendment seed-res-bill-g9', NULL),
        ('seed-credit-commission-g1', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -32800.00, 'seed-res-bill-g1', '2026-09-03T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-g2', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -20800.00, 'seed-res-bill-g2', '2026-09-09T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-g3', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -52000.00, 'seed-res-bill-g3', '2026-09-27T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-g4', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -30000.00, 'seed-res-bill-g4', '2026-10-05T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-g7', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -28000.00, 'seed-res-bill-g7', '2026-10-10T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-g5', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -41000.00, 'seed-res-bill-g5', '2026-10-13T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-g8', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -20000.00, 'seed-res-bill-g8', '2026-10-14T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-g6', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -45000.00, 'seed-res-bill-g6', '2026-10-25T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-g9', 'GOMA', 'seed-host-goma-lake', 'COMMISSION', -18750.00, 'seed-res-bill-g9', '2026-11-03T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-refund-g7', 'GOMA', 'seed-host-goma-lake', 'REFUND', 28000.00, 'seed-res-bill-g7', '2026-10-13T07:03:40Z', 'RELEASE', NULL, NULL, NULL),
        ('seed-credit-refund-g8', 'GOMA', 'seed-host-goma-lake', 'REFUND', 10000.00, 'seed-res-bill-g8', '2026-10-24T10:00:00Z', 'RELEASE', NULL, NULL, NULL),
        ('seed-credit-goma-low-topup', 'GOMA', 'seed-host-goma-lowcredit', 'TOPUP', 60000.00, NULL, '2026-10-01T09:00:00Z', 'BANK_TRANSFER', 'seed-bank-ref-1001', NULL, 'seed-staff-billing'),
        ('seed-credit-goma-low-debit', 'GOMA', 'seed-host-goma-lowcredit', 'ADJUSTMENT', -25000.00, NULL, '2026-10-15T09:00:00Z', 'ERROR_CORRECTED', NULL, 'Top-up recorded twice by mistake.', 'seed-staff-billing'),
        ('seed-credit-goma-zero-topup', 'GOMA', 'seed-host-goma-zerocredit', 'TOPUP', 30000.00, NULL, '2026-09-10T09:00:00Z', 'CASH_RECEIVED', 'seed-cash-receipt-0910', NULL, 'seed-staff-billing'),
        ('seed-credit-goma-zero-debit', 'GOMA', 'seed-host-goma-zerocredit', 'ADJUSTMENT', -30000.00, NULL, '2026-10-20T09:00:00Z', 'CHARGEBACK_REVERSAL', NULL, 'Cash payment reversed by the bank.', 'seed-staff-billing'),
        ('seed-credit-kin-topup-1', 'KIN', 'seed-host-kin-river', 'TOPUP', 200000.00, NULL, '2026-08-05T09:00:00Z', 'BANK_TRANSFER', 'seed-bank-ref-kin-0805', NULL, 'seed-staff-billing'),
        ('seed-credit-commission-k2', 'KIN', 'seed-host-kin-river', 'COMMISSION', -24000.00, 'seed-res-bill-k2', '2026-10-02T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-commission-k3', 'KIN', 'seed-host-kin-river', 'COMMISSION', -30000.00, 'seed-res-bill-k3', '2026-10-09T09:00:00Z', 'BOOKING', NULL, NULL, NULL),
        ('seed-credit-kin-promo', 'KIN', 'seed-host-kin-paused', 'TOPUP', 50000.00, NULL, '2026-07-02T09:00:00Z', 'PROMOTIONAL_CREDIT', NULL, 'Welcome credit.', 'seed-staff-billing')
    ) AS t(id, city_id, "operatorId", "entryType", amount, "reservationId", "createdAt", "reasonCode", "paymentRef", note, "createdByUserId")
)
INSERT INTO stay_credit_entries (id, city_id, "operatorId", "entryType", amount, "balanceAfter", "reservationId", "reasonCode", "paymentRef", note, "createdByUserId", "createdAt")
SELECT id, city_id, "operatorId", "entryType", amount,
       SUM(amount) OVER (PARTITION BY "operatorId" ORDER BY "createdAt", id),
       "reservationId", "reasonCode", "paymentRef", note, "createdByUserId", "createdAt"
  FROM entries
ON CONFLICT DO NOTHING;

INSERT INTO stay_credit_accounts ("operatorId", city_id, balance)
SELECT "operatorId", city_id, SUM(amount) FROM stay_credit_entries WHERE "operatorId" LIKE 'seed-host-%' GROUP BY "operatorId", city_id
ON CONFLICT ("operatorId") DO UPDATE SET balance = EXCLUDED.balance, version = stay_credit_accounts.version + 1, "updatedAt" = now();

UPDATE stay_reservations r SET "commissionChargedAmount" = x.charged
  FROM (SELECT "reservationId", -SUM(amount) AS charged FROM stay_credit_entries
         WHERE "reservationId" LIKE 'seed-res-bill-%' GROUP BY "reservationId") x
 WHERE r.id = x."reservationId";
