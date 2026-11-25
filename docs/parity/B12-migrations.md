# B12 migrations and seeds parity

The first migration implements the kernel contract in `P0-2-kernel.md`. It has been read-checked against the kernel queries; it has not been applied to a database. Source schemas are from `tzone-rs` main at `780396f`. Mungini uses a single database and snake case kernel columns, so legacy data needs explicit mapping at cutover.

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Audit records | `stay-service/migrations/00004_operations_events.sql`, `src/audit.rs` | `backend/migrations/00001_kernel.sql`, `kernel/src/audit.rs` | done |
| Outbox events | `stay-service/migrations/00004_operations_events.sql`, `src/outbox.rs` | `backend/migrations/00001_kernel.sql`, `kernel/src/outbox.rs` | done |
| Idempotency keys | `stay-service/migrations/00005_idempotency_keys.sql`, `src/properties/queries.rs` | `backend/migrations/00001_kernel.sql`, `kernel/src/idempotency.rs` | done |
| Event delivery deduplication | `stay-service/migrations/00004_operations_events.sql`, `src/sync/consumer.rs` | `backend/migrations/00001_kernel.sql` | open: dispatcher and transactional subscriber handling are not implemented |
| Remaining Stay schemas and domain migrations | `stay-service/migrations/` | `backend/migrations/` | open |
| Identity and host schemas | `auth-service/migrations/` | `backend/migrations/00002_identity.sql` | open: partial access foundation; lifecycle and complete model remain |
| Goma and Kinshasa demo data and edge cases | `stay-service/seeds/` | `backend/seeds/identity/`, `backend/seeds/properties/01-amenities.sql`, `00-order.txt` | open: identity access and translated amenity examples only; Stay feature data remains |
| Data copy and count verification | existing per-city databases | Phase 3 copy scripts | open |
