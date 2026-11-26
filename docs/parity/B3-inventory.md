# Parity ledger B3, inventory

Source on main (`tzone-rs` at `780396f`): `stay-service/src/{availability,assignments,blocks,restrictions,calendar,sync}`, `race_tests.rs`, `db_tests.rs`, `rate_admin/unit_types*.rs` and `unit_type_validation.rs` (moved here by lead decision), `admin_calendar`, gateway `routes/stays/{blocks,calendar,feed_links,inventory,rate_admin,admin_unit_types}.rs`, `routes/admin/stay_calendar.rs`, migrations 00002, 00012 to 00015, 00029 to 00031, 00033 to 00035, 00041, seeds `goma.sql`, `kin.sql`, `goma_operations_scenarios.sql`. Target: `backend/crates/domain/src/inventory`. Rust is read-checked only, never compiled or run by the agent. Status: done, dropped by decision D#, open.

Defaults followed: D1 own identity (kernel `Actor`), D2 own hosts (host scope through `Actor::ensure_host_scope`, the host id is the legacy `"operatorId"` column), D4 one database with `city_id`.

## Endpoints, subjects and permissions

Host routes sit under `/api/host`, admin under `/api/admin`, public under `/api/public`. The gateway mounted the merchant routes under `/stays` and the admin routes under `/stays` as well. Legacy permission to Mungini name: `STAY_VIEW_PROPERTIES` to `properties.read`, `STAY_MANAGE_AVAILABILITY` to `inventory.write`, `STAY_MANAGE_INTEGRATIONS` to `inventory.integrations`, `STAY_MANAGE_CONFIGURATION` to `properties.configure`, `STAY_VIEW_RESERVATIONS` to `reservations.read`, `STAY_MANAGE_RESERVATIONS` to `reservations.write`, `STAY_VIEW_GUEST_DETAILS` to `reservations.guest_details`, platform moderator call (`auth.stay.platform_moderator`) to `Actor::require_platform(properties.moderate)`.

| Legacy (route, NATS subject, permission) | Mungini | Status |
|---|---|---|
| `GET /availability`, `stay.availability.get`, view properties | `GET /api/host/availability`, `availability/handler.rs` | done |
| `GET /properties/{id}/blocks`, `stay.block.list` | `GET /api/host/properties/:property_id/blocks`, `blocks/handler.rs` | done |
| `POST /blocks`, `stay.block.create`, manage availability | `POST /api/host/blocks` | done |
| `GET /blocks/{id}`, `stay.block.get` | `GET /api/host/blocks/:block_id` | done |
| `DELETE /blocks/{id}`, `stay.block.remove` | `DELETE /api/host/blocks/:block_id` (body `expectedVersion`) | done |
| admin `GET /stays/blocks`, `stay.admin.block.list` | `GET /api/admin/blocks?propertyId=` | done |
| admin `POST /stays/blocks`, `stay.admin.block.create` | `POST /api/admin/blocks` | done |
| admin `POST /stays/blocks/{id}/remove`, `stay.admin.block.remove` | `POST /api/admin/blocks/:block_id/remove` | done |
| `GET /properties/{id}/restrictions`, `stay.restriction.list` | `GET /api/host/properties/:property_id/restrictions` | done |
| `POST /restrictions/bulk`, `stay.restriction.bulk` | `POST /api/host/restrictions/bulk` | done |
| admin `POST /stays/restrictions/bulk`, `stay.admin.restriction.bulk` | `POST /api/admin/restrictions/bulk` | done |
| `POST /unit-assignments`, `stay.unit.assign`, manage reservations | `POST /api/host/unit-assignments` | done |
| `POST /unit-assignments/{id}/move`, `stay.unit.move` | `POST /api/host/unit-assignments/:assignment_id/move` | done |
| `GET /reservation-items/{id}/unit-assignments`, `stay.unit.assignments`, view reservations | `GET /api/host/reservation-items/:item_id/unit-assignments` | done |
| `GET /reservation-items/{id}/unit-options`, `stay.unit.options` | `GET /api/host/reservation-items/:item_id/unit-options` | done |
| `GET /calendar`, `stay.calendar.get` | `GET /api/host/calendar` | done |
| admin `GET /stays/calendar`, `stay.admin.calendar.get` | `GET /api/admin/calendar` | done |
| `POST /calendar/inventory-default`, `stay.calendar.inventory_defaults.update` | `POST /api/host/calendar/inventory-default` | done |
| admin `POST /stays/inventory-defaults`, `stay.admin.inventory_defaults.update` | `POST /api/admin/inventory-defaults` | done |
| `GET /calendar/feed.ics`, `stay.calendar.feed` | `GET /api/host/calendar/feed.ics` (text/calendar) | done |
| `GET/POST /calendar/feed-tokens`, `stay.calendar.feed_token.list/create` | `GET/POST /api/host/calendar/feed-tokens` | done |
| `DELETE /calendar/feed-tokens/{id}`, `stay.calendar.feed_token.revoke` | `DELETE /api/host/calendar/feed-tokens/:token_id` | done |
| public `GET /stays/calendar/feeds/{city}/{token}`, `stay.calendar.feed_by_token` | `GET /api/public/calendar/feeds/:city/:token`; the path in newly created tokens is `/api/public/calendar/feeds/{CITY}/{tokenId}.{secret}.ics`; unknown city, malformed or revoked token is `404 FEED_NOT_FOUND` | done (old subscription URLs change at cutover, see C3) |
| `GET/POST .../properties/{id}/unit-types`, `stay.unit_types.list`, `stay.unit_type.create` (host) | `GET/POST /api/host/properties/:property_id/unit-types`, `unit_types/handler.rs` | done |
| `PATCH/DELETE .../unit-types/{id}`, `stay.unit_type.update`, `.archive` (host) | `PATCH/DELETE /api/host/properties/:property_id/unit-types/:unit_type_id` (archive takes query `expectedVersion`, default 0) | done |
| admin unit type routes and `stay.admin.unit_types.*`, `stay.admin.unit_type.*` | the same four routes under `/api/admin/properties/...` | done |
| `stay.operator.sync`, JetStream `STAY_SYNC`, `sync/{apply,consumer,types}.rs` (operator status copied from auth into `stay_operators`) | none: hosts are written by the `hosts` module directly | dropped by decision D2 |
| `sync/types.rs` tests (status and kind mapping, payload parse) | none, the mapping no longer exists | dropped by decision D2 |
| NATS `subscriptions.rs` in every folder and `admin_calendar/subscriptions.rs` | axum routes in `routes.rs` | dropped by decision (no NATS) |
| gateway `ensure_writable` (view-only city mode) | kernel `Actor` extractor rejects writes in view-only mode with `CITY_VIEW_ONLY` | done |

## Behaviour and adaptations

| Legacy | Mungini | Status |
|---|---|---|
| `blocks`: range 1 to 365 nights, reason catalogue (7), PLATFORM reserved for admin, unit type belongs to property, idempotency key `block.create`, property row lock, one allocation per night through `take_night`, conflict lists unavailable nights, audit `property.block_created`, outbox `property.block_created` | `blocks/{validation,service}.rs`, `nights/take.rs`; same order of checks; conflict is `NIGHTS_UNAVAILABLE` with `nights` and `dates` params | done |
| `blocks`: remove with version check, platform block only removable by admin, release of the block nights, audit and outbox `property.block_removed` | `blocks/service.rs::remove`, `nights/release.rs` | done |
| legacy quirk kept: admin created blocks store `origin = 'OPERATOR'` with reason `PLATFORM` (the check allows `PLATFORM` origin) | ported as is in `blocks/service.rs` | done, note for a separate fix |
| `restrictions`: 7 kinds, basis rules, value 0 to 365, window at most 730 days, bulk 1 to 50 operations, `ADD` and `REMOVE`, idempotent bulk with stored response, audit and outbox `property.restrictions_changed` | `restrictions/{validation,bulk,queries}.rs`; stored response is the typed `BulkRestrictionResult` | done |
| `restrictions::check::{evaluate, strictest, contains, overlaps}`, unit rule overrides property rule, strictest value wins, turnover buffer for single-unit pools | `restrictions/check.rs` unchanged logic; unit tests in `check_tests.rs` (6) | done |
| rule violations were English sentences in `violations` and in 400 messages | typed `Violation { code, params }` (`ARRIVAL_CLOSED`, `DEPARTURE_CLOSED`, `MIN_NIGHTS`, `MAX_NIGHTS`, `ADVANCE_NOTICE`, `MAX_HORIZON`, `TURNOVER_BUFFER`); `enforce` returns `400 STAY_RULE_VIOLATED` with the first one in params | done (adapted to rules, section 4) |
| `restrictions::amendment_check::enforce_amendment` | `restrictions/amendment_check.rs` | done |
| `restrictions::batch::violations_batch` (search engine) | `restrictions/batch.rs`, returns `Vec<Vec<Violation>>`; guests only tests emptiness | done |
| `availability::handler`: 365 night cap, unit type belongs, booking day in the property time zone, per night remaining, `bookable` | `availability/service.rs`, kernel `timezone::local_date` | done |
| `availability::queries::remaining_by_night`, `quotes::queries::{open_nights, room_available}`, `reservations::queries::{take_night, insert_allocation}`, `reservations::inventory_release::release` | `nights/{remaining,take,release}.rs`; `insert_allocation` takes `city` and generates the id; `take_range` added for blocks; reservations and guests already call these | done |
| `assignments`: assign, move, history, options, turnover buffer, unit lock, state checks, overlap exclusion constraint mapped to a conflict | `assignments/{service,queries,buffer}.rs`; overlap is `UNIT_OVERLAP`; `insert_assignment`, `release_for_item`, `release_for_reservation`, `lock_item_context`, `lock_unit`, `turnover_blocked` exported for reservations | done |
| `assignments::buffer` rules (unit scoped wins, largest buffer, window filter) | `chosen_buffer` pure function with 3 unit tests | done |
| `calendar::handler`: 93 day cap, units page with totals, nights, nightly prices, reservations with guest names only with guest detail permission, blocks, restrictions, local today for the active unit | `calendar/{view,service,queries,types}.rs` | done |
| calendar nightly price (`quotes::pricing::nightly_minor`, lowest over ACTIVE plans, skip plans without a base rate) | `calendar/prices.rs` calls `pricing::rates::{load_rules, nightly_minor}` and formats with kernel `Money` | done |
| calendar unit cover url (`template_media::read::covers_for_unit_types`, signed) | `calendar/view.rs` reads `stay_template_media` cover key and signs it with `properties::media::image_url`; media without a storage key (legacy inline bytes) are skipped | done |
| calendar names `nameEn`, `nameFr` | kept as `nameEn` and `nameFr` (pricing, guests and reviews read the same columns); per-locale content tables are not applied, see Translatable content | done, decision for the owner |
| `calendar::feed_tokens`: token `ftk-<id>.<64 hex>`, only the SHA-256 hash is stored, revoke, list, create, public feed by token | `calendar/feed_tokens.rs` with 4 unit tests | done |
| `calendar::ics`: iCalendar text, `PRODID` and UID domain `troqzone` | `calendar/ics.rs`, `PRODID -//Mungini//Stays//EN` and UID domain `stays.mungini`; event UIDs differ from legacy so external calendars re-import | done (adapted, product name) |
| `calendar::inventory_defaults` | `calendar/defaults.rs` | done |
| `rate_admin::unit_types` list, create, `load`, `attach_images` | `unit_types/{read,write,queries}.rs`; images signed through `properties::media::image_url` from `stay_template_media` | done |
| `rate_admin::unit_type_updates` (resize inventory, quantity locked once nights, allocations or reservations exist) | `unit_types/write.rs`, `pools.rs::resize_pool`, `queries::quantity_is_locked` | done |
| `rate_admin::unit_type_archive` (blocked by reservations, archives rate plans, retires units) | `unit_types/archive.rs`, `pools.rs::retire_units` | done; it still runs `UPDATE stay_rate_plans SET status = 'ARCHIVED'` (pricing table), pricing exposes no function for it, replace the statement when B4 adds one |
| `unit_type_validation::validate_unit_type` (imported by `accommodation_templates`) | `unit_types::validate_unit_type(&UnitTypeFields)` and `validate_inventory_model`, 5 unit tests; B2 imports through `inventory::unit_types` | done |
| `accommodation_categories::ensure_selectable` and `accommodation_templates::operator_of` used by create and update | read-only SQL in `unit_types/queries.rs` on `stay_accommodation_categories` and `stay_accommodation_templates` (B2 tables), error codes `UNIT_TYPE_CATEGORY_UNKNOWN`, `UNIT_TYPE_TEMPLATE_UNKNOWN` | done |
| `rate_admin::keys` idempotency for unit types (`unit_type.create`, `unit_type.update`) | `unit_types/idempotency.rs` over kernel idempotency, stored full view, same replay and busy behaviour | done |
| pool provisioning inside unit type create and resize | public `inventory::pools::{create_pool, resize_pool, retire_units}` | done |
| `media::access::{property_for, property_for_actor}`, `membership::operator_ids_for`, `operator_scope` | `scope.rs` (`host_property`, `admin_property`, `actor_property`, `lock_property`); reads `stay_properties` (`id`, `"operatorId"`, `city_id`, `timezone`) | done |
| `audit::record` and `outbox::enqueue` per write | kernel `audit::record(&actor.audit())`, `outbox::enqueue(city, ...)` with the legacy action and event names | done |
| free-form `ServiceError` messages | `messages.rs` typed codes with params, translated by clients | done |
| `calendar/types.rs` pagination test, `blocks`, `restrictions` validation tests, `ics` tests | next to the code | done |

## Tests

| Legacy | Mungini | Status |
|---|---|---|
| `race_tests.rs::competing_removals_of_one_restriction_apply_exactly_once`, `competing_keyed_bulk_edits_apply_once` | `restrictions/race_tests.rs` (ignored, need `MUNGINI_TEST_DATABASE_URL` with the Goma seed) | done |
| `race_tests.rs::hold_expiry_and_acceptance_race_leaves_one_consistent_outcome` | belongs to B5 (holds and reservations) | done (owned by B5) |
| `db_tests.rs::only_one_of_many_concurrent_buyers_gets_the_last_night`, `a_unit_cannot_hold_two_overlapping_assignments` | `db_tests.rs` | done |
| `calendar/inventory_tests.rs` (default capacity and physical blocks agree) | `db_tests.rs` | done |
| unit tests of `blocks`, `restrictions`, `assignments::buffer`, `calendar` | unit tests in the same files | done |

## Notifications

Legacy inventory code sends no notification and no document. Outbox events (`property.block_created`, `property.block_removed`, `property.restrictions_changed`, `property.inventory_changed`, `reservation.unit_assigned`) are written in the same transaction as before. No row.

## Seeds

Legacy `goma.sql`, `kin.sql`, `goma_operations_scenarios.sql` inventory rows are ported as `backend/seeds/inventory/01` to `05` (listed in `00-order.txt`), with `city_id`, for Goma and Kinshasa. They cover unit types ACTIVE, INACTIVE, ARCHIVED, both inventory models, pools with default capacity on and off, physical units ACTIVE, OUT_OF_SERVICE and RETIRED, nights with RESERVATION, HOLD and BLOCK allocations in states ACTIVE, RELEASED and EXPIRED, a zero capacity night, blocks of every reason and origin (one removed, one at physical unit level without allocation), restrictions of every kind and basis (one removed), unit assignments ASSIGNED, MOVED, CHECK_IN and one released, and feed tokens active and revoked (secrets are 64 times `1`, `2`, `3`). Assignment rows are inserted only when the reservation item exists (B5 seeds). `seed-pool-*` ids and unit type ids used by pricing, billing and guests seeds are kept.
Open for seeds: `calendar-contracts.sql` is reservation snapshot data and belongs to B5. Allocations reference reservation ids (`seed-res-goma-confirmed`, `seed-res-goma-requested`, `seed-res-kin-confirmed` and new `seed-res-goma-cancelled`, `seed-res-goma-expired`, `seed-res-goma-dorm-a`, `seed-res-goma-dorm-b`, `seed-res-kin-suite-hold`) without a foreign key; B5 seeds should create reservations with those ids to make them visible.

## Schema contract

Migration `backend/migrations/00020_inventory.sql`. Legacy table and column names kept (camel case quoted columns) plus `city_id text NOT NULL CHECK IN ('GOMA','KIN')` on tables with rows per city. Foreign keys to `stay_properties`, `stay_accommodation_categories` and `stay_accommodation_templates` are not created (B2 tables do not exist yet); add them when those migrations land.

Owned, written:

| Table | Columns | Notes |
|---|---|---|
| `stay_unit_types` | `id`, `city_id`, `"propertyId"`, `"inventoryModel"`, `"categoryCode"`, `"templateId"`, `"nameEn"`, `"nameFr"`, `"descriptionEn"`, `"descriptionFr"`, `"maxAdults"`, `"maxChildren"`, `"bedroomCount"`, `"bedCount"`, `"bathroomCount"`, `accessibility jsonb`, `status`, `version`, `"createdAt"`, `"updatedAt"` | unique `(id, "propertyId")`; legacy 00002, 00030, 00031 merged; indexes of 00021 (active by property, gin accessibility) |
| `stay_physical_units` | `id`, `city_id`, `"propertyId"`, `"unitTypeId"`, `"localCode"`, `status`, `"createdAt"`, `"retiredAt"` | unique `("propertyId","localCode")` |
| `stay_inventory_pools` | `id`, `city_id`, `"propertyId"`, `"unitTypeId"` unique, `"sellableQuantity"`, `"defaultNightlyCapacity"` default true, `version`, `"createdAt"` | legacy 00029 merged |
| `stay_pool_physical_units` | `"poolId"`, `"physicalUnitId"` unique | |
| `stay_night_inventory` | `"poolId"`, `"stayDate"`, `"sellableCapacity"`, `"allocatedCount"`, `version` | no `city_id`, reached through the pool |
| `stay_inventory_allocations` | `id`, `city_id`, `"poolId"`, `"physicalUnitId"`, `"referenceType"`, `"referenceId"`, `"startDate"`, `"endDate"`, `quantity`, `state`, `"expiresAt"`, `"createdAt"`, `"releasedAt"` | partial index on active pool ranges |
| `stay_availability_blocks` | `id`, `city_id`, `"propertyId"`, `"unitTypeId"`, `"physicalUnitId"`, `"startDate"`, `"endDate"`, `reason` (7 values), `origin`, `"actorUserId"`, `"conflictDisposition"`, `version`, `"createdAt"`, `"removedAt"` | legacy 00012, 00041 merged |
| `stay_restrictions` | `id`, `city_id`, `"propertyId"`, `"unitTypeId"`, `kind` (7 values), `basis`, `"startDate"`, `"endDate"`, `value`, `"actorUserId"`, `version`, `"createdAt"`, `"removedAt"` | legacy 00013, 00015 merged |
| `stay_unit_assignments` | `id`, `city_id`, `"reservationItemId"`, `"physicalUnitId"`, `"startDate"`, `"endDate"`, `reason`, `"actorUserId"`, `"createdAt"`, `"releasedAt"` | gist exclusion on active ranges per unit (needs `btree_gist`, created in the migration); foreign key to `stay_reservation_items` |
| `stay_calendar_feed_tokens` | `id`, `city_id`, `"propertyId"`, `"secretHash"`, `"createdByUserId"`, `"createdAt"`, `"revokedAt"` | |
| SQL functions | `stay_unallocated_physical_blocks(pool_id text, night date)`, `stay_room_available(unit_type text, from_date date, to_date date, exclude_reservation text)` (turnover buffer version of 00035) | names kept for pricing, guests, reviews |

Written outside the module (kept, to be moved behind owner functions): `stay_reservation_items."physicalUnitId"` (assignments `set_item_unit`), `stay_rate_plans.status` (unit type archive).

Read, owned elsewhere: `stay_properties` (`id`, `"operatorId"`, `city_id`, `timezone`) from B2; `stay_accommodation_categories` (`code`, `active`), `stay_accommodation_templates` (`id`, `"operatorId"`), `stay_template_media` (`id`, `"templateId"`, `status`, `"storageKey"`, `"isCover"`, `"sortOrder"`, `"createdAt"`, `"altEn"`, `"altFr"`) from B2; `stay_rate_plans` (`id`, `city_id`, `"unitTypeId"`, `status`), `stay_rate_rules` from B4; `stay_reservations`, `stay_reservation_items`, `stay_reservation_guests`, `stay_amendments` from B5. Kernel tables through kernel functions only (`kernel_audit_records`, `kernel_outbox`, `kernel_idempotency_keys`). The legacy `stay_audit_records`, `stay_outbox`, `stay_idempotency_keys`, `stay_inbox` are not used.

Translatable content: stay-service stores unit type names and descriptions as `nameEn`, `nameFr`, `descriptionEn`, `descriptionFr` columns, and rule 6 asks for per-locale content tables. Pricing, guests and reviews already read the legacy columns, so the Mungini shape differs from the rule and was not changed here. Decision for the owner: keep the four columns, or add `stay_unit_type_content` and `"sourceLocale"` together with B2, B4, B6 and B7 in one pass.

## Public surface (for dependents)

`inventory::nights::{take_night, insert_allocation, take_range, release, ReleaseState, ReleasedInventory, remaining_by_night, open_nights, room_available}`, `inventory::assignments::{insert_assignment, release_for_item, release_for_reservation, lock_item_context, lock_unit, turnover_blocked}`, `inventory::restrictions::{enforce, enforce_amendment, violations, violations_batch, StayRequest, Violation, overlapping, apply_bulk_edit}` and `restrictions::check::enforce`, `inventory::pools::{create_pool, resize_pool, retire_units}`, `inventory::unit_types::{validate_unit_type, validate_inventory_model, UnitTypeFields, UnitTypeView}`, `inventory::calendar::feed_text`, `inventory::scope::{PropertyRef, load, lock_property}`, routes `public_routes`, `guest_routes` (empty), `host_routes`, `admin_routes`, `openapi`.

## Open

| Item | Status |
|---|---|
| Foreign keys from inventory tables to `stay_properties`, categories and templates | open until the B2 migration exists |
| Per-locale unit type content (rule 6) | open, owner decision above |
| Move the two writes outside the module (`stay_reservation_items."physicalUnitId"`, `stay_rate_plans.status`) behind functions of reservations and pricing | open, small follow up |
| Compile, tests and seed load not run (read-checked only) | open for the owner |
