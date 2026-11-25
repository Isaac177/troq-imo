# External distribution and advanced growth

Dependencies: D13-D17, R/I/P/Q/F. These are retained advanced features, with explicit provider/business gates. Do not ship fake connected states or simulated successful synchronization.

## X01: Calendar feeds

Manager can add named import feeds to a property/resource, view last success/error/freshness, pause/retry/remove mappings and generate revocable export URLs. Export contains busy intervals and opaque references, never guest names, contact details, prices or access instructions.

Import persists provider/feed event UID, recurrence identity, external revision and normalized local-date interval. Handle updates, cancellations, duplicate UIDs, recurring events and timezone/all-day semantics. Bound expansion horizon and payload size. Define authoritative full-snapshot versus partial-feed deletion behavior so a truncated feed cannot free occupied dates.

External blocks use I allocation rules. Conflict with a confirmed stay creates an operational incident; do not silently cancel guests. Feed refresh is asynchronous and cannot guarantee instant cross-channel inventory consistency. UI states that limitation clearly.

Protect URL fetches from internal/private/metadata endpoints, DNS rebinding and redirect bypasses. Restrict schemes, ports, redirects, duration, bytes and recurrence expansion. Credentials remain server-side. Revoking export tokens stops subsequent reads.

## X02: Direct channel integrations

Provider onboarding requires actual approved API access. Map property, unit type, rate plan, occupancy and external identifiers. Explicitly list supported operations: availability/rates, restrictions, reservation create/change/cancel and acknowledgment.

Maintain provider operation IDs, idempotency, retry/backoff, ordered resource revisions and reconciliation snapshots. Out-of-order updates cannot revert newer rates/availability. Incoming bookings allocate through the same inventory engine; conflicting bookings produce a visible incident and contracted acknowledgment behavior.

Connection screen shows capability matrix, mapping validation, latest sync, errors, quota state and reconcile controls. Disconnecting stops future sync but retains external reservation history and unresolved operations. Do not scrape booking platforms as an integration substitute.

## X03: Groups and corporate bookings

Multi-room checkout is itemized and atomically holds one property's inventory. A group has organizer, guest roster, room assignments, payment responsibility and optional invitation links. Invitation scope cannot expose other guests' private information.

Corporate support requires verified existing company/employee contracts: traveler, purchaser, approver, cost center, spending policy and invoice recipient. Booking on behalf must record authorization and separate payer from occupant. Approval expiry and inventory hold duration must be bounded; approval does not reserve rooms indefinitely.

Cross-property group travel links independent reservations; expose partial success/failure. Never promise atomic cross-city fulfillment without a separately designed coordinator and compensation contract.

## X04: Loyalty, affiliates and promotions

Reuse only verified existing programs. Define qualifying amount/event, attribution window, self-referral prevention, program funding and cancellation/refund reversal. Reward maturity should follow approved stay/payment milestones, not a page view or uncollected booking.

Affiliate attribution is snapshotted and auditably corrected; payout uses payment-service ledger. Existing affiliate source references orders, so reservation events need a real contract extension. Points/credits must not introduce foreign currency or obscure CDF payable amounts.

Saved-search and availability alerts deduplicate changes, respect consent and avoid promising stock; clicking revalidates live availability. Curated/sponsored collections follow A07 and C06.

## X05: Add-ons and TROQ service bundles

Optional accommodation add-ons can include breakfast, extra bed, late checkout or authorized property service. Declare capacity, dates, pricing, tax/commission treatment and cancellation policy. A late checkout can affect turnover and cannot be sold independently of inventory feasibility.

Delivery/transport/other TROQ services need explicit owning-service booking contracts and separate fulfillment status. A stay booking success must not imply a delivery order succeeded. Define bundle partial failure, cancellation and refund allocation before enabling purchase.

## X06: Dynamic rates and long stays

Dynamic pricing uses documented source inputs, floor/ceiling, approval mode, versioned output and audit. Recompute only future uncontracted prices; never alter accepted booking totals. Failed inputs fall back to approved manual rates, not invented predictions.

Long stays require explicit maximum duration, cleaning cadence, payment schedule, extension and early-departure terms. Do not infer tenancy/legal treatment from normal nightly stays. Any jurisdiction-specific terms require owner-supplied policy.

## Acceptance

Fixtures: recurring feed update, revoked export, stale connection, booking conflict, provider quota/error, partially fulfilled group, corporate approval expiry, refund-reversed reward and failed bundle add-on.

Test private-data exclusion, SSRF defenses, feed deletion semantics, out-of-order channel updates, replay safety, bounded holds, reward reversals and independent bundle statuses. A provider-gated feature remains visibly unavailable until fully integrated and tested.
