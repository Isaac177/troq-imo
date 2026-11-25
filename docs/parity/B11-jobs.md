# B11 jobs parity

Source is `tzone-rs` main at `780396f`. A transactional single-event delivery primitive exists, but no subscribers or worker startup are wired. It is not evidence of working end-to-end delivery. Read-checked only; no Rust build or database execution was run.

| Legacy feature or state | Legacy location | Mungini location | Status |
|---|---|---|---|
| Transactional event enqueue and event payload | `stay-service/src/outbox.rs` | `kernel/src/outbox.rs` | open: domain-specific enrichment is not implemented |
| Reservation, property and case enrichment | `stay-service/src/outbox_enrich.rs`, `outbox.rs` | Domain event producers | open |
| Pending and failed event delivery, retries and published state | `stay-service/src/outbox.rs` | `jobs/src/delivery.rs`, `backoff.rs`, `backend/migrations/00001_kernel.sql` | open: due events record failures and exponential retry times; worker startup and runtime verification missing |
| Idempotent consumer processing | `stay-service/src/sync/consumer.rs` | `jobs/src/delivery.rs`, `backend/migrations/00001_kernel.sql` | open: inbox, subscriber writes and publish state share a transaction; domain subscribers and integration verification missing |
| Reservation hold expiry and reconciliation | `stay-service/src/reservations/expiry.rs` | Reservations module and jobs startup | open: B5 and inventory and billing functions required |
| Guest retention anonymization | `stay-service/src/reservations/retention.rs` | Reservations module and jobs startup | open: B5 required |
| Search-event retention | `stay-service/src/search_engine/events.rs`, `reservations/expiry.rs` | Guests module and jobs startup | open: B6 required |
| Media storage backfills | `stay-service/src/media/storage.rs`, `template_media/storage.rs`, `main.rs` | Properties module and jobs startup | open: B2 required |
| NATS endpoint consumers | `stay-service/src/consumers.rs` | API routes | open: replaced by direct HTTP routes as each domain is ported |
| Outbox and event tests | `stay-service/src/outbox.rs` and dependent domain tests | `jobs/src/delivery_tests.rs` | open: database tests for rollback, concurrency and missing subscribers written but not run; legacy and domain integration tests remain |

If no subscriber accepts an event or a subscriber fails, the delivery primitive rolls back subscriber changes and inbox records, records a failed attempt, and schedules a retry. Subscribers must make their database changes through the supplied transaction; external side effects need a separate durable delivery mechanism. Retries remain scheduled indefinitely with a capped delay; operator handling for persistent failures, worker startup and execution of database concurrency tests remain open.
