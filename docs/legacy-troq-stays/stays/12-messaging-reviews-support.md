# Communication, reviews and support

Dependencies: R/Q/S/F and existing messaging/review service contracts.

## M01: Conversations

Support pre-booking inquiry threads linked to public property and optional proposed dates, and reservation threads linked to authoritative reservation/city. Participants derive from verified guest ownership and active scoped operator assignments. A supplied thread ID never grants access.

UI: inbox with unread counts, property/reservation filters, assignee, guest-visible messages and delivery status; detail alongside booking context. Internal staff notes are a separate record type and endpoint authorization path, visibly labeled and never forwarded to guests.

Message sending validates length/type, attachments, rate limits and participant access at send time. Render text safely. Revoke removed staff from future thread access. Define access to historical messages after operator transfer before implementation.

## M02: Templates and scheduled messages

Templates have en/fr content, approved typed variables, purpose and version. Preview rendered guest data before publishing. Scheduled arrival/check-out instructions reference current reservation state at dispatch, so cancellation/amendment invalidates outdated schedules.

Access codes and exact directions are sensitive payloads; release according to D18 and never insert into general marketing notifications. Translation, if provider-backed, distinguishes original from translated content and does not silently alter contractual policies.

## M03: Notifications

| Trigger | Recipient and outcome |
|---|---|
| Application/revision decision | Operator receives reason and correction link |
| Request received/accepted/declined/expired | Relevant guest/operator, current actionable state |
| Payment due/succeeded/failed/unknown | Payer/guest with correct obligation status |
| Booking confirmed/amended/cancelled | Guest/operator with latest contract reference |
| Arrival/departure reminder | Guest/staff with policy-appropriate details |
| Refund/payout progression | Authorized financial recipient only |
| Task assignment/urgent issue | Assigned staff or escalation owner |
| New message/review/case update | Authorized participant with private deep link |

Delivery channels follow verified existing capability: in-app, push, email and any separately approved SMS provider. Preferences distinguish transactional and optional marketing messages. Deduplicate by event/template/recipient/channel. Retry transient failures; show failed delivery to authorized operators; prevent repeated manual resend spam.

## M04: Verified reviews

Eligibility derives from completed stay items and defined review window, not any user-submitted reservation ID. Define one guest review per reservation/property or another approved uniqueness rule before schema. Categorized ratings cover agreed dimensions; calculate average/count only from eligible published reviews.

Guests submit text, ratings and approved media, see moderation state and edit within policy. Operators reply/report but cannot delete unfavorable reviews. Platform moderates with reason, evidence and appeal history. Hidden/rejected reviews are excluded consistently from aggregates. Never label unverified feedback as a verified stay.

## M05: Support and disputes

Case types include property mismatch, arrival failure, host cancellation, payment/refund issue, damage claim and conduct complaint. Record reservation, parties, category, priority, assignee, deadlines, evidence, guest-visible messages, internal notes and resolution.

States: OPEN, ASSIGNED, WAITING_FOR_GUEST, WAITING_FOR_OPERATOR, UNDER_REVIEW, RESOLVED, APPEALED, CLOSED. Define allowed transitions and responsible actor. Financial resolution creates an authorized refund/adjustment request in payment-service; changing case status alone cannot move money.

Escalation has configurable deadlines and an assigned owner. Attachments remain private with separate viewer authorization. Emergency contact/help content must be sourced and maintained; do not fabricate local emergency numbers.

## Acceptance and fixtures

Seed inquiry, confirmed-stay thread, reassigned staff, failed notification, cancelled scheduled message, eligible/ineligible review, reported review and appealed dispute.

Test private/internal separation, removed staff access, stale deep links, duplicate event delivery, cancellation before scheduled dispatch, review uniqueness, aggregate recalculation and refund linkage without unauthorized money movement.
