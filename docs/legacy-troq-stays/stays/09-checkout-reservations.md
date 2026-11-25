# Checkout and reservation lifecycle

Dependencies: R/I/P/F; D05, D06, D08, D13. Booking confirmation and payment settlement are separate facts.

## Q01: Checkout UI and data

Steps: selected accommodation/quote; lead guest and per-room occupants; contact; optional special requests/extras; policy and price review; payment or request submission; outcome.

Show property/city, local dates, nights, rooms, occupancy, inclusions, cancellation deadline, complete CDF breakdown and payment schedule. Lead guest can differ from payer if D13 permits; reservation owner, payer and occupants remain distinct identities. A special request is not guaranteed until explicitly accepted.

Require verified account according to the agreed booking policy. Bind quote/hold to that identity. Preserve safe draft data after network failures, but do not persist sensitive payment or identity document data in local storage.

## Q02: State machines

| Aggregate | States |
|---|---|
| Hold | ACTIVE, CONSUMED, EXPIRED, RELEASED |
| Reservation | REQUESTED, AWAITING_PAYMENT, CONFIRMED, DECLINED, EXPIRED, CANCELLED, COMPLETED |
| Amendment | PROPOSED, AWAITING_ACCEPTANCE, AWAITING_PAYMENT, APPLIED, REJECTED, EXPIRED, WITHDRAWN |
| Stay item | NOT_ARRIVED, CHECKED_IN, CHECKED_OUT, NO_SHOW |

Payment/refund/payout states live in F. A reservation can be CONFIRMED with a later payable balance if its accepted schedule permits. Reservation COMPLETED is derived through explicit completion rules for its items, not a synonym for paid.

## Q03: Transition contract

| Command | Preconditions | Effects |
|---|---|---|
| Submit instant booking | Current quote, valid occupancy/policy, active inventory hold | AWAITING_PAYMENT or CONFIRMED if permitted payment schedule; emit durable event |
| Submit booking request | Request mode, current quote, valid hold policy | REQUESTED; deadline and inventory treatment from D05 |
| Accept request | Assigned permission, unexpired request, availability valid | AWAITING_PAYMENT or CONFIRMED; new payment deadline if needed |
| Decline request | REQUESTED, authorized actor/reason | DECLINED; release any allocations; refund/void obligations if applicable |
| Confirm after payment | Correct payment obligation satisfied, allocation valid | CONFIRMED; consume hold; notification outbox |
| Expire request/payment | Deadline passed and eligible state | EXPIRED; release allocation; reconcile any concurrent payment |
| Cancel | Current evaluated cancellation entitlement/authority | CANCELLED, release remaining inventory under policy, create refund obligation |
| Complete | All items finalized and policy permits completion | COMPLETED; review eligibility and settlement eligibility event |

Inventory and reservation transitions use one city transaction. Financial service effects are reliable asynchronous commands/events with compensation, not distributed transaction claims. Invalid/repeated transitions return existing outcome or a typed conflict.

## Q04: Holds and payment race

After guest accepts quote, allocate a bounded temporary hold and establish durable reservation/payment reference. Payment initiation uses a stable idempotency key. Timer displays server expiry; client countdown is informational only.

If payment outcome is unknown, poll authoritative operation status. Never create a second charge merely because the first response timed out. If payment settles after inventory release, recheck availability under transaction. Confirm only when contract/inventory remain valid; otherwise create a tracked refund/exception obligation and notify guest truthfully.

Request mode must not leave arbitrary-length inventory locks. D05 defines whether approval reserves inventory and when payment is taken. Workers release expired holds even after restart, with transactional protection against concurrent confirmation.

## Q05: Amendments and cancellation

Amendment preview accepts proposed dates, unit/rate, quantities or occupants and returns availability, revised contract, price difference, required payments/refunds and expiry. Secure replacement allocations before releasing old ones; while pending, both allocations may consume capacity according to a bounded hold policy.

Guest acceptance and any required collection happen before applying. On decline/expiry, release replacement allocations and preserve original booking. Applying is version-checked and atomic for booking/inventory; financial compensation is tracked if a later step fails. Do not blindly retry side effects.

Cancellation preview uses stored policy, actual time and financial history. Commit binds to evaluated version and recalculates if changed. Guest sees refundable amount, withheld charges and refund status separately. Partial room cancellation is item-aware; never cancel unrelated items.

Host cancellation and relocation require D08 allocation of costs, guest consent, substitute booking linkage and support ownership. Deleting a reservation is not cancellation.

## Q06: My stays and documents

Tabs: pending, upcoming, active, past and cancelled. Detail shows independent reservation/payment/stay statuses, contract, messages, timeline, actions and documents. Actions depend on state and ownership. Cross-city retrieval follows B03, including stale/partial indicators.

Confirmation voucher and payment receipt are separate documents. Sharing a reference alone never authorizes private detail access. Calendar export excludes access codes and sensitive guest data. Cached offline voucher is marked with last refreshed status.

## Seeds and acceptance

Cover each state and transition, multi-room partial cancellation, payment timeout, duplicate submission, expired request, host decline, late success, failed refund and pending amendment. Test same idempotency key with changed payload rejection.

Concurrency cases: cancel versus confirm, amendment versus cancellation, final-room contention, expiration versus payment, duplicate callbacks and repeated completion. Assert one inventory outcome, one obligation per intent and durable recovery work for any financial mismatch.
