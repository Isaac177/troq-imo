# Administration

Dependencies: R, B, U, financial and verification decisions. Use existing admin app shell. Every endpoint enforces city/platform scope; hiding actions is not authorization.

## Screen inventory

Overview; operator applications; application detail; listing revision queue; listing review; reservations; reservation detail; finance/refunds/payouts; disputes; reviews; configuration; campaigns; analytics/exports; audit and workflow failures.

## A01: Overview and exceptions

Filters: city, operator, property, booking-created range and stay-date range as separately labeled dimensions. Show confirmed reservations, booked nights, occupancy, collected money, platform earned commission, cancellation rate, outstanding refunds and blocked workflows. Each metric links to a consistently filtered list.

Never equate booked gross value with collected cash or recognized revenue. Occupancy denominator excludes closed/out-of-service inventory according to an explicit reporting definition. Compare equivalent periods. SQL supplies aggregates; UI does not sum paginated rows.

## A02: Operator verification

Application fields: identity type/reference, contact, supporting document references, submission revision, reviewer, timestamps, decision and reasons. Documents remain private. States: DRAFT, SUBMITTED, UNDER_REVIEW, CHANGES_REQUESTED, APPROVED, REJECTED, WITHDRAWN. Approval is distinct from user account activation.

Review UI shows document status, relevant identity data, duplicate candidates and prior decisions. Reviewer claims an application with a version check; concurrent decisions conflict. Request corrections identifies fields and localized reason codes. Resubmission creates a new revision. Suspension prevents new business as configured but preserves servicing of existing bookings.

## A03: Listing moderation

Show published and proposed revisions side by side, with changed fields, photos, location, unit capacities, policy references and completeness failures. Publication requires verified operator, valid sellable accommodation, required content/media and approved policies.

Moderators approve/reject the exact revision reviewed. Subsequent edits invalidate a pending approval where relevant. Define sensitive fields needing re-review through D18. Rejection never removes an existing published revision unless an explicit suspension action is taken.

## A04: Configuration and taxonomy

Manage property/bed/amenity/accessibility/meal types, rule templates, policy templates, reason codes, destinations and notification templates. Fields have stable code, en/fr names, active flag, ordering and revision. Retire referenced values; never delete history. Only global owner writes shared taxonomy; replication status is visible.

Finance configuration separates commission agreements, platform fees, funding, payment schedules and settlement policies. Preview effective date and affected future sales. Existing reservation snapshots remain unchanged.

## A05: Reservation intervention

Search by reference, guest, operator, property, city, arrival/departure and separate reservation/payment/stay states. Detail tabs: contract, guests, charges, events, messages, changes and cases, each permission-filtered.

Support may propose amendments or cancellations through the same domain engine as managers. No generic database edit button. Exceptions require reason, authority and any configured second approval. Relocation creates a linked replacement workflow and defines original cancellation/refund and price responsibility before confirmation.

## A06: Finance oversight

Refund queue shows contractual entitlement, collected/refunded amounts, policy exception, requester and approval limit. Payout queue shows eligible balance, holds, destination verification, preparer and approver. Disable self-approval where required. Approval must re-evaluate current ledger/version to prevent stale decisions.

Reconciliation screen groups missing callbacks, mismatched amounts, ambiguous provider outcomes and failed operations. Actions are retry status lookup, approve a bounded correction or escalate; never manually mark provider payment successful without a verified financial process.

## A07: Cases, reviews and campaigns

Case workspace includes assignee, priority, SLA deadline, linked reservation, evidence and resolution. Internal notes are visibly separate from guest/operator messages. Review moderation requires reason and preserves original content and appeal history under retention rules.

Campaign editor controls city/property eligibility, placement, dates, content, budget and funding. Scheduled publication uses timezone-aware timestamps. Sponsored placements must be identified to guests. Do not fabricate ratings or scarcity.

## A08: Audit, exports and emergency actions

Audit filters include actor, city, operator, resource, action and date; sensitive payloads are redacted. Export is asynchronous, scoped at generation and download, expires, and has size/date limits. Record export actor and fields.

Emergency pause can target new reservations, property publication or payouts independently. Preview affected properties/bookings and required follow-up. Pausing sales must not prevent guests from viewing confirmed bookings or requesting support.

## API and data obligations

Use admin endpoint family in API spec, permissions R03 and common version/idempotency requirements. Moderation decision payload: revisionId, decision, reasonCode, optional localized corrective guidance, expectedVersion. Financial actions use financial request IDs rather than arbitrary amounts posted to reservation records.

## Seeds and acceptance

Create submitted/rejected/resubmitted applications; competing reviewers; approved property with pending revision; suspended property with future guests; failed refund; held payout; active dispute and expired campaign.

Verify scoped totals, same-revision approval, separation of financial duties, immutable booking terms, permission-filtered tabs, asynchronous export revocation and emergency continuity for existing guests.
