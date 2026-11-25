# API and event contracts

These are proposed logical endpoints. Reconcile route prefixes and envelopes with the actual gateway conventions before implementation, preserving the operations and semantics below. Do not blindly create a competing authentication or response format.

## API01: Common contract

Prefix shown as /stays. City-scoped resources require validated city context consistent with property authority. Body city/resource IDs cannot override authenticated scope. IDs are opaque strings using existing conventions. Dates are YYYY-MM-DD on wire and dd/mm/yyyy in UI. Timestamps carry an offset/UTC; property timezone is separate. Amounts use exact CDF representations, preferably decimal strings matching verified financial conventions.

List query: page >= 1, pageSize bounded by shared constant (proposed maximum 100), allowlisted sort/direction and typed filters. Response: items, total, page, pageSize, totalPages. Total reflects identical scoped SQL filters. Cross-city projections identify dataAsOf and partial/stale state where needed; do not claim an exact live total when it is unknown.

Mutation common fields: resource-specific input, expectedVersion for edits, reasonCode where relevant. Financial/booking/job submissions require Idempotency-Key. Scope key by actor/domain/operation; store canonical request hash and durable result. Same key/different payload is a conflict. Specify retention sufficient for supported retries/provider replay, then test it.

Error contract: stable code, localized-message key or client mapping, optional fieldErrors, correlationId and safe conflict context. Codes include VALIDATION_FAILED, ACCESS_DENIED, NOT_FOUND, VERSION_CONFLICT, UNAVAILABLE_DATES, QUOTE_EXPIRED, PRICE_CHANGED, INVALID_TRANSITION, PAYMENT_PENDING, OPERATION_PENDING and DEPENDENCY_UNAVAILABLE. Use existing HTTP conventions consistently; private resources must not leak existence to unauthorized callers.

## API02: Public and guest endpoints

| Method/path | Input/output and authority |
|---|---|
| GET /destinations | Query/type/page; public supported destinations |
| GET /properties | Dates, occupancy, facets, sort, page; public property summaries and matching total |
| GET /properties/{id} | Published localized DTO; no private documents/guest data |
| GET /properties/{id}/availability | Bounded dates, occupancy, unit/rate filters; available options, no allocation |
| POST /quotes | Property, dates, item quantities/occupancy, extras/coupon; authoritative quote snapshot |
| POST /holds | Quote ID/version; guest-bound hold ID and server expiry |
| POST /reservations | Hold/quote, guests, accepted policies, booking mode; reservation and next required action |
| GET /me/reservations | Scoped tabs/date filters/page; own booking summaries |
| GET /reservations/{id} | Owner or permitted scoped member; field-filtered details |
| POST /reservations/{id}/amendment-previews | Proposed item/date/occupancy changes; revised contract and delta |
| POST /reservations/{id}/amendments | Accepted preview/version; pending amendment workflow |
| POST /amendments/{id}/accept | Authorized guest acceptance/version; next state/payment action |
| POST /reservations/{id}/cancellation-previews | Item scope/reason; financial and inventory consequence preview |
| POST /reservations/{id}/cancel | Accepted current preview/version; cancellation and refund operation reference |
| GET /operations/{id} | Authorized operation result/pending state for retry recovery |
| GET/POST /me/wishlists | Server-paged own collections/create |
| POST/DELETE /me/wishlists/{id}/items/{propertyId} | Ownership checked add/remove; public property validation |
| GET/POST /me/saved-searches | Own consent-aware alert configuration |

Payment initiation/status uses verified payment-service routes with STAY_RESERVATION reference. Do not implement a parallel payment authority inside these routes.

## API03: Operator endpoints

All prefix /manage; actor needs scoped grants. Read and write permissions differ.

| Endpoint family | Operations/contract |
|---|---|
| /operators, /applications | Create/resume draft, submit revision, read decision/corrections |
| /properties, /properties/{id}/revisions | Scoped list/create/read/update, preview, submit/pause/archive |
| /properties/{id}/media | Authorized upload registration, attach/detach/order, status |
| /properties/{id}/unit-types, /physical-units | Capacity/room metadata and versioned lifecycle |
| /properties/{id}/calendar | Bounded paged unit/date view and bulk-change preview/commit |
| /properties/{id}/blocks | Create/release reasoned allocations; conflict output |
| /properties/{id}/rate-plans, /rate-rules | Versioned plan/rule creation/update/archive and price preview |
| /reservations | Filtered scoped list, authorized manual booking through same engine |
| /reservations/{id}/accept, /decline | Request-only valid transition, reason/version |
| /reservations/{id}/assignments | Date-bounded compatible physical-unit assignments/moves |
| /stay-items/{id}/check-in, /check-out, /no-show | Operational transition with actor/version/exception |
| /tasks, /maintenance | Scoped lists, create/assign/update/complete/inspect |
| /team/invitations, /memberships | Auth-owned scoped grant workflows and revocation |
| /finance/statements, /refund-requests | Payment-owned query/command adapters, no stay-ledger writes |
| /integrations | Connections, mappings, feeds, freshness and reconcile jobs |
| /analytics, /exports | SQL aggregates and authorized asynchronous exports |

For each family, implementation must publish concrete methods, payload schemas, required permissions and examples in checked API documentation before frontend integration. Families are not permission to invent unsupported CRUD actions.

## API04: Admin endpoints

Prefix /admin: applications/review decisions; listing revisions/moderation; reservations/interventions; configuration/taxonomies/policy versions; refund approvals; settlement preparation/payout approvals; cases; review moderation; campaigns; reports/exports; audit; workflow recovery and emergency sales pauses.

Review command: revisionId, decision, reasonCode, guidance, expectedVersion. Approval binds to exact revision. Financial command: request/settlement ID, decision/reason and expectedVersion; amount/destination derive from approved authoritative records, not arbitrary body overrides.

## API05: Event envelope

Fields: eventId, eventType, schemaVersion, occurredAt, producer, authoritativeCity, aggregateType, aggregateId, aggregateVersion, correlationId, causationId and minimal payload. Do not include documents, access codes, full message content or unnecessary PII in broadly consumed events.

| Producer/event family | Consumers/effect |
|---|---|
| Auth membership/grants changed | Invalidate scoped authorization caches |
| Stay application/revision decided | Operator notification, public projection only after publication |
| Stay inventory changed | Search/cache invalidation and external channel update |
| Stay reservation requested/confirmed/amended/cancelled/expired | Messaging, payment obligations, projections and integrations |
| Stay stay-item checked-out/reservation completed | Review eligibility and payment settlement evaluation |
| Payment obligation/collection changed | Stay eligibility reconciliation, guest notification |
| Payment refund/payout changed | Authorized financial notifications and projections |
| Review published/hidden | Public rating aggregate refresh |
| Stay task/case changed | Assigned staff/support notification |

## API06: Reliable delivery

Write aggregate and outbox in the same owning-database transaction. Use verified persistent NATS delivery configuration rather than assuming core pub/sub is durable. Consumers commit deduplication and side effects together. Retry/backoff and dead-letter/replay operations are observable and scoped.

Events may duplicate or arrive out of order. Apply aggregate version checks and fetch authority when a gap requires reconciliation. Never roll back newer state from an older event. External provider calls use durable operations/idempotency and explicit unknown outcomes; network delivery is not exactly-once.

## Acceptance

Contract tests cover route city/ownership, list filters/totals, DTO privacy, enums in each affected client, shared error mapping, request-hash conflict, event duplicates/version gaps and legacy retail compatibility. Every mutation documents source state, resulting state, permissions, allocation/money effects, audit and recovery behavior.
