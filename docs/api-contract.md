# API contract

P0-4 is in progress. The API currently serves `GET /api/public/health`, `GET /api/public/amenity-groups`, `GET /api/public/amenities` and `GET /api/openapi.json`. The health route checks the database and returns the kernel success envelope with a request ID, using status 503 when the database is unavailable. The OpenAPI document is derived from the health handler with utoipa and includes its request and response ID headers. `mungini-api openapi` prints the same document without requiring configuration or a database.

A request ID middleware preserves a supplied `x-request-id` or generates one, places it on the request for extractors, and returns it on the response. The health envelope carries that same ID.

The API uses the kernel configuration and connects to its database and storage. Until B1 supplies the identity directory, authenticated access fails closed. The two amenity catalog routes return only active records with `page` and `pageSize`; the amenity route also accepts `groupCode`, and both accept `locale` (default `en`). Admin writes remain unmounted.

The target contract is audience routes under `/api/public`, `/api/guest`, `/api/host`, and `/api/admin`, with a generated TypeScript client in `packages/api`. Each domain module must expose routes and its OpenAPI descriptions through a public surface. The router and specification must be assembled from those modules when they are available. Do not describe domain endpoints until their handlers exist.

Remaining work: generate and commit the TypeScript client and OpenAPI document, assemble module routers and specifications, implement consistent success and error schemas, verify CORS, add compression and tracing, and verify the contract against actual module handlers. The authenticated directory must be replaced by B1. Rate limiting requires a Redis integration. Rust has been read-checked only; the owner runs cargo.

The generator and its output must also respect the repository rule prohibiting code comments. The generator has not been selected or run, and no generated artifacts should be treated as available until the owner can compile the API and the complete pipeline is verified.

Browser origins come from comma-separated `CORS_ALLOWED_ORIGINS`; invalid entries prevent startup. CORS permits the methods and authentication, content, city and request ID headers used by the API. An empty list permits no cross-origin browser access. Owner compile and browser verification remain open.
