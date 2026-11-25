# Performance, observability and recovery

Dependencies: B/API; D20 targets. Measure on representative data rather than promise unsupported latency.

## PERF01: Query and pagination design

Every list takes page/pageSize with capped bounds, executes filters and stable ordering server-side, and returns filtered total. SQL uses LIMIT/OFFSET per repository policy. Date/resource windows bound calendars and exports. Expensive deep pagination must be measured and limited through explicit product constraints, not changed silently to another contract.

Index operator/property/city scope, reservation guest/status/dates, nightly inventory, active allocation ranges, rate applicability, task assignee/status/date and outbox scheduling. Choose composite order from actual query plans. Avoid N+1 per-night/per-property queries and querying all city databases on each page.

Availability queries first narrow by city/property/capacity/facets, then evaluate requested date range efficiently. Never fetch every reservation to calculate remaining stock. Dashboard aggregates run in SQL; cache/materialize expensive reports with freshness metadata and tested refresh behavior.

## PERF02: Cache and media

Redis caches public detail/taxonomies/search candidates with versioned keys and bounded TTL. Include city, locale, dates/occupancy and relevant filters in keys. Never share private user/operator responses under public keys. Invalidate on publication, rate and inventory changes; authoritative checkout revalidation remains mandatory.

Images use generated variants, cacheable public delivery, explicit dimensions and lazy loading. Private documents use access-controlled delivery. Virtualize large manager calendars and fetch visible periods/unit pages only. Debounce search inputs and discard out-of-order responses.

## PERF03: Jobs and events

Durable jobs: hold/request expiry, overdue obligation evaluation, scheduled messages, media processing, exports, channel/feed sync, financial reconciliation, review eligibility, settlement preparation and projection rebuild.

Each job has deduplication identity, bounded batch/time, lock/lease ownership, retry policy, next-run timestamp, last error and terminal failure state. A restarted worker safely resumes; two workers cannot double-pay or double-release. Lease expiry alone does not prove a previous external action failed.

Use transactional outbox and persistent verified message transport configuration. Monitor lag and oldest unprocessed event. Provide scoped replay/reconcile tooling; never require manual SQL repair for routine failures.

## PERF04: Observability

Propagate correlation IDs gateway to services, events and provider operations. Structured logs include operation/resource references but redact personal/payment data. Metrics include search/quote latency, allocation conflicts, hold expiry lag, booking conversion, pending payment age, refund/payout failures, outbox lag, sync freshness and projection drift.

Alerts require actionable owner and runbook. Differentiate normal sellout conflicts from infrastructure failures. Track event counts/cardinality safely; never use guest names or unrestricted IDs as metric labels.

## PERF05: Capacity acceptance

D20 must approve expected cities/properties/units, search traffic, concurrent checkout and batch sizes. Proposed initial benchmark targets for review: search/quote p95 under 1 second under agreed load, internal reservation transaction p95 under 500 ms excluding providers, and 100 concurrent contenders for one remaining unit producing exactly one successful allocation. These are test targets, not claims of current performance.

Record hardware, dataset, concurrency, query plans and provider exclusions with results. Include sustained load, burst contention, large portfolio calendar, deep list pages and cross-city partial failure. Define memory/connection pool/job backpressure limits.

## PERF06: Recovery and owner-run rollout

Backup and restore authoritative city records and payment ledger; verify restore exercises with owner-approved recovery objectives. Rebuild caches/public projections from authority plus durable events. Reconcile payment/provider outcomes after recovery before dispatching new payouts.

Add forward migration tracking per city, feature flags per city/property, and compatibility for old service instances during rollout. Pause new sales independently from servicing existing bookings. A failed migration in one city is visible and does not imply success in all cities.

Owner deployment checklist: provision new service/config/secrets; migrate owning databases in compatible order; start consumers/workers; run smoke booking/refund checks against approved environment; enable selected cities; verify reconciliation and alerts. Agents may prepare instructions/configuration, not deploy or push images.

## Failure acceptance

Exercise database restart, Redis loss, NATS outage/redelivery, worker crash, provider timeout, source city unavailable, missed invalidation and stale integration. Booking/money invariants survive; uncertainty is visible; retries converge without duplicates. Never replace a failed dependency with fake success.
