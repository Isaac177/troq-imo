# Mungini architecture

Status: target architecture for review, written 25/11/2026. Items marked (D#) depend on an open decision in `MUNGINI_MIGRATION_PLAN.md`; the default is what agents build until the owner answers.

## Principles

1. One repo, one backend binary, many modules. A module is a folder with a clear public surface, not a deployable.
2. Writes are owned, reads are free. Only the owning module writes its tables. Any read model may join across tables in one SQL statement, because lists must filter, sort, paginate and aggregate in SQL (rule 7).
3. Share logic across clients, not UI across platforms.
4. The API contract is generated. Rust routes produce an OpenAPI spec, the TypeScript client and types are generated from it. Nobody hand-writes a shared type.
5. Fewer moving parts than TROQ: no NATS request/reply, no gateway hop, no service-to-service auth calls.

## Repository layout

```
Mungini/
  AGENTS.md
  README.md
  docs/                  Mungini specs
    legacy-troq-stays/   outdated TROQ Stay docs, reference only
    parity/              one ledger per slice proving every legacy feature is implemented or dropped by decision
  backend/
    Cargo.toml           one workspace
    crates/
      kernel/            shared plumbing, no business rules
      domain/            business modules, one folder each
      api/               axum routes, OpenAPI, main binary
      jobs/              background workers
    migrations/          numbered, forward-only
    seeds/               seed-* demo data per city
  apps/
    web/                 guest site, Next.js (D7)
    host/                property operators, Vite, mobile first
    admin/               platform staff, Vite
    mobile/              Expo app, guest and host areas
  packages/
    api/                 generated client and types
    types/               hand-written shared types that are not API shapes
    i18n/                en and fr locales
    state/  storage/  theme/  utils/  hooks/
    ui-web/              DOM components for web, host, admin
    ui-native/           React Native components for mobile
  infra/
    docker-compose.dev.yml  docker-compose.yml  Dockerfile
```

## Backend

### Crates

| Crate | Holds |
|---|---|
| `kernel` | Postgres pool, `City`, ids, `Money` (CDF), pagination (`page`, `pageSize`, total), error type, idempotency keys, audit records, outbox writer, timezone, file storage client (S3 API: MinIO in dev, Cloudflare R2 in production), auth extractors (current user, host membership, permission checks), config |
| `domain` | The modules below. Each has `types`, `queries`, `service`, and a thin `handler` layer, every file at most 300 lines |
| `api` | Router assembly, middleware, OpenAPI generation, `main.rs` |
| `jobs` | Outbox publisher, hold expiry, retention, scheduled work, event consumers |

If `cargo watch` rebuilds get slow, split `domain` into a few crates along module groups. The module boundaries below already allow it.

### Domain modules

Source column names are the folders in `tzone-rs/stay-service/src` unless a service is named.

| Mungini module | Responsibility | Ported from |
|---|---|---|
| `identity` | Users, sessions, OTP login, permission catalog, presets, grants, agreements, ownership transfers (D1) | `auth-service` Stay tables and `scoped_grants`, `permissions.rs`, `membership.rs` |
| `hosts` | Hosts (operators), members, documents, status history, onboarding, host detail (D2) | `operators`, `operator_scope.rs`, `operator_detail`, `auth-service` Stay operator code |
| `properties` | Properties, revisions, media, amenities, accommodation categories and templates, template media, listing lifecycle | `properties`, `media`, `amenities`, `accommodation_categories`, `accommodation_templates`, `template_media`, `listings` |
| `inventory` | Unit types, physical units, pools, nightly inventory, assignments, availability, blocks, restrictions, calendar and feed links, external sync | `availability`, `assignments`, `blocks`, `restrictions`, `calendar`, `sync` |
| `pricing` | Rate plans and rules, promotions and funding, platform settings, quotes | `rate_admin`, `admin_rate_tools`, `quotes`, `platform_settings` |
| `reservations` | Reservations, holds, state machine, amendments, guest messages, cases, operations tasks, damage claims | `reservations`, `amendments`, `guest_messages.rs`, `cases`, `operations`, `admin_claims` |
| `guests` | Wishlists, saved searches, discovery, search engine, guest data retention | `wishlists`, `public_discovery`, `search_engine` |
| `reviews` | Reviews, replies, moderation | `reviews` |
| `billing` | Commission, settlements, credits (D5) | `commission`, `admin_commission`, `credits` |
| `notifications` | Notification copy, documents (voucher, cancellation, damage claim), templates, push | `messaging-service` Stay files, `troq-notify/src/stays.rs` |
| `admin` | Read models for platform staff: dashboard, reservations, guests, claims, calendar, commission, reports | `admin_dashboard`, `admin_reservations`, `admin_guests`, `admin_calendar`, `reports` |

Cross-cutting files in `stay-service` (`audit.rs`, `outbox.rs`, `outbox_enrich.rs`, `paging.rs`, `city.rs`, `timezone.rs`, `error.rs`, `envelope.rs`, `files.rs`, `config.rs`, `db.rs`, `migrations.rs`, `engagement.rs`, `consumers.rs`, `content.rs`, `change.rs`, `handler.rs`) go to `kernel` or `jobs`, not to a domain module.

### Module rules

1. Writes are owned. Another module changes your data by calling your public functions, never by running its own `INSERT` or `UPDATE` on your tables.
2. Reads may join. A read model lives in the module that serves the screen and may join any tables. It must filter, sort and paginate in SQL and take totals from aggregates.
3. A module exposes its public surface from `mod.rs`. Other modules do not import its private files.
4. In-process events. A module writes an outbox row in the same transaction as its change. The `jobs` crate delivers it to the modules that subscribe (notifications, billing, search index). Handlers must be idempotent.
5. Handlers are thin: parse, check permission through the kernel, call the service, return. Business rules live in the service.
6. Every error is a typed variant, mapped to an HTTP status in one place.
7. Tests sit next to the code. Concurrency-sensitive logic (inventory, holds) keeps the race tests that exist in `stay-service/src/race_tests.rs`.

### Data

- One Postgres (D4). Every business row carries `city_id`. The kernel resolves the city from the request and every query filters on it. Platform staff may query across cities with the right permission. Global tables (amenities, accommodation categories, text pages, notification templates) have no `city_id`.
- Table names keep the `stay_` prefix from the source where they are ported as is, or take the module name as prefix. Pick one per module and write it in the module's README.
- PostGIS for geo search and map queries.
- Migrations: one numbered sequence in `backend/migrations/`, forward-only, named by module (`00001_identity.sql`). Start with a baseline that reproduces the final Stay schema, then add normally. Once a migration has run anywhere it is never edited.
- Redis for caches and rate limits. Media in S3-compatible storage through the `STORAGE_S3_*` variables: MinIO in dev only, Cloudflare R2 in production, as in TROQ.
- Seeds: port `stay-service/seeds/*.sql` as `seed-*` rows for both Goma and Kinshasa, so every screen has demo data and every status and edge case is covered.

### Transport and contract

- Plain HTTP with axum 0.7. Routes are grouped by audience: `/api/public`, `/api/guest`, `/api/host`, `/api/admin`.
- OpenAPI generated from the handlers. `packages/api` is generated from it and committed.
- Auth: the `identity` module issues tokens, the kernel extractor verifies them. Keep the `AUTH_OTP_CHANNEL` switch (email until WhatsApp is paid) and the WhatsApp code.
- No NATS in the first version. If an external consumer ever needs events, add a publisher in `jobs` that reads the same outbox.

### What is deliberately dropped

- The `api-gateway` hop and the NATS subject per endpoint.
- The `stay.events.*` JetStream stream (replaced by the in-process outbox).
- The `auth.stay.*` request/reply calls (replaced by function calls).
- One database per city, per D4.
- The CRM consumer that turned Stay reservations into TROQ leads (D8).

## Clients

### Apps

| App | Audience | Toolchain | Source of the port |
|---|---|---|---|
| `apps/web` | Guests: discovery, detail, booking, trips, favorites, profile | Next.js, React 19 (D7) | `troqzone-web` Stay code |
| `apps/host` | Hosts and their staff, designed at 375px first | Vite, React 19 | `troqzone-business` Stay code |
| `apps/admin` | Platform staff | Vite, React 19 | `troqzone-admin` Stay code |
| `apps/mobile` | Guests and hosts | Expo, React Native | `troqzone-app` merchant Stay area and mobile Stay screens |

Keep each app on its current toolchain. Do not move the Vite apps to `Bun.serve` or Next.js.

### Packages

Platform-agnostic, used by every client:

- `api`: generated client and types.
- `i18n`: `en` and `fr` locales, one folder per area. The mobile `shared-i18n` locales for Stay and the Stay locales in the three web apps are merged here, deduplicated.
- `state`, `storage`, `theme` (design tokens), `utils` (dates dd/mm/yyyy, money, validation), `hooks` (data fetching and form hooks that use only stable React APIs).

Per platform:

- `ui-web`: components for web, host and admin.
- `ui-native`: components for mobile. No attempt to share components across DOM and React Native.

### React versions

Every Mungini app runs React 19: Next.js, both Vite apps and the Expo mobile app. The TROQ web apps are on React 18, so ported code is checked for React 19 changes (for example `ref` as a prop, removed legacy APIs, stricter types) as part of each client slice. Use one React version across the workspace, pinned in the root, so hoisting cannot load a second copy. Shared packages declare `react` as a peer dependency and never import `react-dom` or `react-native`. Add monorepo resolver config for Metro. Upgrade deliberately, all apps together.

### Client rules

Everything in `MUNGINI_AGENT_RULES.md` and `MUNGINI_UI_DESIGN.md` applies. In short: components contain only JSX with logic in hooks, one component per file, API calls in services built on the generated client, types in `types/<domain>/`, navigation generated from a module registry filtered by permission, no hardcoded strings, no raw form controls, no typed IDs, transitions on interactive UI, phone input with country selector, dates as dd/mm/yyyy. The theme tokens live in `packages/theme` and the primitives in `packages/ui-web`, built once before screens use them.

## Infra

- `infra/docker-compose.dev.yml`: Postgres with PostGIS, Redis, MinIO (dev only). One backend container.
- Images are built on the server from the Dockerfile. There is no container registry and no image push.
- Deploys are the owner's job. Agents write the files and do not push images or run production compose.

## Boundary with TROQ

After cutover the two products share nothing at runtime: no database, no NATS, no token. Code that both need (OTP flow, payment provider clients, storage client) is copied into Mungini and then evolves separately. If a single TROQ account across both is wanted later (D1), add an OIDC layer on top; do not share a database.
