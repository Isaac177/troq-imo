# Payment at the property: design for deposits, damage claims and cancellation charges

Status: draft for owner review. No code yet for damage claims.
Decisions it relies on: D06 (pay at the property), D08 (cancellation policy), D09 (deposits), D19 (refunds). See `00-decisions.md`.

## 1. What is settled

- All money for a stay is paid at the property (D06). Nothing is charged online.
- The platform never holds, collects or refunds money for a stay. It records what is owed and the documents that say so.
- Amounts are CDF only. Stored as `numeric(14,2)`.
- No damage deposit is collected by default (D09). Accommodation price and any deposit are kept separate.
- Refunds are not created under this model, so no refund flow exists (D19).

## 2. What the platform records and sends

| Obligation | Created when | Amount due | Document sent to guest |
|---|---|---|---|
| Accommodation | Booking confirmed | Whole stay total, at the property | Booking voucher (built) |
| Cancellation charge | Guest cancels a confirmed stay | Penalty from the policy at the time of cancellation, at the property | Cancellation confirmation (built) |
| Damage claim | Operator opens a claim after check-out | Amount the operator claims, at the property | Damage claim notice (this design) |

The damage claim is a notice. It does not move money. The guest pays the property directly, as with every other charge.

## 3. Damage claim: proposed rules

These are proposals for the owner to confirm or change.

1. **Who opens it.** The property operator, from the business app, after check-out. Only for a stay in state CHECKED_OUT.
2. **Window.** The claim must be opened within 7 days after check-out. After that, it is closed.
3. **Evidence.** At least one evidence item (photo or note) per claim. Stored with the stay (the cases module already holds evidence).
4. **Amount.** A positive amount in CDF, with an itemised reason (text). No amount above the accommodation total of the stay.
5. **Notice.** The guest receives the damage claim notice (PDF and email) with the amount, the reason, the evidence list and the dispute window.
6. **Dispute.** The guest may dispute within 7 days of the notice, by replying through the existing guest message thread. A disputed claim stays open for operator and platform review. No payment is ever expected from the platform.
7. **Close.** A claim closes when the guest pays the property and the operator marks it settled, or when the dispute window ends with no dispute, or when a dispute is decided.
8. **Document.** Each claim notice is a numbered, immutable document (same engine, same write-once rules). A correction is a new notice that points at the old one.

## 4. What must change in stay-service

- Remove the hard block on damage cases only after the owner confirms section 3. The current block lives in `stay-service/src/cases/rules.rs`.
- Add a claim amount and an itemised reason to the case. Keep the existing case evidence and audit trail.
- Add a `reservation.damage_claimed` event with the amount, reason, guest contact and dispute deadline. The snapshot fields follow the cancellation event pattern.

## 5. What messaging builds

- A `stay_damage_claim` document (template, en and fr), issued on the claim event, stored, emailed to the guest with the PDF attached.
- Same idempotency rule as the other stay documents: one notice per claim.

## 6. Open values for the owner

1. Deposit amount: keep at zero, or set a fixed CDF amount per property?
2. Claim window after check-out: 7 days (proposed)?
3. Dispute window after the notice: 7 days (proposed)?
4. Who settles a claim: the operator alone, or the operator with platform review on disputes?
5. Maximum claim: the accommodation total (proposed), or a fixed cap?

## 7. Not in this design

- Online deposits, card holds, installments, refunds (D06, D19).
- Tax on claims (counsel, same as invoices).
