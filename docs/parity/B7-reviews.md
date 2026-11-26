# Parity ledger B7, reviews

Source on main: `tzone-rs/stay-service/src/reviews` (11 files), gateway `routes/stays/reviews.rs`, `routes/stays/engagement_routes.rs`, `routes/public/stays.rs`, `routes/merchant/stays.rs`, `routes/admin/stays.rs`, `routes/admin/stay_review_handlers.rs`, `stay-service/src/engagement.rs` (subscriptions), migration `00024_guest_messages_reviews.sql`, seed `goma_engagement.sql`, auth migration `00029_stay_review_presets.sql`. Target: `backend/crates/domain/src/reviews`, `backend/migrations/00015_reviews.sql`, `backend/seeds/reviews`. Status: done, dropped by decision D#, open. Rust is read-checked only, never compiled.

Legacy review states are PENDING, APPROVED and HIDDEN (there is no published or rejected state). Wire route paths drop the `/stays` prefix because the audience prefix replaces it.

## Endpoints

| Legacy location | Mungini location | Status |
|---|---|---|
| gateway `POST /reservations/{id}/review` (customer), NATS `stay.review.submit`, `handler::submit_review` | `POST /api/guest/reservations/:reservationId/review`, `submit.rs`, `guest_handlers.rs` | done |
| gateway `POST /reviews/{id}/reply` (merchant), NATS `stay.review.reply`, `handler::reply_review` | `POST /api/host/reviews/:reviewId/reply`, `reply.rs`, `host_handlers.rs` | done |
| gateway `GET /properties/{id}/reviews` (merchant), NATS `stay.review.list`, `read::list_reviews` | `GET /api/host/properties/:propertyId/reviews`, `property_reviews.rs`, `host_handlers.rs` | done |
| gateway `GET /stays/properties/{id}/reviews` (public, no user), same NATS subject | `GET /api/public/properties/:propertyId/reviews`, same service with no actor, so only APPROVED rows and the status filter ignored | done |
| gateway admin `GET /stays/reviews`, NATS `stay.review.admin.list`, `admin_list.rs` | `GET /api/admin/reviews`, `admin_list.rs`, `admin_handlers.rs` | done |
| gateway admin `GET /stays/reviews/moderation-queue`, NATS `stay.review.admin.queue`, `admin_handler::moderation_queue` | `GET /api/admin/reviews/moderation-queue`, `moderation.rs` | done |
| gateway admin `POST /stays/reviews/{id}/moderation`, NATS `stay.review.admin.decide`, `admin_decide.rs` | `POST /api/admin/reviews/:reviewId/moderation`, `moderation.rs` | done |
| NATS `stay.review.moderate`, `handler::moderate_review` (no gateway route) | no route: superseded by `decide_review`, which is the same status change plus reason code and audit | done (merged, unreachable in legacy) |
| NATS `stay.review.admin.moderate`, `admin_handler::moderate_review` (no gateway route) | merged into `decide_review` as above | done (merged, unreachable in legacy) |
| NATS `stay.review.queue`, `read::list_moderation_queue` (no gateway route) | merged into the admin queue, which now also returns unit type cover urls | done (merged, unreachable in legacy) |
| gateway view-only city guard `ensure_writable` on submit and reply | kernel `Actor` extractor rejects writes in view-only city mode | done |
| NATS metadata city and `db::pool_for(city)` | `Actor.city` or `RequestCity`, every query filters `city_id` (D4) | done (D4) |

## Behaviour

| Legacy location | Mungini location | Status |
|---|---|---|
| submit: rating 1 to 5, comment optional trimmed max 2000 chars | `validators.rs` `rating`, `optional_text` | done |
| submit: caller must be the reservation guest, else not found | `submit.rs`, code `REVIEW_RESERVATION_NOT_FOUND` (404) | done |
| submit: reservation must be CHECKED_OUT, else conflict | `submit.rs`, `REVIEW_NOT_CHECKED_OUT` (409) | done |
| submit: one review per reservation (unique violation is conflict) | `queries::insert`, `REVIEW_ALREADY_EXISTS` (409), unique constraint kept | done |
| submit: new review starts PENDING | column default | done |
| submit: outbox `reservation.review_submitted` in the same transaction | `events.rs` through `kernel::outbox::enqueue`; payload enriched with `guestUserId`, `ownerUserId`, `propertyName`, `guestName`, `operatorId` as the legacy `outbox_enrich` did, so the notifications module can address it | done |
| reply: text required, trimmed, max 2000 | `validators::required_text` | done |
| reply: host scope with STAY_MANAGE_REVIEWS on operator and property | `Actor::ensure_host_scope(reviews.write, host, property)`; a non-member host is 404 | done |
| reply: only APPROVED reviews can be answered | `REVIEW_NOT_APPROVED` (409) | done |
| reply: sets reply and replied-at, outbox `reservation.review_replied` | `queries::set_reply`, `events.rs` | done |
| list for a property: status filter validated, paging validated | `validators::optional_review_status`, `PageRequest::validated` | done |
| list for a property: moderators and permitted hosts see every status, everyone else only APPROVED with the filter ignored | `property_reviews::can_see_all` (`reviews.moderate` via `require_scoped(None, None)`, or `reviews.write` through `Actor::allows` on the property's host) | done |
| list ordering newest first, SQL LIMIT and OFFSET, total from COUNT | `list_queries::list_for_property` | done |
| unit type id, names and cover url attached to each row | `sql.rs` subselects, `covers.rs` (cover chosen by `isCover`, sort order, created, id, signed through `properties::media::image_url`) | done |
| admin queue: status default PENDING, oldest first | `moderation::moderation_queue`, `list_queries::list_queue` | done |
| admin decide: status APPROVED or HIDDEN, HIDDEN needs a reason code from ABUSIVE, SPAM, OFF_TOPIC, PRIVACY, OTHER | `validators::moderation_status`, `hide_reason` | done |
| admin decide: row lock, status change, moderator and time stored, audit `review.moderate` with from and to and the reason code | `moderation::decide_review`, `kernel::audit::record` | done |
| admin list: filters status, rating 1 to 5, search over property name, host name, reservation reference, comment with escaped ILIKE; joined property, host, reservation, primary guest name | `admin_list.rs` | done |
| permission: gateway needs the platform admin role, auth NATS `auth.stay.platform_moderator` checks STAY_MODERATE_LISTINGS or the admin role, `list_moderation_queue` uses STAY_MODERATE_REVIEWS | all admin routes call `Actor::require_platform(reviews.moderate)`; platform roles are not a shared enum (D2), identity must grant `reviews.moderate` at platform scope to platform staff | dropped by decision D1 and D2 (single permission, see open question 1) |
| timestamps sent as Postgres `::text` | `DateTime<Utc>` serialised as RFC 3339, clients format dd/mm/yyyy in the city zone | done (adapted) |
| error text strings | typed codes in `codes.rs`: REVIEW_RATING_INVALID, REVIEW_TEXT_REQUIRED, REVIEW_TEXT_TOO_LONG, REVIEW_STATUS_INVALID, REVIEW_MODERATION_STATUS_INVALID, REVIEW_HIDE_REASON_REQUIRED, REVIEW_NOT_FOUND, REVIEW_RESERVATION_NOT_FOUND, REVIEW_NOT_CHECKED_OUT, REVIEW_ALREADY_EXISTS, REVIEW_NOT_APPROVED | done |
| legacy bug: `admin_decide`, `admin_queries` and `admin_handler` return rows missing the unit type columns that `ReviewRow` requires, so sqlx fails with a missing column and the admin decide and queue calls cannot succeed | not ported: every query selects the full column list from `sql.rs` | done (bug fixed, noted per brief) |
| legacy `reservation_of` extra query after a reply | the reply row carries `reservationId`, one query less | done |
| tests: the reviews source has none | `validators.rs` unit tests (rating, text bounds, statuses, hide reason, like escaping) | done |

## Permissions

| Legacy | Mungini | Status |
|---|---|---|
| STAY_MANAGE_REVIEWS (presets OPERATOR_OWNER, PROPERTY_MANAGER, auth migration 00029) | `reviews.write` (`kernel::permissions::names::REVIEWS_WRITE`), preset grants are owned by `identity` | done |
| STAY_MODERATE_REVIEWS (preset PLATFORM_REVIEWER) | `reviews.moderate` (`REVIEWS_MODERATE`) | done |

## Notifications

| Legacy | Mungini | Status |
|---|---|---|
| outbox `reservation.review_submitted` to the host owner | emitted by `events.rs`, consumed by `notifications::reservation_events` (`stay.review_submitted.host`) | done |
| outbox `reservation.review_replied` to the guest | emitted by `events.rs`, consumed by `notifications::reservation_events` (`stay.review_replied.guest`) | done |

## Seeds

| Legacy | Mungini | Status |
|---|---|---|
| `goma_engagement.sql` 3 reviews (approved with reply, pending, hidden) and their checked-out reservations | `seeds/reviews/01-reviews-goma.sql`: approved with reply, approved without reply (French comment), pending, pending with rating only, hidden without comment, hidden spam comment, checked-out awaiting review, confirmed not reviewable | done |
| `kin.sql` has no review | `seeds/reviews/02-reviews-kin.sql`: approved with reply, approved without reply, pending, hidden, checked-out awaiting review | done |
| loading order | appended to `seeds/00-order.txt` | done |

## Schema contract

Migration `00015_reviews.sql`, written by B7. Legacy table and column names are kept, `city_id` is added.

`stay_reviews`: `id text pk`, `city_id text` (GOMA or KIN), `"reservationId" text` unique, foreign key to `stay_reservations` on delete cascade, `"propertyId" text`, `"operatorId" text` foreign key to `hosts` (D2: the legacy operator is the Mungini host), `"guestUserId" text`, `rating smallint` 1 to 5, `comment varchar(2000)`, `status varchar(16)` PENDING, APPROVED or HIDDEN (default PENDING), `"operatorReply" varchar(2000)`, `"operatorRepliedAt"`, `"moderatedBy"`, `"moderatedAt"`, `"createdAt"`, `"updatedAt"`. Indexes: (`city_id`, `"propertyId"`, status) and (`city_id`, status, `"createdAt"`). Legacy had one index on (`"propertyId"`, status). There is no foreign key to properties because that table is not in the migrations yet.

Reads of other modules' tables, with legacy names (reads may join):

- `stay_reservations`: `id`, `city_id`, `"guestUserId"`, `"operatorId"`, `"propertyId"`, `state`, `reference` (migration 00012).
- `stay_reservation_items`: `"reservationId"`, `"unitTypeId"`; `stay_reservation_guests`: `"reservationItemId"`, `role`, `"fullName"` (00012).
- `hosts`: `id`, `display_name` (00002, replaces `stay_operators."displayName"`); `host_members`: `host_id`, `user_id`, `is_owner`, `status`, `created_at` (replaces `stay_operators."ownerUserId"` for the owner notification).
- Not yet migrated, owned by properties and inventory: `stay_properties` (`id`, `city_id`, `"operatorId"`, `"nameEn"`, `"nameFr"`), `stay_unit_types` (`id`, `"nameEn"`, `"nameFr"`, `"templateId"`), `stay_template_media` (`"templateId"`, `status`, `"storageKey"`, `"isCover"`, `"sortOrder"`, `"createdAt"`, `id`). These queries must be re-checked when those migrations land.

Translatable content: legacy stores property and unit type names as `nameEn` and `nameFr` columns, and reviews themselves are single-language user text with no translations. The Mungini rule (per-locale content tables) differs for the property and unit type names; B7 owns neither, so the reads and the wire fields `unitTypeNameEn` and `unitTypeNameFr` follow the legacy columns, isolated in `sql.rs` and `admin_list.rs`, and change when properties and inventory settle their content tables.

SQL functions: none. Template media bytes stored in the database and uploaded lazily by `ensure_url` in legacy are not handled: covers are returned only for rows that already have a `"storageKey"`.

## Public surface for dependents

`reviews::public_routes`, `guest_routes`, `host_routes`, `admin_routes`, `openapi`, `submit_review(kernel, actor, reservation_id, input)`, `reply_review`, `decide_review`, `moderation_queue`, `list_property_reviews(kernel, city, actor, property_id, status, page)`, the error codes, `REVIEW_SUBMITTED`, `REVIEW_REPLIED`, status constants. Reservations never write `stay_reviews`.

## Open questions for the owner

1. Admin routes require the platform-scoped `reviews.moderate` permission, not the legacy platform admin role or STAY_MODERATE_LISTINGS. Identity must grant it to platform staff.
2. Public review page lists only APPROVED reviews and exposes no reviewer name, as in legacy.
3. Review aggregates (average rating) are in `reports` and search ranking (B6, B10), not in this module.

No row is open.
