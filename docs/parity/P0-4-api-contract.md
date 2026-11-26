# P0-4 API contract

| Requirement | Mungini location | Status |
|---|---|---|
| API process and database health endpoint | `backend/crates/api/src/main.rs`, `health.rs` | done: live request verified |
| OpenAPI from handlers and offline output | `backend/crates/api/src/spec.rs`, `health.rs` | done |
| Module routes and specification registry | `backend/crates/api/src/registry.rs` | done: all domain modules, identity and hosts merged; 238 paths generated |
| Generated OpenAPI document and TypeScript client | `packages/api/openapi.json`, `generated.ts`, `index.ts` | done: `bun run generate` against the rebuilt binary |
| Request and response ID headers | `backend/crates/api/src/request_id.rs` | done: middleware wired, headers present on live responses |
| Shared error schemas | `kernel/src/error/mod.rs` | done: one typed error body and one code mapping |
| CORS, compression, tracing and rate limiting | `cors.rs`, `main.rs` | partial: CORS configured, tracing subscriber added; compression and Redis rate limiting remain open |
| Identity directory | `backend/crates/api/src/directory.rs` replaced by `domain::identity::IdentityDirectory` at startup | done: authenticated routes resolve real grants |
| Route verification | live requests on `:8091` | done: public, guest, host and admin routes exercised with and without tokens |

The generated client is committed. Regenerate it after every API change with `bun run generate` in `packages/api`; the script rejects a binary older than backend source. CORS origins come from `CORS_ALLOWED_ORIGINS` and an invalid entry prevents startup.
