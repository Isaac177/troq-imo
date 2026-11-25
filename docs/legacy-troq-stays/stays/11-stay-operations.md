# Front desk, housekeeping and maintenance

Dependencies: R/I/Q; D15 guest registration rules. Physical stay status is distinct from reservation and payment status.

## S01: Operational workspace

Dashboard filters property/date and shows arrivals, in-house guests, departures, unassigned rooms, not-ready rooms, urgent tasks and no-show candidates. Each count is server aggregated. Mobile staff receive a task-focused list; desktop supports room/date views.

Reservation summaries show only authorized contact fields. Cleaners and maintenance staff see unit/task context without guest payment information or unnecessary names/documents.

## S02: Arrivals and check-in

Arrival list shows reserved type/quantity, occupancy, arrival estimate, assignment/readiness and approved payment eligibility. Check-in requires confirmed reservation, compatible room assignment, eligibility under accepted payment terms and any D15-required registration completion.

Record actual arrival time, staff actor and exception reason. Early check-in and late arrival require explicit inventory/readiness rules. Multi-room reservations support per-item arrival, not forced all-at-once check-in. Access instructions/codes are released only according to disclosure policy and logged where sensitive.

## S03: Room assignment and moves

Assignment obeys I06 and protects all occupied nights. Show compatible available rooms and readiness; do not expose rooms from another property. Moving an in-house guest ends one assignment and begins another with retained history and appropriate housekeeping tasks.

Complimentary upgrade versus priced upgrade is explicit. A priced change uses amendment acceptance. Room move cannot silently change original cancellation terms or number of paid nights.

## S04: Departures, no-shows and changes

Check-out records actual departure and unresolved service/financial items according to policy. It triggers room-cleaning work and updates stay completion without falsely marking unpaid money collected.

No-show workflow uses property-local cutoff, arrival history and policy. A timer can flag candidates; automatic penalty or cancellation requires approved policy. Late arrival after no-show follows a controlled correction with current availability checks.

Extensions and early departures use Q05 price/inventory/policy evaluation. Early departure is not automatically a full refund. Completion of one room does not complete an entire multi-room reservation.

## S05: Housekeeping

Room readiness states: DIRTY, ASSIGNED, CLEANING, AWAITING_INSPECTION, READY. Define whether inspection is mandatory per property. Task states: OPEN, ASSIGNED, IN_PROGRESS, BLOCKED, COMPLETED, CANCELLED.

Task fields: property/unit, service date, type/checklist, priority, assignee, due time, notes, permitted photos and completion actor. Prevent duplicate turnover tasks by reservation/item/occurrence key. Reassignment retains history. READY requires defined checklist/inspection evidence, not an unrestricted field write.

A room being cleaned does not automatically remove future sellable inventory; maintenance blocks do. Display risk to upcoming arrival and escalate delayed readiness.

## S06: Maintenance and guest requests

Maintenance issue includes unit/common area, severity, description, attachment, assignment, due time, status and resolution. Staff can report work with maintenance permission; blocking sellable capacity separately requires availability permission. Warn about affected bookings and create escalation before displacing guests.

Guest requests include reservation, category, desired time and notes. Show received/assigned/in-progress/resolved/closed status. Charges require an accepted priced add-on flow; staff cannot invent billable amounts through task notes.

## Backend and reliability

Commands are scoped and versioned: assign unit, check in/out, mark no-show, assign/complete task, inspect room, report/resolve issue. State transitions create audit and outbox events. Repeated completion does not duplicate tasks or settlement eligibility.

Offline task viewing may be supported with last-sync indicator. Queued task updates require conflict checks on reconnect. Offline check-in/payment confirmation is not presumed safe; require an explicit design before enabling.

## Seeds and acceptance

Seed expected arrival, late guest, partial group arrival, unready room, blocked task, unit move, early departure, no-show correction and maintenance conflict.

Test unauthorized guest access, incompatible assignment, stale readiness updates, duplicate turnover generation, concurrent check-in/room move, and no-show versus late check-in. Verify finance state remains independent of operational completion.
