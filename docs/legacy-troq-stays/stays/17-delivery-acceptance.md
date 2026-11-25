# Delivery plan and acceptance checklist

Read README and decisions before assigning work. This ordering retains advanced scope; it does not redefine the project as a smaller MVP. No delegation is implied by this document.

## QA01: Dependency slices

| Slice | Feature IDs | Complete result | Predecessors/gates |
|---|---|---|---|
| 1 | R01-R05, B01-B05 | Scoped memberships, one new role, compatible contracts and service skeleton with real behavior | D01-D03/D12 decisions before schema |
| 2 | O01-O06, A02-A04, U | Operator application through reviewed published property, real units/media | Slice 1; D04/D18 |
| 3 | I01-I06, P01-P05 | Concurrency-safe calendar, restrictions, rates and reproducible quotes | Slice 2; D05-D11 |
| 4 | C01-C06 | Public bilingual discovery/detail, maps if approved, saved stays/alerts | Slice 3; public projection decisions |
| 5 | Q01-Q06, F01-F04 | Guest booking through payment, amendment, cancellation and refund | Slice 3/4; real provider support |
| 6 | S01-S06, F05-F06 | Arrival through departure, tasks, settlement and reconciliation | Slice 5; D10/D15/D19 |
| 7 | M01-M05, A05-A08 | Conversations, notifications, reviews, cases and full admin operations | Relevant earlier slices |
| 8 | X01-X06 | Approved integrations, groups/corporate, rewards, bundles and dynamic rates | D13-D17; provider contracts |
| 9 | SEC/PERF/API across all | Load, privacy, recovery, legacy compatibility and owner rollout package | All applicable slices |

Apply security, tests and observability within each slice; slice 9 verifies the assembled system rather than postponing these obligations.

## QA02: Feature completion contract

For every feature ID, implementation evidence must include:

1. Settled applicable decisions and explicit included/excluded subfeatures.
2. Owning service/app and authoritative entities; next forward migration when needed.
3. Exact API methods, field types/bounds, permissions, scope, errors and idempotency behavior.
4. State transition/inventory/money effects and retry/recovery design.
5. Screens, navigation entry, role/permission behavior and all U03 states.
6. en/fr translations, CDF/date/phone compliance and responsive/accessibility verification.
7. Durable domain events and integration effects where applicable.
8. Recognizable synthetic seeds covering happy path, each state and important failures.
9. Automated tests for invariants and real integration boundaries, plus UI verification.
10. Changed services/apps, validation output and owner-only manual steps.

A page with mocked data, dead controls, unconditional success or a hidden broken backend is not complete.

## QA03: Seed matrix

| Fixture family | Required examples |
|---|---|
| Identity | Existing merchant with stay membership; new manager; desk/cleaner/accountant presets; revoked/expired invitation; second unrelated operator |
| Properties | Single apartment; pooled-room hotel; linked whole building/rooms; second city; draft/rejected/published/paused/suspended/archived revisions |
| Inventory | Last room, all sold, partial date gap, maintenance, owner block, external block, expiring/consumed hold, same-day turnover |
| Pricing | Missing nightly rate, seasonal override, capacity limit, child pricing, coupon exhausted, non-stackable discounts, exact rounding boundary |
| Reservations | Requested/declined/expired/awaiting payment/confirmed/cancelled/completed; multi-room; accepted/rejected/expired amendment |
| Payments | Paid/partial/unknown/failed; duplicate callback; partial/failed refund; deposit claim; held/failed/paid payout; post-payout refund |
| Operations | Unassigned arrival, unready room, partial check-in, unit move, no-show, late arrival correction, maintenance incident |
| Communication | Inquiry, scheduled reminder invalidated by cancellation, failed delivery, private internal note, eligible/ineligible review, appealed case |
| Advanced | Stale calendar, external conflict, channel error, corporate approval expiry, partial group fulfillment, reversed reward, bundle failure |

Use seed-* identifiers or recognizable supported UUID mapping if schemas require UUID. Seed creation is idempotent and preserves existing rows. Use synthetic documents/media approved for demo use; never real guest identity records or credentials. No arbitrary insert SQL against shared databases outside authorized migration/seed workflow.

## QA04: Invariant tests

- Parallel last-room requests: at most capacity succeeds; no negative inventory.
- Linked property/room inventory: conflicting allocation cannot succeed.
- Pooled reservation plus physical assignment: no overlapping assigned room.
- Quote expiry/price changes: no unaccepted amount charged.
- Duplicate submission/callback/event: one logical reservation/ledger effect.
- Payment after hold expiry: reallocate safely or create tracked compensation.
- Concurrent cancellation/amendment/check-in/payout: valid final state and no orphan allocations.
- Partial refunds: combined refunds and reserved refund operations never exceed collected eligible amount.
- Ledger: balanced entries; settlement cannot include already paid entries.
- Policy edits: old reservation terms remain unchanged.
- Actor/resource/city substitution: unauthorized access denied at service boundary.
- Revocation and export: permission changes invalidate writes and subsequent downloads.
- Outage/crash: recoverable work persists, no assumed provider failure or duplicate external charge.

Use real PostgreSQL for locking/constraint tests; mocked repositories cannot prove concurrency invariants.

## QA05: UI and contract tests

Verify client/admin/manager paths in en/fr, narrow/large screens, slow/offline states, keyboard/focus, map alternative list and role-specific navigation. Assert hidden private fields are absent from responses, not merely visually hidden.

Contract tests exercise existing retail payment/review/notification behavior alongside new stay resource types. Verify role/permission enum compatibility across affected Rust/TS consumers and city migration order. Test deep links without assuming selected city equals booking city.

## QA06: Commands and reporting

Run cargo check from tzone-rs or affected service folders; run focused domain/integration tests. Inspect each touched JS package.json and run its actual bun run typecheck or build script. Use bun tooling only. Do not assume a typecheck script exists or report skipped checks as passing.

If parallel implementation is later explicitly authorized, the integrating agent runs the full relevant build/typecheck after merging. A child report is not integration evidence.

For documentation-only changes, validate links, coverage IDs, unresolved decision references and consistency; code builds are unnecessary because no executable code changed.

## QA07: Launch acceptance

Review D21 line by line against this package. Required features are functional, seeded and tested; provider-gated items have real integration evidence or explicit owner-approved exclusion. Performance/recovery targets are measured. Approval flows and on-call ownership are defined. Owner receives migration/deployment/runbook instructions and performs deployment.

Never label the entire module done while a required feature remains a stub, unresolved business gate or unverified financial operation.
