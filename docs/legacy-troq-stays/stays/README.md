# TROQ Stays implementation specifications

Status: proposed implementation specification, prepared 12/10/2026. Documentation only; not implemented or deployment-authorized.

## Purpose and authority

Build an advanced accommodation module spanning platform administration, property operations, public web, mobile, and Rust services. This package retains the advanced scope discussed with the owner. Delivery order is dependency ordering, not permission to silently remove advanced features.

Read repository AGENTS.md first. Its rules take precedence. Read this index, the decisions register, permissions, architecture, UI rules, and the feature specification before implementing a slice. These specifications do not authorize agents to invent business identity, provider capabilities, legal policy, or financial terms.

The owner explicitly requested very few enum roles and permission-based restrictions. The target introduces one enum value, stay_manager. Staff job descriptions are permission presets, never new role enums. Existing administrators and customers keep their roles. Existing merchant users gain scoped stay access without being forced to replace their current role.

## Specification map

| ID | File | Coverage |
|---|---|---|
| D | [00-decisions.md](00-decisions.md) | Unresolved business decisions and implementation gates |
| R | [01-roles-permissions.md](01-roles-permissions.md) | One new role, memberships, permission catalog, presets, revocation |
| B | [02-architecture-data.md](02-architecture-data.md) | Services, city routing, conceptual entities, migrations |
| U | [03-ui-system.md](03-ui-system.md) | Shared visual and interaction contract |
| A | [04-admin.md](04-admin.md) | Approval, oversight, policy, finance, support, reports |
| O | [05-operators-properties.md](05-operators-properties.md) | Onboarding, portfolios, property editor, media, taxonomy |
| I | [06-inventory-calendar.md](06-inventory-calendar.md) | Units, capacity, calendar, blocks, concurrency |
| P | [07-pricing-policies.md](07-pricing-policies.md) | Rates, restrictions, promotions, quote calculation |
| C | [08-client-discovery.md](08-client-discovery.md) | Entry, search, map, detail, saved stays |
| Q | [09-checkout-reservations.md](09-checkout-reservations.md) | Checkout, state machines, booking, amendments, cancellation |
| F | [10-payments-settlements.md](10-payments-settlements.md) | Collection, refunds, ledger, commission, payouts |
| S | [11-stay-operations.md](11-stay-operations.md) | Front desk, housekeeping, maintenance, service requests |
| M | [12-messaging-reviews-support.md](12-messaging-reviews-support.md) | Communication, notifications, reviews, disputes |
| X | [13-integrations-growth.md](13-integrations-growth.md) | External calendars/channels, corporate/groups, loyalty, affiliates |
| API | [14-api-events.md](14-api-events.md) | Endpoint families, payloads, errors, event reliability |
| SEC | [15-security-privacy.md](15-security-privacy.md) | Authorization, files, abuse, sensitive workflows |
| PERF | [16-performance-operations.md](16-performance-operations.md) | Queries, caching, metrics, jobs, recovery |
| QA | [17-delivery-acceptance.md](17-delivery-acceptance.md) | Dependency slices, fixtures, tests, completion checklist |
| SHARED | [21-shared-business-capabilities.md](21-shared-business-capabilities.md) | Stay-manager access to shared CRM, finance, promotions, settings, team, and notifications |

## How an implementation agent must work

1. Identify feature IDs and affected workspaces. Read their local instructions and relevant existing source before designing changes.
2. Check decision gates. Continue independent work but do not commit schema or business behavior dependent on an unresolved identity or financial decision.
3. Confirm existing schemas and migration sequence against the actual checkout. Entity names and routes here are target contracts, not claims that they already exist.
4. Implement a complete vertical slice: migration, domain rules, scoped API, events, shared contracts, permission-aware UI, translations, seed data, and verification.
5. Keep code files at most 300 lines; extract changed behavior from oversized files. No comments in code. Components render JSX and call hooks containing logic.
6. Do not introduce fake integrations, placeholder pages, hardcoded UI strings, foreign currency, or successful-looking local-only booking actions.
7. Every mutation has an authorization rule, state preconditions, inventory/financial consequences, idempotency policy, audit entry, and defined failure recovery.
8. Check Rust and touched JS workspaces. Document changed services/apps, test results, unresolved limitations, and owner-run manual steps.
9. Do not deploy, push images, or alter a shared database by ad-hoc SQL.

## Scope status vocabulary

- Required: part of the specified module; must be implemented and verified for the applicable launch configuration.
- Decision-gated: retained in scope but dependent implementation awaits the numbered decision.
- Provider-gated: requires a real provider contract/capability; never replace with a stub.
- Proposed default: explicit recommendation for owner review, not an approved business policy.

No feature is complete solely because its happy-path UI exists. Apply the shared acceptance requirements plus the feature-specific cases.
