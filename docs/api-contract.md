# API contract

The API serves `GET /api/openapi.json` and the audience routes under `/api/public`, `/api/guest`, `/api/host` and `/api/admin`. The current document has 238 paths across guests, reviews, notifications, pricing, billing, inventory, reservations, properties, admin, identity and hosts.

`mungini-api openapi` prints the same document without configuration or a database. `bun run generate` in `packages/api` writes `openapi.json`, `generated.ts` and the typed `createApiClient` export; it refuses a binary older than backend source and never runs cargo.

Identity is wired: `GET /api/guest/session` resolves the caller from the signed access token through `IdentityDirectory`, and every authenticated route fails closed with `AUTH_REQUIRED` or `PERMISSION_MISSING`. Public authentication routes are `otp-channel`, `register/otp`, `register/verify`, `register`, `login` and `refresh`; cities, the permission catalog and host onboarding reference data are public reads.

Every response carries a request ID. The error body is `{ success: false, error: { code, params? } }`; clients translate codes through their locale files. A tracing subscriber reads `RUST_LOG` and defaults to `info`.

CORS origins come from `CORS_ALLOWED_ORIGINS` and an invalid entry prevents startup. Compression and Redis rate limiting are not implemented.
