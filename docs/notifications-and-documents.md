# Notifications and documents

Sources: `backend/crates/domain/src/notifications`, migration `00009_notifications.sql`, `reservations/events.rs`, and the legacy `troq-notify/src/stays.rs` and `messaging-service` Stay files. Mungini owns its copy; nothing is shared with TROQ.

## Pipeline

1. A module writes an outbox event in the same transaction as its change (`kernel_outbox`).
2. The jobs delivery loop hands each event to the notifications handler, once per subscriber (`kernel_inbox` makes it idempotent).
3. The handler turns the event into notices (`reservation_events.rs`, `property_events.rs`) and may enqueue a document.
4. For each notice, recipients are resolved: a single user, or the platform team (active users with a `PLATFORM` grant). The actor is removed from the recipients. A `dedupe_key` drops repeats.
5. A row is written to `notifications` (the in-app inbox, titles and bodies in `en` and `fr`, a deep link by audience), then one row per wanted channel in `notification_deliveries`.
6. A delivery worker claims pending rows and sends them. Failures retry with `next_attempt_at`.

## Channels

| Channel | State |
|---|---|
| In-app | always written for an event the user has not switched off |
| Push | Firebase (`FCM_SERVER_KEY`), device tokens in `fcm_tokens` (`ANDROID`, `IOS`, `WEB`, `DESKTOP`), attempts logged |
| Email | sent through a contact directory the host application supplies, from `EMAIL_FROM`; skipped when no directory is configured |
| WhatsApp | code present, but the delivery loop skips it as unsupported |
| Document | issues a PDF (below) |

Preferences: per user and per event (`notification_event_preferences`: `in_app`, `push`, `email`). Without a stored choice the default is in-app on, push on, and email on only for events marked so in the catalog. A more specific key (event) beats a `category:` key.

## Events

Twenty events, all under category `stays`. Priority is `LOW`, `NORMAL`, `HIGH` or `URGENT`. Email by default is on where marked.

| Event | Audience | Priority | Email |
|---|---|---|---|
| `reservation_held` | host | HIGH | no |
| `reservation_accepted` | guest | HIGH | yes |
| `reservation_cancelled` | guest and host (two notices) | HIGH | guest yes, host no |
| `damage_claimed` | guest | HIGH | yes |
| `amendment_proposed` | guest | HIGH | no |
| `amendment_accepted`, `amendment_declined` | host | NORMAL | no |
| `message` | guest or host, the other side | NORMAL | no |
| `review_submitted` | host | NORMAL | no |
| `review_replied` | guest | NORMAL | no |
| `special_request_decided` | guest | HIGH | no |
| `property_created`, `revision_submitted` | platform team | NORMAL | no |
| `property_suspended` | host | URGENT | yes |
| `property_reinstated` | host | HIGH | yes |
| `case_opened` | platform team, and host | HIGH | no |
| `case_closed` | host | NORMAL | no |

Event keys are prefixed `stay.` and suffixed by audience, for example `stay.reservation_held.host`. Notable gaps in the code: an expired reservation notifies nobody, a declined request has no guest notice (only `request_accepted` does), and check-in, check-out and no-show produce no notice. These match legacy; whether they should stay so is an open question.

## Documents

Three kinds, English and French, immutable once written:

| Kind | Issued on | Unique per |
|---|---|---|
| `stay_voucher` | `reservation.held` and `reservation.request_accepted` | reservation |
| `stay_cancellation` | `reservation.cancelled` | reservation |
| `stay_damage_claim` | `reservation.damage_claimed` | case |

- Rendered from Handlebars templates (`templates/documents/*.hbs`) with shared design files and `en`/`fr` strings, one numbered document each.
- Stored in object storage. The row in `messaging_document` keeps the key, file name, SHA-256, page count, byte size, language, guest and host, `supersedes_id` for a correction, and cannot be updated or deleted (a trigger refuses it). A correction is a new document pointing at the old one.
- Language: `en` when the user locale starts with `en`, otherwise French.
- Guests and hosts list and download their documents through `/documents` routes in their API group.
- The voucher is also issued for a `REQUEST` booking at hold time, before the host accepts. Whether that is intended is not confirmed.

## Dropped from legacy

The NATS subjects, the `STAY_EVENTS` stream and the CRM consumer that created TROQ leads from reservations (D8).

## Unverified

- Delivery worker startup and rendering engine runtime behaviour (read from code only).
- WhatsApp enablement and templates.
- Email provider and the contact directory implementation.
- Legal content of the voucher and cancellation documents (tax, terms).
- Quiet hours, rate limits and digest rules; none found in the code.
