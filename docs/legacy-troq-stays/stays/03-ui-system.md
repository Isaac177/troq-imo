# UI and interaction contract

Applies to admin, business, public Next.js and Expo. Read the feature-specific screen requirements together with this file.

## U01: Existing patterns and visual language

Inspect the live page or component source before selecting shared components. Existing business PortalLayout uses portal navigation, responsive container spacing and protected content. Existing mobile MarketplaceHome uses shared theme/state/UI and module navigation. Reuse verified patterns; do not copy existing inline component logic that violates current AGENTS.md.

Use current TROQ theme tokens, fonts, spacing and icon library. No invented hex palette, decorative card accent borders, eyebrow labels, nested cards or Unicode arrows. Neutral dividers separate sections. Status badges include text. Use the existing dark theme where supported, with deliberate contrast verification.

Public discovery is photo-led. Administrative pages are information-dense tables with consistent filter/action bars. Manager operations center on calendars, date-focused lists and reservation details. A new full-screen manager application is gated by D04; default proposal is the existing business workspace.

## U02: Shared primitives to verify or implement

| Primitive | Contract |
|---|---|
| Destination selector | Search city/neighborhood, selected label, loading/error/empty, keyboard selection, clear/reset |
| Stay date picker | Two dates, unavailable nights, minimum stay explanations, keyboard navigation, property-local calendar semantics |
| Occupancy selector | Rooms/adults/children and child ages, per-room allocation, capacity validation, styled steppers |
| Money breakdown | Nightly lines, discounts, mandatory fees, optional extras, total, due now/later/property, all CDF |
| Status/timeline | Separate reservation, payment and stay labels; actor/time/reason where authorized |
| Server table | URL-backed filters, sorting, page/pageSize, total, selection invalidation after filter changes |
| Confirmation panel | Consequence preview, exact amount/date, explicit action label, pending state and error recovery |
| Media editor | Processing/rejected state, ordering, cover, alt text, assignment, upload retry |
| Permission editor | Explicit scope, grouped grants, preset preview, inherited versus direct access |
| Calendar grid | Unit rows, date columns, sticky labels, virtualized range, accessible alternate list |

## U03: Screen state contract

Every screen specifies initial loading, refresh, loaded, empty, validation failure, service error, permission denied and stale data. Network-dependent mutation screens also specify submitting, ambiguous response and recovery.

Empty states distinguish no inventory configured, no filter matches and temporarily unavailable inventory. Use real calls to action authorized for the viewer. A missing permission must not look like an empty portfolio.

Never optimistically display confirmed booking, successful payment, issued refund or completed payout. On timeout show pending verification with a status lookup. Preserve idempotency context for retries. Repeated taps must not create duplicate actions.

## U04: Layout and interaction

- Public desktop: search bar, results grid/list with optional map; detail page gallery and sticky booking panel.
- Public mobile: compact destination/dates summary, filter sheet, full-screen calendar, bottom booking action respecting safe areas and keyboard.
- Admin: filterable table, side detail drawer for inspection, dedicated detail page for complex evidence/finance cases.
- Manager: property switcher, calendar/reservation navigation, right-side edit drawer on wide screens, full-screen edit on narrow screens.
- Multi-step editors save drafts server-side, indicate save status, recover after refresh, and show unsaved-change warnings only for actual unsaved changes.
- Drag interactions require keyboard/tap alternatives. Bulk calendar changes require range summary and conflict preview before commit.
- Apply 200-300 ms ease-in-out transitions; honor reduced motion. Preserve scroll/filter/search state on back navigation.

## U05: Localization and accessibility

All touched interface strings go through shared-i18n en/fr, including errors, status names, placeholders, empty states, validation and notification templates. Persist stable machine codes, never localized status text. Localized property content has explicit missing-translation handling.

Display dates dd/mm/yyyy. Deadline labels include property timezone when ambiguity is possible. Phone inputs use international country selection and geo-detected default with manual override. Currency formatting always means CDF.

Provide visible labels, field-linked errors, focus restoration, logical keyboard order, screen-reader status announcements, adequate touch targets and contrast. Do not encode availability or task state with color alone. Map functionality needs an equivalent list.

## U06: Implementation and verification

Components contain JSX and call custom hooks; all state, effects, labels, formatting, handlers and fetching live in adjacent hooks or the existing feature hooks directory. No code file exceeds 300 lines. Shared contracts belong in the appropriate packages, not copied per app.

Verify narrow phone, tablet and desktop; English/French; light/dark where supported; keyboard and screen reader; long names; slow network and offline refresh. Screenshots are evidence, not a substitute for functional tests.

Receipt/voucher/invoice layouts need separate readable print/download output, localized content and only authorized fields. A receipt and a legally regulated invoice are not interchangeable; invoice rules depend on D11.
