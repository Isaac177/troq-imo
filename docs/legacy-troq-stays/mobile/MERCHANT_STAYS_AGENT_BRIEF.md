# Brief for stays feature agents (wave 1 and 2)

Repo: /Users/jogues/Documents/TROQ. Module: `troqzone-app/packages/merchant/src/stays/`. Plan: `troqzone-app/docs/MERCHANT_STAYS_MOBILE_PLAN.md` (screen list with IDs and web sources). Web source of truth: `troq-front/troqzone-business/src` (`components/stays`, `hooks/stays`, `utils/stays`, `types/stay*.ts`).

Wave 0 is done and verified (type check clean, 33 tests pass). Build on it, do not rebuild it.

## Read first

1. `/Users/jogues/Documents/TROQ/AGENTS.md`, `troqzone-app/AGENTS.md`, `troqzone-app/docs/MERCHANT_BUSINESS_MOBILE_PLAN.md`.
2. The plan file above, rows for your screens.
3. `stays/home/` (finished feature folder to copy the structure from), `stays/navigation/stayScreens.ts`, `stays/navigation/staySections.ts`, `stays/shared/**`.
4. The web source files named in the plan for each of your screens, and the matching `useStay*` hooks. Port contracts, rules and logic, not DOM, Tailwind, router or Lucide code.

## Foundation contract (all under `stays/shared/`, import from your folder as `../../shared/...`)

- Types: `shared/types` (index.ts) has every web type. Money is CDF only.
- Utils: `shared/utils/*` has the pure logic ported from web with the same names (calendar grid, restrictions, crm calendar, reservation and task actions, stay phase, dates dd/mm/yyyy, formatCdf, apiErrors with `stayFailureMessage`, listing validation, phone parts, csv, readiness items, etc.). Tones are tokens (`StayTone`, `StayTextTone`) resolved with `toneColor` from `useStayTheme`.
- API: `shared/api`: `useStayHttp()` (get, post, patch, put, del; AxiosError kept), `stayPath(path, params)`, `stayKey`, `useStayScope`, `useStayQuery`, `useStayPagedQuery` (server paging, `numberOf(index)` for the `#` column, `reset()` on filter change, returns `isForbidden`, `isEmpty`, pager fields), `useStayMutation({ run, invalidate })`. Resource names for invalidation: operators, summary, properties, propertyOptions, revisions, unitTypes, templates, media, catalog, calendar, rates, promotions, reservations, tasks, credits, cases, reviews, trips.
- Hooks: `useStayContext()` (state, operator, operatorId, property, selectOperator, selectProperty), `useStayTheme`, `useStayNotice`, `useStayAccess`, `useStayConfirmation`, `useStayOccupancy`, `useStayCreditSummary`, `useStayReservationsSummary`, `useStayDateRangeField`.
- Components (default exports in `shared/components/`): `StayScreenFrame` (title, leading 'menu'|'back', actions, notice), `StayAccessGate`, `StaySheet` (bottom sheet up to 92%, use for every sheet), `StayOperatorSelector`, `StayPropertySelector`, `StaySelectField`, `StaySelectSheet`, `StayCreditStrip`, `StayStatusBadge`, `StayPanel`, `StaySection`, `StayTabStrip`, `StayPager`, `StayDateRangePicker`, `StayDateRangeField`, `StayOccupancySelector`, `StayMoneyBreakdown`, `StayConfirmSheet`, `StayTypeCover`, `StayTypeGallery`. Reuse the existing `Merchant*` components in `packages/merchant/src/components` before writing new ones.
- Idempotency keys: `newIdempotencyKey` from `crm/records/utils/newKey.ts`. File share: `crm/records/utils/shareFile.ts`.
- Access: wrap screens in `StayAccessGate`; read the selected operator and property from `useStayContext`.

## Registering your screens

- Create `<feature>/stayScreens.<feature>.ts` exporting `STAY_<FEATURE>_SCREENS: StayScreen[]` (`{ name, component, section, tabBar? }`), as in `home/stayScreens.home.ts`.
- Route names for the main sections are fixed in `navigation/staySections.ts` (`StayReservations`, `StayCalendar`, `StayTasks`, `StayListings`, `StayTemplates`, `StayRates`, `StayCredits`, `StayCases`, `StayReviews`, `StayTrips`). Detail screens use new names prefixed `Stay` and carry the `section` they belong to; use `tabBar: false` for detail screens.
- Do NOT edit `navigation/stayScreens.ts`, `staySections.ts`, `stayTabs.ts`, `merchantSections.ts`, `MerchantNavigator.tsx` or `packages/shared-i18n/index.ts`. The lead wires your area file and locale files at merge. Params other screens pass you: `StayReservations` gets `{ businessId, state }`, `StayCases` gets `{ businessId }`, `StayCredits` gets `{ businessId }`.

## Locales

Create `merchantStays<Feature>.json` in both `packages/shared-i18n/locales/en` and `fr` with identical keys. The top-level key of each file is your area (for example `listings`), so the final path is `merchant.stays.<area>.*`. Reuse `merchant.stays.shared.*` keys where they exist. Every user-facing string, including accessibility labels, goes through i18n. Do not edit `shared-i18n/index.ts`; list in your report the exact import lines the lead must add.

## Hard rules

- No comments anywhere in code.
- Components contain only JSX. State, effects, handlers, derived data, fetching, validation, formatting live in `use*` hooks next to the component (`hooks/` folder).
- Max 300 lines per file. Prefer a new file over a long one.
- No placeholder, stub or throwaway code. A screen is finished only when its API, permission and forbidden states, loading, empty and error states and its main workflow all work.
- Lists paginate on the server with `#` row numbers. Filters run on the server.
- Dates dd/mm/yyyy, CDF only, phone fields with a country selector and geo-detected default (never a bare phone input), selectors instead of typed ids, no raw radio/checkbox/select styling, animations on interactive UI.
- Phone design: body and table text 12px, labels and secondary 11px, page titles 15px, card figures 15px, nothing above 18px, tappable controls 36px or less, short-verb buttons become icon-only with an accessibility label, one column, sheets up to 92%, KPI cards 2x2.
- Touch only the folders you own and your own locale files. Never edit another agent's files.
- Do not run `cargo`, any build, or any dev server. Do not touch `tzone-rs` or `troq-front`. If an endpoint is missing or behaves differently from web, report it.
- Bun only. Verify with the mobile type check: from `troqzone-app/apps/mobile` run `bunx tsc -p tsconfig.check.json --noEmit`. That config only compiles files reachable from the app, so also type-check your folders with a temporary tsconfig that includes `packages/merchant/src/stays/**/*` (delete it afterwards). Run `bun test` on your utils from `packages/merchant` with a path filter (3 product-catalog tests already fail and are not yours). Other agents are editing sibling folders at the same time: ignore type errors that are clearly in their folders, and report only yours.

## Final report

Files created; route names and the exported area array; locale import lines for the lead; shared props other agents need; endpoints missing or different from web; screens not finished and why; the real type-check and test results. Do not claim visual verification.
