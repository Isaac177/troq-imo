# Reservations lifecycle

Source of truth is the code: `stay-service/src/reservations`, `amendments`, `cases`, `operations`, `admin_claims`, `platform_settings` on `main`, compared with `backend/crates/domain/src/reservations` and `backend/migrations/00012` to `00014` in Mungini. Mungini keeps behaviour identical, so the two agree unless stated.

## Quote and hold

1. A guest asks for a quote for a unit type, rate plan, dates and guest counts. A quote lives `QUOTE_TTL_MINUTES` (default 15, range 5 to 120). A stay is 1 to 365 nights.
2. The guest books the quote. Checks: quote exists and has not expired, it was not booked already (unique per quote), check-in is not in the past, the guest has a name and a phone or email, the guest accepted the exact policy versions of the rate plan, the special request is at most 500 characters.
3. In one transaction the property is locked, the promotion budget is locked if a discount applies, the reservation, one item and the primary guest are inserted, each night is taken from inventory, a room is assigned, and an audit record and `reservation.held` event are written.
4. The booking mode of the rate plan decides the first state. `INSTANT` creates `CONFIRMED` and takes the host commission at once. `REQUEST` creates `REQUESTED` with an approval deadline `APPROVAL_WINDOW_HOURS` ahead (default 24, range 1 to 168).
5. Inventory is held through allocations. A `REQUEST` reservation holds its nights as `HOLD` allocations that expire at the approval deadline. An `INSTANT` reservation holds them as `RESERVATION` allocations with no expiry.
6. The reservation stores a contract snapshot: quote, nightly charges, accepted policies, free-cancellation deadline, discount, commission. Later policy or price changes never alter it.

## States

`PENDING_PAYMENT`, `REQUESTED`, `CONFIRMED`, `DECLINED`, `CANCELLED`, `EXPIRED`, `CHECKED_IN`, `CHECKED_OUT`, `NO_SHOW`.

Allowed moves (`core/state.rs`):

| From | To |
|---|---|
| `PENDING_PAYMENT` | `CANCELLED`, `EXPIRED` |
| `REQUESTED` | `CONFIRMED`, `DECLINED`, `CANCELLED`, `EXPIRED` |
| `CONFIRMED` | `CANCELLED`, `CHECKED_IN`, `NO_SHOW` |
| `CHECKED_IN` | `CHECKED_OUT` |

`DECLINED`, `CANCELLED`, `EXPIRED`, `CHECKED_OUT` and `NO_SHOW` are final. `PENDING_PAYMENT` is never created by any code path, because nothing is paid online. It stays in the schema and filters. Writes use an `expectedVersion` and fail with a conflict when the row changed.

## Transitions

- Host accepts or declines a `REQUESTED` reservation. Accept moves to `CONFIRMED`, takes the commission and keeps the nights. Decline moves to `DECLINED` and releases the holds. Both fail after the approval deadline.
- Expiry: a sweep (every 30 seconds in legacy, `holds/expiry.rs` in Mungini, worker startup not verified) moves `PENDING_PAYMENT` or `REQUESTED` reservations whose hold expired to `EXPIRED`, releases the nights and the commission, and also anonymizes guests past retention.
- Guest withdraws a `REQUESTED` (or `PENDING_PAYMENT`) reservation: it becomes `CANCELLED` with reason `GUEST_WITHDRAWN`, holds released.
- Guest cancels a `CONFIRMED` reservation. The cancellation policy has `freeUntilHours` (0 to 8784) and `penaltyPercent` (0 to 100). The free deadline is that many hours before midnight of the arrival day in the property time zone. Cancelling by the deadline costs nothing (`FREE_CANCELLATION`). After it the charge is the penalty percent of the total (`PENALTY_APPLIES`). The charge is due at the property and nothing is refunded online. The nights are released. The kept commission is the penalty share (see `payments-and-billing.md`).
- Host cancels a `CONFIRMED` reservation with a reason (up to 300 characters). Reason code `OPERATOR_CANCELLED`, charge 0, commission fully released. Mungini adds a platform cancellation path (`platform_cancel.rs`) that legacy did not have as a separate file; its rules are unverified.
- Check-in: `CONFIRMED` to `CHECKED_IN`, only on or after the arrival date and before check-out date in the property time zone, with a usable assigned unit. Needs the check-in permission.
- Check-out: `CHECKED_IN` to `CHECKED_OUT`.
- No-show: `CONFIRMED` to `NO_SHOW`, only after the arrival date has passed. The commission is not released.
- Special request: a free-text request (max 500) on the booking. The host accepts or declines it (`NONE`, `REQUESTED`, `ACCEPTED`, `DECLINED`); the guest is notified.

## Amendments

- Only a `CONFIRMED` stay whose check-in is still in the future can be amended. The new stay must be 1 to 365 nights, restrictions and availability are re-checked, and the new price is computed from the contract.
- Either the guest or a host member with reservation write permission can propose. One proposal can be open per reservation. It stores the delta amount and expires after `AMENDMENT_WINDOW_HOURS` (default 24, range 1 to 168).
- In legacy only the guest accepts or declines (`PROPOSED` to `ACCEPTED` or `DECLINED`). Older proposals become `EXPIRED` or `SUPERSEDED`. Accepting re-checks version, dates and availability, updates the reservation and adjusts the commission (an increase needs enough credit).

## Guest messages

A thread per reservation between the guest and the host (`GUEST` or `OPERATOR` sender, 1 to 2000 characters). Each message notifies the other side.

## Cases and claims

- Case types: `GUEST_COMPLAINT`, `OPERATOR_COMPLAINT`, `DAMAGE_CLAIM`, `REFUND_DISPUTE`, `NO_SHOW_DISPUTE`, `GENERAL`. States: `OPEN`, `AWAITING_RESPONSE`, `RESOLVED`, `CLOSED`.
- Guests open `GUEST_COMPLAINT` or `GENERAL`. Hosts open `OPERATOR_COMPLAINT`, `GENERAL` or `DAMAGE_CLAIM`. `REFUND_DISPUTE` and `NO_SHOW_DISPUTE` are refused because the payment rules for them are not defined. `AWAITING_RESPONSE` is accepted by the checks but no code sets it.
- A case gets a response deadline `CASE_RESPONSE_DAYS` ahead (default 7, range 1 to 60). Staff can assign and escalate (once) an open case. A reason up to 2000 characters is required.
- Evidence: PDF, PNG or JPEG, 1 byte to 5 MiB, only while the case is open or awaiting response.
- Damage claim: only for a `CHECKED_OUT` stay, opened within `CLAIM_WINDOW_DAYS` after check-out (default 7, range 1 to 60), with a positive amount not above the stay total. It notifies the guest with a dispute deadline of 7 days.
- Staff decide a claim: `APPROVED`, `PARTIAL` (amount above 0 and below the claim) or `REJECTED`, each with a reason code from a fixed list. The case becomes `RESOLVED`. Payout state is `NOT_DUE` for a rejection, else `PENDING`, then staff mark `PAID` (reference required, up to 120 characters) or `FAILED`. The platform records the payout state only; the code moves no money.

## Operations tasks

Types `CHECK_IN`, `CHECK_OUT`, `HOUSEKEEPING`, `MAINTENANCE`, `SERVICE_REQUEST`; priorities `LOW`, `NORMAL`, `HIGH`, `URGENT`; states `OPEN`, `IN_PROGRESS`, `DONE`, `CANCELLED`. Actions: start (`OPEN` to `IN_PROGRESS`, assigns the actor if unassigned), complete, cancel, assign to me, unassign. Which task types a person sees depends on `operations.checkin` (check-in and check-out), `operations.housekeeping` (housekeeping and service request) and `operations.maintenance`.

## Mungini build state

Built: state machine, hold, nights, release, expiry, retention, cancellation, host decision, stay flow, amendments, messages, events. Empty folders: `cases`, `claims`, `operations` (the tables exist in `00013` and `00014`).

## Unverified

- Platform cancellation rules in Mungini.
- Whether a guest-proposed amendment should need host approval (legacy has none).
- Who may reply to a damage claim and how a guest disputes it; legacy has a 7 day dispute deadline in the event but no dispute flow in the code.
- Reasons list for guest cancellation other than the two computed codes.
