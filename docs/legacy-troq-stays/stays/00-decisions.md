# Decisions and boundaries

## Confirmed constraints

- Money is CDF only; bilingual English/French; dates display dd/mm/yyyy.
- Multi-city Postgres ownership follows the existing TROQ topology.
- Existing enum role architecture remains. Target one new role, stay_manager; no job-title role expansion.
- Use scoped permissions for operator/property/platform responsibilities.
- Documentation is authorized now. No implementation or deployment has been requested in this turn.
- Advanced feature coverage stays visible even when external dependencies prevent immediate implementation.

## Decision register

| ID | Decision required | Proposed direction | Blocks |
|---|---|---|---|
| D01 | Supported accommodation inventory | Entire homes plus pooled hotel room types; physical room allocation; explicitly decide private/shared rooms and beds | Property/unit schema and search facets |
| D02 | Individuals, businesses, and existing company linkage | One operator concept with verified individual/business subtype; reuse existing identity only after mapping | Onboarding schema, ownership, contracts |
| D03 | Brand, operator, property and media ownership | Operator owns properties; optional brand association must not imply permissions or media rights | Brand foreign keys, transfers, media reuse |
| D04 | Manager application location | Stays workspace in current Vite business app, selected through module navigation | Routes, onboarding entry, navigation |
| D05 | Instant and request booking | Support both; choose allowed modes per property/rate plan and approval timeout | Holds, payment timing, availability effects |
| D06 | Payment methods and schedules | Implement only confirmed provider capabilities; choose prepay/deposit/pay-at-property explicitly | Checkout, overdue handling, commission collection |
| D07 | Commission and fees | Versioned operator agreement with explicit fee base and funding responsibility | Quotes, ledger, statements |
| D08 | Cancellation/refund/no-show policy | Versioned policy templates, exact deadlines and exceptions | Price contract and cancellation engine |
| D09 | Damage deposits and claims | Separate deposit obligation from accommodation price; decide collection/release/dispute terms | Provider operations and financial exposure |
| D10 | Settlement timing and authority | Release after a defined stay milestone and dispute checks; owner decides schedule/thresholds | Payout jobs and approvals |
| D11 | Taxes, invoices and document retention | Owner supplies applicable rules; no invented rates or retention periods | Receipts/invoices, tax reporting, deletion |
| D12 | Cross-city identity and discovery | Verify canonical user identity; replicated public search and private booking references with city authority | Global My stays, cross-city search, memberships |
| D13 | Groups, multi-room and corporate | Multi-room atomic booking; explicit group/corporate purchase and approval rules | Reservation item structure and billing |
| D14 | External inventory channels | Calendar feeds and contracted channel API integrations separately | Availability SLA and conflict resolution |
| D15 | On-property operations | Front desk, housekeeping, maintenance included; confirm guest registration requirements | Sensitive guest fields and operations workflow |
| D16 | Loyalty, affiliates, add-ons, service bundles | Reuse verified contracts, choose qualification and refund reversal rules | Growth accounting and integrations |
| D17 | Price automation | Deterministic rules first; decide scope/authority of automated recommendations and publishing | Dynamic pricing behavior |
| D18 | Verification and address disclosure | Define required documents, review standards, exact-location and access-code visibility | Publication, privacy, arrival UX |
| D19 | Financial permissions and limits | Separate refund request/approval, settlement preparation/approval, payout-detail editing | Approval workflows and step-up policy |
| D20 | Availability and service objectives | Benchmark a representative portfolio; approve load, latency, recovery and freshness targets | Capacity acceptance and launch readiness |
| D21 | Launch feature checklist | Review every feature ID and explicitly mark required or excluded; exclusions require owner decision | Release scope only; never silently drop work |

## Gate handling

Record owner answers here as dated decisions with rationale and impacted files. Do not convert proposed defaults into settled facts merely because an agent starts implementation.

Business identity gates D01-D03 and D12 must be answered before schema implementation. Financial gates D06-D11 and D19 must be answered before real payment/refund/payout behavior. A provider failure or missing contract is a visible blocked feature, not permission to simulate success.

Architectural exploration, source inspection, UI state mapping, and contract proposals can proceed without those answers. Do not repeatedly ask for already settled decisions.

## Exclusions unless explicitly added

No flight booking, vehicle rental, currency conversion, insurance sales, full restaurant POS, payroll, or accounting ERP. Integration of TROQ delivery or services is a separately defined add-on contract, not automatic inheritance of retail-order behavior.

## Change discipline

A changed decision updates all affected specifications together: schema ownership, API payloads, permission checks, UI, seeds, tests and delivery sequence. Preserve historic booking contracts when policy configuration changes.

## Owner decisions log

Decided 12/10/2026 by the owner's delegation ("decide here"). Each entry accepts the proposed direction unless noted. Reversible by a later dated entry.

- D01 (12/10/2026): Entire homes plus pooled room types, with physical room allocation. Shared beds and linked whole-building sales stay out of Slice 1. Impacts 02, 06, 07.
- D02 (12/10/2026): One operator concept with a verified individual or business subtype. Link to existing companies only after mapping, in the onboarding slice. Impacts 01, 05.
- D03 (12/10/2026): The operator owns properties. A brand is an optional label that grants no permissions and no media rights. Impacts 01, 05.
- D12 (12/10/2026): Cross-city identity is not assumed. users.id is a random UUID generated per city database and users.email is unique only within a city (auth-service migrations and user_writes.rs), so the same person can hold different IDs in different cities. Slice 1 is city-scoped: memberships, reservations and My stays resolve within one city. Cross-city My stays is deferred until a canonical identity mechanism is designed and approved. Impacts 01, 02, 09.
- Payment location (12/10/2026): Accommodation payment obligations stay in payment-service, per B01. Impacts 10.
- Permission names (12/10/2026): Rust and TS permission lists come from one shared source. Impacts 01.

## Decisions made by the implementation agent (12/10/2026, owner asked the agent to decide)

These entries were chosen by the agent under the owner's instruction "you can decide some of the things". Each is reversible by a later dated entry. Impacts are listed per entry.

- D06 Payment: pay at the property for this slice. No online payment, no payment-service change, no provider contract. Instant bookings become CONFIRMED when made; request-to-book accepted by the operator becomes CONFIRMED. Impacts 09, 10, 11, and the reservation state machine.
- D11 Retention: guest name and contact details are kept for 365 days after check-out (`STAY_GUEST_RETENTION_DAYS=365` in the dev environment). Confirmed by the owner as the operating default. Not a legal retention period; tax and invoice retention rules remain open under D11. Impacts 09, 15.
- D05 Request-to-book: rate plans with booking mode REQUEST hold nights for a 24-hour approval window (proposed default, owner may change). Impacts 06, 09.
- D14 Integrations: excluded from this slice. Calendar feeds and channel APIs are not built.
- Case evidence files: excluded from this slice. Needs the file-storage integration first.
- D02/D18 Operator onboarding: operators are still created and approved by platform moderators in auth-service. Stay-service keeps its status projection through the auth sync. Document verification waits for D18.
- Amendments: deferred. Not built in this slice.
- Amendments (agent decision): a guest can propose new dates for a confirmed stay that has not started. One pending amendment per stay, valid 24 hours (proposed default). The price difference is recorded and paid at the property. Impacts 09, 10.
- No-show (agent decision): a confirmed stay whose arrival date has passed can be marked no-show, and its nights are released. Charging a no-show is left to the cancellation policy (D08). Impacts 09.
- Stay records (agent decision): check-in assigns a free physical unit of the booked type and records the arrival; check-out records the departure. Impacts 06, 11.
- Operator onboarding (agent decision, supersedes the D02/D18 line above for the flow): an active account creates an operator, which starts as PENDING_REVIEW with the creator as owner. A platform moderator approves it. Document verification still waits for D18. Impacts 01, 05.
- Integrations (D14, agent decision, supersedes the exclusion above): a read-only iCal feed per property is built. Channel APIs and feed import are not built. Subscriptions need a tokenised feed URL, which is not built yet. Impacts 13.
- Case evidence files (agent decision, supersedes the exclusion above): files up to 5 MB (PDF, PNG, JPEG) are stored in the stay database, checked against their content. Object storage is the later option. Impacts 09, 12.
- Calendar screen (owner request): the stays calendar is a full-screen page in the business app, using the calendar endpoint. Impacts 03, 04.
- Calendar subscriptions (agent decision): tokenised feed links, created and revoked by operators with reservations management. Only a hash of the secret is stored. Impacts 13.
- Channel integrations (D14): still unbuilt. Channel APIs need a signed contract with an outside provider, which the owner has not supplied. Impacts 13.

## Pricing decisions made by the implementation agent (spec 07)

Chosen by the agent under the owner's instruction to finish 07. Each is reversible by a later dated entry.

- Price precedence (P02): base override, then dated amount and percentage rules by priority, then child charges, then one automatic length-of-stay promotion. Two base overrides at the same top priority are rejected. Discounts round down to the minor unit. Impacts 07, quotes, calendar prices.
- Child charge: a per-child, per-night amount on each rate plan (`childNightlyAmount`, migration 00016). It is not multiplied by the per-guest factor. Child ages are not collected, so no age bands apply until the owner approves age cut-offs. Impacts 07, quotes, seeds.
- Length-of-stay promotion: minimum nights, percentage (up to 100), optional cap, optional rate plan and optional stay window (the stay must sit fully inside it). One promotion per quote: largest discount, then higher priority, then smaller id. Impacts 07, quotes, seeds.
- Cancellation deadline: arrival date at local midnight minus `freeUntilHours`, stored in the quote and in the reservation contract. The property has no check-in time field yet. Impacts 07, reservations.
- Cancellation evaluation: `GET /api/stays/reservations/{reservationId}/cancellation` (NATS `stay.reservation.cancellation`) returns the refund and penalty for a confirmed reservation. Penalty is `penaltyPercent` of the total after the deadline. It moves no money. Impacts 07, 09, 10.
- Redemption budgets (migration 00017): a promotion stops applying to new quotes once its redemptions reach `redemptionLimit`. Redemptions count reservations in REQUESTED, CONFIRMED, CHECKED_IN or CHECKED_OUT. Hold creation locks the promotion row and recounts, so concurrent holds cannot exceed the limit. Impacts 07, reservations, promotions.
- Offers instead of coupon codes: promotions apply automatically when the stay qualifies. No code is typed by guests, which satisfies AGENTS rule 21. Impacts 07, quotes.
- Price explanation: each night lists every rule with APPLIED or EXCLUDED and a reason (OUTSIDE_DATE_RANGE, WEEKDAY_NOT_SELECTED, SUPERSEDED_BY_HIGHER_PRIORITY_BASE). Impacts 07, business rates screen.
- Manager rate administration (stay-service `rate_admin`, gateway `/api/stays/properties/{propertyId}/rate-plans`, `/rules`, `/promotions`, `/unit-types`, `/rate-policies`, `/price-preview`): plans are versioned with expectedVersion; a plan cannot be activated without an unlimited base rate; a new base rate that overlaps another base rate at the same priority is refused; percentages must be above 0 and at most 100; redemption limits cannot drop below redemptions already made. Every mutation writes an audit record. Impacts 07, 01 (permissions STAY_MANAGE_PROPERTIES).
- Business rates screen: `/stays/rates` in troqzone-business, reached from the stays operators page. Plans are compared in rows; the editor, dated rules, price preview and offers are sections. Impacts 07, 03.
- Idempotency on rate mutations: create and update requests take an optional `idempotencyKey`. The key is claimed in the same transaction as the write, and the response is stored and replayed on a retry. Concurrent requests with one key cannot both write. Rule removal accepts the key as a query parameter; the business screen does not send one for removal. Impacts 07, 14 (API policy).
- Pay-at-property schedule: every quote shows the whole total as due at the property, with nothing due now or later (D06). The seeded payment policies still say `PREPAY`; the schedule follows D06 and ignores that field until the owner confirms the policy content. Impacts 07, 10.
- Verification on the local dev database: seeds applied and the ignored DB tests pass (7 of 7). New DB races cover the redemption budget and idempotency keys.
- Missing-night seed: the Goma villa has a second active plan whose base rate covers only March 2027. Quotes for other nights are refused with a missing-rate conflict. Its id sorts after the existing plan, so the calendar keeps showing the original prices. The calendar picks the first active plan by id, which is fragile; a plan-selection rule is needed before more plans are added. Impacts 07, calendar.

Backend pass for guest and discovery APIs (owner instruction: finish the backend, skip the UI):
- Hold acceptance (spec 07 P05): the hold must send the policy version ids the quote shows; they must match the quote exactly, or the hold is refused. The accepted ids and the acceptance time are stored in the contract. Impacts 07, 09.
- Special requests (spec 09 Q01): a request is stored with state REQUESTED (up to 500 characters) and is not guaranteed. An operator with STAY_MANAGE_RESERVATIONS accepts or declines it with optimistic version checks (migration 00019). Impacts 09, 14.
- Reservation view carries version, specialRequest and specialRequestState. My reservations filter by tab (pending, upcoming, active, past, cancelled) in SQL, using the property's local date. Guests read their own amendments with GET /api/stays/reservations/{id}/amendments. Impacts 09, 14.
- Public discovery (spec 08 API02, anonymous, through the gateway's public router and its rate limit): destinations (city count), property search with dates, guests, instant-booking filter, sort (relevance and name are alphabetical until ranking factors are approved; price needs dates), server pagination. Search checks room counts per night, physical-room blocks, occupancy and restrictions, and prices each plan with the quote's own pricing. Candidate limit: 200 properties per search. Public property detail and availability show only published properties and approved operators. Exact coordinates are not exposed (D18 default). Media is served only when READY and the property is published. Impacts 08, 14, 15, 16.
- Wishlists and saved searches (spec 08 C05, API02): owner-only lists with PRIVATE or SHARED visibility, at most 20 lists of 200 stays, adding only published properties; saved searches with an alert-consent flag, at most 20 per guest. No share links and no alerts yet (they need notifications, spec 12). Impacts 08, 12.
- Seed data: the Goma seeds include two wishlists, a shared list and a saved search. A cancelled reservation seed now carries its cancellation time. Impacts 09, seeds.

Search engine (owner instruction: one reusable engine with its own specification):
- The engine is `stay-service/src/search_engine`, specified in `docs/stays/20-search-engine.md`. Public search, availability and the catalog run through it. Filters: dates, guests, instant booking, property type (derived from the unit model), meal plan, payment collection, accessibility, amenities, stay price range, cancellation terms and distance. Sorts: relevance, name, price, distance. Facets and stats cover all results; analytics are recorded only with consent (migration 00021).
- Decided (engine version 2, section 13 of the engine document): D18 location privacy (public points rounded to two decimals on both sides of the distance; no coordinates or address in public results; exact location after a confirmed booking is spec 09), D-S1 relevance order, D-S2 child ages required per child (0 to 17) with no age-band pricing until the owner sets cut-offs, D-S3 analytics retention of six months, D-S4 no projection now with a stated trigger.

Owner instruction "use the proposed defaults" (applied to the gated items):
- D06 (payment): pay at the property. No installments exist, so no installment schedule is built. Impacts 07, 10.
- D08 (cancellation): versioned policy with an exact free-cancellation deadline. Guest cancellation of a confirmed stay (`POST /api/stays/reservations/{reservationId}/cancel`, NATS `stay.reservation.cancel`) moves the stay to CANCELLED, releases its nights and unit assignments, and records the penalty as due at the property. No refund is created, because nothing is collected under pay-at-property. The cancellation time, reason and charge are stored on the reservation (migration 00018). Idempotency key supported. Impacts 07, 09, 10.
- Guest acceptance: the reservation contract stores `acceptedAt` when the hold is created, alongside the quote, policy versions and cancellation deadline. The guest summary and acceptance screen belong to specs 08 and 09 and are not built.
- D09 (deposits): no damage deposit is collected, and no deposit amount or claim terms are set. The default keeps deposits separate from accommodation price, so none is charged. Impacts 07, 10.
- D17 (pricing): deterministic rules only. No automated price recommendation or publishing is built. Impacts 07.
- D19 (financial roles): no refund request or approval flow exists, because no refund is created under pay-at-property. Refund roles are split when a refund flow is built. Impacts 07, 10.

Still open in spec 07, each waiting on a later spec or a decision the defaults do not settle:
- Guest policy summary screens and the acceptance UI: specs 08 and 09.
- Installments and deposit amounts: need owner terms beyond the defaults above.
- Browser check of `/stays/rates`: needs a login for the local GOMA business app.
