# B12 migrations and seeds parity

The first migration implements the kernel contract in `P0-2-kernel.md`. It has been read-checked against the kernel queries; it has not been applied to a database. Source schemas are from `tzone-rs` main at `780396f`. Mungini uses a single database and snake case kernel columns, so legacy data needs explicit mapping at cutover.

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Audit records | `stay-service/migrations/00004_operations_events.sql`, `src/audit.rs` | `backend/migrations/00001_kernel.sql`, `kernel/src/audit.rs` | done |
| Outbox events | `stay-service/migrations/00004_operations_events.sql`, `src/outbox.rs` | `backend/migrations/00001_kernel.sql`, `kernel/src/outbox.rs` | done |
| Idempotency keys | `stay-service/migrations/00005_idempotency_keys.sql`, `src/properties/queries.rs` | `backend/migrations/00001_kernel.sql`, `kernel/src/idempotency.rs` | done |
| Event delivery deduplication | `stay-service/migrations/00004_operations_events.sql`, `src/sync/consumer.rs` | `backend/migrations/00001_kernel.sql` | open: transactional dispatcher and subscriber handling are written; worker startup, real consumers and runtime verification remain |
| Remaining Stay schemas and domain migrations | `stay-service/migrations/` | `backend/migrations/` | open |
| Identity and host schemas | `auth-service/migrations/` | `backend/migrations/00002_identity.sql` | open: partial access foundation; lifecycle and complete model remain |
| Goma and Kinshasa demo data and edge cases | `stay-service/seeds/` | `backend/seeds/`, `00-order.txt` | open: multiple modules now have demo seeds; full source coverage and database rehearsal remain |
| Data copy and count verification | existing per-city databases | Phase 3 copy scripts | open |

The seed order was reconciled from the current foreign keys and seed references: hosts and properties precede inventory; inventory and policies precede rates; reservations precede assignments and reviews; billing reservations precede credit entries and settlement items. The paused Kinshasa billing example uses a property owned by the paused host. This was a static review only, not a database rehearsal; existing database rows are not updated by conflict-ignoring seed inserts.

All currently present SQL seed files are listed exactly once in `00-order.txt`; this only checks file coverage and does not prove that their SQL or data dependencies execute successfully.
