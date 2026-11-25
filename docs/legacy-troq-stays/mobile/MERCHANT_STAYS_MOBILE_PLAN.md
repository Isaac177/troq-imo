# Stay management in the merchant mobile app

## Objective

Bring the full stay-management feature set of `troq-front/troqzone-business` (the `/stays` workspace) into the Expo app as a separate module inside `packages/merchant`, next to `crm/`. A tenant (accommodation operator) must be able to run listings, calendar, rates, reservations, daily tasks, credits, cases and reviews from a phone.

This file is the audit and the dispatch plan. The module is built by several agents in parallel, then merged and verified by the lead.

## Audit summary

Source of truth: `troq-front/troqzone-business/src`.

- Routes: `routes/staysRoutes.ts`. Layouts: `StaysLayout`, `StaysCalendarLayout`.
- Navigation: `components/admin/nav/staysNavigation.ts` (11 entries: listings, accommodation types, calendar, rates, reservations, tasks, credits, cases, reviews, operators, trips).
- UI: `components/stays/**` (about 5,600 lines, 17 folders). Logic: `hooks/stays/**` (69 hooks). Pure logic: `utils/stays/**`. Contracts: `types/stay*.ts` (17 files).
- Backend: `tzone-rs/stay-service`, reached through `/api/merchant/stays/*`. No backend change is planned. If a screen needs an endpoint that does not exist, the agent reports it and does not touch Rust.
- Access: role `stay_manager` plus stay permissions (`STAY_VIEW_PROPERTIES`, `STAY_MANAGE_PROPERTIES`, `STAY_VIEW_RESERVATIONS`, `STAY_MANAGE_RESERVATIONS`, `STAY_VIEW_GUEST_DETAILS`, `STAY_MANAGE_CHECKIN`, `STAY_MANAGE_HOUSEKEEPING`). The backend stays authoritative, mobile only hides what the user cannot use.
- Existing mobile code to avoid colliding with: the customer-side stays in `apps/mobile/screens/Stay*.tsx`, `apps/mobile/components/stays/**`, and the locale `stays.json`. The business module uses new locale files named `merchantStays*.json`.
- Pattern to copy: `packages/merchant/src/crm/` (navigator, sections file, tabs file, per-feature folders with `screens/ components/ hooks/ types/ utils/`).

Endpoints used by the web stays workspace, grouped by feature:

| Feature | Endpoints |
|---|---|
| Operators | `operators`, `operators/:id/contact`, `business/analytics/summary` |
| Listings | `properties`, `properties/:id`, `properties/:id/revisions[/:rev][/submit\|withdraw]`, `properties/:id/lifecycle`, `properties/:id/media`, `media/:id/content`, `amenities`, `accommodation-categories` |
| Unit types | `properties/:id/unit-types[/:typeId]` |
| Accommodation types | `accommodation-templates[/:id]`, `accommodation-templates/:id/media[/:mediaId]` |
| Calendar | `calendar`, `calendar/inventory-default`, `calendar/feed-tokens[/:id]`, `blocks[/:id]`, `restrictions/bulk`, `unit-assignments`, `unit-assignments/:id/move`, `business/reservations/:id/amendments`, `amendments/:id/accept\|reject`, `crm/*` |
| Rates | `properties/:id/rate-plans[/:planId][/rules[/:ruleId]]`, `properties/:id/promotions[/:id]`, `rate-policies`, `price-preview` |
| Reservations | `business/reservations[/:id][/decision\|check-in\|check-out\|no-show\|cancel\|amendments]`, `business/reservations/export`, `reservation-items/:id/unit-options` |
| Tasks | `tasks`, `tasks/:id/transition` |
| Credits | `credits/mine` |
| Cases | `cases`, `cases/:id`, `cases/:id/evidence`, `cases/:id/close`, `cases/evidence/:id` |
| Reviews | `reviews`, `reviews/:id/reply` |
| Trips | `reservations/mine`, `reservations/:id/cancellation`, `reservations/:id/cancel` |

## Module layout

```text
packages/merchant/src/stays/
  navigation/   StaysNavigator, staySections, stayTabs, stayScreens (the registry)
  shared/       api/ hooks/ components/ types/ utils/   (built in wave 0)
  home/         listings/         templates/      calendar/
  rates/        reservations/     tasks/          credits/
  cases/        reviews/          trips/          notifications/
```

Every feature folder contains `screens/ components/ hooks/ types/ utils/`. Components hold JSX only, logic lives in `use*` hooks.

Navigation: `StaysNavigator` is a drawer navigator registered as `MerchantStays` in `MerchantNavigator`, with a `stays` section in `merchantSections.ts` that only shows for users with stay access. Bottom tabs: Reservations, Calendar, Listings, Tasks, Menu. Menu lists the rest.

## Screens to build

Kinds: `screen` is a navigable route, `sheet` is a full-width bottom sheet (up to 92% of the content height), `shared` is a reusable component. Every list is server-paginated with a `#` column or row number. Every screen needs loading, empty, error and forbidden states, in English and French, checked at 375px.

### Shell and shared (wave 0)

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S01 | Stays home | screen | `StayOperatorsPage`, `useStayReservationsSummary` | Operators with property count, pending requests, open cases, status. KPI cards 2x2 from the analytics summary. Shortcuts to pending work. |
| S02 | Access states | shared | `useStayOperators`, `apiErrors` | No operator, operator not approved, 403, retry. |
| S03 | Stays drawer and bottom tabs | shared | `staysNavigation`, `StaysLayout` | Section groups, active key, back handling. |
| S04 | Operator and property selector | shared sheet | `StayDestinationSelector`, `StayCalendarPropertyPicker` | Replaces every typed id. Persists the last choice. |
| S05 | Credit banner and strip | shared | `StayCreditBanner`, `StayCreditStrip` | Shown on reservations, rates, calendar when balance is low. |
| S06 | Status badge, panel, section, tab strip, pager, refresh | shared | `StayPropertyStatus`, `StayPanel`, `StaySection`, `StayTabStrip`, `StayRatePager` | Reuse `Merchant*` components first. |
| S07 | Date range picker, occupancy selector, money breakdown, confirm sheet, type cover and gallery | shared | `StayDateRangePicker`, `StayOccupancySelector`, `StayMoneyBreakdown`, `StayConfirmPanel`, `StayTypeCover`, `StayTypeGallery` | dd/mm/yyyy, CDF only. |

### Listings

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S10 | Listings list | screen | `StayPropertiesPage`, `StayPropertyList` | Cover, status, from price, units, pending requests, photo count. |
| S11 | Listings filters | sheet | `StayPropertyFilters` | Status tabs, operator, name search. |
| S12 | Create listing, step 1 details | screen step | `StayCreateDetailsStep`, `AccommodationCategoryField` | Category, EN/FR name and description, neighborhood, address, coordinates, check-in and check-out times, house rules EN/FR, pets, smoking, amenities. |
| S13 | Create listing, step 2 guest | screen step | `StayCreateGuestStep`, `StayContactSection` | Contact phone with country selector, email, operator contact. |
| S14 | Create listing, step 3 photos | screen step | `StayCreatePhotosStep` | Camera or gallery upload, queue, cover, order, remove. |
| S15 | Create listing, step 4 units | screen step | `StayCreateUnitsStep`, `StayTemplatePicker` | Pick accommodation types or define unit types and counts. |
| S16 | Create listing, step 5 pricing | screen step | `StayCreatePricingStep`, `useStayCreatePricing` | Base rate, rate policy. |
| S17 | Create listing, step 6 review and submit | screen step | `StayCreateReviewStep`, `createRemaining` | Summary, remaining items, submit. |
| S18 | Listing editor | screen | `StayPropertyEditorPage`, `useStayPropertyEditor` | Status header, begin edit, details and guest steps, read-only when not editable. |
| S19 | Revision banners and actions | shared | editor page | Submitted (withdraw), changes requested (guidance and reasons), submit revision. |
| S20 | Readiness panel | shared | `StayReadinessPanel` | Blockers list that jump to the step to fix. |
| S21 | Lifecycle sheets | sheet | editor page, `StayConfirmPanel` | Pause, resume, archive with consequences. |
| S22 | Amenity picker | sheet | `AmenityPicker`, `amenityIcons` | Searchable, grouped. |
| S23 | Media editor | screen | `StayMediaEditor`, `StayMediaDropzone`, `StayMediaQueue`, `StayMediaTile` | Used by listings and accommodation types. Authenticated image content. |
| S24 | Unit types editor | screen and sheet | `StayUnitTypesEditor`, `useStayUnitTypes` | List, add, edit, archive, occupancy, units count, images. |
| S25 | Operator contact | sheet | `StayOperatorContact` | Edit operator contact. |

### Accommodation types

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S30 | Accommodation types list | screen | `StayTemplatesPage`, `StayTemplateList` | Paginated. |
| S31 | Accommodation types filters | sheet | `StayTemplateFilters` | Category, search. |
| S32 | Accommodation type form | sheet | `StayTemplateSheet`, `useStayTemplateEditor` | Create and edit, EN/FR names, occupancy, beds, area. |
| S33 | Accommodation type images | shared | `StayTemplateImages`, `useStayTemplateMedia` | Reuses S23. |

### Calendar

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S40 | Calendar (agenda, week, month) | screen | `StayCrmCalendar`, `StayCrm*View`, `useStayCrmCalendar` | Phone default is agenda. Mini month for date jump. |
| S41 | Calendar filters and legend | sheet | `StayCrmSidebar`, `StayCrmLegend`, `StayCalendarLegend` | Properties, event types. |
| S42 | Calendar event details | sheet | `StayCrmEventDetails`, `StayCrmEventBar` | Reservation or block, with its actions. |
| S43 | Availability | screen | `StayCalendarAvailability`, `StayCalendarGrid`, `StayCalendarListView`, `useStayCalendarPage` | Phone layout is per unit type with a horizontal day strip and a list view, not the desktop grid. |
| S44 | Date range actions | sheet | `StayCalendarRangePanel`, `useStayCalendarBulk`, `calendarRestrictionOperations` | Close, open, minimum stay, bulk restrictions over a selected range. |
| S45 | Move assignment | sheet | `StayCalendarMovePanel`, `useStayCalendarMove` | Move a stay to another unit. |
| S46 | Pending amendment | sheet | `StayCalendarPendingAmendment` | Accept or reject. |
| S47 | Request decision | sheet | `StayCalendarDecision`, `useStayCalendarDecision` | Accept or decline a request-to-book. |
| S48 | Default inventory | sheet | `StayCalendarInventory`, `useStayCalendarInventory` | Set default units per type. |
| S49 | Blocks | sheet | `StayCalendarBlocks`, `stayCalendarBlocks` locale | Create and delete blocks. |
| S50 | Calendar sync | screen | `StayCalendarSync`, `StayCalendarFeeds` | Create, copy and revoke feed links, share sheet. |
| S51 | Calendar empty state | shared | `StayCalendarEmptyState` | No property yet, link to create one. |

### Rates

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S60 | Rates | screen | `StayRatesPage`, `useStayRatesPage` | Property selector, tabs plans and promotions, pager. |
| S61 | Rate plan detail and editor | sheet | `StayRatePlanSheet`, `StayRatePlanEditor` | Edit plan, policies. |
| S62 | Rate plan create | sheet | `StayRatePlanCreateSheet`, `StayRatePlanCreateForm` | Unit type, policy, base price. |
| S63 | Rate rules | sheet | `StayRateRulesPanel`, `useStayRateRules` | Add, edit, delete rules. |
| S64 | Promotions list | screen tab | `StayPromotionsTable`, `useStayPromotions` | Paginated. |
| S65 | Promotion create and edit | sheet | `StayPromotionCreateSheet` | Dates, discount, plans. |
| S66 | Price preview | sheet | `StayPricePreviewPanel`, `useStayPricePreview` | Dates and occupancy, money breakdown. |

### Reservations

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S70 | Reservations list | screen | `StayReservationsPage`, `StayReservationList` | Nine state tabs, summary KPIs, credit strip, `#` column. |
| S71 | Reservations filters | sheet | `StayReservationFilters` | Operator, search, dates. |
| S72 | Reservation detail | screen | `StayReservationDetailRow`, `StayReservationSummary` | Guest (gated by guest-details permission), items, special requests, money breakdown, stay phase. |
| S73 | Reservation actions | sheet | `StayReservationRowActions`, `reservationActions` | Accept, decline, check in, check out, no-show, accept or decline special request. Each with confirmation. |
| S74 | Amend reservation | sheet | `StayReservationAmend`, `StayReservationRoomPicker` | New dates and room from unit options. |
| S75 | Cancel reservation | sheet | `StayReservationCancel` | Consequences, refund, reason. |
| S76 | Export reservations | action | `useStayReservationsExport`, `reservationCsv` | CSV written with `expo-file-system`, shared with the share sheet. |

### Tasks

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S80 | Tasks list | screen | `StayTasksPage`, `StayTaskList` | Paginated, property required. |
| S81 | Tasks filters | sheet | `StayTaskFilters` | Property, state, type, priority. |
| S82 | Task detail and actions | sheet | `StayTaskActions`, `taskActions` | Start, complete, cancel, assign to me, unassign, checklist. |
| S83 | Create task | sheet | `StayTaskCreateSheet` | Type, priority, checklist, unit, reservation item chosen from selectors. |

### Credits

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S90 | Credits | screen | `StayCreditsPage`, `StayCreditBalance`, `StayCreditLedger` | Balance, rate, low threshold, paginated ledger by entry type, operator selector. Top-up flow is not in web, so it is not built (see open questions). |

### Cases

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S100 | Cases list | screen | `StayCasesPage`, `StayCaseList` | Mine and operator views. |
| S101 | Cases filters | sheet | `StayCaseFilters` | Operator, state, type. |
| S102 | Open case | sheet | `StayCaseOpenSheet`, `useStayCaseOpen` | Type, reason, reservation chosen from a selector. |
| S103 | Case detail | screen | `StayCaseDetailSheet`, `useStayCaseDetail` | Parties, deadline, evidence list, add evidence from camera, gallery or documents, remove evidence. |
| S104 | Close case | sheet | `useStayCaseDetail` | Resolution text, version check. |

### Reviews

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S110 | Reviews list | screen | `StayReviewsPage`, `StayReviewList` | Paginated. |
| S111 | Reviews filters | sheet | `StayReviewFilters` | Operator, rating, replied. |
| S112 | Reply to review | sheet | `StayReviewReplyRow` | Post a reply. |

### Trips

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S120 | Trips list | screen | `StayTripsPage`, `StayTripList` | The operator's own bookings as a guest. |
| S121 | Trip detail | screen | `StayTripDetailRow` | Dates, money breakdown. |
| S122 | Cancel trip | sheet | `StayTripCancel`, `useStayTripCancel` | Cancellation quote then cancel. |

### Notifications

| ID | Screen | Kind | Web source | Notes |
|---|---|---|---|---|
| S130 | Stay notification deep links | mapping | `features/notifications/lib/refreshTargets.ts`, `categoryIcons.ts` | Reservation, task, case, review and credit notifications open the matching screen and refresh its query. |

Total: 7 shared groups, 25 listing items, 4 accommodation-type items, 12 calendar items, 7 rate items, 7 reservation items, 4 task items, 1 credit item, 5 case items, 3 review items, 3 trip items, 1 notification mapping.

## How the work is dispatched

### Constraints that shape the dispatch

- The working directory is not a git repository, so worktree isolation is not available. Agents share one checkout. Each agent therefore owns a set of folders and never edits another agent's folder.
- Files that several agents would otherwise edit are edited only by the lead at merge time:
  - `navigation/stayScreens.ts` (the registry of routes), `navigation/staySections.ts`, `navigation/stayTabs.ts`
  - `packages/merchant/src/navigation/merchantSections.ts` and `MerchantNavigator.tsx`
  - `packages/shared-i18n/locales/index` files
- Each agent writes its own locale files, `merchantStays<Feature>.json` in `en` and `fr`, and its own area file `<feature>/stayScreens.<feature>.ts` that exports its routes. The lead adds one line per area to the registry.
- No agent runs `cargo`, a build, or a dev server. No agent touches `tzone-rs`. The only check is the type check of `apps/mobile` (`tsconfig.check.json`) plus `bun test` for pure utils.

### Waves

```text
Wave 0  Foundation            1 agent, blocking
Wave 1  Feature slices        6 agents in parallel
Wave 2  Listing creation      1 agent, after the listing editor lands
Wave 3  Integration           lead
```

### Wave 0: foundation (blocks everything)

Owner: one agent. Folders: `stays/navigation/`, `stays/shared/`, locale `merchantStaysShared.json`.

Deliverables:
- `StaysNavigator`, sections, tabs, access gating through `merchantAccess` and stay permissions, registered in `MerchantNavigator` and `merchantSections.ts`.
- `shared/types`: the platform-neutral types from `types/stay*.ts`, split into files of at most 300 lines.
- `shared/utils`: the pure logic from `utils/stays/**` (calendar grid, restrictions, rate formatting, reservation and task actions, stay phase, date and time, api errors, validation, csv), with `bun test` ports of the existing tests.
- `shared/api`: query keys and a `useStayHttp` hook over the existing `useApi`, paginated query helper (`page`, `pageSize`, total).
- Shared components S02 to S07 and the credit strip, using the existing `Merchant*` components.
- S01 stays home, since it proves the navigator and the access states.

Done when: the stays entry appears for a stay manager, S01 loads real data, `bunx tsc` is clean, and the shared API is documented in the agent report so wave 1 can import it.

### Wave 1: six parallel agents

| Agent | Scope | Screens | Folders and locale |
|---|---|---|---|
| A, Listings | Browse, editor, lifecycle, media, units | S10, S11, S18 to S25 | `listings/`, `merchantStaysListings.json` |
| B, Calendar | All calendar work | S40 to S51 | `calendar/`, `merchantStaysCalendar.json` (reuses `stayCalendarBlocks.json` keys) |
| C, Reservations | Reservations and amendments | S70 to S76 | `reservations/`, `merchantStaysReservations.json` |
| D, Rates | Plans, rules, promotions, preview | S60 to S66 | `rates/`, `merchantStaysRates.json` |
| E, Operations | Tasks, cases, reviews | S80 to S83, S100 to S104, S110 to S112 | `tasks/`, `cases/`, `reviews/`, `merchantStaysOps.json` |
| F, Account | Credits, trips, notifications, accommodation types | S30 to S33, S90, S120 to S122, S130 | `credits/`, `trips/`, `templates/`, `notifications/`, `merchantStaysAccount.json` |

Agent A also owns the shared media editor (S23), the amenity picker (S22) and the unit types editor (S24), and publishes their props in its report because wave 2 and agent F (S33) reuse them. Agent F builds S33 after A reports, or leaves S33 for the lead if A is late.

Size and risk: B is the largest (the web calendar is about 1,300 lines plus 12 hooks), so it is started first. If B runs long, split it into B1 (S40 to S42, S51, S50) and B2 (S43 to S49) with disjoint subfolders `calendar/agenda/` and `calendar/availability/`.

### Wave 2: listing creation

Agent G, after A lands. Scope: S12 to S17, the wizard host with draft state, step badges, remaining-items logic, and submit. Folders: `listings/create/`, locale `merchantStaysCreate.json`. Reuses S22, S23, S24 from A and the template list endpoint from F's types.

### Wave 3: integration (lead)

1. Add each agent's area file to `stayScreens.ts`, sections and tabs, one agent at a time.
2. Run the type check of `apps/mobile` and `bun test` after every merge, then once at the end (a clean agent report only covers that agent's folders).
3. Check duplicate i18n keys and that every key exists in `en` and `fr`.
4. Verify at 375px in English and French: each screen above, once, with few screenshots. Report which screens were checked.
5. Seed check (below), then the final list of changed apps and manual steps for the owner.

## Brief given to every agent

Common brief, followed by the scope row of that agent:

- Read `AGENTS.md` at the repo root and in `troqzone-app/`, `docs/MERCHANT_BUSINESS_MOBILE_PLAN.md`, and the CRM module in `packages/merchant/src/crm/` as the pattern to match.
- Read the web source files listed in the screen table before writing, and the matching `useStay*` hooks. Port contracts and logic, not DOM, Tailwind, router or Lucide code.
- Hard rules: no comments in code, components contain only JSX with logic in `use*` hooks, at most 300 lines per file, all strings in `en` and `fr`, CDF only, dates dd/mm/yyyy, phone inputs with country selector, selectors instead of typed ids, server pagination with a `#` column, transitions on interactive UI, no hardcoded text.
- Phone rules for business screens: body 12px, labels 11px, page titles 15px, card figures 15px, controls 36px or less, short verb buttons become icon-only with `aria-label` and `title` below `sm`, secondary table columns hidden on phone, no sideways page scroll, sheets up to 92% of the content height.
- Touch only your own folders and locale files. Do not edit the registry, `merchantSections.ts`, `MerchantNavigator.tsx` or another agent's folder. Export your routes from your own area file.
- Do not run `cargo`, builds or dev servers. Run `bunx tsc` for `apps/mobile` and `bun test` for your utils only.
- Register nothing as a placeholder. A screen is finished only when its API, permission guard, loading, empty, error and forbidden states and its main workflow work.
- Final report: files created, route names exported, shared props other agents need, endpoints that were missing or behaved differently from web, screens not finished and why. No claim of visual verification, the lead does that.

## Seed data

Rule 9 requires realistic demo data for every feature. `tzone-rs/stay-service/seeds` must be checked against this list and extended with `seed-*` ids where missing: operators in each status, listings in every status (draft, approved, published, paused, suspended, archived), revisions submitted and with changes requested, unit types with images, rate plans with rules and promotions, reservations in all nine states including a pending special request and a pending amendment, blocks, feed tokens, tasks of every type, state and priority, credit ledger entries of every type, cases open and closed with evidence, reviews with and without a reply. This is a backend and data task for the owner's tooling, done before wave 3 verification.

## Open questions

1. Trips (S120 to S122): the customer side of the mobile app already has `StayTripsScreen`. The recommendation is to link to it from the stays Menu instead of rebuilding it. Decide before agent F starts.
2. Credits: the web page has a balance and ledger but no top-up flow. If tenants must top up from the phone, a payment flow and a backend endpoint are needed and are not part of this plan.
3. Calendar on phone: the plan uses an agenda-first calendar and a per-unit-type day strip. A full desktop-style grid is not planned.
4. CSV export on phone writes a file and opens the share sheet. Confirm that is acceptable for tenants.
5. Whether `stays` appears in the shop drawer for every merchant account or only for `stay_manager` users. The plan gates on stay access.
