# B11 jobs parity

Source is `tzone-rs` main at `780396f`. Rust and SQL are read-checked only; nothing was compiled or run, and no database test was executed.

Design: no NATS. Modules write `kernel_outbox` rows in the same transaction as their change. `jobs::run(state, shutdown)` delivers each row in one transaction to the subscribers that accept it (`TransactionalSubscriber`, one `kernel_inbox` row per subscriber and event), then marks it `PUBLISHED`; failures are recorded with exponential backoff in `next_attempt_at`. Scheduled jobs call the owning module's public functions through `jobs/src/tasks/adapters.rs`.

| Legacy feature or state | Legacy location | Mungini location | Status |
|---|---|---|---|
| Transactional event enqueue and payload shape | `stay-service/src/outbox.rs` `enqueue` | `kernel/src/outbox.rs` | done |
| Reservation, property and case snapshot and enrichment | `outbox.rs` `reservation_snapshot`, `outbox_enrich.rs` | event producers (reservations, properties, cases) enrich before `kernel::outbox::enqueue` | open: depends on B5 and B2 emitting full snapshots |
| NATS JetStream publisher, `STAY_EVENTS` stream, `subject_for`, `ensure_event_stream` | `outbox.rs` `run_publisher`, `publish_batch`, `publish_acknowledged` | none | dropped by decision (architecture: no NATS) |
| Pending and failed polling, 1 s idle and 5 s error pause, retry, published state | `outbox.rs` `run_publisher` | `jobs/src/delivery.rs` `deliver_one`, `worker.rs` `run`, `backoff.rs`, migration `00008_outbox_retry.sql` | done (adapted: per-row exponential backoff capped at 1 h instead of immediate retry) |
| Idempotent consumer inbox | `sync/apply.rs` `stay_inbox` | `kernel_inbox` written in the delivery transaction | done |
| Subscriber registry, one line per module | none (new) | `jobs/src/registry.rs`, `subscribers.rs`, `pattern.rs` | done |
| Notification fan-out from events (legacy consumers in messaging-service and troq-notify) | messaging-service Stay files | `jobs/src/notifications.rs` subscriber `notifications` calling `domain::notifications::{accepts, handle_event}` | done |
| Email, push and document delivery of queued notifications | messaging-service workers | `jobs/src/tasks` `notification_delivery` every 5 s, batch 50, `domain::notifications::deliver_pending(kernel, &NotificationsConfig::from_env(), None, 50)` | open: contact directory is `None` until identity implements `ContactDirectory`, so email deliveries are not claimed |
| Billing and search-index subscribers | none (new) | registered by B8 and B6 in `subscribers.rs` | open |
| Events no subscriber accepts (for example `operations.task.*`, `claim.decided`, `property.block_created`) | published to the stream with no consumer | `deliver_one` marks them `PUBLISHED` with no inbox row; failures of an accepting subscriber still roll back and retry | done by lead decision |
| Hold expiry and reconciliation, 30 s sweep, batch 100 | `reservations/expiry.rs` `run`, `sweep` | `jobs/src/tasks` `hold_expiry` calling `domain::reservations::expire_due_holds(kernel, batch)`; implemented in `domain/src/reservations/holds/expiry.rs` | open: `reservations` is not declared in `domain/src/lib.rs` and has no `mod.rs` re-exporting `expire_due_holds` |
| Guest anonymization after `retainUntil`, only when retention days is set | `reservations/retention.rs` `anonymize_expired`, `main.rs` | `jobs/src/tasks` `guest_retention` calling `domain::guests::anonymize_expired_guests(kernel, 100)`, started only when `GUEST_RETENTION_DAYS` is set, warning otherwise | done (logic lives in guests; jobs does not duplicate it) |
| Retention date at write time (`configure`, `retain_until`) | `reservations/retention.rs` | `domain::guests::retain_until` | done (guests) |
| Search-event purge older than 6 months, 1000 rows per run | `search_engine/events.rs` `purge_expired`, called from `expiry.rs` | `jobs/src/tasks` `search_event_purge` calling `domain::guests::purge_search_events(kernel, 1000)` | done |
| Operator status sync consumer, `STAY_SYNC` stream | `sync/consumer.rs`, `sync/apply.rs` | none; hosts own status in process | dropped by decision D2 and no NATS |
| Property and template media blob to storage backfill loops | `media/storage.rs` `backfill`, `template_media/storage.rs`, `main.rs` | none; Mungini stores keys only, copy scripts (C1) upload legacy blobs | dropped (no inline content column in Mungini) |
| CRM lead consumer | TROQ crm-service | none | dropped by decision D8 |
| `consumers.rs`, `engagement.rs` NATS request and reply routers | `consumers.rs`, `engagement.rs` | HTTP routes in `api` per module | dropped (no NATS); routing is each module's API slice |
| Stoppable runtime and startup | `main.rs` spawns | `jobs::run(state, shutdown)` | done: api must call it |
| Tests | `reservations/db_expiry_tests.rs`, `retention.rs` test | `jobs` unit tests (backoff, pattern, registry, schedule, stop) and `delivery_tests.rs` database tests; expiry and retention tests belong to B5 and guests | open: nothing executed |

## Schema contract

No new columns. Uses `kernel_outbox` including `next_attempt_at` (migration 00008) and `kernel_inbox`.

## Open dependencies

- `domain::reservations` must be declared in `domain/src/lib.rs` and export `expire_due_holds(&Kernel, i64) -> kernel::Result<usize>`.
- `ContactDirectory` implementation from identity, then pass it to `deliver_pending` in `tasks/adapters.rs`.

The worker still refuses to start with no registered subscriber, and `run` then leaves events pending: with nothing registered every event would otherwise be marked delivered and lost.
