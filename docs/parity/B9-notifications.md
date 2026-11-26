# Parity ledger B9, notifications and documents

Source: `tzone-rs` main (`messaging-service` Stay files, `troq-notify/src/stays.rs`, `stay-service` outbox). Target: `backend/crates/domain/src/notifications`. Rust is read-checked only, never compiled or run by the agent. Statuses: done, dropped by decision, open.

Defaults followed: D1 own identity (platform team resolved from `identity_grants` with scope PLATFORM, no role names), D2 hosts (host notifications go to `ownerUserId`, as on main; `hostId` falls back to legacy `operatorId` in event data), D3 CDF only, D4 one database with `city_id`, no NATS (events arrive from the `jobs` outbox delivery, durable side effects go through `notification_deliveries`). Product name comes from `PRODUCT_NAME` (default Mungini).

## Notification subjects and copy (20)

| Legacy location | Mungini location | Status |
|---|---|---|
| `troq-notify/src/stays.rs` `RESERVATION_HELD_HOST` = `stay.reservation_held.host` (HIGH, REMINDER, email default false); `messaging-service/src/stays/notify.rs` on `reservation.held`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `RESERVATION_HELD_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `RESERVATION_ACCEPTED_GUEST` = `stay.reservation_accepted.guest` (HIGH, INFO, email default true); `messaging-service/src/stays/notify.rs` on `reservation.request_accepted`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `RESERVATION_ACCEPTED_GUEST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `customer` becomes `guest`) |
| `troq-notify/src/stays.rs` `RESERVATION_CANCELLED_GUEST` = `stay.reservation_cancelled.guest` (HIGH, ALERT, email default true); `messaging-service/src/stays/notify.rs` on `reservation.cancelled`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `RESERVATION_CANCELLED_GUEST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `customer` becomes `guest`) |
| `troq-notify/src/stays.rs` `RESERVATION_CANCELLED_HOST` = `stay.reservation_cancelled.host` (HIGH, ALERT, email default false); `messaging-service/src/stays/notify.rs` on `reservation.cancelled`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `RESERVATION_CANCELLED_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `DAMAGE_CLAIMED_GUEST` = `stay.damage_claimed.guest` (HIGH, ALERT, email default true); `messaging-service/src/stays/notify.rs` on `reservation.damage_claimed`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `DAMAGE_CLAIMED_GUEST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `customer` becomes `guest`) |
| `troq-notify/src/stays.rs` `AMENDMENT_PROPOSED_GUEST` = `stay.amendment_proposed.guest` (HIGH, REMINDER, email default false); `messaging-service/src/stays/notify.rs` on `reservation.amendment_proposed`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `AMENDMENT_PROPOSED_GUEST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `customer` becomes `guest`) |
| `troq-notify/src/stays.rs` `AMENDMENT_DECLINED_HOST` = `stay.amendment_declined.host` (NORMAL, INFO, email default false); `messaging-service/src/stays/notify.rs` on `reservation.amendment_declined`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `AMENDMENT_DECLINED_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `AMENDMENT_ACCEPTED_HOST` = `stay.amendment_accepted.host` (NORMAL, INFO, email default false); `messaging-service/src/stays/notify.rs` on `reservation.amendment_accepted`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `AMENDMENT_ACCEPTED_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `MESSAGE_GUEST` = `stay.message.guest` (NORMAL, MESSAGE, email default false); `messaging-service/src/stays/notify.rs` on `reservation.message_sent`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `MESSAGE_GUEST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `customer` becomes `guest`) |
| `troq-notify/src/stays.rs` `MESSAGE_HOST` = `stay.message.host` (NORMAL, MESSAGE, email default false); `messaging-service/src/stays/notify.rs` on `reservation.message_sent`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `MESSAGE_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `REVIEW_SUBMITTED_HOST` = `stay.review_submitted.host` (NORMAL, REVIEW_RECEIVED, email default false); `messaging-service/src/stays/notify.rs` on `reservation.review_submitted`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `REVIEW_SUBMITTED_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `REVIEW_REPLIED_GUEST` = `stay.review_replied.guest` (NORMAL, REVIEW_RECEIVED, email default false); `messaging-service/src/stays/notify.rs` on `reservation.review_replied`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `REVIEW_REPLIED_GUEST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `customer` becomes `guest`) |
| `troq-notify/src/stays.rs` `SPECIAL_REQUEST_DECIDED_GUEST` = `stay.special_request_decided.guest` (HIGH, INFO, email default false); `messaging-service/src/stays/notify.rs` on `reservation.special_request_decided`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `SPECIAL_REQUEST_DECIDED_GUEST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `customer` becomes `guest`) |
| `troq-notify/src/stays.rs` `PROPERTY_CREATED_ADMIN` = `stay.property_created.admin` (NORMAL, INFO, email default false); `messaging-service/src/stays/notify.rs` on `property.created`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `PROPERTY_CREATED_ADMIN`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `admin` becomes `admin`) |
| `troq-notify/src/stays.rs` `REVISION_SUBMITTED_ADMIN` = `stay.revision_submitted.admin` (NORMAL, INFO, email default false); `messaging-service/src/stays/notify.rs` on `property.revision.submitted`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `REVISION_SUBMITTED_ADMIN`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `admin` becomes `admin`) |
| `troq-notify/src/stays.rs` `PROPERTY_SUSPENDED_HOST` = `stay.property_suspended.host` (URGENT, ALERT, email default true); `messaging-service/src/stays/notify.rs` on `property.suspended`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `PROPERTY_SUSPENDED_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `PROPERTY_REINSTATED_HOST` = `stay.property_reinstated.host` (HIGH, INFO, email default true); `messaging-service/src/stays/notify.rs` on `property.reinstated`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `PROPERTY_REINSTATED_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `CASE_OPENED_ADMIN` = `stay.case_opened.admin` (HIGH, ALERT, email default false); `messaging-service/src/stays/notify.rs` on `case.opened`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `CASE_OPENED_ADMIN`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `admin` becomes `admin`) |
| `troq-notify/src/stays.rs` `CASE_OPENED_HOST` = `stay.case_opened.host` (HIGH, ALERT, email default false); `messaging-service/src/stays/notify.rs` on `case.opened`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `CASE_OPENED_HOST`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |
| `troq-notify/src/stays.rs` `CASE_CLOSED_PARTIES` = `stay.case_closed.parties` (NORMAL, INFO, email default false); `messaging-service/src/stays/notify.rs` on `case.closed`; copy pair in `notify/copy/stays.rs` | `notifications/catalog.rs` `CASE_CLOSED_PARTIES`, `copy.rs`, `reservation_events.rs` or `property_events.rs` | done (audience `stay` becomes `host`) |

## Documents, templates, fixtures, payloads

| Legacy location | Mungini location | Status |
|---|---|---|
| `documents/stay_voucher.rs` `issue_stay_voucher`, `build_stay_voucher` | `documents/issue.rs`, `build.rs`, `preview.rs` (kind `StayVoucher`) | done |
| `documents/stay_cancellation.rs` | same files, kind `StayCancellation` | done |
| `documents/stay_damage_claim.rs` | same files, kind `StayDamageClaim` | done |
| `documents/payload/stay.rs` `StayVoucherPayload` (state CONFIRMED, CDF only, totalAmount, property name by locale then en, dd/mm/yyyy) | `documents/voucher.rs`, `payload_common.rs` | done |
| `documents/payload/stay_cancellation.rs` (state CANCELLED, chargeDueAtProperty string or number, default 0) | `documents/cancellation.rs` | done |
| `documents/payload/stay_damage_claim.rs` (state CHECKED_OUT, claimAmount, claimReason, disputeDeadline) | `documents/damage_claim.rs` | done |
| `templates/documents/stay_voucher.hbs` | `templates/documents/stay_voucher.hbs` (logo image replaced by `{{brand}}` text) | done |
| `templates/documents/stay_cancellation.hbs` | `templates/documents/stay_cancellation.hbs` | done |
| `templates/documents/stay_damage_claim.hbs` | `templates/documents/stay_damage_claim.hbs` | done |
| `templates/fixtures/stay_{voucher,cancellation,damage_claim}-{en,fr}.json` (6) | `templates/fixtures/` same names, `operatorId` renamed `hostId`, seed ids | done |
| `templates/i18n/{en,fr}.json` sections `document`, `stayVoucher`, `stayCancellation`, `stayDamageClaim` | `templates/i18n/{en,fr}.json`, `{brand}` placeholder replaced from config, no TROQ wording | done |
| `templates/design/*.css`, Onest fonts | `templates/design/`, `assets/Onest/` (logo not carried, no TROQ branding) | done |
| `documents/engine`, `checks.rs`, `pipeline.rs`, `layout.rs`, `template.rs`, `assets.rs`, `i18n.rs`, `format.rs`, `kind.rs` | `documents/` same names, Stay kinds only, anyhow kept inside, kernel `Error` at the edge | done |
| `documents/store.rs` upload to private bucket, record row, `generated` event | `documents/store.rs` through `kernel::storage::Storage`, key `<city>/documents/<kind>/<entity type>/<entity>/<uuid>.pdf` | done |
| `documents/share.rs` presigned share URL sent by WhatsApp | WhatsApp text links to the guest trip page instead | dropped by decision (documents are served as bytes, never presigned URLs) |
| `documents/handler.rs` admin `list`, `download` (presigned URL, `downloaded` event) | `documents/read.rs` `list`, `fetch` returning PDF bytes, `downloaded` event kept | done (bytes instead of URL) |
| admin-only access (`MANAGE_SYSTEM_SETTINGS`) | admin routes need platform `audit.read`; guest sees own documents (`guest_user_id`), host sees documents of hosts with `reservations.read` | done (adapted, guest and host reads added so the app can preview in place) |
| `documents/preview.rs` CLI `document-preview` | `GET /api/admin/documents/previews/{kind}?locale=` returns the fixture PDF as bytes | done (adapted) |
| `documents/consumer.rs`, `merchant_contract`, `invoice`, `credit_note`, `crm_invoice`, `affiliate`, receipts | marketplace documents, not Stay | dropped (not Stay) |
| `stays/mod.rs` `send`: voucher on `reservation.held` and `reservation.request_accepted`, cancellation on `reservation.cancelled`, damage claim on `reservation.damage_claimed` | `handle.rs` `document_for`, `deliver_document.rs` | done |
| `send`: email to `guest.contactEmail` with PDF attached, else WhatsApp to `guest.contactPhone` | `deliver_document.rs` | done |
| Document issued once per entity (unique indexes 00012 to 00014) | `messaging_document` unique partial indexes, `documents/store.rs` `exists` | done |
| Locale from event metadata language, default French | `Locale::parse(snapshot.language)`, default French (B5 snapshots carry no language, so French) | done (ported as is) |

## Channels, inbox, preferences, push

| Legacy location | Mungini location | Status |
|---|---|---|
| `notify/service.rs` `notify` (resolve, drop actor, dedupe 10 minutes, prefs, insert, channels) | `dispatch.rs` `send` | done |
| actor taken from NATS metadata (always empty for outbox events) | `Notice.actor_id` stays empty | done (ported as is) |
| `notify/store.rs` insert, `recent_duplicates`, `unread_count` | `inbox_write.rs`, `inbox_read.rs` | done |
| `notify/prefs.rs` precedence event, category, `*`, URGENT forces in-app and email | `prefs.rs` | done |
| `notify/render.rs` `fill`, `pick` | `render.rs` | done |
| `notify/links.rs` Stay deep links | `links.rs` (guest `/trips`, host `/reservations`, `/properties`, `/cases`, `/tasks`, admin `/audit`) | done (paths follow the legacy shapes without the `/stays` prefix, owner to confirm against app route tables) |
| `notify/recipients.rs` users by role, profiles by NATS call | `recipients.rs` platform team from `identity_grants`; contacts through `ContactDirectory` | open: identity contact lookup is wired, but user contact migration and end-to-end delivery verification remain |
| `notify/email_channel.rs` bilingual email | `channels/email.rs`, `email_html.rs` (provider client copied, HTML built in code, no TROQ template) | done |
| `notify/whatsapp_channel.rs`, `whatsapp.rs` | `channels/whatsapp.rs` (provider endpoint kept, placeholder email domain `@mungini.local`) | done |
| `AUTH_OTP_CHANNEL` switch | kernel `Config::otp_channel`, WhatsApp code kept in `channels/whatsapp.rs` | done |
| `notify/push_channel.rs`, `firebase.rs` | `channels/push.rs`, `firebase.rs` (v1 and legacy key paths, dead token deactivation) | done |
| `notify/realtime.rs` NATS realtime publish | clients read `unread-count` | dropped by decision (no NATS) |
| `notification/queries.rs` tokens, delivery attempts | `channels/push.rs` | done |
| notification center routes (list, read, seen, preferences) in gateway | `/api/{guest,host,admin}/notifications`, `/unread-count`, `/{id}/read`, `/read-all`, `/notification-preferences`, `/push-tokens`, `/documents` | done (paginated in SQL, typed codes) |
| `mark_seen` | read marks seen too | done (adapted) |
| `notification_templates` table, `notification_preferences` table, admin broadcast `admin_send.rs`, test send | not used by Stay flows | dropped (not Stay) |
| in-process delivery instead of NATS consumers `stay.events.>` | `handle.rs` `handle_event`, `accepts`; `deliver.rs` `deliver_pending` | done in this module, open for registration in `jobs` (see below) |

## Events consumed

`reservation.*` (handled: held, request_accepted, cancelled, damage_claimed, amendment_proposed, amendment_declined, amendment_accepted, message_sent, review_submitted, review_replied, special_request_decided; the rest accepted and ignored), `property.*` (created, revision.submitted, suspended, reinstated handled), `case.*` (opened, closed handled). Required data keys: `reservationId`, `guestUserId`, `ownerUserId`, `guestName`, `propertyName` {en, fr}, `amendmentId`, `senderRole`, `body`, `specialRequestState`, `propertyId`, `revisionId`, `caseId`; document snapshot keys as in the fixtures. B5 `reservations/events.rs` already emits these.

## Permissions

| Legacy | Mungini | Status |
|---|---|---|
| `MANAGE_SYSTEM_SETTINGS` platform admin for document list and download | `audit.read` at platform scope | done (adapted) |
| host documents | `reservations.read` at host scope | done |

## Seeds

| Legacy | Mungini | Status |
|---|---|---|
| `seeds/notifications.sql` (marketplace rows) | `backend/seeds/notifications/01-notifications.sql`: every one of the 20 subjects for Goma, a subset for Kinshasa, read and unread, en and fr | done |
| none | `02-preferences-tokens-deliveries.sql`: preferences, active and dead push tokens, delivery attempts, deliveries in every state | done |
| none | document rows | dropped: documents need a stored PDF, they are produced from events and the admin preview route renders every kind and locale from fixtures |

## Tests

Ported or added beside the code: catalog, copy parity, render, links, notice builder, event mapping, preference precedence, delivery retry, document kinds, formats, i18n parity, assets, fixtures to rendered HTML, email and WhatsApp helpers, push helpers, engine `normalize`. Not run.

## Open items

1. Identity owns the `ContactDirectory`. The job uses it for email and WhatsApp fallback; absent contacts defer the email delivery for one hour without consuming its failure-attempt budget. Contacts still need migration and end-to-end verification.
2. `jobs` registration (B11): a `TransactionalSubscriber` named `notifications` whose `accepts` is `domain::notifications::accepts` and whose `handle` calls `handle_event`, plus a periodic task calling `deliver_pending`.

## Schema contract

Migration `backend/migrations/00009_notifications.sql`. Legacy names kept, columns snake case, `city_id` where per city.

| Table | Columns | Notes |
|---|---|---|
| `notifications` | id, city_id, user_id, type, title, title_en, title_fr, body, body_en, body_fr, data, status, priority, icon, image, deep_link, event_key, category, audience, params, entity_type, entity_id, actor_id, dedupe_key, source_event_id, read_at, seen_at, expires_at, created_at, updated_at, deleted_at | unique (user_id, event_key, source_event_id) makes the subscriber idempotent |
| `notification_event_preferences` | user_id, event_key, in_app, push, email, updated_at | key `*`, `category:stays` or an event key |
| `fcm_tokens` | id, user_id, device_id, token (unique), platform, app_version, device_info, is_active, last_used_at, invalidated_at, created_at, updated_at | |
| `notification_delivery_attempts` | id, notification_id, platform, device_token, status, error_message, fcm_response, attempted_at, succeeded_at | |
| `notification_deliveries` | id, city_id, dedupe_key (unique), channel, user_id, notification_id, payload, state, attempts, last_error, next_attempt_at, created_at, done_at | new durable queue for email, push and document work |
| `messaging_document` | id, city_id, kind, locale, entity_type, entity_id, document_number, storage_key, file_name, sha256, page_count, bytes, supersedes_id, guest_user_id, host_id, created_by, created_at | immutable trigger, unique per reservation or case and kind |
| `messaging_document_event` | id, document_id, event_type, actor_id, created_at | immutable |

Reads: `identity_grants`, `identity_users` (platform team). Global templates are code and files, there is no `city_id` template table.

Identity now provides `IdentityContacts` and contact columns in migration 00024. Notification delivery remains disconnected from that directory: preexisting accounts without migrated contact values would otherwise have deliveries permanently skipped. Account migration or a non-destructive missing-contact policy is required before wiring.

Email deliveries with no identity contact remain pending and retry hourly instead of being skipped; this intentionally differs from the legacy best-effort flow to avoid discarding notifications before account migration. Missing users or suspended accounts also stay pending until contact data becomes available or an operator resolves them. The job only runs when `JOBS_ENABLED` is true. Existing `FAILED` or `SKIPPED` deliveries are not automatically replayed.

Document delivery recovery is implemented with migration 00031: an issued document records its originating delivery, allowing retries of that delivery to reuse the stored PDF while other deliveries remain deduplicated. Missing contact details are resolved from identity when possible; otherwise delivery is deferred. Historical documents without a source delivery require reconciliation, and a provider response followed by a process failure before queue commit can still cause a duplicate delivery. Rust and SQL are read-checked only; migration, storage, provider and runtime verification remain open.
