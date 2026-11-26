# Payments and billing

Default accepted by the owner: D5. Mungini has its own `billing` module that ports commission and credits. Provider code is copied from `payment-service`, never shared. Sources: `stay-service/src/{commission,credits,admin_commission,quotes/schedule.rs,reservations}`, migrations `00047` to `00052` of `stay-service`, and `backend/crates/domain/src/billing` in Mungini.

## Summary

| Money | Moves where today | Who holds it |
|---|---|---|
| Stay price, cancellation charge, damage claim | Guest pays the property directly | The host. The platform never holds, collects or refunds it |
| Commission | Deducted from the host's prepaid credit balance | The platform, as a ledger |
| Credit top-ups | Staff record cash, mobile money or bank transfer received outside the system | The platform, as a ledger |
| Commission settlements | Not implemented (see below) | Nobody |
| Damage claim payout | Staff record a payout state and reference | No system moves it |

`payment-service` has no Stay reference. No payment provider is called by any Stay code path.

## Payment at the property

- A quote's payment schedule is always: due now 0, due later 0, due at property the whole total.
- Reservations have a `paymentRef` column that nothing writes. The search filter accepts `PREPAY` and `PAY_AT_PROPERTY` as payment collection codes, but no code sets `PREPAY`.
- The payment policy version attached to a rate plan has kind `PAYMENT`; its content is stored and shown, and the guest accepts it at booking. The code does not interpret it.
- No deposit and no refund flow exist. A cancellation charge is recorded on the reservation and due at the property.

## Commission

- Rate: per host, in basis points (0 to 10000), kept as history in `stay_operator_commission_rates` (`rateBps`, `effectiveFrom`, reason, note, who changed it). The latest row effective now wins. With no row, the city setting `DEFAULT_COMMISSION_BPS` applies (default 0).
- Rate change reasons: `CONTRACT_AGREED`, `RENEGOTIATED`, `LAUNCH_OFFER`, `PLATFORM_POLICY`, `ERROR_CORRECTED`.
- Snapshot at booking: the reservation stores `commissionRateBps`, `commissionAmount` (rate of the total) and `operatorNetAmount` (total minus commission). Rates are applied with two decimals. Later rate changes do not alter an existing reservation.

## Credits

- Each host has one account (`stay_credit_accounts.balance`) and an append-only ledger (`stay_credit_entries`) with `balanceAfter`.
- Entry types: `TOPUP` (positive), `COMMISSION` (negative), `REFUND` (positive), `ADJUSTMENT` (either sign). At most one `COMMISSION` and one `REFUND` per reservation.
- Top-up reasons: `CASH_RECEIVED`, `MOBILE_MONEY_RECEIVED`, `BANK_TRANSFER`, `PROMOTIONAL_CREDIT`. Adjustment reasons: `ERROR_CORRECTED`, `GOODWILL`, `CHARGEBACK_REVERSAL`. Only staff write them.
- Commission is taken when a reservation becomes `CONFIRMED` (instant booking, or host acceptance). It is released (a `REFUND` entry) when a reservation expires, is cancelled by the host (full release), or is cancelled by the guest (everything except the commission on the penalty share). A no-show keeps the commission. An amendment writes an `ADJUSTMENT` for the change, and fails with `insufficient credit` when the host cannot cover an increase.
- Booking gate: a host with a rate above 0 cannot be booked when the balance is at or below `MIN_CREDIT_CDF` (default 0). The listing then reads as unavailable. A booking also fails when the balance is below the commission it would take. A host at rate 0 is never blocked.
- `LOW_CREDIT_CDF` (default 50 000 CDF) marks an account as low for staff and host views.
- City settings in `stay_platform_settings` (per city, with reason codes `PLATFORM_POLICY`, `MARKET_CONDITIONS`, `ERROR_CORRECTED`): `DEFAULT_COMMISSION_BPS`, `MIN_CREDIT_CDF`, `LOW_CREDIT_CDF` and the non-billing windows listed in `reservations-lifecycle.md`.

## Settlements

The schema has `stay_commission_batches` (reference, host, period, count, total, state `PENDING`, `APPROVED`, `PAID`, `FAILED`, payment reference) and `stay_commission_batch_items` (one per reservation). No Rust or front-end code reads or writes them, and the permissions `STAY_MANAGE_SETTLEMENTS`, `STAY_APPROVE_PAYOUTS` and `STAY_APPROVE_REFUNDS` guard no endpoint. The code therefore defines no settlement flow. Under the prepaid credit model the platform already holds the commission, so what a settlement would pay out is not defined. Do not port it without an owner decision.

## What Mungini keeps (D5 default)

- Commission rates and history, the snapshot on the reservation, the credit ledger, the booking gate, staff top-ups and adjustments, city settings.
- Credits and commission operate in the same transaction as the reservation change (`reservations/links/billing.rs` re-exports `take_commission`, `release_commission`, `release_for_penalty`, `sync_amended_commission` and `ensure_can_book`).
- Mungini state: `domain/src/billing` has commission snapshot and rate, credit account, charge, gate, ledger and state code, amounts, settings and validators. Admin and HTTP folders are empty. Migration `00018_billing.sql` creates the rate history, credit account and entry tables and also the two commission batch tables, so the unimplemented settlement schema was ported as is.

## Unverified

- Whether Mungini needs online guest payment later. Nothing in the code prepares it.
- Whether staff top-ups should be replaced by a provider checkout for hosts.
- What the permissions `billing.request_refunds`, `billing.approve_refunds`, `billing.payout_details` and `billing.approve_payouts` are for, since no refund or payout exists.
- Settlement semantics (above).
- Tax and invoicing for commission.
