# P0-4 API contract

Rust is read-checked only. No cargo command has been run.

| Requirement | Mungini location | Status |
|---|---|---|
| API process and database health endpoint | `backend/crates/api/src/main.rs`, `health.rs` | implemented, owner compile and runtime verification open |
| OpenAPI from handler and offline output | `backend/crates/api/src/spec.rs`, `health.rs` | implemented, owner compile and generation verification open |
| Module routes and specification registry | `backend/crates/api` | open; domain modules not yet available |
| Generated OpenAPI document and TypeScript client | `packages/api/scripts/generate.mjs` | open: generation pipeline added; stale binary correctly rejected, current API must compile before generating; old binary used only for temporary printer/type check |
| Request and response ID headers in health OpenAPI | `backend/crates/api/src/health.rs` | implemented; owner compile and generated-spec verification open |
| Request ID response propagation | `backend/crates/api/src/request_id.rs` | implemented; tests written but not run, owner compile and runtime verification open |
| Shared error schemas | `backend/crates/api` | open |
| CORS, compression, tracing and rate limiting | `backend/crates/api/src/cors.rs` | open: configured CORS wired; owner compile and browser verification, compression, tracing and rate limiting remain |
| Identity directory | `backend/crates/api/src/directory.rs` | open; B1 replaces the fail-closed directory |

No application features or seeds were added in this foundation slice.
