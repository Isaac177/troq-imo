# Inventory, availability and calendar

Dependencies: D01, D05, D14; B/P/Q. This is the inventory authority for all channels and manager actions.

## I01: Inventory models

Support SINGLE_UNIT for a specific home/unit and POOLED_UNITS for interchangeable rooms of a type. Physical unit assignment may occur later for pooled bookings. Shared beds and linked whole-property/room sale are decision-gated extensions that must use explicit resources, not textual labels.

A linked sale consumes every required constituent resource. A building booking and its room bookings must use the same allocation authority; independent counters are forbidden. Pooled counts and later room assignment must prevent a reservation from requiring incompatible room changes midway unless the guest accepted that arrangement.

## I02: Night semantics and capacity

Stays occupy [checkIn, checkOut) local dates. Check-out date is not a sold night. Validate checkOut > checkIn, positive bounded quantities and approved booking horizon. Same-day turnaround uses separate restrictions/readiness, never an extra paid night by accident.

Available quantity equals sellable capacity minus active allocations/blocks under defined expiry handling. Separate baseline room count, out-of-service capacity, confirmed allocations and temporary holds. Do not double-subtract a maintenance block represented in both capacity and allocations.

Overbooking is disabled by default. No UI or privileged direct update may produce negative capacity. If the business later wants overbooking, require a new explicit policy and tests.

## I03: Atomic allocation

For single resources use PostgreSQL 17-compatible exclusion constraints over allocation ranges where applicable. For pooled resources lock every affected nightly row in deterministic resource/date order and validate all nights before allocation. Include holds and blocks in the invariant. Retry bounded serialization/deadlock failures safely.

Allocation and reservation/hold state transitions share a transaction. Redis is not the sole correctness lock. Expired holds require transactional release/reclassification; exclusion constraints cannot depend on a continuously changing now() predicate.

Quote creation does not allocate. Hold creation revalidates current price/rules/capacity atomically. Final confirmation rechecks hold ownership/expiry and payment eligibility. Expiry racing with payment confirmation yields one valid outcome and a defined financial recovery path.

## I04: Manager calendar

Views: month overview, week/day operations, multi-unit grid and accessible list. Cells show price, remaining quantity, restrictions and block source. Reservation bars expose only authorized guest fields. Show timezone and selected property clearly.

Range selection opens an edit panel for dates/weekdays, affected unit types, rate/inventory/block action and reason. Preview affected cells and conflicts. Bulk mutation is all-or-nothing within the documented bounded operation, or explicitly reports per-item outcomes for a job; never silently partially apply.

Drag-to-move a reservation invokes amendment preview, not a direct date update. Selection/expansion animates and has keyboard alternatives.

## I05: Blocks and availability rules

Block types: owner use, maintenance, external booking and manual closure. Persist origin, actor, resource, dates, quantity and reason. Blocking occupied inventory requires resolving the affected reservations; no automatic cancellation.

Restrictions: advance notice, maximum booking horizon, minimum/maximum nights, closed-to-arrival/departure and turnover buffers. Distinguish arrival-based versus stay-through minimum-night rules. Price rules and restrictions have explicit precedence from P.

## I06: Physical unit assignment

Assign only compatible ready/eligible units within the reserved type and date interval. Room moves create dated assignments retaining history. An assignment spanning several nights cannot overlap another assignment even if the pooled reservation counts are valid. Upgrades change financial terms only through an accepted amendment where needed.

## API/data and performance

Commands: query availability, allocate/release hold, create/remove block, bulk calendar edit, assign/move unit. Include city/property, resource IDs, date range, quantity, expectedVersion, reason and idempotency key where mutating allocations.

Index resource/date and active allocation ranges. Fetch only visible date windows/unit pages. Bound span and batch size. Conflicts return machine-readable dates/resources and current version without leaking other guests.

## Seeds and acceptance

Seed a one-unit apartment, a three-room pool, one maintenance room, linked building/rooms, back-to-back stays, expiring holds and external blocks.

Concurrency tests use real Postgres connections: two last-unit buyers, overlapping ranges, overlapping whole/room allocation, hold expiry versus confirmation, block versus booking, room reassignment and competing bulk edits. Assert no oversell, orphan allocations or duplicate releases after retries/crashes.
