# Parity ledger P0-2, kernel crate

Source: `tzone-rs/stay-service/src` cross-cutting files, `troq-storage`, `troq-roles`, gateway city, auth and stay permission middleware. Target: `backend/crates/kernel/src`. Status: done, dropped by decision D#, open. Rust is read-checked only, never compiled by the agent.

Defaults followed: D1 own identity (kernel verifies tokens through `AccessDirectory`, identity implements it), D2 own `hosts` entity (`HostId`, scope `HOST`), D4 one database with `city_id`, D5 billing module (billing permissions are plain names).

| Legacy location | Mungini location | Status |
|---|---|---|
| `audit.rs` `AuditEntry`, `record` | `audit.rs` `AuditEntry`, `AuditActor`, `record`; writes `kernel_audit_records` with `city_id`; actor and request id come from `Actor::audit()` | done |
| audit: actor required else Forbidden | `Actor` cannot exist without an authenticated user | done |
| `city.rs` `City` enum, `code`, `FromStr` | `city.rs` `City` over a data table (`GOMA`, `KIN`: code, name, timezone), `from_code`, serde as code | done |
| `city.rs` `ensure_resource_in_route_city`, `CityMismatch` | `city.rs` same function, `Error::city_mismatch` (403, `CITY_MISMATCH`, params route and resource) | done |
| `city.rs` tests (4) | `city.rs` tests (6) | done |
| `config.rs` `Config`, env loading | `config/` reads `APP_ENV`, `API_PORT`, `DATABASE_URL`, `REDIS_URL`, `CITY_CODES`, `JWT_SECRET`, `JWT_REFRESH_SECRET`, `AUTH_OTP_CHANNEL`, `GUEST_RETENTION_DAYS`, `STORAGE_*` exactly as `infra/.env.example`; `bind_addr()` is 0.0.0.0 on `API_PORT` | done |
| `config.rs` `STAY_DATABASE_URL`, `STAY_DATABASE_URL_KIN` | `DATABASE_URL`, one database | dropped by decision D4 |
| `config.rs` `NATS_*`, `STAY_SERVICE_PORT` | replaced by `API_PORT`, no NATS | dropped by decision (architecture: no NATS) |
| `config.rs` `STAY_GUEST_RETENTION_DAYS` | `GUEST_RETENTION_DAYS` (`Config::guest_retention_days`, same positive-integer rule); the retention job is B11 | done |
| `db.rs` pools per city, `pool_for`, `ping`, migrations at init | `db.rs` `Db::connect`, `pool`, `ping`, `migrate`; one pool; `migrate` is exposed and not called automatically | done |
| `db.rs` `pool_for(city)` | none, city is a column filter | dropped by decision D4 |
| `migrations.rs` | `Db::migrate` using `../../migrations` | done |
| `envelope.rs` `reply` success and failure bytes over NATS | `envelope.rs` `Envelope::ok` as HTTP JSON `{success, data, metadata.requestId}`; failures are `Error::into_response` | done |
| `envelope.rs` NATS payload decoding | none | dropped by decision (no NATS) |
| `error.rs` `ServiceError` variants and `code()` | `error/mod.rs` `Error` (BadRequest, Unauthorized, Forbidden, NotFound, Conflict, Internal) with typed `ErrorCode` constants in `error/codes.rs`; status mapped once in `Error::status` | done |
| `error.rs` free-form messages in responses | body is `{success:false, error:{code, params?}}`; clients translate the code. Internal detail is logged and never returned | done (adapted to rules, section 4) |
| `error.rs` `From<sqlx::Error>` | same, to `Internal` | done |
| `files.rs` `clean_file_name` | `files.rs` same rule (1 to 200 chars, no slashes), code `FILE_NAME_INVALID`, tests added | done |
| `content.rs`, `change.rs`, `handler.rs` | byte-identical orphan copies of `template_media/{content,change,handler}.rs`, not declared in `main.rs`; the live code is ported by B2 | dropped (dead duplicates, no kernel behaviour) |
| `membership.rs` `operator_ids_for` (NATS `auth.stay.my_operators`) | `AccessDirectory::access_for` returns `Access.host_ids`; `Actor::host_ids()` | done |
| `operator_scope.rs` `ensure_operator_scope`, `allows` | `Access::ensure_host_scope`, `allows` (non-member is `NOT_FOUND`, then scoped permission) | done |
| `permissions.rs` 18 constants `STAY_*` | `permissions/names.rs` `<domain>.<action>` constants; `LEGACY` maps all 30 legacy keys used across stay-service, gateway and auth (including `STAY_MANAGE_TEAM`, `STAY_VIEW_AUDIT`, `STAY_MANAGE_BUSINESS_SETTINGS`, `STAY_APPROVE_PAYOUTS`) to the new names, `from_legacy` for copy and preset scripts | done |
| `permissions.rs` `StayGrant`, scope types PLATFORM, BUSINESS, PROPERTY | `permissions/grants.rs` `Grant`, `Scope::{Platform, Host, Property}`, `GrantRow` wire shape with `HOST` replacing `BUSINESS` (copy scripts map it); unknown scope fails closed | done (D2) |
| `permissions.rs` `require`, `require_scoped`, `require_scoped_any`, `scoped_access`, `covers` | `permissions/access.rs` `Access` methods of the same names plus `require_platform` (replaces NATS `auth.stay.platform_moderator`) | done |
| `permissions.rs` tests (2) | `grants.rs` and `access.rs` tests (11) | done |
| `permissions.rs` non-`STAY_` keys from `metadata.permissions` | one grant list, no second path | dropped by decision D1 |
| `nats.rs` `MessageMetadata` (city, request id, user id, language, permissions, grants) | `Actor` (user, city, request id, access) built by the extractor | done |
| `nats.rs` request/reply, subscribe, grant injection | none | dropped by decision (no NATS, no gateway) |
| gateway `city.rs` header `x-city`, query `city=` | `auth/request.rs` `requested_city`, `resolve_city` (must be listed in `CITY_CODES`), default Goma else first listed | done |
| gateway `stay_permissions.rs` default city GOMA | `Config::default_city` | done |
| gateway `auth.rs` bearer JWT, expiry, user id claim | `auth/claims.rs` `TokenVerifier` (HS256, `JWT_SECRET`, access type only), `Claims` shape is the contract for the `identity` module | done (D1) |
| gateway `auth.rs` `admin` secret fallback | one secret | dropped by decision D1 |
| gateway `auth.rs` revoked-before check in Redis | `AccessDirectory::revoked_before` and `is_revoked`; storage is identity's choice. A lookup failure now returns the error (legacy failed open) | done |
| gateway `auth.rs` view-only city mode blocking writes | `auth/actor.rs` `city_access_for`, `CityAccess`; the `Actor` extractor rejects POST, PUT, PATCH, DELETE with `CITY_VIEW_ONLY`; `ensure_writable` for explicit checks | done |
| gateway request id layer | `RequestId` extractor reads `x-request-id` or generates one; the api crate adds the header layer | done |
| `outbox.rs` `enqueue` same-transaction writer | `outbox.rs` `enqueue(tx, city, aggregate_type, aggregate_id, event_type, data)` into `kernel_outbox`; same payload shape `{eventId, aggregateType, aggregateId, eventType, data, metadata}` | done |
| `outbox.rs` `enqueue_reservation`, `reservation_snapshot` | reads `reservations`, `hosts`, `properties` tables, so the owning module calls `outbox::enqueue` with its own snapshot (B5) | done (handed to B5 by module rule 1) |
| `outbox_enrich.rs` | same reason, enrichment reads domain tables; owners enrich before `enqueue` (B5, B2, B1) | done (handed to B1, B2, B5) |
| `outbox.rs` `run_publisher`, `publish_batch`, `ensure_event_stream`, `subject_for` | jobs crate publisher over the outbox table (B11); NATS stream dropped | dropped by decision (no NATS) for the stream; publisher is B11 |
| `consumers.rs`, `engagement.rs`, `guest_messages.rs`, `main.rs` | not cross-cutting plumbing: B11 (jobs and inbox), B5 | done (owned by B11, B5) |
| `paging.rs` `PageRequest`, `Page`, `validated`, `limit`, `offset` | `paging.rs` same wire (`items`, `total`, `page`, `pageSize`), typed codes `PAGE_INVALID`, `PAGE_SIZE_INVALID` | done |
| `paging.rs` tests (3) | `paging.rs` tests (3) | done |
| `timezone.rs` `SUPPORTED_TIMEZONES`, `offset_for_timezone` | `timezone.rs` same, plus `to_local`, `local_date`, `format_local` (dd/mm/yyyy HH:mm), `City::timezone` | done |
| `timezone.rs` tests (2) | `timezone.rs` tests (5) | done |
| `properties/queries.rs` idempotency key functions (`claim`, `stored_response`, `store_response`, `resource_for`) | `idempotency.rs` same four, keyed by city, plus `clean_key` (max 128) | done |
| quotes `money.rs` `parse_minor`, `format_minor`, `percent_of` (amounts are integer hundredths in Rust, `numeric(14,2)` text on the wire and in SQL via `::text::numeric`) | `money.rs` `Money` (minor units), `parse`, `parse_input`, `to_wire`, `percent_of`, checked ops, serde as decimal string, `CURRENCY = "CDF"`; no other currency exists | done |
| money tests (4) | `money.rs` tests (6) | done |
| `troq-storage` `Storage` (put, get_bytes, head, delete, presign get and put, retry and timeout) | `storage/client.rs`, `presign.rs` copied, not shared | done |
| `troq-storage` `StorageConfig::from_env` | `storage/config.rs`, same variable names as `infra/.env.example` | done |
| `troq-storage` `Scope`, `key_for`, `validate_key`, `key_in_scopes`, `unique_name`, `sanitize_segment` | `storage/keys.rs`, `Scope::for_city(City)` replaces `Scope::city(&str)` | done |
| `troq-storage` `data_url.rs` | `storage/data_url.rs` | done |
| `troq-storage` `stored.rs` `classify`, `public_url`, `resolve_public` | `storage/stored.rs` (keys, external urls, inline data) | done |
| `troq-storage` `stored.rs` `legacy_to_key`, `stored_path`, `key_from_stored`, `scoped_key`, `is_scope_segment`, `/uploads` legacy paths | Stay stores keys only (`stays/properties`, `stays/templates`); legacy paths are marketplace data | dropped (TROQ marketplace legacy, not used by Stay) |
| `troq-storage` `StorageError` | `storage/error.rs` with conversion to `Error` (`STORAGE_KEY_INVALID`, `STORAGE_OBJECT_NOT_FOUND`, else internal) | done |
| `troq-storage` tests (5) | `keys.rs`, `stored.rs`, `data_url.rs` tests | done |
| `media/storage.rs` Stay upload helper (sha256 key from id, png and jpeg only, private bucket) | property and template image rules belong to B2 `properties`; kernel gives `Storage`, `key_for`, `Scope` | done (handed to B2) |
| `troq-roles` role enum (`stay_manager`, platform roles) | `auth/claims.rs` `Role::{Guest, Host, Staff}`; fine access through permissions | dropped by decision D2 (no shared role enum) |
| `race_tests.rs`, `db_tests.rs` | test inventory and reservation queries, not kernel behaviour; ported with B3 and B5 | done (owned by B3, B5) |
| OTP channel switch `AUTH_OTP_CHANNEL` | `Config::otp_channel` (`Email` default, `Whatsapp`) | done (identity owns the flow and the WhatsApp code) |

## Schema contract for the baseline migration (B12)

Tables the kernel reads and writes. Names use the module prefix `kernel_`, columns are snake case, every row has `city_id` (`GOMA` or `KIN`). Legacy tables had camelCase columns and one database per city, so the copy script maps them.

| Table | Columns | Key |
|---|---|---|
| `kernel_audit_records` | `id text`, `city_id text`, `actor_user_id text`, `actor_scope jsonb`, `action varchar(64)`, `resource_type varchar(32)`, `resource_id text`, `reason text null`, `correlation_id text`, `change_record jsonb default '{}'`, `created_at timestamptz default now()` | `id`; indexes on (`resource_type`, `resource_id`, `created_at desc`) and (`actor_user_id`, `created_at desc`) |
| `kernel_outbox` | `id text`, `city_id text`, `aggregate_type varchar(32)`, `aggregate_id text`, `event_type varchar(64)`, `event_version int default 1 check >= 1`, `payload jsonb`, `state varchar(16) default 'PENDING' check in PENDING, PUBLISHED, FAILED`, `attempts int default 0`, `last_error text null`, `created_at timestamptz default now()`, `published_at timestamptz null` (published iff state is PUBLISHED) | `id`; partial index on `created_at` where state in PENDING, FAILED |
| `kernel_idempotency_keys` | `city_id text`, `scope varchar(64)`, `idempotency_key varchar(128)`, `resource_id text`, `response jsonb null`, `created_at timestamptz default now()` | (`city_id`, `scope`, `idempotency_key`) |
| `kernel_inbox` (jobs, B11) | `consumer_name varchar(64)`, `event_id text`, `event_type varchar(64)`, `processed_at timestamptz default now()` | (`consumer_name`, `event_id`) |

## Contracts other slices implement

- `identity` implements `kernel::auth::AccessDirectory` (`access_for` returns grants, host ids and optional home city; `revoked_before` returns the cutoff timestamp) and issues HS256 access tokens signed with `JWT_SECRET` with `Claims` (`sub`, `role`, `type = "access"`, `iat`, `exp`, optional `authCity`, `cityAccessMode`).
- `api` implements `KernelState` for its state, adds the `x-request-id` layer and calls `Db::migrate` when the owner decides.
- No row is open.
