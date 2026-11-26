# Parity ledger B6, guests

Source on `main`: `tzone-rs/stay-service/src/{public_discovery,search_engine,wishlists}`, gateway `routes/stays/{public_discovery,me,query,reply}.rs`, `routes/public/stays.rs`, `routes/private/customer_stays.rs`, migrations 00008, 00020, 00021, `reservations/retention.rs`, `reservations/expiry.rs`, seeds `goma.sql`. Target: `backend/crates/domain/src/guests`, `backend/migrations/00017_guests.sql`, `backend/seeds/guests`. Rust is read-checked only, nothing compiled or run, tests not run.

Defaults followed: D1 (guest is the kernel `Actor`, any authenticated user), D2 (the host of a property is a row of `hosts`, joined on the legacy column `"operatorId"`, listed when `hosts.status = 'ACTIVE'`, replacing `stay_operators.status = 'APPROVED'`), D4 (one database, `city_id` on every row, PostGIS for distance).

Wire changes that apply to every row: success bodies are the kernel envelope, errors are typed codes from `guests/messages.rs` (legacy free-form text dropped), routes live under `/api/public` and `/api/guest` without the `stays` segment, wishlist and saved-search ids are not changed, wishlist and saved-search routes use `/wishlists` and `/saved-searches` instead of `/me/wishlists` and `/me/saved-searches`. Translatable fields keep Stay's shape by owner decision: the legacy `nameEn`/`nameFr`, `descriptionEn`/`descriptionFr`, `altEn`/`altFr` columns, no per-locale content tables and no `source_locale`. Responses return one resolved `name`, `description`, `alt` chosen by the `locale` query parameter (default `en`, `fr` picks the French column): the requested column, falling back to the other when it is empty (`listing.rs` `pick`). Converting to per-locale tables is a later migration.

## Endpoints

| Legacy location | Mungini location | Status |
|---|---|---|
| `GET /api/public/stays/destinations`, NATS `stay.public.destinations` | `GET /api/public/destinations`, `discovery/handlers.rs` `destinations`, `discovery/queries.rs` `published_count` | done |
| `GET /api/public/stays/properties`, `stay.public.properties.search` | `GET /api/public/properties`, `search/` (`run_search`), query parsing in `search/params.rs` keeps the gateway rules (comma lists, lenient numbers and dates) | done |
| `GET /api/public/stays/search/catalog`, `stay.public.search.catalog` | `GET /api/public/search/catalog`, `search/catalog.rs`. The `documentation` key (pointed at a TROQ doc) is dropped, `endpoint` now `/api/public/properties`. Amenity codes come from `property_amenities` (B2), category codes from `stay_accommodation_categories` | done |
| `GET /api/public/stays/properties/{id}`, `stay.public.property.get` | `GET /api/public/properties/{propertyId}`, `discovery/detail.rs` | done |
| `GET /api/public/stays/properties/{id}/availability`, `stay.public.property.availability` | `GET /api/public/properties/{propertyId}/availability`, `discovery/availability.rs`. Legacy behaviour kept: `children` above 0 is refused because no child ages can be sent (port as is, fix in a separate task) | done |
| `GET /api/public/stays/properties/{id}/calendar`, `stay.public.property.calendar` | `GET /api/public/properties/{propertyId}/calendar`, `discovery/calendar.rs`, 1 to 186 days, today in UTC as legacy | done |
| `GET /api/public/stays/media/{id}/content`, `stay.public.media.content` | `GET /api/public/media/{mediaId}/content`, `discovery/media.rs`. Legacy lazy upload of bytea content to storage (`ensure_url`, backfill loops) is dropped: Mungini stores only keys, the cutover copy script uploads | dropped (no legacy bytea in Mungini; copy script C1 uploads) |
| `GET /api/stays/me/wishlists`, `stay.wishlist.list` | `GET /api/guest/wishlists`, `wishlists/` | done |
| `POST /api/stays/me/wishlists`, `stay.wishlist.create` (idempotency key) | `POST /api/guest/wishlists`, kernel idempotency, scope `wishlist.create`. Keys are now checked by the kernel (1 to 128 characters) | done |
| `PATCH /api/stays/me/wishlists/{id}`, `stay.wishlist.update` | `PATCH /api/guest/wishlists/{wishlistId}` | done |
| `GET /api/stays/me/wishlists/{id}/items`, `stay.wishlist.items.list` (`propertyIds` filter) | `GET /api/guest/wishlists/{wishlistId}/items`, `wishlists/item_filter.rs`, one SQL for filtered and unfiltered, total from SQL | done |
| `POST /api/stays/me/wishlists/{id}/items/{propertyId}`, `stay.wishlist.item.add` | `POST /api/guest/wishlists/{wishlistId}/items/{propertyId}` | done |
| `DELETE /api/stays/me/wishlists/{id}/items/{propertyId}`, `stay.wishlist.item.remove` | `DELETE /api/guest/wishlists/{wishlistId}/items/{propertyId}` | done |
| `GET /api/stays/me/saved-searches`, `stay.saved_search.list` | `GET /api/guest/saved-searches`, `saved_searches/` | done |
| `POST /api/stays/me/saved-searches`, `stay.saved_search.create` | `POST /api/guest/saved-searches` | done |
| `DELETE /api/stays/me/saved-searches/{id}`, `stay.saved_search.delete` | `DELETE /api/guest/saved-searches/{savedSearchId}` | done |
| `GET /api/public/stays/amenities`, `accommodation-categories`, `properties/{id}/reviews`, `calendar/feeds/...` | not guests: B2 (amenities, categories), B7 (reviews), B3 (feeds) | dropped from this slice (owned by B2, B7, B3) |

## Permissions

| Legacy | Mungini | Status |
|---|---|---|
| Public routes: no session | `/api/public` routes use `RequestCity` only | done |
| Wishlists and saved searches: `session_user` (authenticated user id required, own rows only) | `Actor` extractor on `/api/guest`, every query filters `city_id` and `"guestUserId"`, no permission constant needed. City view-only tokens cannot write (kernel) | done |

## Behaviour and limits

| Legacy | Mungini | Status |
|---|---|---|
| 20 lists per guest, 200 items per list, 20 saved searches, name 1 to 80 characters, parameters an object up to 2048 bytes, visibility PRIVATE or SHARED | `wishlists/types.rs`, `wishlists/validators.rs`, `saved_searches/validators.rs`, same values, typed codes with params | done |
| Items and counts only show properties that are published, hosted by an approved operator and not disabled by the credit gate (`credits::gate::enabled_sql`) | `listing.rs` `listed` (status PUBLISHED, host ACTIVE, credit gate SQL reading `stay_operator_commission_rates`, `stay_credit_accounts`, `stay_platform_settings` with legacy names; B8 may expose the fragment later) | done |
| Search engine v3: validation, candidates in batches of 250, pricing, temp table staging, facets, stats, paging in SQL, repeatable read | `search/{query,normalize,candidates,pricing,stage,aggregates,rank,engine,response,facets}.rs`, same rules and numbers | done |
| Distance: haversine on coordinates rounded to 2 decimals | `search/candidates.rs`: `ST_Distance` between the geography of the property latitude and longitude rounded to 2 decimals (as legacy) and the search point, divided by 1000, plus an `ST_DWithin` prefilter on the unrounded point with a 2 km margin. No new column: the point is built from `latitude` and `longitude`. Geography uses the spheroid, so values differ from the sphere haversine by up to about 0.3 percent. A GIST expression index on `ST_SetSRID(ST_MakePoint(longitude::float8, latitude::float8), 4326)::geography` is recommended in the properties migration | done (adapted, D4) |
| Search text matches `nameEn` or `nameFr` | same, both columns | done |
| Money in `numeric` text, `format_minor` and `parse_minor` | kernel `Money` (`to_wire`, `parse_input`), `AMOUNT_INVALID` | done |
| `search_events` stored only with `analyticsConsent`, no child ages, no search text, no guest id | `search/events.rs`, `anonymised`, unit test | done |
| Search events purged after six months (1000 per sweep, inside the hold expiry sweep) | `retention/search_events.rs` `purge_search_events(kernel, batch)`, called by jobs task `search_event_purge` (B11) | done |
| Guest anonymization (`reservations/retention.rs`, `anonymize_expired`, 100 per sweep, audit `guest.anonymize`, outbox `reservation.guest_anonymized`) and `retain_until` | `retention/anonymize.rs` `anonymize_expired_guests(kernel, batch)` called by the jobs adapter, `retention/dates.rs` `retain_until(days, check_out)` | open: B5 also wrote `reservations/holds/retention.rs` with the same logic. Both are idempotent (`anonymizedAt IS NULL`, `SKIP LOCKED`) but there must be one owner. Recommended: B5 deletes its copy and calls `guests::retain_until` and the jobs task, or the owner moves the function back to `reservations` and changes the jobs adapter |
| Reviews column `reviewCount` and `verifiedRating` in search hits (always 0 and null in legacy) | same constants | done (port as is; B7 may fill) |
| Legacy `STAY_GUEST_RETENTION_DAYS` | kernel `Config::guest_retention_days` | done (kernel) |

## Notifications and events

| Legacy | Mungini | Status |
|---|---|---|
| Wishlists, saved searches and discovery send no notifications and emit no outbox events. Saved search `alertConsent` is stored only, no alert job exists on `main` | same | done (no legacy feature). The alert delivery is not built in legacy either |
| Outbox `reservation.guest_anonymized` | emitted by `anonymize_expired_guests` through `kernel::outbox::enqueue` | done (see anonymization row) |

## Tests

| Legacy | Mungini | Status |
|---|---|---|
| `search_engine/query_tests.rs` (10) | `search/query_tests.rs` (12, adds guest, child age and paging cases) | done |
| `search_engine/stage_tests.rs` (1 text test, 1 database test over 1002 properties) | text test in `query_tests.rs`; the 1002 property database test is not ported | open: needs a database harness for the temp tables, add with the owner's test database |
| `search_engine/db_tests.rs` (11, ignored) | `db_tests.rs` ports lakeside offers, instant, accessibility, distance and analytics consent (3 of the 11 are folded in); meal plan, payment, price sort, sold out and page tests are not ported | open: remaining db cases once the properties, pricing and inventory seeds exist |
| `wishlists/db_tests.rs` (3, ignored) | `db_tests.rs` (4, adds idempotent create and city separation) | done |
| `public_discovery/db_tests.rs` (1, ignored) | not ported | open: needs the property seed with content rows |
| `reservations/retention.rs` unit test | `retention/dates.rs` (2) | done |
| New unit tests | `listing.rs`, `locale.rs`, `search/params.rs`, `search/catalog.rs`, `search/candidates.rs`, `discovery/calendar.rs`, `wishlists/tests.rs`, `saved_searches/tests.rs` | done |

## Seeds

| Legacy | Mungini | Status |
|---|---|---|
| `goma.sql` wishlists (2), items, saved search (1) | `seeds/guests/01-guests-goma.sql`: 4 lists (private, shared, empty, one holding listed, paused and archived properties), 3 saved searches (with and without alerts, with child ages, with geo), 3 search events (recent, thirty days, 200 days old for the purge) | done |
| `kin.sql` (none) | `seeds/guests/02-guests-kin.sql`: 3 lists (private, shared, one pointing at a draft property), 2 saved searches, 2 search events (one 240 days old) | done |
| Listed in `00-order.txt` after the property, reservation and review seeds | appended `guests/01-guests-goma.sql`, `guests/02-guests-kin.sql` | done |

## Open dependencies on other slices

| Item | Status |
|---|---|
| Property schema (B2): no properties migration exists yet. `guests` reads `stay_properties` with the legacy columns `id`, `city_id`, `"operatorId"`, `status`, `"verificationStatus"`, `timezone`, `amenities` (jsonb array of codes), `"nameEn"`, `"nameFr"`, `"descriptionEn"`, `"descriptionFr"`, `latitude`, `longitude`; `stay_property_media` with `id`, `"propertyId"`, `status`, `"isCover"`, `"sortOrder"`, `"createdAt"`, `"mimeType"`, `"altEn"`, `"altFr"`, `"storageKey"`; `stay_template_media` with the same media columns plus `"templateId"`; `stay_accommodation_categories` (`code`, `active`). Names are in `guests/listing.rs`, `search/candidates.rs`, `discovery/queries.rs`, `search/catalog.rs` | open: check against the properties migration when B2 writes it |
| Foreign key from `stay_wishlist_items."propertyId"` to the property table was dropped from `00017_guests.sql` because the table does not exist at that number. The service refuses unlisted properties on add | open: B2 adds the foreign key (or cleans items when a property is deleted) in its migration |
| Inventory (B3): `00020_inventory.sql` checked: `stay_unit_types` has `"nameEn"`, `"nameFr"`, `"descriptionEn"`, `"descriptionFr"`, `"categoryCode"`, `"templateId"`, `accessibility`, counts, `"inventoryModel"`; pools, night inventory and both SQL functions exist. `guests/collaborators.rs` imports `crate::inventory::nights::remaining_by_night` and `crate::inventory::restrictions::{violations_batch, StayRequest}`, and the SQL functions `stay_room_available` and `stay_unallocated_physical_blocks` plus the pool and night tables | open until `pub mod inventory;` is in `domain/src/lib.rs` and the SQL function migration exists |
| Pricing (B4): `crate::pricing::{quotes, rates, promotions}` items used in `collaborators.rs`; `00019_pricing.sql` checked: `stay_policy_versions` (`"tombstonedAt"`), `stay_rate_plans` (`"childNightlyAmount"`, `"cancellationPolicyVersionId"`, `"paymentPolicyVersionId"`) match | done |
| Billing (B8): credit gate tables in `listing.rs`; `00018_billing.sql` and `00019_pricing.sql` checked: `stay_operator_commission_rates` (`"operatorId"`, `"rateBps"`, `"effectiveFrom"`, `"createdAt"`), `stay_credit_accounts` (`"operatorId"`, `balance`), `stay_platform_settings` (per city: `city_id`, `key`, `"valueInt"`, the gate now filters `ps.city_id = hosts.city_id`) | done |
| Reservations (B5): `stay_reservations` and `stay_reservation_guests` match the anonymization SQL (checked against `00012_reservations.sql`) | done |

## Schema contract

Tables created by `00017_guests.sql` (legacy names, legacy camelCase columns, `city_id` added; kept legacy `"guestUserId"`):

| Table | Columns | Keys and indexes |
|---|---|---|
| `stay_wishlists` | `id`, `city_id`, `"guestUserId"`, `name varchar(80)`, `visibility` (PRIVATE, SHARED), `"createdAt"`, `"updatedAt"` | primary key `id`; index (`city_id`, `"guestUserId"`, `"createdAt"`) |
| `stay_wishlist_items` | `"wishlistId"`, `"propertyId"`, `"addedAt"` | primary key (`"wishlistId"`, `"propertyId"`); cascade foreign key to wishlists; index on `"propertyId"` |
| `stay_saved_searches` | `id`, `city_id`, `"guestUserId"`, `parameters jsonb` (object), `"alertConsent"`, `"createdAt"` | primary key `id`; index (`city_id`, `"guestUserId"`, `"createdAt"`) |
| `stay_search_events` | `id`, `city_id` (legacy column `city`), `fingerprint`, `engine_version`, `query jsonb`, `result_count`, `candidate_plans`, `priced_plans`, `duration_ms`, `"createdAt"` | primary key `id`; indexes (`fingerprint`, `"createdAt"`) and `"createdAt"` |

Read only (owned elsewhere): `hosts` (identity), `stay_properties`, `stay_unit_types`, `stay_property_media`, `stay_template_media`, `stay_accommodation_categories`, `property_amenities`, `property_amenity_groups` (B2), `stay_inventory_pools`, `stay_night_inventory`, functions `stay_room_available`, `stay_unallocated_physical_blocks` (B3), `stay_rate_plans`, `stay_rate_rules`, `stay_promotions`, `stay_policy_versions` (B4), `stay_operator_commission_rates`, `stay_credit_accounts`, `stay_platform_settings` (B8). Written outside the module: `stay_reservation_guests` (anonymization, see the open row), `kernel_audit_records`, `kernel_outbox`, `kernel_idempotency_keys` (through the kernel).

Translatable content: `stay-service` stores it as `nameEn`, `nameFr`, `descriptionEn`, `descriptionFr`, `altEn`, `altFr` columns. Mungini keeps that shape for now (owner decision); the guests module only reads them and resolves the language per request. Per-locale content tables are a later migration, and only `listing.rs` `pick` and the search text clause in `search/candidates.rs` would change.

Temporary tables `stay_search_offers` and `stay_search_properties` are created per search transaction (`ON COMMIT DROP`), as in legacy.

## Public surface for dependents

`domain::guests`: `public_routes`, `guest_routes`, `host_routes` (empty), `admin_routes` (empty), `openapi`, registered in `api/src/registry.rs`. Functions: `anonymize_expired_guests(&Kernel, batch)`, `purge_search_events(&Kernel, batch)`, `retain_until(Option<i64>, NaiveDate)`, `property_is_listed(&PgPool, City, &str)`, `run_search(&PgPool, &Storage, City, locale, SearchQuery, consent)`, wishlist and saved-search service functions, and the typed error codes of `messages.rs`.
