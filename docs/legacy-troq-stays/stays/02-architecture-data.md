# Architecture and conceptual data model

Dependencies: decisions D01-D03 and D12; [permissions](01-roles-permissions.md).

## B01: Service boundaries

Add a Rust stay-service in the existing Cargo workspace after business identity gates are resolved. Follow existing axum/sqlx/NATS/Redis/MinIO conventions. Do not implement lodging as product stock or retail delivery orders.

| Owner | Authoritative data |
|---|---|
| auth-service | User identity, scoped grants, operator membership, access revocation |
| stay-service | Accommodation operator profile, property/units, availability, rates, reservation contract, physical stay, operational tasks |
| payment-service | Payment intents/attempts, refunds, double-entry ledger or verified equivalent, commission liabilities, settlement and payouts |
| messaging-service | Threads, delivery attempts and notifications, subject to extending existing contracts |
| review-service | Stay review eligibility, reviews/replies/reports, moderation |
| api-gateway | Route exposure, verified identity/city context, request controls; no duplicated domain rules |

Stay service retains payment references and a rebuildable status projection, never an independent money ledger. Existing payment source includes orderId references; inspect all producers/consumers before adding STAY_RESERVATION references. Never insert fake retail orders to satisfy that contract.

## B02: Conceptual entities

Field lists describe required concepts, not finalized SQL. Select actual ID and naming conventions after examining owning service schema. Store foreign-service IDs as references with verified contracts, not cross-service cascading deletes.

| Entity | Required fields and invariant |
|---|---|
| Operator | Identity reference/type, approved profile, status, verification revision; relationship to existing companies decided by D02 |
| Property | Operator, optional brand, authoritative city, timezone, localized content, address/coordinates, publication revision, version |
| PropertyRevision | Proposed content snapshot, author, review status/reasons; published revision remains intact during review |
| UnitType | Property, inventory model, capacities, bed/bath configuration, localized description, accessibility attributes |
| PhysicalUnit | UnitType, unique property-local identifier, operational status; never reassign historically occupied identity |
| InventoryPool | Sellable quantity and linkage to physical units or constituent resources |
| NightInventory | Pool/date, sellable capacity and allocations; unique pool/date; capacity cannot be negative |
| InventoryAllocation | Reservation/hold/block reference, resource, date range/quantity, state and expiry where applicable |
| AvailabilityBlock | Resource, date range, reason, origin, actor and conflict disposition |
| RatePlan | UnitType, occupancy basis, meals, payment/cancellation policy version, status |
| RateRule | Applicability, dates/weekdays, amount or adjustment, priority, version |
| Quote | Guest/session, property/city, item/night breakdown, policies, expiry, version and total CDF |
| Reservation | Guest identity, city/property/operator, reference, state, source, contract snapshot, version |
| ReservationItem | UnitType/rate plan, quantity, occupancy, dates, nightly charges and later room assignments |
| ReservationGuest | Minimum necessary occupant details, associated item, consent/retention basis |
| Amendment | Original version, proposed contract, replacement allocations, delta obligation, acceptance and expiry |
| StayRecord | Reservation item, assignments, actual arrival/departure, authorized exceptions |
| OperationsTask | Property/unit, task type, assignee, state, priority, checklist and attachments |
| Case | Reservation/operator references, type, parties, evidence references, deadlines and resolution |
| IntegrationMapping | Property/unit/rate identifiers, provider credentials reference and synchronization state |
| AuditRecord | Actor, scope, action, resource, reason, correlation and redacted change record |
| Outbox/Inbox | Durable event payload/version, delivery state, unique consumer/event deduplication |

Global stay taxonomies and policy templates have one owning service and are replicated to city databases with versions and tombstones. Do not create multiple writable masters.

## B03: City and identity routing

Property city determines authoritative reservation, inventory and quote storage. Browsing city is not an authorization claim. Validate all resource references against route city. Reject mixed-city reservation items.

Check whether user IDs are globally stable in current auth flows. Until D12 settles this, no assumption that matching text IDs from two cities represent the same person is permitted.

Proposed cross-city public discovery uses a versioned public projection. Proposed My stays uses a private user-to-reservation locator and server-side paging over that projection. Fetch current sensitive/actionable details from the source city. A projection failure must not erase a confirmed reservation; outbox replay repairs it. Expose stale/partial status and never fabricate exact live totals from unreachable cities.

Book one property per reservation; group itineraries may relate separate reservations but cannot promise cross-city atomic transactions. D13 governs any broader group contract.

## B04: Integrity and lifecycle

Use date values for lodging nights and timezone-aware timestamps for actions. Persist property timezone and contractual deadlines. Amounts use exact CDF arithmetic; transport decimal strings if existing financial contracts require them.

Indexes follow scoped filters and date lookups. Foreign keys protect local relationships. Unique constraints protect natural identities, idempotency keys and external mappings. Optimistic versions protect operator edits. Database transactions protect inventory transitions.

Unpublishing does not delete reservations. Archival excludes future sale while retaining historical relationships. Ownership transfer must define treatment of future bookings, money, documents and staff before reassignment. A booked property's city cannot be casually edited; require a dedicated migration/transfer decision.

## B05: Migration and contract rollout

Use next numbered forward migrations in each owning service. Never edit applied migrations. Coordinate new Postgres enum values with deployment sequencing; do not assume a new value can be used in the same transaction that adds it. Maintain backward-compatible event and DTO decoding during rollout.

Register the new service in workspace and local development/build configuration only when implemented. Owner executes production rollout. Define per-city migration tracking, failed-city recovery and compatibility with old application instances.

## Acceptance

- Schema design includes constraints/indexes and explains each cross-service reference.
- Confirmed reservations can be reconstructed from durable city records even if Redis and projections are lost.
- Different cities cannot consume each other's inventory.
- Historical contracts remain unchanged after property edits, policy updates, owner transfer or taxonomy retirement.
- Contract migration preserves existing retail payments, review types and company permissions.
