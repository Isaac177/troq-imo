# Admin stays: remaining work

Goal: the admin app (`troq-front/troqzone-admin`) must offer everything a stay manager has in the business app (`troq-front/troqzone-business`), across all businesses, plus platform-only tools. Admins are the platform managers, so every admin screen is cross-business, filterable by business, city and property, and every action is audited.

## Already in admin

- Listings: review queue and properties tab, approve (publishes the property), request changes, reject, suspend, reinstate.
- Property page: details, guests, amenities, photos, revision history.
- Create property for a chosen business.
- Businesses list with search and status.
- Audit log, presets, amenity and amenity-group catalogs, accommodation categories.

## Rules every item follows

Project rules come from `AGENTS.md`. UI rules come from `DESIGN.md`. The business stays pages are the reference implementation, so read the matching business page and reuse the admin equivalents before writing anything new.

### Structure
- Page component has JSX only. State, effects, handlers, formatting and fetching live in a hook under `hooks/admin-stays/` (rule 17).
- Max 300 lines per file. Split into a page, a filters component, a table component, a footer and a hook.
- Reuse `StayAdminPanel`, `StayAdminPager`, `StayAdminStatus`, `StayAdminSection`, `StayAdminDialog`, `StayAdminConfirm` and `useStayAdminSearch` in `components/admin/stays/`. Do not create parallel versions.
- Register the route in `routes/admin/stays.ts` and the nav entry in `components/admin/sidebar/sidebar-data.ts` under `admin.sidebar.stays`.
- No comments in code.

### Data
- Lists paginate on the server: `page` and `pageSize`, SQL `LIMIT`/`OFFSET`, return `total`, filters in SQL. Totals and dashboard figures come from SQL aggregates (rule 7).
- Backend work goes in `stay-service` and `api-gateway` (`routes/admin/stays.rs`, `routes/admin/stay_team.rs`). Schema changes are a new numbered migration in `tzone-rs/stay-service/migrations/`. Do not run `cargo`.
- Money is CDF only. Dates are `dd/mm/yyyy`, timestamps `dd/mm/yyyy HH:mm` in the stay city time zone (`Africa/Lubumbashi` for Goma, `Africa/Kinshasa` for Kinshasa) through `utils/admin-stays/stayDateTime.ts`. Never `toLocaleString`.
- Never make a user type an id or slug. Business, property, operator, user and room fields are selectors fed by the API (`StayCombobox`, rule 21).
- Seed realistic demo data for each feature, ids prefixed `seed-`, covering every status and edge case (rule 9).

### UI (from DESIGN.md)
- Panel: `rounded-card`, 1px `border`, `muted/40` header strip, white body with `-mx-px -mb-px overflow-hidden` so the line curves at both ends. No cards inside cards, no accent bars, no colored card borders, no eyebrow labels, no unicode arrows (use the icon library).
- Table: `#` column first and right aligned, continuing across pages. Phone keeps `#`, title and main figure; secondary columns use `hidden sm:table-cell` or `hidden md:table-cell`. Rows are clickable with a trailing chevron. Footer row holds the total on the left and icon-only previous and next on the right. Loading dims the table to `opacity-60`. Empty and error states are centered in the body.
- Toolbar: search with leading icon and an icon-only button below `sm`, status filter as 32px tabs on the right, clear button only while a filter is active, and the refresh icon (ghost, `size-4` icon, spins while fetching, `aria-label` and `title`) on every list.
- Status is a dot plus label, never a filled pill: draft and archived grey, approved `sky-500`, published `success`, paused `amber-500`, suspended and error `destructive`.
- Text tones by role: identity primary, place `sky`, counts `violet`, people `teal`, attention `amber`, problem `destructive`. One tone per piece of text.
- Type from tokens only: `text-micro`, `text-label`, `text-body`, `text-title`. Weights 400, 500, 600. `tabular-nums` on numbers and dates.
- Controls are 36px (`h-9`) or less, small icon buttons `size-8`. Verb buttons become icon-only below `sm`. Use the design system `Input`, `Select`, `Toggle`, `Textarea`. No raw radio, checkbox or select. Phones use the international phone input with country selector.
- Destructive actions use the destructive button behind a confirm panel.
- Everything interactive transitions with `duration-200` to `duration-300` and `ease-in-out`.
- Layout: `space-y-4 md:space-y-6`, page padding 16px phone and 32px from `md`, editors `max-w-5xl`.
- Every string through i18n in English and French (admin locales in `troqzone-admin/src/i18n/locales/{en,fr}/admin/`, new keys under `admin.stays.*`).
- Check each screen at 375px in English and French, and say which screens were checked.
- Verify with `bunx tsc -p tsconfig.app.json --noEmit` in the admin app. No builds.

## Work items

Order is by value and effort. Items 1 to 3 first.

### 1. Review moderation
- Backend exists: `/stays/reviews/moderation-queue` and `/stays/reviews/{reviewId}/moderation`.
- Add page `/stays/reviews`: table with `#`, guest, property (place tone), rating, comment excerpt, submitted date, status dot. Filters: status, property, rating, search.
- Row opens a sheet with the full review, host reply, and actions hide, restore, flag, each with a reason selector.
- Nav entry `admin.sidebar.stays.reviews`.
- Seed: pending, visible, hidden, flagged, with and without host reply.

### 2. Property lifecycle on admin
- Add publish, pause and archive to the property page header next to suspend and reinstate, using the existing `lifecycle` rules in `stay-service/src/listings/lifecycle_ops.rs`. Publish re-runs readiness and shows its blockers. Archive is blocked while upcoming stays exist and shows the count.
- Add admin routes in `routes/admin/stays.rs`, audit each action, confirm panel for archive.
- Show the action set per current status only.

### 3. Business detail page
- Route `/stays/operators/$operatorId`, opened from the businesses table row.
- Sections: summary figures (properties, open reservations, pending requests, open cases, 30 day revenue in CDF), properties table, team and permissions, recent reservations, cases, documents.
- Approve, suspend and reinstate with a reason selector and confirm panel. Show suspension history.
- KPI cards two by two on phone. Figures from SQL aggregates.

### 4. Unit types and rooms on the property page
- New tab `rooms` on `AdminStayPropertyPage` mirroring the business unit-types editor: unit types (name, capacity, beds, base price in CDF, amenities) and rooms inside each type.
- Admin endpoints for list, create, update, archive. Respect room assignments and upcoming stays when archiving.
- Photos per unit type reuse `StayPhotosManager`.

### 5. Reservations across all businesses
- Page `/stays/reservations`: table with `#`, reference (identity tone), guest (people), property and unit type (place), check-in and check-out (`dd/mm/yyyy`), nights (count), total in CDF, status dot. Filters: status, business, property, city, date range, search. Export.
- Row opens a detail sheet: guest, payment state, timeline, amendments, case links.
- Actions: cancel with cancellation preview, amend, assign or move room, resend confirmation. Platform reasons are selectors.
- New admin endpoints in `stay-service` and `api-gateway`, paginated in SQL, with summary aggregates for the header figures.

### 6. Calendar and availability for any property
- Page `/stays/calendar` with a business and property picker first, then the availability grid, blocks, inventory defaults, bulk restrictions, room moves and calendar feeds, matching the business calendar.
- Admin can block dates with reason `PLATFORM` so a manager sees why.
- Reuse the grid logic from the business calendar by moving shared code into a shared package rather than copying.

### 7. Rates
- Page `/stays/rates` with a property picker: rate plans with rules, promotions, policies, price preview.
- Admin can edit any plan and see which promotions are platform funded. Audit every change.

### 8. Cases
- Page `/stays/cases`: all cases across businesses. Columns: `#`, reference, type, business, property, opened date, age (attention tone when overdue), status dot.
- Detail sheet with evidence, timeline, assign to platform user (selector from `/stays/users`), escalate, close with outcome.
- Link to the related reservation.

### 9. Damage claims
- Backend schema exists in migration 00026. Add a claims tab inside cases or a page `/stays/damage-claims`: claim amount in CDF, evidence, host and guest statements, decision (approve, partial, reject) with reason, payout state.
- Check `api-gateway/src/routes/admin/payment_claims.rs` before adding endpoints, to avoid duplicating the payment side.

### 10. Tasks
- Page `/stays/tasks`: housekeeping and operations tasks across businesses. Filters: status, business, property, assignee, due date. Create and transition with the same states as business.

### 11. Accommodation types (templates)
- Page `/stays/accommodation-types`: list templates per business and the platform catalog. Create, edit, retire. Pricing is CDF. Presets can seed templates.

### 12. Trips and guest bookings
- Page `/stays/trips`: search by guest, reference or phone. Support actions on behalf of the guest: cancel, resend, open a case. Phone fields use the international input.

### 13. Platform dashboard
- Landing view for stays: bookings, revenue in CDF, occupancy, cancellations, commission and payouts, by city and by business, over a date range.
- All figures from SQL aggregates through one admin summary endpoint. KPI cards two by two on phone. Charts follow the neutral palette and the status colors only.

### 14. Platform controls
- Settings page: commission rate, service fees, default cancellation policies, review windows, per city options.
- Values stored per city database, changes audited, forward-only migrations for new settings tables with seeded defaults.
- Built: default commission rate, quote hold, request approval window, amendment response window, damage claim window, case response time (page `/stays/settings`, table `stay_platform_settings`).
- Not built: service fees (guests pay at the property, no platform fee path exists), default cancellation policies (policies live per rate plan, there is no default concept), review windows (reviews have no window rule to configure), other per-city options.

## Definition of done per item

- Backend endpoints added and read through for correctness (no cargo). List the services that changed and any manual step, since deploys are the owner's job.
- Page, hook, filters, table, footer split into files under 300 lines.
- i18n in English and French, no hardcoded text.
- Seed rows added with `seed-` ids and every status covered.
- `bunx tsc` clean in the admin app.
- Checked at 375px in English and French, with the screens named in the report.
