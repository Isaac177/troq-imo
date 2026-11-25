# Mungini migration plan

Status: draft for owner review, written 25/11/2026. Read this first, then `MUNGINI_ARCHITECTURE.md`, `MUNGINI_STAY_INVENTORY.md`, `MUNGINI_AGENT_TASKS.md`, `MUNGINI_AGENT_RULES.md` and `MUNGINI_UI_DESIGN.md`. The rules and UI design files apply to every agent working on Mungini. They combine the TROQ `AGENTS.md` with the rules and design system of `/Users/jogues/Documents/troqbound`. The root TROQ `AGENTS.md` still applies to work inside the TROQ repos (the removal slices).

## What we are doing

Stay (accommodation booking) leaves TROQ and becomes Mungini, a standalone product. Mungini lives in one repository at `/Users/jogues/Documents/TROQ/Mungini` and is built as a monorepo:

- one Rust backend, a modular monolith (one binary, one Cargo workspace, domain modules instead of services)
- every client in the same repo: guest web, host app, admin, mobile
- shared packages between the clients

TROQ keeps the marketplace only. Mungini is expected to grow larger than the marketplace and to be multi-city from the start.

## Why

Stay today is a separate microservice (`stay-service`) wired into the marketplace through NATS, `api-gateway`, `auth-service`, `messaging-service` and `crm-service`, and it is spread across three web apps and the mobile app. One Stay change can touch five Rust services and four JavaScript workspaces, and sharing code or queries between services forced copies. The owner wants one repo with backends as modules and frontends beside them.

## Current state

Updated 25/11/2026 after inspecting the working tree. Mungini is a Git repository; its foundation and current migration work are largely untracked. Preserve those files. The claims and parity ledgers record the current state of individual slices.

- The kernel, development infrastructure, client skeletons and web UI primitives exist. API contract generation, jobs and the baseline migration are partial.
- Stay domain modules, feature screens, demo seeds and the data-copy rehearsal remain open. A shell or partial ledger does not establish feature parity.
- TROQ removal is gated by completed parity ledgers and owner review. Do not delete legacy code before that gate.

## Decisions made

| Decision | Source |
|---|---|
| Project name is Mungini, folder `Mungini` at the TROQ root | owner |
| One big repo for Mungini: backend modules plus all frontends | owner |
| Modular monolith backend, not microservices | owner |
| Backend stays in Rust | owner, after discussion |
| React Native mobile app lives in the same monorepo as the web clients, sharing logic packages, not UI components | owner, after discussion |
| TROQ stays three repos (`tzone-rs`, `troq-front`, `troqzone-app`) and is not merged into one | owner |

## Open decisions (gates)

Work that depends on an open decision must wait or follow the default and say so in the report. Rule 16 of `AGENTS.md` applies: confirm the shape of anything with business identity before writing it.

| ID | Question | Default if the owner does not answer |
|---|---|---|
| D1 | One TROQ login and wallet across both products, or separate accounts? | Separate. Mungini has its own identity module. Users who used Stay are copied at cutover. No shared database. |
| D2 | Host identity. Today Stay hosts are TROQ businesses (`company_type` stay, `stay_operators_to_businesses` migration, `stay_manager` role). What is a host in Mungini? | Mungini owns a `hosts` entity. Existing stay operators and their members are copied across at cutover. |
| D3 | Currency. | CDF only, as in TROQ rule 6. |
| D4 | One Postgres with `city_id` on rows, or one database per city as in TROQ? | One database with `city_id`, PostGIS for geo search. |
| D5 | Payments. `payment-service` has no Stay references, so Stay money today is commission, credits and settlements inside `stay-service` plus payment at the property. Where does guest money flow in Mungini? | Mungini has a `billing` module that ports commission, credits and settlements. Provider integrations are copied from `payment-service`, never shared. Legacy notes in `Mungini/docs/legacy-troq-stays/stays/09-payment-at-property-design.md` and `10-payments-settlements.md` may help, but check them against the code. |
| D6 | Port with git history or clean copy? | Clean copy from `main` at the SHAs above. The Mungini repo starts fresh. |
| D7 | Guest web framework. | Next.js (SEO matters for listing pages). Host and admin on Vite. All apps on React 19 (decided). |
| D8 | Hosts used the TROQ business app for CRM, team, settings and notifications (legacy `21-shared-business-capabilities.md`). What do hosts get in Mungini? | Team, settings and notifications are ported. CRM is out of scope for the first release. Owner to confirm. |
| D9 | Mobile distribution. Stay screens live inside the TROQ mobile app today. | Mungini mobile is a separate Expo app with its own bundle id. TROQ mobile drops Stay in Phase 3. |
| D10 | What does the TROQ marketplace show after removal where it promotes Stay (home promo section, nav entries, drawer links)? | Remove them. A link to Mungini can be added later. |
| D11 | Visual language. TROQ's `DESIGN.md` (green, Onest, status dots, curved panels; the stays pages are its reference) and Troqbound's `DESIGN.md` (lime, Plus Jakarta Sans, tone pills) differ in look and agree on behaviour. Which look does Mungini use, and does it get its own brand colour and typeface? | Keep the TROQ stays language for the ported screens so parity is easy to check, expressed as semantic tokens in one theme file so a Mungini brand change is a one file edit. Comparison in `MUNGINI_UI_DESIGN.md` section 2. |
| D12 | Locale file format. TROQ uses JSON files in `shared-i18n`. Troqbound uses typed per-feature TypeScript files where `fr` must satisfy the type of `en`, so a missing key is a compile error. | Typed per-feature files in `packages/i18n`, English and French. JSON locales are converted during the port. |

## Source of truth for extraction

- Read the original Stay code from `main`, at the SHAs above. On a `remove-stay` branch use `git show main:<path>` or `git diff main -- <path>`.
- Stay feature work on `main` is frozen during the migration. If a Stay fix lands on `main` anyway, add a line to the claims table in `MUNGINI_AGENT_TASKS.md` so the matching Mungini module gets it.
- `MUNGINI_STAY_INVENTORY.md` lists every file and table to port or remove. Regenerate it with the commands at its end before trusting it after a long gap.

## Legacy Stay as reference, and the parity gate

The legacy Stay code and docs are the reference for what Mungini must do. They are not the design for how. Every legacy feature is adapted to the standalone model (own identity, own hosts, own billing, no gateway, no NATS), not copied as is.

- Reference means: read the legacy code and docs to learn the feature, its states, rules and edge cases, then implement it correctly in the Mungini architecture.
- Nothing is dropped silently. A legacy feature that Mungini will not carry over (for example the CRM link, D8) is recorded as an explicit decision with the owner's yes.
- Parity ledger. Each port slice writes `Mungini/docs/parity/<slice>.md`: one row per legacy feature, endpoint, screen, permission, notification, seed and state, with its legacy location, its Mungini location, and a status (`done`, `dropped by decision D#`, or `open`).
- Removal gate. Phase 4 deletes legacy code from TROQ only for rows that are `done` or `dropped by decision`. A slice with any `open` row cannot be removed. Before the `remove-stay` branches merge, every ledger is closed and the owner has reviewed it.
- The inventory in `MUNGINI_STAY_INVENTORY.md` is the checklist of what exists. The ledgers prove that each item was implemented or consciously dropped.

## Phases

### Phase 0: Mungini foundation

Exit: the repo builds an empty app end to end.

- `.gitignore`, root `AGENTS.md` for Mungini (see "Rules carried over"), `README.md`.
- Cargo workspace skeleton with the `kernel`, `domain`, `api` and `jobs` crates (see architecture).
- Bun workspace skeleton with `apps/*` and `packages/*`.
- `infra/` with dev compose (Postgres with PostGIS, Redis, MinIO for dev only; production storage is Cloudflare R2) and the Dockerfile. Production images are built on the server, with no registry.
- OpenAPI to TypeScript generation wired from the API crate into `packages/api`.
- The old Stay docs are copied, unedited, to `Mungini/docs/legacy-troq-stays/`. They were written to embed Stay inside TROQ (shared role enum, TROQ businesses as hosts, business app UI, NATS and gateway), so they are reference only, not the Mungini spec. Write fresh Mungini specs in `Mungini/docs/` once D1, D2 and D5 are answered, reusing only what is still valid.

### Phase 1: backend port

Exit: every Stay endpoint exists in Mungini with the same behaviour, seeds load, no NATS request/reply.

Port module by module (slices B1 to B11 in the tasks file). The method for each module:

1. Copy the module folder from `stay-service/src/<module>` into `domain/src/<module>`.
2. Replace each NATS handler plus its gateway route with one axum route that calls the module directly. The gateway route files under `api-gateway/src/routes/stays/` show the HTTP shape; the NATS handler shows the logic.
3. Replace `metadata.city` and `metadata.permissions` forwarding with the kernel extractors.
4. Replace `auth.stay.*` NATS calls to `auth-service` with direct calls into the `identity` and `hosts` modules.
5. Add migrations to the Mungini sequence and port the seeds (`stay-service/seeds/`).
6. Keep behaviour identical. Do not redesign business rules. The code on `main` is the source of truth for behaviour. The legacy docs in `Mungini/docs/legacy-troq-stays/` are outdated for architecture and may help to explain a rule, but never override the code.

### Phase 2: clients

Exit: every Stay screen exists in the matching Mungini app, talking to the new API through the generated client.

- `apps/web` from `troqzone-web` Stay code.
- `apps/host` from `troqzone-business` Stay code.
- `apps/admin` from `troqzone-admin` Stay code.
- `apps/mobile` from `troqzone-app` (`packages/merchant/src/stays` and `apps/mobile` Stay screens).
- Shared packages extracted from the places listed in the architecture file.

A client slice can start as soon as the backend slice that serves it has its routes in the OpenAPI spec.

### Phase 3: data and cutover

Exit: Mungini serves production traffic, TROQ no longer serves Stay.

- Copy scripts for the existing Stay data: `stay-db` for each city, `stay_*` tables in the `auth-service` database, Stay columns in `crm-service`, Stay documents and templates in `messaging-service`.
- Rehearse on dev databases only. The owner runs production.
- Owner deploys Mungini, then merges the `remove-stay` branches and deploys TROQ.

### Phase 4: removal from TROQ

Prepared on the `remove-stay` branches in parallel with phases 1 and 2, but only after the matching Mungini slice has its parity ledger closed (see the parity gate above). Never delete code that has not been implemented in Mungini or dropped by an owner decision.

- R1 `tzone-rs`: delete `stay-service` and every Stay module, route, subject, template and role listed in the inventory. Schema removal is a new forward-only numbered migration in the owning service. Never edit an applied migration.
- R2 `troq-front`: delete the Stay components, hooks, utils, routes, types, locales and nav entries in `troqzone-web`, `troqzone-business`, `troqzone-admin` and `troqzone-investor`.
- R3 `troqzone-app`: delete `packages/merchant/src/stays`, the mobile Stay screens and components, the Stay locales and the Stay entries in shared types and navigation.
- After each, run the frontend type check and grep for leftovers (commands in the inventory).
- Update the shared contracts in every place: the `stay_manager` role lives in `troq-roles/roles.json` and in the `roles.ts` files of all clients.

## How an agent works on this

1. Read `AGENTS.md`, this plan, the architecture and the inventory.
2. Claim a task in `MUNGINI_AGENT_TASKS.md` before starting. One agent per slice.
3. Stay inside the slice's folders. Integration files (module registry, route registry, locale index) are append-only: add your own line and touch nothing else.
4. Read the original code on `main`, port it, adapt the edges.
5. Verify, then mark the claim with what was and was not checked.

Verification:

- Frontend: `bunx tsc` in each workspace you touched (for example `bunx tsc -p tsconfig.app.json --noEmit`). No builds.
- Rust: do not run `cargo`. The owner's `cargo watch` compiles. Read the code carefully and say "read-checked only" in the report. This carries over to Mungini until the owner says otherwise.
- After merging parallel agent output, run the type check yourself in every workspace (rule 10).
- Phone screens: check at 375px in English and French and name the screens checked (rule 22, applies to the host app).

## Rules carried over to Mungini

The full rules are in two files, and Phase 0 turns `MUNGINI_AGENT_RULES.md` into the Mungini `AGENTS.md`:

- `MUNGINI_AGENT_RULES.md`: the 22 TROQ hard rules, the Troqbound frontend structure rules (one component per file, page logic in a hook, API calls in services, types in `types/`, registry driven navigation), backend structure rules adapted to Rust, migration safety, multilingual content rules, and the working process.
- `MUNGINI_UI_DESIGN.md`: tokens, typography, layout, panels, tables, the primitives to build once, page archetypes, motion, i18n, dates and money, and the page checklist.

What does not carry over from Troqbound: NATS, the HTTP gateway, Hono, Drizzle, the verticals model and its locale list. Rule 5 changes meaning: Mungini starts a fresh numbered migration sequence, and once a migration has run anywhere it is never edited.

## Reporting

When a slice is done, report: what moved, what was adapted, which checks ran (type check yes or no, 375px screens), which seeds were added, and any manual step left for the owner. For Phase 4, list the services and apps that changed in TROQ and the manual steps: remove the `stay-db` containers and volumes, the `stay-service` entries in compose and bake files, the `troq-stay` image, the `STAY_*` environment variables (the encrypted `.env.enc` files are the owner's to edit), and the `STAY_EVENTS` JetStream stream and its consumers.

## Risks

- Host identity is the hardest cut. Stay hosts are TROQ businesses and use the `stay_manager` role, scoped grants and presets owned by `auth-service`. Settle D2 before writing `identity` and `hosts`.
- `stay_manager` is a value of the shared role enum. Removing it touches `troq-roles`, the gateway authorization middleware and every client's `roles.ts`, and users who hold it need a plan.
- Mixed locale files. Several shared locale files hold Stay keys beside unrelated keys (`merchant.json`, `adminDesign.json`, `notificationCenter.json`). Remove only the Stay keys.
- CRM coupling. `crm-service` consumes Stay events to create leads and has `stay_operator_id` columns on audit and webhook tables. Removal needs a new CRM migration.
- Replicated global tables. Categories, amenities and notification templates replicate per city today. Mungini owns its own copies.
- Public URLs. `/[lang]/stays` pages and `stays-image` and `stays-detail` API routes in `troqzone-web` may be linked or indexed. Decide on redirects at cutover.
- App store. Removing Stay from the TROQ mobile app needs a TROQ release and a new Mungini app listing (D9).
