# F3 host app parity

Status: host reservations slice implemented on 26/11/2026, read-checked and type-checked only. Legacy reference is `troq-front/troqzone-business/src/components/stays/reservations` and `src/hooks/stays/useStayReservationsPage.ts`. Rust is not compiled by the agent.

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Reservations list with pagination | `StayReservationsPage.tsx`, `StayReservationList.tsx`, `useStayReservationsPage.ts` | `apps/host/src/views/reservations/ReservationsView.tsx`, `hooks/reservations/useHostReservations.tsx`, `services/reservations/reservations.api.ts` | done |
| Property, status and search filters | `StayReservationFilters.tsx` | same hook and view; property options come from `/api/host/properties`, status options from the state list | done |
| Date range filter | `StayReservationFilters.tsx` | `OperatorReservationQuery`, `operator_queries.rs`, `useHostReservations.tsx`, `ReservationsView.tsx` | open: inclusive check-in date filtering applies to server-paginated rows, totals and CSV export; URL state and bilingual labels added. Rust/SQL read-checked only; API contract regenerated; runtime and 375px checks remain |
| Status tabs with server counts and summary cards | `StayReservationTabs.tsx`, `StayReservationSummary.tsx` | status selector | open: no aggregate or count endpoint for the host scope |
| Credit strip | `components/stays/credits/StayCreditStrip.tsx` | none | open: billing credit exposure for the host app is not designed yet |
| Accept and decline requests | `StayReservationRowActions.tsx` | drawer actions calling `/decision` with `expectedVersion` and server `allowedActions` | done |
| Check in, check out, no show | `StayReservationRowActions.tsx` | drawer actions calling `/check-in`, `/check-out`, `/no-show` | done |
| Cancel with reason | `StayReservationCancel.tsx`, `useStayReservationCancel.ts` | cancel dialog calling `/cancel` with a reason of at most 300 characters | done |
| Amend stay dates | `StayReservationAmend.tsx`, `useStayReservationAmend.ts` | date range dialog calling `/amendments` and reporting the proposal as sent | done |
| Accept or decline special request | `StayReservationDetailRow.tsx` | drawer actions calling `/special-request` | done |
| Detail fields: property, unit, guest contact, occupancy, nights, total, commission, net | `StayReservationDetailRow.tsx` | drawer facts, guest block and night breakdown | done |
| Assigned physical unit and room picker | `StayReservationRoomPicker.tsx` | `components/reservations/ReservationAssignment.tsx`, `hooks/reservations/useReservationAssignment.ts`, assignment and move routes | done: shows the assigned room, lists compatible room options, assigns a room or moves the stay from a chosen date; gated by `reservations.write` |
| Unit type image gallery | `StayTypeGallery.tsx` | `components/reservations/ReservationDrawer.tsx` | partial: the unit type cover from the reservation response is shown with the unit name as alt text; a full multi-photo gallery needs the typed unit summary in the generated contract |
| Guest messages and standalone inbox | reservation detail legacy link to messages | `apps/host/src/{views,hooks,services}/messages`, `components/reservations/ReservationMessages.tsx`, `GET /api/host/message-conversations`, reservation message routes | open: host-scoped paginated conversations, search, paginated threads and replies implemented; per-member unread counts and monotonic read markers implemented; API contract regenerated; migration application and verification pending |
| Export reservations | `StayReservationsFooter.tsx` | `apps/host/src/views/reservations/ReservationsView.tsx` toolbar action, `hooks/reservations/useReservationsExport.ts`, `utils/csv/reservationsCsv.ts`, `services/reservations/reservations.api.ts` against `GET /api/host/reservations/export` | done: paged CSV with UTF-8 BOM, quoted fields, CRLF lines, dd/mm/yyyy dates, CDF money and translated state labels; gated by `analytics.export` |
| Permission gating | legacy permission checks in the page hook | module requires `reservations.read`; action buttons use server `allowedActions`; the message thread requires `messages.write` | done |
| Phone layout | legacy table rules | one column toolbar, hidden secondary columns, row title carries the state, drawer full width below `sm` | implemented, 375px runtime check not run |
| Seed states | `stay-service/seeds` reservations | existing `backend/seeds/reservations` rows cover requested, confirmed, checked in, checked out, cancelled and expired in Goma | done |

Limits recorded for the owner:

- `OperatorReservationView` has no property time zone, so `approvalDeadlineAt` and `createdAt` display as dd/mm/yyyy without a time. A later contract change should add the property time zone or a preformatted local timestamp.
- The list type-checks against the committed generated client and the running API returned the expected rows. The generated client was not regenerated in this slice.
- Runtime, console and 375px English/French checks were not run.
- The message thread was read against the running API for a seeded confirmed reservation with two messages; sending a message was not exercised.
- Room options were read for a confirmed stay needing a room (1 option) and a confirmed stay already assigned (1 alternative). Assignment and move writes were not exercised.
- The reservation CSV export was type-checked only. No file was downloaded and the export route was not called against the running API.
- The export pages with `pageSize` 100 because the kernel validates page sizes up to 100 (`MAX_PAGE_SIZE`); the slice brief asked for 200, which the server rejects.

## Calendar slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Calendar read model with unit types, nightly capacity and prices | `components/stays/calendar`, `hooks/stays` calendar hooks, `stay-service/src/calendar` | `apps/host/src/views/calendar/CalendarView.tsx`, `hooks/calendar/useHostCalendar.ts`, `services/calendar/calendar.api.ts` | done against `GET /api/host/calendar` |
| Strip view per unit type with open and full nights | `components/stays/calendar` strip view | `components/calendar/UnitStrip.tsx` | done |
| Month grid view with aggregated allocation, price range and a day details sheet | legacy calendar month view and day sheet | `components/calendar/CalendarMonthView.tsx`, `CalendarMonthGrid.tsx`, `CalendarMonthCell.tsx`, `CalendarDaySheet.tsx`, `hooks/calendar/useCalendarMonth.ts` | done: type-checked only, 375px and runtime not checked |
| Property and date range selection | calendar filters | toolbar in `CalendarView.tsx`, 93 night limit | done |
| Reservations in the period, opening the reservation detail | calendar reservation overlays | `components/calendar/CalendarReservationList.tsx` opens the shared reservation drawer | done |
| Block list with remove | calendar blocks | `components/calendar/CalendarBlockList.tsx`, `DELETE /api/host/blocks/{id}` | done |
| Create a block with unit type, dates and reason | calendar block dialog | `components/calendar/BlockDatesDialog.tsx`, `POST /api/host/blocks` | done; button requires `inventory.write` |
| Restrictions in the period | calendar restriction overlays | `components/calendar/CalendarRestrictionList.tsx`, `RestrictionDialog.tsx`, `hooks/calendar/useCalendarRestrictions.ts`, `POST /api/host/restrictions/bulk` | done: add minimum/maximum nights, advance notice, horizon, turnover buffer, closed-to-arrival and closed-to-departure rules for all or one unit type, with optional dates and removal; gated by `inventory.write` |
| Drag to move and day decision actions | calendar drag and sheet flows | `hooks/calendar/useCalendarDayActions.ts`, `components/calendar/CalendarDaySheet.tsx` | open: the day sheet can create one-night blocks and restrictions, or remove existing ones with confirmation; reservation actions remain in the reservation drawer and drag-to-move remains open. Type-checked only; write and phone verification pending |
| ICS feed tokens | legacy calendar feed settings | `components/calendar/CalendarFeeds.tsx`, `hooks/calendar/useCalendarFeeds.ts`, `services/calendar/feeds.api.ts` | open: permission-gated paginated list, create and one-time link, copy and revoke implemented; existing active and revoked seed tokens; writes, runtime and phone checks not run |
| Phone layout | legacy calendar shell | horizontal strip scrolls inside its section, toolbar stacks, dialogs are full width | implemented, 375px runtime check not run |
| Seed data | `stay-service/seeds` calendar rows | Goma property with units, reservation, restrictions parsed from `/api/host/calendar` | done |

Calendar limits:

- The running API returned 5 unit types, 14 nights per unit, one reservation and 6 restrictions for the demo property, in `Africa/Lubumbashi`.
- Block creation and removal were not exercised, so the demo data is unchanged.
- Restriction add and remove were not exercised; the bulk route and validation were type-checked only.
- The month grid and the day details sheet are type-checked only. 375px in English and French, dark mode, the console and runtime behaviour were not checked.
- Month totals and the day sheet aggregate the unit-type page returned by the API, which is 10 unit types per page. The unit pager is shown in month view as well, so paging changes the visible totals.
- Days outside the selected date range are shown muted with no allocation or price, and are not clickable.

## Credits slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Credit balance and account state | `components/stays/credits/StayCreditStrip.tsx`, `src/hooks/stays` credits | `apps/host/src/views/credits/CreditsView.tsx`, `hooks/credits/useHostCredits.tsx` against `GET /api/host/credits` | done: balance, commission rate, enabled or low state and low threshold |
| Credit ledger with pagination | legacy credits page | `PagedTable` over `GET /api/host/credits/ledger` | done: date, type, amount, balance after and note or reservation reference |
| Entry type labels | legacy credit labels | `packages/i18n/locales/{en,fr}/hostCredits.ts` | done for top-up, commission, refund and adjustment |
| Commission settlement screens | legacy credits and commission areas | none | open |
| Payout details | legacy host settings | none | open |
| Phone layout | legacy credit strip and list | two KPI cards, table keeps type and amount on phone, date and balance hidden below `sm` | implemented, 375px runtime check not run |

Credits limits:

- The host summary and ledger were requested against the running API for the demo host: balance 802150.00 CDF, rate 12.50%, 18 ledger rows.
- Date times use the active city time zone from `/api/public/cities`. The ledger response itself carries no time zone.

## Notifications slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Host notification inbox with server pagination | legacy notification center | `apps/host/src/views/notifications/NotificationsView.tsx`, `hooks/notifications/useHostNotifications.ts`, `GET /api/host/notifications` | done: all and unread filters, unread count, mark one or all read |
| Notification rows with type, priority, city-time timestamp and read state | legacy notification rows | `components/notifications/NotificationRow.tsx` | done; type and priority codes fall back to the raw code and a muted tone when unknown |
| Deep link navigation | legacy notification targets | hook follows internal links only, after marking read | done; external and protocol-relative links are ignored |
| Phone layout | legacy notification center | paginated rows, icon-only mark-all below `sm`, controls 32px | implemented, 375px runtime check not run |

Notifications limits:

- Unread count and read-all return only a 200 response in the contract, so the client treats missing data as an internal error.
- Backend unread count expiry filtering awaits owner compilation and live verification.
- No live notification request or 375px check was run.

## Rates slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Rate plan list per property | `components/stays/rates`, legacy `rate_admin` | `apps/host/src/views/rates/RatesView.tsx`, `hooks/rates/useRatePlans.tsx`, `GET /api/host/properties/{propertyId}/rate-plans` | done |
| Create a rate plan with unit type, occupancy basis, meal plan, booking mode, cancellation and payment policy, child amount | legacy rate plan form | `components/rates/RatePlanDialog.tsx`, `POST /api/host/properties/{propertyId}/rate-plans` | done; policy versions come from `/api/host/rate-policies` |
| Edit plan status, booking mode, meal plan and child amount | legacy rate plan editor | `components/rates/RatePlanDialog.tsx`, `PATCH .../rate-plans/{ratePlanId}` | done |
| Nightly rules list, create and remove | legacy rate rules | `hooks/rates/useRateRules.tsx`, `components/rates/RateRuleDialog.tsx`, `.../rules` routes | done for nightly price, amount and percentage adjustments; no bulk editor |
| Promotions list and create with minimum nights, discount, cap, dates, priority and redemption limit | legacy promotions | `hooks/rates/usePromotions.tsx`, `components/rates/PromotionDialog.tsx`, `.../promotions` routes | done |
| Pause and activate a promotion | legacy promotion status | `PATCH .../promotions/{promotionId}` | done |
| Policy version picker | legacy policy selection | `GET /api/host/rate-policies` | done, read only |
| Price preview for a stay | `StayPricePreviewPanel.tsx`, `useStayPricePreview.ts` | `components/rates/PricePreview.tsx`, `hooks/rates/usePricePreview.ts`, `POST /api/host/price-preview` | open: selected plan, dates, guest counts, subtotal, discount, total and nightly rule explanations implemented; existing pricing seeds cover plans and rules. Host and i18n typechecks passed; runtime and 375px EN/FR checks remain |
| Promotion funding funded by admin | legacy admin controls | none | open, platform side |
| Phone layout | legacy rate screens | stacked toolbar, plans and promotions tables hide secondary columns below `sm`, dialogs full width | implemented, 375px runtime check not run |
| Permissions | legacy `STAY_RATES_*` | module requires `properties.read`; create, edit and rule buttons require `pricing.write`; promotion actions require `pricing.campaigns` | done |

Rates limits:

- Read requests were verified against the running API for the demo property: 7 rate plans, 3 cancellation policies, 5 unit types and 3 promotions.
- Plan creation, plan update, rule creation/removal and promotion writes were not exercised, so no demo pricing data changed.
- Unit type names come from the unit type response, not from a typed ID.

## Settings slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Account facts: name, email, phone and home city | legacy business settings account block | `apps/host/src/views/settings/SettingsView.tsx`, `hooks/settings/useHostSettings.ts` from the session | done, read only |
| Notification preferences per host event with in-app, email and push switches | legacy notification settings | `GET` and `PUT /api/host/notification-preferences`, autosave and notice | done |
| Guest contact email and phone for the host | `StayBusinessProfileCard.tsx`, host contact update | `GET` and `PATCH /api/host/hosts/{hostId}/contact`, `components/settings/HostContactSection.tsx`, `hooks/settings/useHostContact.ts` | open: scoped contact read and settings form implemented, API contract regenerated; runtime and phone checks pending |
| Account name, password and language editing | legacy account settings | `apps/host/src/views/settings/SettingsView.tsx`, `hooks/settings/useHostSettings.ts`, `PATCH /api/guest/profile` | partial: first name, last name and preferred language are editable; email, phone and password editing remain open |
| Phone layout | legacy settings sections | one-column account rows, switches wrap below the event label | implemented, 375px runtime check not run |

Settings limits:

- The running API returned two host preferences: reservation held and review submitted. A no-op write of the reservation-held preference with the current values succeeded.
- A no-op profile update with the current name and language values succeeded, so the account edit route is verified without changing data.
- Guest contact settings load the selected host before allowing edits; the GET client uses the regenerated API contract. Existing `backend/seeds/identity/02-host-contact.sql` supplies contact examples.

## Reviews slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Property review list with rating, unit, comment, date and status | `components/stays/reviews`, legacy host review screens | `apps/host/src/views/reviews/ReviewsView.tsx`, `hooks/reviews/useHostReviews.tsx`, `GET /api/host/properties/{propertyId}/reviews` | done |
| Status filter for published, pending and hidden reviews | legacy review filters | status selector in the hook | done |
| Reply to a published review | legacy host reply | `components/reviews/ReviewReplyDialog.tsx`, `POST /api/host/reviews/{reviewId}/reply` | done; only `APPROVED` reviews offer the action |
| Review moderation | platform admin | none | open, platform side |
| Guest name on the review | legacy row | `reviews/host_list.rs`, `hooks/reviews/useHostReviews.tsx` | open: the host-only response includes the primary guest name after membership and property-scoped review permission checks; anonymized or missing names use a translated fallback. Existing seeds cover named guests and `reviews/03-review-anonymized.sql` covers anonymization; Rust and SQL read-checked only, contract regeneration and verification pending |
| Phone layout | legacy review list | rating and comment stay visible, date, status and reply state hide below `sm`, reply opens a full-width dialog | implemented, 375px runtime check not run |
| Permissions | legacy `STAY_REVIEWS_*` | module requires `reviews.write`; reply is checked by the server | done |

Reviews limits:

- The running API returned 6 reviews for Lakeside Villa, covering approved, pending and hidden states with and without comments.
- Replying was not exercised, so no review data changed.

## Property management slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Property list | `components/stays/properties`, host properties page | `apps/host/src/views/properties/PropertiesView.tsx`, `hooks/properties/useProperties.tsx` | done, server paginated and searchable |
| Create a draft property with bilingual name, address, descriptions and city time zone | legacy property create wizard | `components/properties/PropertyCreateDialog.tsx`, `POST /api/host/properties` | done; the time zone comes from the active city, never typed |
| Property facts drawer with status, city, address, unit counts, photos, revision, open stays, pending requests and guest contact | legacy property detail | `components/properties/PropertyDrawer.tsx` | done |
| Publish, pause and archive | legacy property lifecycle | lifecycle actions calling `POST /api/host/properties/{propertyId}/lifecycle` with confirmation | done; publish shows readiness blockers returned by the server |
| Property content and revision editor | legacy property wizard steps | `components/properties/PropertyRevisions.tsx`, `RevisionEditorDialog.tsx`, `hooks/properties/usePropertyRevisions.ts`, `useRevisionEditor.ts` | done: revision list, create from current content, edit bilingual names, descriptions, location and coordinates, arrival times, house rules, policies, guest contacts and amenities, save draft, submit for review and withdraw; readiness blockers are shown |
| Media upload and ordering | legacy property media | `components/properties/PropertyPhotos.tsx`, `hooks/properties/usePropertyMedia.ts`, host media routes | done: image upload with bilingual alt text, optional cover on upload, set cover, move up and down, remove; missing storage objects render a filename placeholder. the regenerated `PropertyMediaView` is used and image URLs come from the media content endpoint |
| Unit type create, update and archive | legacy property units | `components/properties/PropertyUnits.tsx`, `hooks/properties/usePropertyUnits.ts`, host unit-type routes | done: bilingual name, category, inventory model, occupancy, room counts, sellable quantity and descriptions; archive is refused when reservations exist |
| Admin content moderation | legacy admin listing queue | none in the admin app | open: without it a new draft cannot reach `APPROVED`, so publish is only usable for already-approved properties |
| Phone layout | legacy property screens | toolbar stacks, list hides secondary columns, drawer and dialogs are full width | implemented, 375px runtime check not run |

Property limits:

- Property creation, publish, pause and archive were not exercised, so seeded properties are unchanged.
- The list and property payloads were already read against the running API by the rates and reviews screens.
- Unit type list and category reads returned 5 unit types and the active catalog. Unit type writes were not exercised.
- Media list returned 3 rows: a ready cover, a ready photo and a rejected PDF, all without storage URLs, which exercises the placeholder path. Media writes were not exercised.
- The revision list returned one approved revision, and its detail returned the full listing content. Revision create, save, submit and withdraw were not exercised.







Host message unread state uses a per-member marker through the last displayed message, ordered by creation time and ID. Earlier pages do not mark later messages read. `backend/migrations/00032_message_read_markers.sql` and `backend/seeds/reservations/10-message-read-markers.sql` are read-checked only and not applied.

## Host analytics report

| Legacy capability | Legacy location | Mungini location | Status |
|---|---|---|---|
| Reservation counts, gross booking value and review rating | `troqzone-business/src/hooks/stays/useStayReservationsSummary.ts` | `apps/host/src/{views,hooks,services}/reports`, `GET /api/host/analytics/summary` | done: host-scoped aggregates, city and check-in range, bilingual labels and URL state; a host property selector narrows the report and is carried in the URL. Existing Goma and Kinshasa reservation and review seeds provide data. Runtime and 375px English/French checks remain. |

## Operations tasks

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Task list per property with pagination | `components/stays/tasks/StayTasksPage.tsx`, `hooks/stays/useStayTasksPage.ts` | `apps/host/src/views/tasks/TasksView.tsx`, `hooks/tasks/useHostTasks.tsx`, `GET /api/host/tasks` | done against the contract; runtime not checked |
| Property selector and state and type filters | `StayTaskFilters.tsx` | hook URL state and `Select` controls | done; task types are limited to the member's operations permissions |
| Create a task with type, priority and checklist | `StayTaskCreateSheet.tsx` | `components/tasks/TaskCreateDialog.tsx`, `hooks/tasks/useTaskCreate.ts`, `POST /api/host/tasks` | done; property comes from the API-driven selector |
| Start, complete, cancel, assign to me and unassign | `StayTaskActions.tsx`, `utils/stays/taskActions.ts` | `components/tasks/TaskRowActions.tsx`, `POST /api/host/tasks/{taskId}/transition` | done; actions follow the backend state machine and row version |
| Row number, type, state dot, priority, checklist count, assignee, city-time date | `StayTaskList.tsx`, `utils/stays/taskRows.ts` | table columns in `useHostTasks.tsx` | done; secondary columns hide below `sm` |
| Permission gating | legacy `STAY_MANAGE_HOUSEKEEPING` and `STAY_MANAGE_MAINTENANCE` | registry any-of `operations.checkin`, `operations.housekeeping`, `operations.maintenance`; row actions need the same server permission | done; `properties.read` added to the housekeeping preset so the property selector is reachable |
| Seed states | `stay-service/seeds` tasks | `backend/seeds/reservations/05-tasks-goma.sql`, `04-cases-tasks-kin.sql` | done; every state, priority and task type is represented, assigned and unassigned |
| Phone layout | legacy tasks table | row keeps number, type, state and priority; controls 32px and icon-only below `sm` | implemented, 375px runtime check not run |

Tasks limits:

- Type-checked only. No live request, mutation, 375px, dark-mode or console check was run.
- A housekeeping-only member needs `properties.read` for the property selector; the preset now includes it, but the owner must compile and reseed before it applies to existing members.

## Accommodation templates

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Template list with search, status and category filters | `components/stays/templates/StayTemplatesPage.tsx`, `hooks/stays/useStayTemplatesPage.ts` | `apps/host/src/views/templates/TemplatesView.tsx`, `hooks/templates/useHostTemplates.tsx`, `GET /api/host/accommodation-templates` | done against the contract; runtime not checked |
| Create and edit a template with bilingual content, category, model, occupancy, amounts and status | `StayTemplateSheet.tsx`, `hooks/stays/useStayTemplateEditor.ts` | `components/templates/TemplateFormDialog.tsx`, `TemplateFormFields.tsx`, `hooks/templates/useTemplateEditor.ts` | done; update sends `expectedVersion`, accessibility values are preserved |
| Template photos with bilingual alt text, cover and ordering | `StayTemplateImages.tsx`, `hooks/stays/useStayTemplateMedia.ts` | `components/templates/TemplateMediaSection.tsx`, `hooks/templates/useTemplateMedia.ts`, template media routes and content endpoint | done; failed content requests fall back to a filename block |
| Category options from the API | legacy category catalog | `listCategories` from `services/properties/properties.api.ts` | done, no hardcoded category codes |
| Permission gating | legacy `STAY_MANAGE_PROPERTIES` | module requires `properties.read`; write controls require `properties.configure` | done |
| Phone layout | legacy template list | list keeps number, name, amount and status; secondary columns hide below `sm` | implemented, 375px runtime check not run |

Templates limits:

- Type-checked only. No live request, mutation, 375px, dark-mode or console check was run.
- Runtime and write paths are unverified; template seeds already exist in `backend/seeds/properties/06-templates.sql` for Goma and Kinshasa, including archived rows.

## Cases

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Host case list with state filter, reference, reason, deadline and state | `components/stays/cases/StayCasesPage.tsx`, `StayCaseList.tsx`, `hooks/stays/useStayCasesPage.ts` | `apps/host/src/views/cases/CasesView.tsx`, `hooks/cases/useHostCases.tsx`, `GET /api/host/cases` | done against the contract; runtime not checked |
| Open a case from a reservation with type, reason and optional claim amount | `StayCaseOpenSheet.tsx`, `hooks/stays/useStayCaseOpen.ts` | `components/cases/CaseOpenDialog.tsx`, `hooks/cases/useOpenCase.ts`, `POST /api/host/cases` | done; reservation is API-driven by reference |
| Case detail with reason, deadlines, resolution and state | `StayCaseDetailSheet.tsx`, `hooks/stays/useStayCaseDetail.ts` | `components/cases/CaseDetailDrawer.tsx`, `hooks/cases/useCaseDetail.ts` | done; resolution is rendered as text from the stored JSON string |
| Close an open case with a required resolution | `StayCaseDetailSheet.tsx` | `components/cases/CaseCloseDialog.tsx`, `POST /api/host/cases/{caseId}/close` | done with expected version |
| Evidence list, preview and upload | `StayCaseDetailSheet.tsx`, legacy evidence helpers | `components/cases/CaseEvidenceList.tsx`, `CaseEvidencePreview.tsx`, `hooks/cases/useCaseEvidence.ts`, host evidence routes | done for images and PDFs; missing content falls back to the file name |
| Permission gating | legacy `STAY_MANAGE_CASES` | read needs `reservations.read`; open, close and evidence need host-scoped `cases.write`. `cases.write` is now in the delegable host set and the owner, property manager and reservations desk presets | done in code; owner compile and reseed required |
| Phone layout | legacy case list | list keeps number, type, state and deadline; secondary columns hide below `sm` | implemented, 375px runtime check not run |
| Seed states | `stay-service/seeds` cases | `backend/seeds/reservations/03-cases-claims-goma.sql`, `04-cases-tasks-kin.sql` | done; open, awaiting response, resolved, closed, escalated, assigned and unassigned cases |

Cases limits:

- Type-checked only. No live request, mutation, evidence upload, 375px, dark-mode or console check was run.
- The host case list covers the host scope. The legacy "mine" tab is not ported because the Mungini host route is already scoped to the member's host and the guest cases screen covers the guest view.
- Host case rows do not show the claim amount because the current `CaseView` contract does not expose it; the amount is accepted on open only.
