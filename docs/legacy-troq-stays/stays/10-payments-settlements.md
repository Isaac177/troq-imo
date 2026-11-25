# Payments, refunds, ledger and settlements

Dependencies: D06-D11/D19, P/Q. Existing payment integration is order-oriented; contract extension is mandatory, not optional.

## F01: Reservation payment contract

Introduce a verified domain reference such as resourceType=STAY_RESERVATION plus resourceId and authoritative city, retaining backward compatibility for retail orders. Validate the reference server-to-server. Never reuse unrelated order IDs or labels such as fake orders to route accommodation payments.

Payment obligation captures payer, beneficiary/operator, reservation/item allocation, amount/currency, due time, kind, accepted contract version and idempotency identity. Attempts are children of an obligation; retrying an attempt must not create a second obligation.

All amounts are CDF exact values. Validate provider amount/currency/reference before accepting success. Price, commission and beneficiary originate from trusted domain agreements, not client payloads.

## F02: Payment schedules and outcomes

Payment states: CREATED, PENDING, SUCCEEDED, FAILED, CANCELLED; UNKNOWN outcome remains a reconcilable pending operation with explicit provider status, not a failure assumption. Refund states: REQUESTED, APPROVED, PROCESSING, SUCCEEDED, FAILED, REJECTED. Payout states: PREPARED, HELD, APPROVED, PROCESSING, PAID, FAILED, CANCELLED.

Support only schedules approved in D06: prepay, installment deposit/balance or pay-at-property. Model due obligations independently. Overdue action follows accepted terms, notification deadlines and inventory policy. Do not imply card authorization/capture or automated debit capabilities unless verified.

Payment UI: method availability, due amount, provider handoff state, pending verification, retry guidance and reconciled result. Never render a locally optimistic success receipt.

## F03: Financial ledger

Use the existing ledger if it can preserve the required balanced accounting; otherwise design a compatible extension. Record cash/provider clearing, guest liabilities, operator payable, platform fees/commission, refunds and adjustments with immutable journal references.

Every journal operation balances in CDF. Use reversal/compensating entries; do not overwrite booked financial history. Allocate discounts/fees/refunds to reservation items to support partial cancellation. Record who funds a discount. Keep booked value, collected cash, earned revenue and payable balance distinct.

Ledger posting and event outbox are atomic within payment-service database. Consumer deduplication prevents repeated ledger entries. Never increment balances solely from an unverified incoming event.

## F04: Refunds and damage deposits

Refund request includes evaluated entitlement, original collected transaction references, requested amount, reason, policy version and request actor. Refunded plus reserved-for-refund amounts cannot exceed eligible collection. Partial refunds serialize against that balance.

Policy-conforming automatic refunds and exceptional approved refunds must have explicit authority rules. Enforce request/approval separation and limits from D19. Approval rechecks ledger/version. Provider timeout triggers lookup/reconciliation before resubmission.

Damage deposits use separate obligation/ledger treatment. Collection, release and claims depend on D09 and supported provider methods. Claims require evidence, response deadlines, decision history and appeal; never silently deduct arbitrary damage charges.

## F05: Settlements and payouts

Eligibility combines accepted agreement, completed required stay milestone, cleared collection, existing refunds, disputes, reserves and verified payout destination. Aggregate only eligible ledger entries. Snapshot settlement composition so retries cannot pay the same entry twice.

Changing payout details requires STAY_MANAGE_PAYOUT_DETAILS, step-up policy and change audit. Existing prepared payouts must explicitly revalidate or retain their approved destination according to D19; never switch beneficiary silently.

Separate preparation, approval and dispatch. Persist provider idempotency reference before external call. Unknown outcome requires status lookup. Payout failure leaves a visible liability; it does not erase the owed amount.

Refund after payout creates a recoverable operator liability/offset or platform-funded exception according to D07-D10. Pay-at-property requires commission collection and proof/reconciliation; a manager checkbox is not provider settlement evidence.

## F06: Reconciliation and UI

Operator finance shows gross accommodation charges, discounts, fees, commission, collected, refundable, held and available-for-settlement amounts; drill down to ledger-linked statements. Admin sees unresolved differences and action ownership.

Scheduled reconciliation compares provider records, attempts, ledger postings, reservation obligations and payouts. Detect missing/duplicate callbacks, orphan payments, mismatched amounts and stalled refunds. Fix through idempotent domain operations with audit, not direct SQL.

Receipt reflects money actually collected; voucher reflects reservation terms. Invoice/tax behavior is D11-gated. PDF/download links are private and short-lived.

## Seeds and acceptance

Fixtures: paid booking, partial balance, failed attempt then success, unknown outcome, duplicate webhook, partial refund, refund failure, deposit claim, held payout, paid payout followed by refund, pay-at-property commission due.

Test balanced journals, rounding, duplicate/out-of-order notifications, concurrent partial refunds, payout idempotency, revoked approval, changed destination, provider outage and crash between request/send/response. Reconciliation must recover each crash window without duplicate charge or payout.
