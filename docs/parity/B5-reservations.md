# B5 reservations

Source: `tzone-rs/stay-service/src/{reservations,amendments,cases,operations,admin_claims}`, `guest_messages.rs`, `engagement.rs`, `race_tests.rs`, `db_tests.rs`, gateway `routes/stays`, `routes/merchant/stays.rs`, `routes/admin/stays.rs`, `routes/private/customer_stays.rs`, `routes/admin/stay_claims.rs`, and the Stay migrations 00003, 00004, 00006 to 00009, 00018, 00019, 00024, 00026, 00045, 00046, 00048, 00051 (columns on reservations). Target: `backend/crates/domain/src/reservations/`, `backend/migrations/00012` to `00014`, `backend/seeds/reservations/`. Rust is read-checked only, never compiled. Status values: done, dropped by decision D#, open.

Defaults followed: D1 own identity (host member and grant checks go through `Actor`), D2 `hosts` table is the host (legacy column `operatorId` keeps its name and holds `hosts.id`), D3 CDF only, D4 one database with `city_id`, D5 billing module owns commission and credits.

## Module layout

| Folder | Holds |
|---|---|
| `lifecycle/` | state machine (`state.rs`), reservation view, guest list (`mine`), host list and detail, decision, special request, guest cancel and evaluation, host cancel, platform cancel and preview, withdraw, check-in, check-out, no-show, room assignment |
| `holds/` | hold creation (`preflight`, `hold`), booking mode, acceptance and guest validators, hold release and conversion, expiry sweep, race and db tests |
| `amendments/` | propose (guest and host), accept, decline, list |
| `messages/` | reservation messages |
| `cases/` | cases, evidence, platform case actions |
| `operations/` | operations tasks (host and platform) |
| `claims/` | damage claim list, summary, detail, statements, decision, payout |
| `links/` | the only place that names other modules: `pricing`, `inventory`, `billing`, `properties`, `settings` |
| `http/` | handlers by audience and the routers and OpenAPI document |

## Public surface

`mod.rs` exposes `public_routes`, `guest_routes`, `host_routes`, `admin_routes`, `openapi`, and for other modules: `ReservationView`, `OperatorReservationView`, `HoldRequest`, `GuestInput`, `NightLine`, `DiscountView`, `RESERVATION_STATES`, `can_move`, `create_hold`, `apply_decision`, `platform_cancel`, `platform_cancel_preview`, `reservation_view`, `expire_due_holds(kernel, batch)` (called by the jobs adapter), `sweep_expired_holds`, `event_types`, and all error codes from `codes.rs`. Registry line `reservations,` and `pub mod reservations;` are added.

## Outbox events emitted (aggregate type, event type)

| Aggregate | Event types |
|---|---|
| `reservation` | `reservation.held`, `reservation.request_accepted`, `reservation.request_declined`, `reservation.cancelled` (payload `chargeDueAtProperty`, `cancelledBy` OPERATOR or PLATFORM for host and platform cancels), `reservation.expired`, `reservation.checked_in`, `reservation.checked_out`, `reservation.marked_no_show`, `reservation.special_request_decided`, `reservation.amendment_proposed`, `reservation.amendment_accepted`, `reservation.amendment_declined`, `reservation.message_sent`, `reservation.damage_claimed` |
| `case` | `case.opened`, `case.closed`, `case.escalated`, `case.evidence_added`, `claim.decided` |
| `operations_task` | `operations.task.created`, `operations.task.updated` |

Payloads keep the legacy snapshot (reference, state, operatorId, propertyId, propertyAddress, currency, totalAmount, checkIn, checkOut, guest) plus enrichment (guestUserId, ownerUserId, propertyName en and fr, guestName). `ownerUserId` now comes from the active owner row of `host_members`. `reservation.guest_anonymized` is emitted by `guests` (B6).

## Features and endpoints

Routes are mounted under `/api/guest`, `/api/host`, `/api/admin`. Permission names are kernel constants. Host routes check the permission, then `ensure_host_scope` (host membership first, then scoped grant).

| Legacy location (route, NATS subject) | Mungini location | Status |
|---|---|---|
| `reservations/hold.rs`, `POST /stays/reservations`, `stay.reservation.hold` | `holds/hold.rs`, `holds/preflight.rs`, `POST /api/guest/reservations` | done |
| `reservations/handler.rs get_reservation`, `GET /reservations/{id}`, `stay.reservation.get` | `http/guest.rs get`, `GET /api/guest/reservations/:id` | done |
| `reservations/view.rs`, `queries.rs` inserts and view | `lifecycle/view.rs`, `lifecycle/queries.rs` | done |
| `reservations/booking_mode.rs` (3 tests) | `holds/booking_mode.rs` | done |
| `reservations/acceptance.rs` (3 tests), guest validation in hold | `holds/validators.rs` (4 tests) | done |
| `reservations/mine.rs`, `GET /reservations/mine`, `stay.reservation.mine` | `lifecycle/mine.rs`, `GET /api/guest/reservations/mine` (tabs pending, upcoming, active, past, cancelled) | done |
| `reservations/cancellation.rs`, `GET /reservations/{id}/cancellation`, `stay.reservation.cancellation` | `lifecycle/cancellation.rs`, `GET /api/guest/reservations/:id/cancellation` | done |
| `reservations/cancel_guest.rs`, `POST /reservations/{id}/cancel`, `stay.reservation.cancel` (idempotency key) | `lifecycle/cancel_guest.rs`, kernel idempotency scope `reservation.cancel`, `POST /api/guest/reservations/:id/cancel` | done |
| `reservations/withdraw_guest.rs`, `POST /reservations/{id}/withdraw`, `stay.reservation.withdraw` | `lifecycle/withdraw.rs`, `POST /api/guest/reservations/:id/withdraw` | done |
| `reservations/operator.rs` list and get, `GET /business/reservations`, `stay.reservation.operator.list`, `.operator.get` | `lifecycle/operator_list.rs`, `operator_queries.rs`, `operator_view.rs`, `GET /api/host/reservations`, `GET /api/host/reservations/:id` (query `hostId` replaces `operatorId`, query `locale` replaces the language metadata) | done |
| `engagement.rs stay.export.reservations`, `GET /business/reservations/export` | `export_operator_reservations` (permission `analytics.export`), `GET /api/host/reservations/export` | done |
| `reservations/operator_images.rs` | `links/properties.rs` cover and unit type summary reads | done |
| `reservations/decision.rs`, `decision_queries.rs`, `POST /business/reservations/{id}/decision`, `stay.reservation.decide` | `lifecycle/decision.rs`, `decision_queries.rs`, `POST /api/host/reservations/:id/decision` | done |
| `reservations/special_request.rs`, `POST /business/reservations/{id}/special-request`, `stay.reservation.special_request.decide` | `lifecycle/special_request.rs`, `POST /api/host/reservations/:id/special-request` | done |
| `reservations/stay_flow.rs` check-in, check-out, no-show, `stay.reservation.check_in`, `.check_out`, `.no_show` | `lifecycle/stay_flow.rs`, `POST /api/host/reservations/:id/check-in`, `/check-out`, `/no-show`; permission `operations.checkin` | done |
| `reservations/no_show.rs` | `lifecycle/no_show.rs` | done |
| `reservations/operator_cancel.rs`, `POST /business/reservations/{id}/cancel`, `stay.reservation.operator_cancel` | `lifecycle/operator_cancel.rs`, `POST /api/host/reservations/:id/cancel` | done |
| `admin_reservations/cancel.rs` cancel and preview (write part), `stay.admin.reservations.cancel`, `.cancel-preview` | `lifecycle/platform_cancel.rs`, `lifecycle/cancellation.rs platform_preview`, `POST /api/admin/reservations/:id/cancel`, `GET /api/admin/reservations/:id/cancel-preview` (platform permission `properties.moderate`). B10 must not mount these two routes again | done |
| `reservations/unit_assignment.rs` | `lifecycle/rooms.rs`; assignment inserts and releases go through `inventory::assignments` public functions | done |
| `reservations/inventory_release.rs`, `queries.rs take_night`, `insert_allocation` | owned by `inventory::nights` (B3); `holds/release.rs` keeps only hold release and `convert_holds_to_reservations` (the HOLD to RESERVATION update on allocations, a cross-table write kept from legacy because it is part of acceptance) | done |
| `reservations/expiry.rs` sweep, `run` loop | `holds/expiry.rs expire_due_holds(kernel, batch)`; the loop is the jobs `hold_expiry` task. The search-event purge belongs to `guests` | done |
| `reservations/retention.rs` (anonymize, retain_until) | dropped from this module by the lead's decision: `guests::anonymize_expired_guests` and `guests::retain_until` own it; hold and amendment call `guests::retain_until` through `support::retain_until` | dropped by decision (B6 owns) |
| `reservations/amendment_list.rs`, `GET /reservations/{id}/amendments`, `stay.reservation.amendments` | `amendments/list.rs`, `GET /api/guest/reservations/:id/amendments` | done |
| `amendments/propose.rs` guest and operator, `POST /reservations/{id}/amendments`, `/business/reservations/{id}/amendments`, `stay.amendment.propose`, `.propose.operator` | `amendments/propose.rs`, `POST /api/guest/reservations/:id/amendments`, `POST /api/host/reservations/:id/amendments` | done |
| `amendments/respond.rs`, `POST /amendments/{id}/accept`, `/decline`, `stay.amendment.accept`, `.decline` | `amendments/respond.rs`, `POST /api/guest/amendments/:id/accept`, `/decline` | done |
| `amendments/availability.rs`, `queries.rs`, `types.rs`, `db_tests.rs` (2 tests) | `amendments/availability.rs`, `queries.rs`, `types.rs`, `db_tests.rs` | done |
| `amendments/subscriptions.rs`, `reservations` NATS wiring in `consumers.rs` | replaced by routes | dropped by decision (no NATS) |
| `guest_messages.rs`, `GET and POST /reservations/{id}/messages`, `stay.message.list`, `.send` | `messages/`, guest and host routes `/reservations/:id/messages`; host side needs `messages.write` scoped | done |
| `engagement.rs` review and analytics subjects | reviews to B7, analytics to B10 | done (owned elsewhere) |
| `cases/handler.rs`, `read.rs`, `rules.rs` (6 tests), `queries.rs`, `access.rs` | `cases/service.rs`, `read.rs`, `rules.rs`, `queries.rs`, `access.rs`; routes `POST /api/guest/cases`, `/api/host/cases`, `GET /api/guest/cases/mine`, `/api/host/cases`, `GET and POST close` on `cases/:id` for both audiences | done |
| `cases/evidence.rs` (4 tests), `GET and POST /cases/{id}/evidence`, `GET /cases/evidence/{id}` | `cases/evidence.rs`, both audiences, body limit 8 MB kept | done |
| `cases/admin.rs`, `admin_actions.rs`, `stay.admin.cases.*` | `cases/admin.rs`, `admin_actions.rs`, `GET and POST /api/admin/cases`, `GET /api/admin/cases/:id`, `POST close`, `assign`, `escalate`, `GET /api/admin/cases/evidence/:id` | done |
| `operations/handler.rs`, `queries.rs`, `rules.rs` (9 tests), `types.rs`, `stay.task.*` | `operations/service.rs`, `queries.rs`, `rules.rs`, `types.rs`; `GET and POST /api/host/tasks`, `GET /api/host/tasks/:id`, `POST transition` | done |
| `operations/admin.rs`, `stay.admin.tasks.*` | `operations/admin.rs`; `/api/admin/tasks` routes | done |
| `admin_claims/*`, `stay.admin.claims.*` | `claims/`; `GET /api/admin/claims`, `/summary`, `/:caseId`, `POST statement`, `decision`, `payout` | done |
| `race_tests.rs` hold expiry versus acceptance race | `holds/race_tests.rs` (ignored, needs `MUNGINI_TEST_DATABASE_URL`) | done |
| `race_tests.rs` restriction races | inventory slice (B3) owns `restrictions/race_tests.rs` | done (owned elsewhere) |
| `db_tests.rs` last-night buyers and overlapping assignment | `holds/db_tests.rs` | done |
| `reservations/db_cancel_tests.rs` (3), `db_guest_tests.rs` (3), `db_expiry_tests.rs`, `db_room_reserve_tests.rs`, `db_unit_assignment_tests.rs` check-in room selection | `lifecycle/cancel_tests.rs`, `guest_tests.rs`, `holds/expiry_tests.rs`, `lifecycle/room_tests.rs` | done |
| `db_unit_assignment_tests.rs` scheduled move, `db_release_tests.rs`, `db_credit_tests.rs` | assignment moves and night release belong to inventory (B3), credits to billing (B8), each carries its own db tests | done (owned elsewhere) |
| CRM lead consumer of reservation events | not ported | dropped by decision D8 |
| `outbox.rs enqueue_reservation`, `reservation_snapshot`, `outbox_enrich.rs` | `events.rs` | done |

## Permissions

| Legacy | Mungini constant | Used by |
|---|---|---|
| `STAY_VIEW_RESERVATIONS` | `reservations.read` | host list and detail, case list and view |
| `STAY_MANAGE_RESERVATIONS` | `reservations.write` | decision, special request, host cancel, host amendment |
| `STAY_VIEW_GUEST_DETAILS` | `reservations.guest_details` | guest block in the host view |
| `STAY_EXPORT_DATA` | `analytics.export` | host export |
| `STAY_MANAGE_CHECKIN`, `STAY_MANAGE_HOUSEKEEPING`, `STAY_MANAGE_MAINTENANCE` | `operations.checkin`, `operations.housekeeping`, `operations.maintenance` | check-in flow, task types |
| `STAY_MANAGE_MESSAGES` | `messages.write` | host side of messages |
| `STAY_MANAGE_CASES` | `cases.write` | open, close, evidence on the host side |
| `STAY_MODERATE_LISTINGS` through `auth.stay.platform_moderator` | `properties.moderate` with `require_platform` | admin cases, tasks, claims, platform cancel |

## Notifications

This module emits the events above and sends nothing. Templates and delivery are B9. Seeds: reservation and message events are read by `notifications` from the outbox.

## Adaptations

| Area | Change |
|---|---|
| Errors | Typed `ErrorCode` constants in `codes.rs` replace English messages. Internal faults (unknown booking mode, unreadable contract, inconsistent inventory, missing timezone) stay `Internal` |
| Money | Claim amounts, summary totals and approved amounts are decimal strings (legacy returned `f64` in admin claims). Penalty, refund and delta use two decimals as before |
| Identity | `session_user` and `operator_ids_for` replaced by `Actor` and `ensure_host_scope`. Request parameter `operatorId` is now `hostId`. Response views keep the legacy field `operatorId` |
| City | Every lookup filters on `city_id`. Admin open case no longer takes a `city` field, the request city is used |
| Idempotency | Kernel `idempotency` replaces `rate_admin::keys`. A busy key returns `REQUEST_IN_PROGRESS` |
| Dates | Timestamps in messages are `DateTime<Utc>` JSON values, not `::text` |
| Media | Covers are read only. Rows without a `storageKey` (legacy inline content) are skipped with a warning because the lazy upload belongs to B2 |
| Message ids | `msg-*` prefix instead of a bare uuid |
| Case close event | payload also carries `operatorId` so the owner enrichment works |
| Amendment list | missing `nights` in a stored contract gives an empty list instead of a decode error |

Legacy behaviour ported as is, to fix separately: `COMPLETED` appears in the `past` tab predicate but is not a reservation state; `PENDING_PAYMENT` has no producer (payment at the property); evidence is stored as `bytea` in the database; a property-scoped grant does not cover host-level case actions because the scope check passes no property.

## Schema contract

Migrations written by this slice: `00012_reservations.sql`, `00013_reservation_cases.sql`, `00014_operations_tasks.sql`. Legacy table and column names are kept (camelCase quoted columns). The legacy `city` column became `city_id` on `stay_reservations`. `city_id` was added to `stay_amendments`, `stay_reservation_messages`, `stay_cases`, `stay_operations_tasks`. Child tables (`stay_reservation_items`, `stay_reservation_guests`, `stay_records`, `stay_case_evidence`, `stay_claim_statements`, `stay_claim_decisions`) reach the city through their parent. Foreign keys to tables owned by other modules (`stay_properties`, `stay_unit_types`, `stay_rate_plans`, `stay_physical_units`, `stay_quotes`) are not declared here, only to `hosts` and to tables of this slice. Translatable content: none of these tables stores translated text; property and unit type names are read from the legacy `nameEn` and `nameFr` columns of tables owned by B2 and B3 (listing, task and claim read models). If B2 moves them to per-locale content tables, the SQL in `operator_queries.rs`, `mine.rs`, `events.rs`, `operations/admin.rs`, `claims/list.rs`, `claims/detail.rs` and `links/properties.rs` changes.

Tables written: `stay_reservations`, `stay_reservation_items`, `stay_reservation_guests`, `stay_records`, `stay_amendments`, `stay_reservation_messages`, `stay_cases`, `stay_case_evidence`, `stay_claim_statements`, `stay_claim_decisions`, `stay_operations_tasks`. Cross-table writes kept from legacy: `stay_inventory_allocations` (HOLD to RESERVATION conversion in `holds/release.rs`), `stay_reservation_items."physicalUnitId"` is mine.

Reads from other modules: `stay_properties` (name, timezone, operatorId, address), `stay_unit_types`, `stay_template_media`, `stay_property_media`, `stay_physical_units`, `stay_unit_assignments`, `stay_availability_blocks`, `stay_inventory_pools`, `stay_pool_physical_units`, `stay_night_inventory`, `stay_inventory_allocations`, `hosts`, `host_members`. SQL functions used: `stay_unallocated_physical_blocks`, `stay_room_available` (inventory).

Reservation columns owned by billing but stored here: `commissionRateBps`, `commissionAmount`, `operatorNetAmount`, `commissionChargedAmount`, trigger `stay_reservation_net_default`.

## Calls into other modules (all in `links/`)

| File | Used items |
|---|---|
| `links/pricing.rs` | `pricing::quotes::{availability::open_nights, availability::room_available, cancellation::{evaluate, free_cancellation_deadline, CancellationTerms}, charges::{price_stay, StayTerms}, context::{ensure_bookable, guest_factor, load_context, QuoteContext}, store::find}`, `pricing::promotions::{load_active, lock_budget}`, `pricing::rates::load_rules` |
| `links/settings.rs` | `pricing::settings::{get_i64, SettingKey}` (approval and amendment window hours, claim window days, case response days) |
| `links/inventory.rs` | `inventory::assignments::{insert_assignment, release_for_item, release_for_reservation, lock_item_context, lock_unit, turnover_blocked}`, `inventory::nights::{take_night, insert_allocation, release, ReleaseState}`, `inventory::restrictions::{enforce, enforce_amendment, StayRequest}` |
| `links/billing.rs` | `billing::credits::charge::{take_commission, release_commission, release_for_penalty, sync_amended_commission}`, `billing::credits::gate::ensure_can_book` |
| `support.rs` | `guests::retain_until` |
| `links/properties.rs` | no call, reads and presigns through `kernel.storage` |

## Seeds

Files in `backend/seeds/reservations/`, listed in `00-order.txt` after the inventory seeds. Legacy ids were kept (`seed-res-goma-*`, `seed-item-goma-*`, `seed-case-*`, `seed-claim-*`, `seed-task-*`); legacy operator `seed-op-goma-approved` maps to `seed-host-goma-lake` and the legacy owner user to `seed-user-goma-host`.

| File | Covers |
|---|---|
| `01-reservations-goma.sql` | all nine reservation states, instant and request modes, special request NONE, REQUESTED, ACCEPTED, DECLINED, a penalty no-show, an anonymized guest, an additional guest, stay records, amendments in every state (PROPOSED, ACCEPTED, DECLINED, EXPIRED, SUPERSEDED) |
| `02-reservations-kin.sql` | the nine states for Kinshasa, a proposed amendment, stay records |
| `03-cases-claims-goma.sql` | cases OPEN, AWAITING_RESPONSE, RESOLVED, CLOSED, assigned, escalated and platform-opened; claims open with and without evidence, approved, partial paid, rejected, payout failed; statements; evidence; messages |
| `04-cases-tasks-kin.sql` | Kinshasa cases, a claim, messages, tasks in every state and type |
| `05-tasks-goma.sql` | Goma tasks in every state, type and priority |

Inventory seeds (B3) already hold the allocations and assignments for `seed-res-goma-confirmed`, `-requested`, `-expired`, `-cancelled` and `seed-res-kin-confirmed`. Their assignment seed only inserts when the item exists, so `inventory/05-assignments-feeds.sql` is listed a second time at the end of `00-order.txt`; it is idempotent.
