# Mungini UI design rules

Status: draft for owner review, written 25/11/2026. Agents building any Mungini screen follow this file together with `MUNGINI_AGENT_RULES.md`.

Sources, read in full for this document:

- `/Users/jogues/Documents/troqbound/DESIGN.md`: structure, primitives, page archetypes, motion, i18n and data rules.
- `/Users/jogues/Documents/TROQ/DESIGN.md`: the TROQ business app design language, whose stays pages are the reference implementation and are what Mungini ports first.

The two documents agree on behaviour and disagree on look. Behaviour rules below are settled. The look is decision D11 in the migration plan, with a default.

## 1. Principles

1. Phone first. Hosts run their business from a phone, so every host screen is designed at 375px before desktop. The guest site and app are phone first too.
2. Neutral first, one brand colour. Surfaces are neutral. The brand colour appears only on the primary action, the active nav item, selected segments, focus rings and progress.
3. Add, never replace. Redesigning a screen keeps every existing capability, and new ideas are added around them. This is also what the parity gate checks.
4. One product. Permissions gate whether a module, route or action renders. Components never branch on role inside.
5. Everything is stateful in the URL: tab, layout, sort, search, filters and the open record survive a refresh.
6. Everything moves. Panels, drawers, dialogs, tab changes, row reordering and layout shifts ship with a transition.
7. No template tells: no eyebrow or kicker labels above titles, no coloured accent bars or borders on cards, toasts or emails, no unicode arrows as UI glyphs, no cards inside cards. The one allowed bar is the 2px active marker at the left of the active nav item. Section openings carry real information, not generic marketing lines.
8. Interactive states never rely on colour alone.

## 2. Look and feel (decision D11)

Default: keep the TROQ stays language for the ported screens, because the Stay screens already follow it and parity is the goal. Everything is expressed as semantic tokens in one theme file per client family, so a Mungini brand change is a one file edit and never a raw hex in a component.

| Aspect | TROQ stays language (default) | Troqbound language (alternative) |
|---|---|---|
| Accent | brand green, hsl 160 84% 22% light, hsl 160 75% 28% dark fills | lime `#c8ff70` with dark text on it |
| Typeface | Onest with a system stack | Plus Jakarta Sans |
| Surfaces | flat, panels with a tinted header strip and a curved body panel | flat cards with hairline borders |
| Status | 8px dot plus plain label | tone pill |
| Radii | cards 14px | cards and tables 12px, controls 8px, chips 6px |
| Control height | 36px or less | `h-9`, 13px text |

Whichever the owner picks, these hold:

- Dark mode through the `html.dark` class with the same token names. Never write `dark:` colour overrides in components.
- Token names: `background`, `card`, `popover`, `foreground`, `muted`, `muted-foreground`, `border`, `input`, `primary`, `primary-foreground`, `accent`, `destructive`, `ring`, plus status tones.
- Text sizes come only from global tokens and are never overridden in a page:

| Token | Phone | From `md` | Use |
|---|---|---|---|
| `text-micro` | 10px | 10px | tiny counters |
| `text-label` | 11px | 12px | second lines under a title, helper text |
| `text-body` | 12px | 13px | table cells, fields, buttons, nav, running text |
| `text-title` | 15px | 16px | panel titles, top bar page title |

- Weights: 400 body, 500 for buttons and names in tables, 600 for panel titles and the active nav item. Row numbers, dates and money use `tabular-nums`.
- Text tones give a row scannable roles (identity, place, count, people, attention, problem). Tones come from a helper file, never raw colour classes in a page. One tone per piece of text, no coloured rows, and fills stay reserved for status and primary actions.
- Tailwind v4 pitfall: `max-w-sm|md|lg|xl` can resolve to tiny values because of theme collisions. Use `w-*` or `max-w-[Npx]` and check the rendered width.

## 3. Layout, panels, tables

- Page padding 16px on phone, 32px from `md`. Page roots do not add their own padding on phone. Vertical rhythm 16px between blocks on phone, 24px from `md`. Panel padding 12px on phone, 20px from `md`. Editors cap at `max-w-5xl` and centre.
- Shell: fixed 240px sidebar from `lg`, an overlay sidebar on phone, top bar 52px (48px on phone), content scrolls under the bar. The active nav item scrolls into view on navigation.
- Standard page header stack, top to bottom: page header in the top bar, breadcrumb, page actions row (the primary action is last and is the only filled brand button), tabs with counts, toolbar (search, filter, layout toggle), content.
- Panel: one container for forms, tables and lists. An outer rounded border with a tinted header strip (title, one line description, actions) and a separate body panel pulled over the edge so the line under the header curves at both ends. Groups inside a body are separated by dividers or spacing, never by another card.
- Tables sit inside the panel body. Header row is tinted, normal case, no uppercase tags. The first column is always `#`, right aligned and continuing across pages (`(page - 1) * pageSize + index + 1`). The title cell shows the name at weight 500 with a muted second line. Rows are divider separated, hover tinted, and a clickable row opens the record and ends with a chevron icon.
- Phone tables keep `#`, the title and the main figure and hide secondary columns with `hidden sm:table-cell`. The page never scrolls sideways.
- Pagination is a footer row inside the body: total on the left, icon-only previous and next with the page indicator on the right. Pagination, sort, search, filters and counts run on the server. Loading dims the table instead of replacing it. Empty and error states render centred inside the body.
- Toolbar: search with a leading icon and 36px of left padding, a search button that is icon-only below `sm`, status tabs on the right (32px tall, the row scrolls inside itself on phone), a clear button only while a filter is active.

## 4. Controls and primitives

Build these once in `packages/ui-web` (and the matching ones in `packages/ui-native` for mobile) before screens use them. Never hand-roll or guess a component from its name; read the primitive first.

| Need | Primitive | Rule |
|---|---|---|
| Button | `Button` | variants primary, outline, ghost, secondary, destructive, link. Every control is 36px tall or less, icon buttons 32px. Destructive only for irreversible actions and always behind a confirm |
| Text, textarea | `Input`, `Textarea` | never a raw element |
| Select | `Select`, `SearchableSelect` | options always come from the API for any record reference (rule 21) |
| Date and time | `DatePicker`, `DateRangePicker`, `DateTimePicker` | output dd/mm/yyyy and dd/mm/yyyy HH:mm; booking flows capture time anchored to the property's time zone |
| Amount | `AmountInput` | CDF, shared money formatter |
| Phone | `PhoneField` | country selector, geo-detected default |
| Choice | `Checkbox`, `Switch`, `SegmentedControl` | never a raw radio or checkbox |
| Tabs | `UnderlineTabs` | active tab marked by a 2px underline in the brand colour |
| Status | `StatusDot` (default) or `TonePill` (per D11) | one status vocabulary per entity |
| Badge | `Badge` | counts and short tags |
| Empty, loading | `EmptyState`, `Skeleton` | skeletons are shaped like the content |
| Overlays | `Dialog`, `Popover`, `Command`, `Drawer`, `SlideUpDrawer` | bottom drawer is 92% height, 300ms translate |
| Menus | `ActionMenu`, `RowContextMenu` | the row menu and right click offer the same actions |
| Confirm | `ConfirmDialog` | never `window.confirm`; `destructive` prop for delete |
| Bulk | `BulkActionBar` | appears when rows are selected |
| Table helpers | `PagedTable`, `Pagination`, `SortableHeader`, `Avatar` | server driven |
| Icon-only | `IconTooltipButton` | every icon-only button has `aria-label` and `title` |
| Motion | `FadeIn` | wrap content that swaps so there is never a blank frame |
| Panels | `Panel`, `Section` | header slot plus body, per section 3 |
| Toast | one toast component | a plain card with no coloured bar |
| Rich text | `RichTextEditor` | only for long descriptions and policies |

Do not add `window.confirm`, `alert`, raw `select`, `input type=checkbox` or `input type=radio` anywhere.

## 5. Page archetypes

Each archetype names a Mungini example to build first, so later pages copy it.

### 5.1 Settings page

Used for host profile, property settings, team, billing, notifications. An 820px column of sections, each a divided panel with rows: label and hint on the left, control on the right. Selects and switches autosave and toast. Multi-field blocks use an explicit Save. Locked features render locked in place with a badge, they do not disappear. A settings hub is a grid of grouped icon tiles, and a new settings page adds a tile and a route.

### 5.2 List page with grid

Reservations, properties, guests, claims, tasks, reviews. Header stack, status tabs with server-computed counts, toolbar (search, owner or scope segmented control, filters, layout toggle, an Edit view control, the primary New action).

- Layouts: table by default, cards, and board where the entity has a pipeline (reservation status, tasks). Every layout supports the same tabs, filters, sort and bulk actions.
- Grid: sticky `#` and checkbox column, sticky first data column, typed cells (avatar, status, date, money right aligned, relative due text), a footer with count and sums from the server, column widths and hidden columns persisted per grid in local storage with a versioned key.
- Edit view drawer docks right with no scrim: row height, column width, filter, sort, group, export. Sub-screens swap inside the drawer with a Back control.
- URL params: `tab`, `layout`, `q`, `sortBy`, `sortDir` and filter keys. Sort is server side against a whitelist of columns.
- On phone the grid becomes a list of rows with `#`, title and main figure, and the secondary data moves into the record page.

### 5.3 Record page

Reservation, property, host, case, claim. Two panes on desktop: a left aside with the record's facts (resizable, width persisted) and a right tab bar (Overview, Activity, record specific tabs with counts, Tasks, Documents). On phone the aside becomes the first section and tabs scroll inside their own row. Tabs persist in `?tab=`. Linked records are chips that deep link to the other record.

### 5.4 Wizard and create flow

Used for property creation, rate plans, promotions, reservations created by staff. One concern per step, a live preview that updates as you type where it helps, the last step is Review. Required-field gating disables Next, never hides it. Each step is its own component file. Guest booking is a dedicated full width savable page, never a modal.

### 5.5 Tasks and operations board

Housekeeping, maintenance, check-in and service tasks. Cards, board and table over the same data, grouped by due date or assignee, plain group headings with a muted count, drag to reschedule where it makes sense, completing a row fades it out after a short delay.

### 5.6 Calendar

Availability and reservation calendars are the heart of the host app. Strip and month views, selection bar, blocks, rate overlays, move and decision sheets. On phone use the strip view with bottom sheets.

### 5.7 Dashboards

A 12 column grid of metric, chart and list cards. KPI cards show label, value, delta and an optional bar, two by two on phone. Charts follow the shared chart conventions: one colour system, accessible in light and dark.

### 5.8 Messaging

Two panes: conversation list (All, Unread, search) and thread. Reference: the guest messages of a reservation.

### 5.9 Preview dialogs

In-app preview for documents (vouchers, cancellation notices, claim evidence) with no download and no new tab. Bytes arrive through the API, not a presigned URL. The modal fades and scales, closes on Esc and outside click.

## 6. Overlays and motion

- Dialogs: fade and scale, 150 to 200ms, Esc and outside click close, focus trapped.
- Right panel or drawer: translate-x 200 to 300ms. Inline drawers have no scrim. Modal drawers use a light scrim with a 1px blur.
- Bottom drawer: translate-y 300ms.
- Tab or layout change: cross-fade through `FadeIn`.
- List reorder: FLIP, about 300ms.
- Collapsibles and filters animate height and opacity.
- Hover and focus use `transition-colors`.
- Respect `prefers-reduced-motion` for large movement.

## 7. Internationalisation, dates, money

- Every user-facing string goes through i18n in English and French. A touched file with hardcoded text is converted immediately. Pages read copy from the feature's copy object returned by their hook, never from inline literals.
- Locale file format is decision D12 in the migration plan. The default is typed per-feature files in `packages/i18n`.
- Dates are dd/mm/yyyy. Timestamps are dd/mm/yyyy HH:mm in 24 hour time in the property's city time zone (for example `Africa/Lubumbashi` for Goma, `Africa/Kinshasa` for Kinshasa), never the device time zone. Use one shared date helper for each of the two formats and never `toLocaleString` or ad hoc formatting on a stored timestamp. Deadlines and expiries always show their time and take the attention tone.
- Money is CDF and goes through one shared money formatter. Check how Stay stores amounts on the wire before porting and keep that representation.
- Phone fields use `PhoneField` everywhere.

## 8. Data and state in the UI

- Tab, layout, search, sort and filters live in the URL and apply to every layout variant of a page.
- Pagination, sort, search and aggregates are server side. Never slice or sum rows in the client.
- Row grids use a shared `useGridPrefs(storageKey, columns)` hook for widths, hidden columns and selection.
- Deletes of host-owned records go through a soft delete with a Trash view that can restore them, where the entity allows it. Reservations, payments and ledger rows are never deleted, only cancelled or voided with a reason.
- Loading uses skeletons, errors use a toast plus an inline state, empty uses `EmptyState` with an action when one makes sense.
- Plan or permission locked features render locked in place with a badge.

## 9. Recipe: build a screen end to end

1. Find the sibling: the closest archetype above, and open its page, hook, components, API service, locale files and backend module.
2. Backend: migration, queries, service, validators, handler, permission constant, route registration under the right audience, OpenAPI entry. Then regenerate the API client.
3. Frontend: API service, types, hook, page, components, registry entry, route, nav label, locale files in English and French.
4. Seed realistic demo data covering each status and edge case.
5. Verify with `bunx tsc` in each workspace touched. For visual checks use the dev servers already running: phone width in English and French, dark mode, console clean, with few screenshots.
6. Report gaps honestly.

## 10. Page checklist

- [ ] Header stack: page header, breadcrumb, page actions, tabs, toolbar
- [ ] The primary action is the only filled brand button
- [ ] Every control is 36px or less, no raw browser controls
- [ ] Loading skeletons, empty state, error state
- [ ] A transition on every panel, tab and layout change
- [ ] Tab, layout, search and sort in the URL
- [ ] Server-side pagination, sort and counts, `#` column
- [ ] Dates dd/mm/yyyy, timestamps with time zone, phone through `PhoneField`
- [ ] Every string in English and French, nothing hardcoded
- [ ] No comments, no eyebrow labels, no accent bars, no arrow glyphs, no nested cards
- [ ] One hook per page, one component per file, types in `types/`
- [ ] Checked at 375px in English and French, and in dark mode
- [ ] Parity ledger row updated
