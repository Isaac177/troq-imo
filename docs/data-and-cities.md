# Data and cities

Default accepted by the owner: D4, one Postgres database with `city_id` on rows. Legacy was one Postgres database per city (`stay-service` picked the pool from the request city). Sources: `backend/migrations`, `kernel/src/{city,db,config}`, `infra/`, `backend/seeds`, and `stay-service/src/city.rs`, `db.rs`, `seeds/`.

## Cities

- Two cities, `KIN` (Kinshasa, Africa/Kinshasa) and `GOMA` (Goma, Africa/Lubumbashi), in `kernel/src/city.rs`. Legacy had the same two codes.
- `CITY_CODES` selects the cities an instance serves, `KIN,GOMA` by default. The first listed city, or the configured default, is used when a request names none.
- A request names its city with the `x-city` header or a `city` query parameter. A city that is not served is rejected with `CITY_UNSUPPORTED`. A token's `authCity` that differs from the requested city gives view-only access.
- Time zones matter for rules: arrival and free-cancellation deadlines use the property time zone, and no-show and check-in compare against the property's local date.
- Reservation tables carry `CHECK (city_id IN ('GOMA', 'KIN'))`, so adding a city also needs a migration.

## One database

- Every business row carries `city_id`, and every query filters on it. Platform staff reads across cities only with a platform grant.
- Child rows (reservation items, guests, evidence, statements) do not repeat `city_id`; they inherit it through the parent.
- Hosts and properties belong to one city.
- Migrations are one numbered, forward-only sequence in `backend/migrations`. Once a migration has run anywhere it is never edited. The current files are `00001` to `00005`, `00008`, `00009`, `00012` to `00015` and `00017` to `00019`. The numbers `00006`, `00007`, `00010`, `00011` and `00016` are not present, so tables for properties, unit types, inventory and the property location are not in the sequence yet. Other slices own them.
- Table names keep the legacy `stay_` prefix where a table was ported as is (`stay_reservations`, `stay_rate_plans`), and use the module name elsewhere (`identity_users`, `host_members`, `notifications`, `kernel_outbox`).
- Columns of ported tables keep the legacy camelCase spelling in quotes (`"operatorId"`, `"guestUserId"`), while newer tables use snake case. In ported tables `"operatorId"` refers to `hosts.id`.

## Global versus per-city

| Kind | Tables |
|---|---|
| Global, no `city_id` | amenity groups and amenities with their content (`00005`); accommodation categories and templates and notification templates are global in the architecture but not yet in migrations |
| Per city, with `city_id` | audit records, outbox, idempotency keys, hosts, reservations, amendments, messages, cases, tasks, notifications, deliveries, documents, reviews, wishlists, saved searches, search events, rate plans, promotions, quotes, commission rates, credit accounts and entries |
| Per city setting | `stay_platform_settings` is keyed by city and key, so each city has its own commission default, windows and credit thresholds |
| Child rows | inherit the parent's city |
| User | `identity_users.home_city_id` records where the account started; an account is not locked to one city |

In legacy, global tables were replicated into each city database by their owning service. With one database there is nothing to replicate, and Mungini owns its copies.

## Geo

- The dev database image is `postgis/postgis:17-3.5`, and `infra/postgres/init-extensions.sql` runs `CREATE EXTENSION postgis`.
- Legacy stored `latitude` and `longitude` as `numeric(9,6)` on properties (both set or both null, range checked) and did not use PostGIS.
- Mungini search code (`guests/search/candidates.rs`) already reads a `p.location` geography column, uses `ST_DWithin` for radius filters, and `ST_Distance` on a snapped point so the exact property position is not revealed. That column is not in any migration yet.

## Other stores

- Redis 7.2 for caches and rate limits.
- Object storage through the `STORAGE_S3_*` variables: MinIO in development only, Cloudflare R2 in production. Documents, property media and evidence use it. Legacy evidence rows store the file as `bytea` inside Postgres (`stay_case_evidence.content`); the Mungini migration `00013` copies that, which is a decision to confirm.
- No NATS. Events go through the outbox in Postgres.

## Seeds

- Seeds are demo data for development databases only, loaded from `backend/seeds` in the order of `00-order.txt`. Ids start with `seed-` and inserts leave existing rows alone.
- Present: identity and hosts (both cities, active, removed, suspended, expired examples), amenities, pricing (policies, settings, rates for each city, quotes), guests, notifications, reviews.
- Legacy seed files for Goma and Kinshasa remain to port: reservation statuses, cases, damage claims, tasks, operations scenarios, commission, credits, settings, operator detail, template media and calendar contracts. Reservation, case and task seeds are missing, so the reservation screens have no demo data yet.
- Every feature needs seeds covering each status and edge case in both cities (rule 9).

## Cutover copy

At cutover a script copies, per city, legacy Stay data into the Mungini database: stay databases, the `stay_*` tables and grants in `auth-service`, Stay columns in `crm-service` (dropped under D8), and Stay documents in `messaging-service`. Hosts and members are rebuilt from businesses and their grants. Rehearsal on dev databases only; the owner runs production. Details belong in the Phase 3 plan.

## Unverified

- The final shape of property location (PostGIS column or generated from latitude and longitude).
- Whether evidence stays in the database or moves to object storage.
- Whether global catalogs (categories, templates) need a platform scope for editing.
- Backup, retention and per-city data residency requirements.
- Whether a third city is planned.
