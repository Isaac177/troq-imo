# B2 properties and media

Module `backend/crates/domain/src/properties/`. Migrations `00021_properties.sql` (tables) and `00022_properties_foreign_keys.sql` (foreign keys added to inventory, wishlist, pricing and review tables). Seeds `backend/seeds/properties/`. Rust and SQL are read-checked only. No cargo or database command was run, no test was run.

Decisions followed: D2 (hosts are the `hosts` table, the legacy `"operatorId"` column keeps its name and references `hosts(id)`, the wire name is `hostId`), D13 (legacy `nameEn`/`nameFr` style columns, no per-locale tables, no `source_locale`), D4 (`city_id` on property and template rows), D1 (access through the kernel `Actor`). The per-locale amenity tables written earlier by another agent stay, with the bilingual columns added by `00023`.

Two agents worked in this folder. The host revisions (`revisions/`: list, get, create, update, withdraw, submit, readiness, lifecycle, `moderation.rs` decide) were written by the inventory_b2 agent. The `listings/` folder (admin side) was written afterwards and also contains a decide, a lifecycle and a readiness. The duplicate decide, lifecycle and readiness were then consolidated into `listings/`.

## Parity rows

| Legacy location | Mungini location | Status |
|---|---|---|
| `properties/handler.rs` list, get, create (NATS `stay.property.list`, `get`, `create`; gateway `/stays/properties`) | `property/service.rs`, `property/handler.rs`; `GET/POST /api/host/properties`, `GET /api/host/properties/{id}`; permissions `properties.read`, `properties.write` host scoped | done |
| `properties/validation.rs` (names 200, address 255, description 5000, timezone, coordinates pair) and its tests | `property/validation.rs`, `validation.rs` with the same tests | done |
| `properties/queries.rs` columns, filters, effective guest contact, facts | `property/queries.rs`, `facts.rs`; effective contact falls back to `hosts.guest_contact_*` | done |
| `properties` idempotency of create (`stay_idempotency_keys`) | kernel `kernel_idempotency_keys` through `kernel::idempotency` | done |
| create refuses a host that is not APPROVED | host must be `ACTIVE` (the `hosts` table has no APPROVED state), codes `HOST_NOT_ACTIVE`, `HOST_NOT_FOUND` | done (adapted to D2) |
| `brandId` on properties and create input | dropped, Mungini has no brands | dropped by decision D2 |
| `property.created` outbox event | `property.created` with `propertyId`, `hostId`, `ownerUserId`, `propertyName {en, fr}`, `city` | done |
| `media/` list, add, update, remove (gateway `/stays/properties/{id}/media`) | `media/` generic over property and template media: `service.rs`, `queries.rs`, `access.rs`, `handler.rs`; host and admin routes `/properties/{id}/media`, `/properties/{id}/media/{mediaId}`; DELETE takes `expectedVersion` in the body as before | done |
| media validation, PNG and JPEG, 5 MB, alt text, rejected rows kept with a reason | `media/validation.rs`, `media/rejection.rs`; rejected rows store a code (`MEDIA_UNSUPPORTED_TYPE`, ...) in `rejectionReason` instead of English text | done (adapted to the code rule) |
| media client key idempotency, cover rules, sort order, cover promotion on removal, version check | `media/service.rs`, `media/queries.rs` (`update_if_version`, `promote_first_ready`, `lock_owner`) | done |
| media object upload and delete with cleanup on failure | `media/storage.rs`, `media/service.rs` (object removed when the transaction fails) | done |
| `media.content` and `admin.media.content` (presigned URL, fallback to template media) | `GET /api/host/media/{id}/content`, `GET /api/admin/media/{id}/content`; the template fallback is replaced by `/accommodation-templates/{tid}/media/{mid}/content` | done (adapted) |
| bytea `content` column, lazy upload `ensure_url`, `backfill` worker | no `content` column, objects are stored on upload; the copy script C1 must upload legacy bytea to storage and set `"storageKey"` | dropped by decision (new schema, noted for C1) |
| media rename of `propertyId`/`templateId` in views | one `MediaView` with `ownerId` | done (wire change) |
| `amenities/` catalog, admin list, create, update (gateway `/stays/amenities`, `/admin/stays/amenity-groups`) | `amenities/` (written earlier), admin routes `GET/POST /api/admin/amenity-groups`, `PATCH .../{code}`, `GET/POST /api/admin/amenities`, `PATCH .../{code}`; public routes live in `api/src/amenities.rs`; permission `properties.configure` platform scoped (legacy gateway required the platform admin role) | done |
| `amenities` `ensure_known` on revision create, update and admin update | `amenities::ensure_known`, used by `revisions` and `listings/admin_property.rs` | done |
| Canonical amenity groups and codes from legacy migration `00028_amenity_catalog.sql` | `backend/migrations/00027_legacy_amenities.sql`; preserves existing catalog rows | open: 70 legacy records matched by static comparison; migration and catalog tests not run |
| `accommodation_categories/` catalog (public and authenticated), admin list, create, update | `categories/`; `GET /api/public/accommodation-categories`, admin `GET/POST /api/admin/accommodation-categories`, `PATCH .../{code}`; the authenticated catalog route is folded into the public one; audit and outbox added | done |
| `accommodation_categories` ensure_selectable, active codes | `categories::ensure_selectable`, `categories::active_codes`, re-exported as `properties::ensure_category_selectable`, `properties::active_category_codes` | done |
| `accommodation_templates/` list, create, update with version check, covers | `templates/`; host `GET/POST /api/host/accommodation-templates`, `PATCH .../{id}`; admin same under `/api/admin`; host scoped `properties.read` for lists, `properties.configure` for writes | done |
| template `baseNightlyAmount` bigint whole CDF | `numeric(14,2)` CDF, wire decimal string through `kernel::Money`, range 1 to 999999999 | done (money rule) |
| template unit validation from `rate_admin::unit_type_validation` | `inventory::unit_types::{validate_unit_type, validate_inventory_model}` with quantity 1 | done |
| `template_media/` list, add, update, remove, content | `template_media/` over the generic media service; host and admin routes under `/accommodation-templates/{id}/media` | done |
| `template_media/read.rs` `covers_for_templates` | `template_media/read.rs` (`properties::covers_for_templates`) | done |
| `template_media/read.rs` `images_for_unit_types`, `covers_for_unit_types`, `unit_type_summary`, `cover_for_unit_type`, `public_media` | read by inventory (`unit_types`), reservations, reviews (`covers.rs`), guests (`discovery/queries.rs`) and `properties::media::cover_for_unit_type` in their own queries | done (moved to the readers) |
| `listings/` readiness blockers and tests | `listings/readiness.rs` and `revisions/readiness.rs`; blocker `OPERATOR_NOT_APPROVED` is now `HOST_NOT_ACTIVE` | done |
| `listings` host revisions (list, get, create, update, withdraw, submit) | `revisions/` (inventory_b2 agent); host readiness and lifecycle handlers call `listings::service::host_readiness` and `host_lifecycle` | done |
| `listings` admin queue, get revision with live content, decide | `listings/admin_queue.rs`, `listings/service.rs`, `listings/review.rs`; `GET /api/admin/listings/queue`, `GET /api/admin/listings/{revisionId}`, `POST /api/admin/listings/{revisionId}/decision`; permission `properties.moderate` platform scoped | done |
| `listings` suspend, reinstate | `listings/review.rs`; `POST /api/admin/properties/{id}/suspend`, `.../reinstate`; events `property.suspended`, `property.reinstated` with `ownerUserId` and names | done |
| `listings` admin lifecycle (publish from DRAFT allowed) | `listings/lifecycle.rs`; `POST /api/admin/properties/{id}/lifecycle` | done |
| `listings` admin properties list, get, revisions, create, update | `listings/admin_list.rs`, `admin_property.rs`; `GET/POST /api/admin/properties`, `GET/PATCH /api/admin/properties/{id}`, `GET .../revisions` | done |
| `stay.admin.operators.list` | hosts module (B1) | done (not B2) |
| `property.revision.submitted` outbox event | emitted by `revisions/service.rs` submit | done by the revisions agent; payload carries `propertyId`, `hostId`, `ownerUserId`, `propertyName` |
| listing decision events `property.revision.approved`, `changes_requested`, `rejected` | `listings/review.rs` with names and owner | done |
| `listings/db_tests.rs` and fixtures | `listings/db_tests.rs` (five ignored tests on the seed, `MUNGINI_TEST_DATABASE_URL`); the full draft to archived flow test needs inventory and pricing fixtures | partial: not run |
| Gateway `listings.rs`, `listing_admin.rs`, `admin_property.rs`, `admin_templates.rs`, `media.rs`, `template_media.rs`, `amenities.rs`, `accommodation_categories.rs`, `accommodation_templates.rs` | `routes.rs`, `catalog_routes.rs`, `docs.rs`, `catalog_docs.rs` | done |
| Seeds `goma.sql`, `kin.sql` property, revision, media, template, template media rows | `seeds/properties/02-properties.sql`, `02a-details.sql`, `03-revisions.sql`, `04-categories.sql`, `05-property-media.sql`, `06-templates.sql`, `07-template-media.sql`, order in `00-order.txt` | done |
| Per-locale content tables (rule 6) | not used, D13 | dropped by decision D13 |

## Open

| Item | Status |
|---|---|
| Duplicate review, lifecycle and readiness: consolidated into `listings/review.rs`, `listings/lifecycle.rs` and `listings/readiness.rs`; `revisions/moderation.rs`, `lifecycle.rs` and `readiness.rs` deleted, host and admin routes use the `listings/` set | closed |
| Media seed rows point at storage keys under `goma/stays/...` and `kin/stays/...` that no object backs; the owner uploads fixture images before the cover URLs resolve | open for the owner |
| Active-language validation: not applicable under D13 | closed |
| Compile, tests and seed load | open for the owner |
| Host scoped property grants (`Scope::Property`) are held closed by identity until ownership verification; list scoping uses `Actor::scoped_access` as in legacy | open (B1) |

## Schema contract

Migration `00021_properties.sql` (extension `postgis` created there). Legacy table and column names, camel case quoted columns, `city_id text NOT NULL CHECK IN ('GOMA','KIN')` on property and template rows.

| Table | Columns | Keys and indexes |
|---|---|---|
| `stay_accommodation_categories` (global, 10 legacy rows inserted) | `code varchar(48)` pk `^[A-Z0-9_]+$`, `"nameEn"`, `"nameFr"` varchar(80), `icon` `^[A-Za-z0-9]+$`, `"sortOrder"`, `active`, `revision bigint`, `"createdAt"`, `"updatedAt"` | |
| `stay_properties` | `id`, `city_id`, `"operatorId"` fk `hosts(id)`, `timezone`, `status` DRAFT APPROVED PUBLISHED PAUSED SUSPENDED ARCHIVED, `"publishedRevision"`, `"nameEn"`, `"nameFr"`, `"descriptionEn"`, `"descriptionFr"`, `"addressLine"`, `latitude numeric(9,6)`, `longitude numeric(9,6)` (both null or both set), `neighborhood`, `"checkInFrom"`, `"checkInUntil"`, `"checkOutBy"` (HH:MM), `"houseRulesEn"`, `"houseRulesFr"`, `"petsPolicy"`, `"smokingPolicy"`, `"guestContactPhone"`, `"guestContactEmail"`, `amenities jsonb`, `"verificationStatus"`, `version`, `"createdAt"`, `"updatedAt"` | `stay_properties_operator_idx`, `(city_id, status)`, GIST on `ST_SetSRID(ST_MakePoint(longitude::float8, latitude::float8), 4326)::geography` |
| `stay_property_revisions` | `id`, `"propertyId"` fk, `"revisionNumber"`, `snapshot jsonb`, `"authorUserId"`, `"reviewStatus"` DRAFT SUBMITTED CHANGES_REQUESTED APPROVED REJECTED, `"reviewReasons"`, `"reviewGuidance"`, `version`, `"submittedAt"`, `"submittedByUserId"`, `"reviewedAt"`, `"reviewedByUserId"`, `"createdAt"`, `"updatedAt"` | unique `("propertyId","revisionNumber")`, one open revision per property, queue index |
| `stay_property_media` | `id`, `"propertyId"` fk, `status` READY REJECTED REMOVED, `"sortOrder"`, `"isCover"`, `"altEn"`, `"altFr"`, `"fileName"`, `"mimeType"`, `"sizeBytes"`, `"storageKey"` (no `content`), `"rejectionReason"`, `"clientKey"`, `version`, `"addedByUserId"`, `"createdAt"`, `"updatedAt"` | unique `("propertyId","clientKey")`, one cover per property, READY needs size and key |
| `stay_accommodation_templates` | `id`, `city_id`, `"operatorId"` fk `hosts(id)`, `"categoryCode"` fk, `"inventoryModel"`, `"nameEn"`, `"nameFr"`, `"descriptionEn"`, `"descriptionFr"`, `"maxAdults"`, `"maxChildren"`, `"bedroomCount"`, `"bedCount"`, `"bathroomCount"`, `accessibility jsonb`, `"baseNightlyAmount" numeric(14,2)`, `status` ACTIVE ARCHIVED, `version`, `"createdAt"`, `"updatedAt"` | `(city_id, "operatorId", status, "createdAt")` |
| `stay_template_media` | same columns as property media with `"templateId"` fk | same rules |

Migration `00022_properties_foreign_keys.sql` adds foreign keys to `stay_properties` from `stay_unit_types`, `stay_physical_units`, `stay_inventory_pools`, `stay_availability_blocks`, `stay_restrictions`, `stay_calendar_feed_tokens`, `stay_wishlist_items`, `stay_rate_plans`, `stay_reviews`, and to categories and templates from `stay_unit_types`. Reservations and promotions are not constrained.

Read from other modules, names assumed from their migrations: `hosts` (`id`, `city_id`, `display_name`, `status`, `guest_contact_phone`, `guest_contact_email`), `host_members`, `identity_users`, `stay_unit_types`, `stay_physical_units`, `stay_inventory_pools`, `stay_rate_plans`, `stay_rate_rules`, `stay_reservations`, `stay_reservation_items`. All cross-module SQL is in `facts.rs`.

## Public surface

`properties::{public_routes, guest_routes, host_routes, admin_routes, openapi, find_property, lock_property, PropertyView, PROPERTY_STATUSES, property_for, property_for_actor, admin_property, ensure_moderator, ensure_category_selectable, active_category_codes, covers_for_templates}`, `properties::media::{service, kind::MediaKind, cover_for_unit_type, upload_image, delete_image, image_url}`, `properties::amenities::{ensure_known, list_active_groups, list_active_amenities}`.

Moderation service entry points now enforce the platform permission themselves, in addition to the route check. Host and admin lifecycle use the shared listing readiness and transition implementations. Rust has only been read-checked; database and authorization tests remain open.

Template creation now checks and locks the referenced host in the requested city before inserting. This preserves the city boundary formerly provided by separate Stay databases. The SQLx city-scope test is written but has not been run.
