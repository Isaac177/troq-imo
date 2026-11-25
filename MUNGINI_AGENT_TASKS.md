# Mungini agent tasks

Status: draft, written 25/11/2026. Slices for parallel agents. Read `MUNGINI_MIGRATION_PLAN.md`, `MUNGINI_ARCHITECTURE.md`, `MUNGINI_STAY_INVENTORY.md`, `MUNGINI_AGENT_RULES.md` and `MUNGINI_UI_DESIGN.md` before claiming anything. Client slices follow the UI design file for every screen and its page checklist.

## Working rules

- Claim a slice by adding a row to the claims table before you start, and update it when you finish. One agent per slice.
- Stay inside the folders of your slice. Integration files are append-only: the domain module registry (`domain/src/lib.rs`), the route registry in `api`, the locale index in `packages/i18n`, and the app route tables. Add your own line and change nothing else in them.
- Never port or delete code you have not read. Read the original on `main` first. The legacy code and docs are reference: adapt each feature to the standalone Mungini model, do not paste it.
- Every backend and client slice writes a parity ledger in `Mungini/docs/parity/<slice>.md`: one row per legacy feature, endpoint, screen, permission, notification, seed and state, with legacy location, Mungini location and status (`done`, `dropped by decision D#`, `open`). Do not report a slice done while a row is `open`.
- Keep behaviour identical to Stay. If you find a bug, port it as is, note it in the claims table, and fix it in a separate task.
- Do not run `cargo`. Rust slices are read-checked only and the report says so.
- Frontend slices end with `bunx tsc` in every workspace touched. After merging output from several agents, the integrator runs the type check again.
- Gates: if your slice depends on an open decision (D1 to D10 in the plan), follow the default, say so in the claim, and keep the dependent code easy to change.
- No comments in code, files at most 300 lines, strings through i18n, seeds with `seed-*` ids.

## Slices

Dependencies are in the last column. Slices with the same dependency can run at the same time.

### Phase 0, foundation (owner or one lead agent, in order)

| ID | Slice | Output | Depends on |
|---|---|---|---|
| P0-1 | Repo bootstrap | `.gitignore`, `AGENTS.md` generated from `MUNGINI_AGENT_RULES.md`, `README.md`, Bun and Cargo workspace skeletons | none |
| P0-2 | `kernel` crate | Pool, `City`, ids, `Money`, paging, errors, idempotency, audit, outbox writer, storage client, auth extractors, config. Ported from the cross-cutting files of `stay-service` and `troq-storage` | P0-1 |
| P0-3 | Infra | Dev compose with PostGIS, Redis, MinIO. Dockerfile. Production builds on the server, no registry | P0-1 |
| P0-4 | API contract pipeline | OpenAPI generation in `api`, generated client and types in `packages/api` | P0-2 |
| P0-5 | Mungini specs | The old Stay docs are already copied, unedited, to `Mungini/docs/legacy-troq-stays/` for reference only. Write fresh Mungini specs in `Mungini/docs/` (vision, host and guest model, payments flow) after D1, D2 and D5 are answered | P0-1 |
| P0-6 | Client shells | Empty `apps/web`, `apps/host`, `apps/admin`, `apps/mobile` that start, plus the shared package folders | P0-1 |

### Phase 1, backend modules

| ID | Slice | Module folder | Source on `main` | Depends on |
|---|---|---|---|---|
| B1 | Identity and hosts | `domain/src/identity`, `domain/src/hosts` | `auth-service` Stay tables, `scoped_grants`, Stay company handlers, `stay-service` `operators`, `operator_detail`, `operator_scope.rs`, `membership.rs`, `permissions.rs`, `troq-roles` | P0-2, D1, D2 |
| B2 | Properties and media | `properties` | `properties`, `media`, `amenities`, `accommodation_categories`, `accommodation_templates`, `template_media`, `listings` | P0-2, B1 skeleton |
| B3 | Inventory and calendar | `inventory` | `availability`, `assignments`, `blocks`, `restrictions`, `calendar`, `sync`, migrations 00002, 00012 to 00015, 00033 to 00035 | P0-2, B2 skeleton |
| B4 | Pricing and quotes | `pricing` | `rate_admin`, `admin_rate_tools`, `quotes`, `platform_settings`, migrations 00016, 00017, 00032, 00042, 00050 | P0-2, B3 skeleton |
| B5 | Reservations | `reservations` | `reservations` (33 files), `amendments`, `guest_messages.rs`, `cases`, `operations`, `admin_claims`, `engagement.rs` | P0-2, B3, B4 skeletons |
| B6 | Guest discovery | `guests` | `public_discovery`, `search_engine`, `wishlists` | P0-2, B2 |
| B7 | Reviews | `reviews` | `reviews` | P0-2, B5 skeleton |
| B8 | Billing | `billing` | `commission`, `admin_commission`, `credits`, migrations 00047 to 00049, 00051, 00052 | P0-2, D5, B5 skeleton |
| B9 | Notifications and documents | `notifications` | `messaging-service` Stay files, `troq-notify/src/stays.rs` | P0-2, B5 |
| B10 | Admin read models | `admin` | `admin_dashboard`, `admin_reservations`, `admin_guests`, `admin_calendar`, `reports` | B2 to B8 |
| B11 | Jobs | `jobs` crate | `outbox.rs`, `outbox_enrich.rs`, `consumers.rs`, hold expiry, guest retention | P0-2 |
| B12 | Seeds and baseline migration | `backend/migrations`, `backend/seeds` | `stay-service/migrations`, `stay-service/seeds` | runs continuously, one owner |

A "skeleton" means the module's types and public function signatures exist, so dependents can compile against them without waiting for the full port.

Every backend slice also ports the tests that sit with its source (including `race_tests.rs` and `db_tests.rs` for the slices that own them).

### Phase 2, clients

| ID | Slice | Target | Source on `main` | Depends on |
|---|---|---|---|---|
| F0 | UI foundation | `packages/theme`, `packages/ui-web`, app shells and module registries | Primitives and shell from `MUNGINI_UI_DESIGN.md` sections 2 to 4, read from the live TROQ stays pages, `troqzone-business` and `troqzone-admin` shells, and `troqzone-app/packages/shared-ui` for the native counterparts | P0-6, D11 |
| F1 | Shared packages | `packages/*` | `troqzone-app/packages/shared-*`, shared hooks and utils in the three web apps | P0-4, P0-6 |
| F2 | Guest web | `apps/web` | `troqzone-web` Stay folders (63 files) | F1, B2, B3, B4, B5, B6 |
| F3 | Host app | `apps/host` | `troqzone-business` Stay folders and `src/types/stay*.ts` | F1, B1 to B5, B8 |
| F4 | Admin | `apps/admin` | `troqzone-admin` Stay folders and `src/types/stays-*.ts` | F1, B10 |
| F5 | Mobile guest | `apps/mobile` guest area | `troqzone-app/apps/mobile` Stay screens and components | F1, B2 to B6 |
| F6 | Mobile host | `apps/mobile` host area | `troqzone-app/packages/merchant/src/stays` | F5 shell, B1 to B5, B8 |
| F7 | Locales | `packages/i18n` | Stay locale files in the four places listed in the inventory | F1, runs alongside F2 to F6 |

Slices F2 to F6 also depend on F0: no screen is built before the primitives it uses exist.

All Mungini apps use React 19. Code ported from the React 18 TROQ web apps must be checked for React 19 differences, and `bunx tsc` must pass with the React 19 types.

Client slices must check phone layouts at 375px in English and French (rule 22 for host) and name the screens checked.

### Phase 3, data and cutover

| ID | Slice | Output | Depends on |
|---|---|---|---|
| C1 | Copy scripts | Scripts that copy `stay-db` (Goma and Kinshasa), auth Stay tables, CRM Stay columns and messaging Stay templates into the Mungini schema. Dev databases only | B1 to B9 |
| C2 | Rehearsal | Dry run on dev databases with row counts compared per table | C1 |
| C3 | Cutover notes | Owner runbook: order of steps, rollback, redirects for `/[lang]/stays` | C2 |

### Phase 4, removal from TROQ (branch `remove-stay`)

Each removal slice may start once the matching port slice is done and its parity ledger is closed.

| ID | Slice | Repo | What | Depends on |
|---|---|---|---|---|
| R1a | stay-service and infra | `tzone-rs` | Delete `stay-service`, workspace member, compose, bake and Makefile entries, `.env.example` | B1 to B12 done |
| R1b | Gateway | `tzone-rs` | Remove Stay routes, middleware and mounts in `api-gateway` | R1a |
| R1c | Auth and roles | `tzone-rs` | Remove Stay code and tables from `auth-service` with a new migration, `stay_manager` from `troq-roles` and `troq-notify` (D2) | R1b |
| R1d | Messaging and CRM | `tzone-rs` | Remove Stay documents and templates in `messaging-service` and the Stay consumer and columns in `crm-service`, each with a new migration | B9 |
| R2a | Web | `troq-front` | `troqzone-web` Stay folders, promos, nav, locales, role entries | F2 |
| R2b | Business | `troq-front` | `troqzone-business` Stay folders, nav, routes, CRM team presets, locales | F3 |
| R2c | Admin | `troq-front` | `troqzone-admin` Stay folders, nav, routes, locales | F4 |
| R2d | Investor | `troq-front` | Stay entries in roles and notifications | R1c |
| R3 | Mobile | `troqzone-app` | `packages/merchant/src/stays`, mobile Stay screens and components, locales, navigation, roles | F5, F6 |

Removal rules:

- Parity gate: a removal slice starts only when the parity ledger of its matching port slice has no `open` row, and the owner has reviewed it. Say in the claim which ledger you checked.

- Edit only the Stay parts of shared files. Locale files that mix Stay keys with other keys keep the other keys.
- Schema removal is a new forward-only numbered migration in the owning service. Never edit a migration that has already run. Never run ad-hoc SQL against shared databases.
- After each slice run the type check in the workspace and the grep from the inventory, and list what is left and why.
- Report the services and apps that changed and the manual steps for the owner (rule 19).

## Claims

Add a row before you start. Mark it done with what was checked and what was not.

| Task | Claimed by | Status | Files |
|---|---|---|---|
| F0 UI foundation | F0 agent | done except shell rows (open in ledger, owner P0-6 and lead). D11 default followed (TROQ stays look). `bunx tsc --noEmit` clean in `packages/theme` and `packages/ui-web`. Nothing rendered, no 375px check. | `packages/theme`, `packages/ui-web`, `docs/parity/F0-ui-foundation.md` |
| P0-2 | kernel agent | done: public surface and bodies written, ledger closed (D1, D2, D5 defaults, kept behind `AccessDirectory` and `Scope::Host`). Read-checked only, no cargo run. Ported tests not run. No migrations or seeds added, schema contract is in the ledger for B12 | backend/crates/kernel/**, docs/parity/P0-2-kernel.md |
| P0-6 Client shells | agent P0-6 | done (tsc clean in all 10 workspaces, no builds, nothing rendered or run) | apps/*, packages/*/package.json+tsconfig+index.ts, packages/i18n, root package.json, docs/parity/P0-6-shells.md |
| P0-3 Infra | infra agent | done, open items in ledger (prod Postgres, Redis, MinIO placement; read-checked only, nothing run) | infra/**, docs/parity/P0-3-infra.md |
| P0-4 API contract | audit_plan agent | partial: health and handler-derived OpenAPI written, Rust read-checked only; generation, registry, middleware and owner verification open in ledger | backend/crates/api/**, packages/api/**, docs/api-contract.md, docs/parity/P0-4-api-contract.md |
| B4 Pricing and quotes | agent B4 | not started: previous claim stale | backend/crates/domain/src/pricing/**, backend/seeds/pricing/**, docs/parity/B4-pricing.md |
| B2 Properties and media | contract agent | partial: media validation and storage primitives; parity ledger open, read-checked only | backend/crates/domain/src/properties/**, backend/seeds/properties/**, docs/parity/B2-properties.md |
| B12 Migrations and seeds | backend audit agent | partial: kernel baseline migration and parity ledger; domain migrations and seeds open, read-checked only, no database applied | backend/migrations/**, backend/seeds/README.md, backend/seeds/00-order.txt, docs/parity/B12-migrations.md |
| B5 Reservations | agent B5 | not started: previous claim stale | backend/crates/domain/src/reservations/**, backend/seeds/reservations/**, docs/parity/B5-reservations.md |
| B11 Jobs | backend audit agent | partial: parity ledger added; dispatcher and domain jobs remain open, read-checked only | backend/crates/jobs/**, docs/parity/B11-jobs.md |
| B6 Guest discovery | agent B6 | not started: previous claim stale | backend/crates/domain/src/guests/**, backend/seeds/guests/**, docs/parity/B6-guests.md |
| F0 shell theme toggle | frontend agent | implemented; D11 default, host and admin theme controls; tsc clean, rendering and 375px checks pending | apps/host/**, apps/admin/**, packages/i18n/locales/{en,fr}/common.ts, packages/ui-web/src/theme/** |
| F1 shared money formatter | lead | partial: CDF formatter shared; F1 remains open. Type checks passed in utils and ui-web. | packages/utils/**, packages/ui-web/src/format/**, packages/ui-web/package.json |
| B1 identity and hosts | identity agent | partial: unwired access directory, baseline identity and host tables, paginated membership listing, scoped host contact updates, override resolution and host-scoped management, and seed states. D1 and D2 defaults; Rust and SQL read-checked only. See open ledger. | backend/crates/domain/src/identity/**, backend/crates/domain/src/hosts/**, backend/migrations/00002_identity.sql, backend/migrations/00003_host_contact.sql, backend/migrations/00004_identity_overrides.sql, backend/seeds/identity/**, docs/parity/B1-identity-hosts.md |
| F0 shell navigation | shell agent | partial: shared top bar and breadcrumb primitive; actual navigation and integration await routes, tsc passed; 375px unchecked. D11 default. | apps/host/**, apps/admin/**, packages/ui-web/src/shell/**, packages/i18n/locales/**, docs/parity/F0-ui-foundation.md |
| F1 shared date formatting | lead | partial: date utilities shared; ui-web and utils type checks passed, F1 remains open | packages/utils/src/date/**, packages/ui-web/src/format/**, packages/ui-web/src/date/** |
| P0-4 request correlation | contract agent | implemented, tests written but not run; Rust read-checked only | backend/crates/api/**, docs/api-contract.md, docs/parity/P0-4-api-contract.md |
| F0 phone detection fix | shell agent | implemented: async detection survives fallback updates, preserves user changes and handles errors; ui-web type check passed, runtime not checked | packages/ui-web/src/phone/usePhoneField.ts, docs/parity/F0-ui-foundation.md |
| B12 seed inventory | lead | partial: documented identity seed order and remaining coverage; no database changes | backend/seeds/README.md, backend/seeds/00-order.txt, docs/parity/B12-migrations.md |
| B11 transactional event delivery | lead | partial: transaction and inbox delivery primitive; no subscribers, retries, runtime verification or worker startup; Rust read-checked only | backend/crates/jobs/src/**, docs/parity/B11-jobs.md |
| F0 shared dialing codes | shell agent | implemented: retain selected country for shared dialing codes while mounted; ui-web tsc passed, runtime unchecked | packages/ui-web/src/phone/usePhoneField.ts, docs/parity/F0-ui-foundation.md |
| B2 property media foundation | contract agent | partial: image validation and private storage plus global amenity paginated reads, schema and demo seed; tests written, Rust read-checked only; B2 ledger open | backend/crates/domain/src/properties/**, backend/migrations/00005_property_amenities.sql, backend/seeds/properties/**, docs/parity/B2-properties.md |
| B11 delivery verification | lead | partial: rollback, concurrency and missing-subscriber database tests written and read-checked only; not run | backend/crates/jobs/src/delivery_tests.rs, backend/crates/jobs/src/lib.rs, docs/parity/B11-jobs.md |
| F0 controlled calendar | shell agent | implemented: sync externally selected date and move cell state to hook; ui-web tsc passed, runtime unchecked | packages/ui-web/src/date/Calendar.tsx, packages/ui-web/src/date/useCalendar.ts, docs/parity/F0-ui-foundation.md |
| B11 event retries | lead | partial: rollback-safe failure recording and scheduled retries, tests updated but not run, no worker startup; Rust and SQL read-checked only | backend/crates/jobs/src/**, backend/migrations/00008_outbox_retry.sql, docs/parity/B11-jobs.md |
| F0 grid preferences | shell agent | implemented: stable equivalent column arrays, validate stored widths and protect non-hideable columns; ui-web tsc passed, runtime unchecked | packages/ui-web/src/table/useGridPrefs.ts, docs/parity/F0-ui-foundation.md |
| B2 amenity writes | contract agent | partial: active catalog and transactional platform-scoped create/update; read-checked only, no cargo or database execution; active-language validation and API routes remain open | backend/crates/domain/src/properties/amenities/**, docs/parity/B2-properties.md |
| F0 grid width defaults | shell agent | implemented: saved widths override defaults while new columns keep default widths; ui-web tsc passed, runtime unchecked | packages/ui-web/src/table/useGridPrefs.ts, docs/parity/F0-ui-foundation.md |
