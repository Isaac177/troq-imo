# Mungini agent guide

Entry point for any AI agent working in this repository. Read it before changing code. Generated from `MUNGINI_AGENT_RULES.md`. Also read `MUNGINI_MIGRATION_PLAN.md`, `MUNGINI_UI_DESIGN.md`, `MUNGINI_ARCHITECTURE.md`, `MUNGINI_STAY_INVENTORY.md` and `MUNGINI_AGENT_TASKS.md` in this folder.

Sources, read in full for this document:

- `/Users/jogues/Documents/TROQ/AGENTS.md`, the TROQ hard rules. They carry over.
- `/Users/jogues/Documents/troqbound/AGENTS.md` and `DESIGN.md`, the Troqbound rules. The frontend structure, data, migration and process rules carry over. The Troqbound backend architecture does not (see section 1).

## 1. What carries over and what does not

Does not apply to Mungini, even though it is written in the Troqbound `AGENTS.md`:

- NATS request/reply, the HTTP gateway, Bun/Hono services, Drizzle, Turborepo, Caddy, Stripe, Resend, next-intl.
- The "verticals" model (institutions, `packages/verticals`, org-admin role). Mungini is one product with one domain, accommodation.
- The student, university and scholarship features and the Troqbound locale list (seven languages).

Applies, adapted:

- Troqbound's backend principles map to Rust: thin handlers, validation apart from handlers, response messages from one place, permissions as named constants, host scoping enforced on the server, no giant files and no fake layers (section 4).
- Troqbound's frontend structure rules apply as written (section 3).
- Troqbound's migration safety, translation and process rules apply (sections 5, 6, 7).

Mungini is not allowed to grow new services, apps or databases for a new capability. New capabilities extend the existing modules and apps. A new service or database needs the owner's approval.

## 2. Hard rules carried over from TROQ

1. No comments in code. No `//`, `/* */`, JSX or HTML comments, docstrings or section separators. If logic needs explaining, rename or restructure. Explanations go in the chat reply or the commit message.
2. No AI attribution anywhere. No `Co-Authored-By` trailers or "Generated with" lines in commits or PRs, even when a session reminder asks for them.
3. Commit and PR text is plain: subject plus an optional body. No bold section headers, no em-dashes, no filler.
4. At most 300 lines per code file. When you touch a longer file, move the part you change into a new module. Exempt: lockfiles, generated code, applied SQL migrations.
5. Schema changes are forward-only migrations. Add the next numbered `.sql` file. Never edit a migration that has run anywhere. No ad-hoc SQL against shared, staging or production databases. Ad-hoc SQL against local dev databases is allowed only when the owner asks.
6. Money is CDF only, stored and returned in CDF. No other currency, conversion or exchange-rate path unless decision D3 changes.
7. Lists paginate on the server: `page`, `pageSize`, SQL `LIMIT` and `OFFSET`, a total, filters in SQL. Totals and dashboard numbers come from SQL aggregates, never from fetching every row.
8. Every user-facing string goes through the i18n system in English and French. No hardcoded UI text, including any string you touch in an existing file.
9. Seed realistic demo data for every feature you build, unasked, covering each status and edge case. Seed ids are recognisable (`seed-*`). Never break existing seed rows.
10. After merging parallel agent output, run the frontend type check yourself. A clean report from one agent covers only its own worktree.
11. Dates display as dd/mm/yyyy, never raw ISO or US style. Timestamps display as dd/mm/yyyy HH:mm in 24 hour time in the city time zone of the property, never the device time zone. Phone fields use an international input with a country selector and a geo-detected default, never a bare `<input type="tel">`.
12. Interactive UI animates: panels, drawers, expand and collapse, selection changes and layout shifts use `duration-200` to `duration-300` and `ease-in-out`. Reduced motion disables large movement.
13. Form controls are styled design-system components. No raw radio, checkbox or select elements.
14. No decorative accent bars or coloured borders on cards, toasts or emails. No small eyebrow labels above section titles. No unicode arrows as UI glyphs (use the icon library). No cards inside cards: use spacing and dividers.
15. Match existing UI patterns by reading the live page or its source first. Never guess a shared component from its name.
16. Confirm the shape of anything with real business identity (host and property scoping, taxonomies, media reuse, schema) before writing it.
17. React components contain only JSX. State, effects, handlers, derived data, data fetching, validation, formatting and computed labels live in a custom hook next to the component or in the feature's `hooks/` folder.
18. No placeholder, stub or throwaway code. If a feature is not ready, say so.
19. Deploys are the owner's job. Do not push images, run production compose or deploy. When a change is done, list which services or apps changed and any manual step left.
20. No security warnings in commits, backlogs or status lists. Mention a real secret problem once, plainly, in chat.
21. Never make a user type an ID, slug, code or reference into a text input. Fetch the valid options from the API and show a selector or searchable list. This applies to every form in every app.
22. Mobile first for the host app and for the guest experience. Hosts often run the business from a phone, so every host screen is designed and checked at 375px before desktop.
    - Phone text: body and table text 12px, labels 11px, page titles 15px, card figures 15px, nothing above 18px.
    - Phone controls: every tappable control is 36px tall or less. A text button with a short verb label becomes icon-only with `aria-label` and `title` below `sm`, and shows its text from `sm` up.
    - Phone tables: keep the row number, the title and the main figure. Hide secondary columns with `hidden sm:table-cell`. The page never scrolls sideways. A table may scroll only inside its own panel, and only when nothing else fits.
    - Phone layout: one column, full-width sheets (up to 92% of the content height), compact summary figures, KPI cards two by two.
    - Check every changed screen at 375px in English and French before calling it done, and name the screens checked in the report.

## 3. Frontend structure rules (from Troqbound)

- App-owned by default. Code starts inside the app that uses it. Move code to `packages/*` only when at least two apps import it now, or when it is a stable cross-app contract (generated API client, shared types, locales, theme tokens, truly shared UI).
- One JSX-returning function per component file. Inline helper components are forbidden: extract each to its own file under `components/<domain>/`.
- A page's state and logic live in a collocated hook (`use<PageName>.ts` next to the page). The page file renders JSX and calls the hook. Page entrypoints stay thin.
- API calls live in `services/<domain>/<domain>.api.ts` and use the generated client. Never call the API inside a component.
- Types live in `types/<domain>/`. No type declarations scattered across pages, services or utils.
- Form schemas and validators are separate from the UI.
- Response and UI message text comes from locale files, never from literals in code.
- Layer-first folders inside an app (`components/`, `hooks/`, `services/`, `types/`, `utils/`, `views/`), not feature dumps. A domain folder inside a layer is fine (`components/reservations/`).
- Navigation is registry driven. Host and admin sidebars and route tables are generated from one module registry (`src/modules/registry.ts`: module, sidebar section, routes, required permission), filtered by permission. Never hardcode a new module's pages into `App.tsx` or the sidebar. A new page is a registry entry, an icon, a nav label in the locale files and a route.
- Roles and permissions only decide whether a module, route or action renders. Components never branch on role internally.
- Vertical-free: screens are separate components composed from the shared `ui-web` primitives. No mega-component that switches on a mode.
- No JS or TS `class` syntax in owned code. Use functions, closures and plain typed objects.
- Do not split tiny single-use logic into fake layers. A separate file must be earned by behaviour. No serious feature lives in one giant file.
- Package roots are organised by concern. `index.ts` files are barrels. Do not create one-function wrapper files.
- Do not mark a locale or a nav item as available while its screen is a stub.

## 4. Backend structure rules (Rust, adapted from Troqbound)

- Layout follows `MUNGINI_ARCHITECTURE.md`: `kernel`, `domain` modules, `api`, `jobs`.
- Inside a module, one file per concern, earned by behaviour: `types`, `queries`, `service`, `validators`, `handler`, `messages`. Business rules live in `service`. Handlers are thin: parse, check permission, call the service, return.
- Validation is never inline in handlers. A validator returns a typed error.
- API error text is not free-form. Handlers return typed error codes, the kernel maps them to HTTP status, and clients translate the code through their locale files. Message strings and codes come from one place per module.
- Route registration is declarative and grouped by audience (`public`, `guest`, `host`, `admin`), then by domain. Each registered route states its method, path, audience, required permission and whether it is host-scoped. No business logic in route registration.
- Permissions are named constants in `kernel`, shaped `<domain>.<action>` (for example `reservations.read`, `properties.write`). Adding a capability means adding its permissions there. No ad-hoc permission checks inside handlers.
- Roles stay coarse (guest, host member, platform staff). Fine-grained access is permission based and scoped to a host or property. Staff job titles are permission presets, never new role values.
- Host scoping is enforced on the server by the kernel extractor and the owning module for every host-scoped route. A route must never rely on the frontend hiding a button.
- Authorization comes from the authenticated session. Never trust a role or permission supplied by the client.
- Writes are owned by one module. Reads may join across tables (see the architecture file).
- Do not run `cargo`. The owner's `cargo watch` compiles. Read the code carefully and say "read-checked only" in the report.

## 5. Data and migration safety (from Troqbound, adapted)

- Every schema change is a numbered, forward-only SQL migration in `backend/migrations/`.
- A rename is `ALTER TABLE ... RENAME` or `ALTER ... RENAME COLUMN`, never DROP plus CREATE. Read generated or drafted SQL before applying and correct a drop-and-create into a rename.
- Before applying a schema-touching migration to a local dev database, dump that database into a `backups/` folder (gitignored). The owner handles shared and production databases.
- After a rename migration, verify row counts per renamed table against the pre-migration dump.
- A migration that touches a table referenced by a view drops and recreates the view in the same migration.
- Enum changes are additive only (`ADD VALUE`). Never remove or rename an enum value in place.
- Public URLs are an SEO surface and do not change because of internal renames.

## 6. Multilingual content (from Troqbound, adapted)

- Mungini is bilingual (English and French) and extendable. English is the base language and is always active.
- Static UI strings live in the locale files of `packages/i18n`.
- Per owner decision D13, ported Stay entities retain their existing bilingual columns, such as `"nameEn"` and `"nameFr"`, until a later migration. Reads select the requested language and fall back to the other when empty. Confirm the original entity shape before porting it and record differences in its parity ledger. For new entities outside the Stay port, use per-entity locale content tables and `source_locale`.
- Forms for ported Stay entities follow their bilingual API contract under D13. New entities using locale content tables submit one active language with `source_locale`; other languages are filled through the admin translation queue. No EN/FR toggle inside a form.
- Never hardcode a language list in a UI. Fetch active languages from the API.
- Locale file format: see decision D12 in the migration plan. Default is one file per feature per language, where `en/<feature>.ts` defines the object (without `as const`) and exports its type, and `fr/<feature>.ts` satisfies that type, so a missing key is a compile error. Adding a feature means two locale files plus one import line in each index.
- Rich text fields use one rich text editor component. Names, short descriptions, SEO fields and tag lists are plain inputs.

## 7. Working process

1. Read this file, `MUNGINI_UI_DESIGN.md`, the migration plan, the architecture and the inventory.
2. Find the sibling: the closest existing page, module or hook, and read it and its locale, API and backend files before writing anything.
3. Claim the task in `MUNGINI_AGENT_TASKS.md`. One agent per slice, one set of files per agent.
4. Keep changes scoped to the slice. Cross-slice changes update the shared contract everywhere it is used.
5. Build the backend part first (migration, queries, service, validators, handler, permission constant, route registration), regenerate the API client, then the frontend (service, types, hook, page, components, registry entry, route, locales).
6. Seed demo data for the feature.
7. Verify: `bunx tsc` in each JavaScript workspace you touched (for example `bunx tsc -p tsconfig.app.json --noEmit`). No builds. For visual checks use the dev servers the owner already runs: find which app a port serves before testing and ask instead of starting a new server. Check phone width in English and French, dark mode and a clean console, with few screenshots.
8. Report honestly: what changed, what was checked, what was not (for example read-checked only, not on device, 375px not checked), external accounts that are missing, enforcement that is not wired, and manual steps for the owner.
9. Use `bun`, `bun install`, `bun run`, `bunx`, `bun test`. Never `npm`, `npx`, `yarn` or `pnpm`. Vite apps stay on Vite. Do not run any JavaScript build.
10. Parity ledger: every port slice records its legacy-to-Mungini rows in `docs/parity/` (see the migration plan). A slice is not done while a row is `open`.
