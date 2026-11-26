# F3 host app parity

Status: host reservations slice implemented on 26/11/2026, read-checked and type-checked only. Legacy reference is `troq-front/troqzone-business/src/components/stays/reservations` and `src/hooks/stays/useStayReservationsPage.ts`. Rust is not compiled by the agent.

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Reservations list with pagination | `StayReservationsPage.tsx`, `StayReservationList.tsx`, `useStayReservationsPage.ts` | `apps/host/src/views/reservations/ReservationsView.tsx`, `hooks/reservations/useHostReservations.tsx`, `services/reservations/reservations.api.ts` | done |
| Property, status and search filters | `StayReservationFilters.tsx` | same hook and view; property options come from `/api/host/properties`, status options from the state list | done |
| Date range filter | `StayReservationFilters.tsx` | none | open: the host reservation API has no date range parameters |
| Status tabs with server counts and summary cards | `StayReservationTabs.tsx`, `StayReservationSummary.tsx` | status selector | open: no aggregate or count endpoint for the host scope |
| Credit strip | `components/stays/credits/StayCreditStrip.tsx` | none | open: billing credit exposure for the host app is not designed yet |
| Accept and decline requests | `StayReservationRowActions.tsx` | drawer actions calling `/decision` with `expectedVersion` and server `allowedActions` | done |
| Check in, check out, no show | `StayReservationRowActions.tsx` | drawer actions calling `/check-in`, `/check-out`, `/no-show` | done |
| Cancel with reason | `StayReservationCancel.tsx`, `useStayReservationCancel.ts` | cancel dialog calling `/cancel` with a reason of at most 300 characters | done |
| Amend stay dates | `StayReservationAmend.tsx`, `useStayReservationAmend.ts` | date range dialog calling `/amendments` and reporting the proposal as sent | done |
| Accept or decline special request | `StayReservationDetailRow.tsx` | drawer actions calling `/special-request` | done |
| Detail fields: property, unit, guest contact, occupancy, nights, total, commission, net | `StayReservationDetailRow.tsx` | drawer facts, guest block and night breakdown | done |
| Assigned physical unit and room picker | `StayReservationRoomPicker.tsx` | `components/reservations/ReservationAssignment.tsx`, `hooks/reservations/useReservationAssignment.ts`, assignment and move routes | done: shows the assigned room, lists compatible room options, assigns a room or moves the stay from a chosen date; gated by `reservations.write` |
| Unit type image gallery | `StayTypeGallery.tsx` | `unitTypeCoverUrl` unused | open |
| Guest messages | reservation detail legacy link to messages | `components/reservations/ReservationMessages.tsx`, `hooks/reservations/useReservationMessages.ts`, `GET` and `POST /api/host/reservations/{id}/messages` | done in the reservation drawer: chronological thread, city-time stamps and a send box, gated by `messages.write`; a standalone inbox with unread counts remains open |
| Export reservations | `StayReservationsFooter.tsx` | none | open: `/api/host/reservations/export` exists but the UI and file format are not specified |
| Permission gating | legacy permission checks in the page hook | module requires `reservations.read`; action buttons use server `allowedActions`; the message thread requires `messages.write` | done |
| Phone layout | legacy table rules | one column toolbar, hidden secondary columns, row title carries the state, drawer full width below `sm` | implemented, 375px runtime check not run |
| Seed states | `stay-service/seeds` reservations | existing `backend/seeds/reservations` rows cover requested, confirmed, checked in, checked out, cancelled and expired in Goma | done |

Limits recorded for the owner:

- `OperatorReservationView` has no property time zone, so `approvalDeadlineAt` and `createdAt` display as dd/mm/yyyy without a time. A later contract change should add the property time zone or a preformatted local timestamp.
- The list type-checks against the committed generated client and the running API returned the expected rows. The generated client was not regenerated in this slice.
- Runtime, console and 375px English/French checks were not run.
- The message thread was read against the running API for a seeded confirmed reservation with two messages; sending a message was not exercised.
- Room options were read for a confirmed stay needing a room (1 option) and a confirmed stay already assigned (1 alternative). Assignment and move writes were not exercised.

## Calendar slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Calendar read model with unit types, nightly capacity and prices | `components/stays/calendar`, `hooks/stays` calendar hooks, `stay-service/src/calendar` | `apps/host/src/views/calendar/CalendarView.tsx`, `hooks/calendar/useHostCalendar.ts`, `services/calendar/calendar.api.ts` | done against `GET /api/host/calendar` |
| Strip view per unit type with open and full nights | `components/stays/calendar` strip view | `components/calendar/UnitStrip.tsx` | done on phone width; month grid view open |
| Property and date range selection | calendar filters | toolbar in `CalendarView.tsx`, 93 night limit | done |
| Reservations in the period, opening the reservation detail | calendar reservation overlays | `components/calendar/CalendarReservationList.tsx` opens the shared reservation drawer | done |
| Block list with remove | calendar blocks | `components/calendar/CalendarBlockList.tsx`, `DELETE /api/host/blocks/{id}` | done |
| Create a block with unit type, dates and reason | calendar block dialog | `components/calendar/BlockDatesDialog.tsx`, `POST /api/host/blocks` | done; button requires `inventory.write` |
| Restrictions in the period | calendar restriction overlays | `components/calendar/CalendarRestrictionList.tsx`, `RestrictionDialog.tsx`, `hooks/calendar/useCalendarRestrictions.ts`, `POST /api/host/restrictions/bulk` | done: add minimum/maximum nights, advance notice, horizon, turnover buffer, closed-to-arrival and closed-to-departure rules for all or one unit type, with optional dates and removal; gated by `inventory.write` |
| Drag to move, day decision sheet, month view | calendar drag and sheet flows | none | open |
| ICS feed tokens | legacy calendar feed settings | none | open, backend routes exist |
| Phone layout | legacy calendar shell | horizontal strip scrolls inside its section, toolbar stacks, dialogs are full width | implemented, 375px runtime check not run |
| Seed data | `stay-service/seeds` calendar rows | Goma property with units, reservation, restrictions parsed from `/api/host/calendar` | done |

Calendar limits:

- The running API returned 5 unit types, 14 nights per unit, one reservation and 6 restrictions for the demo property, in `Africa/Lubumbashi`.
- Block creation and removal were not exercised, so the demo data is unchanged.
- Restriction add and remove were not exercised; the bulk route and validation were type-checked only.

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
| Price preview for a stay | legacy price preview | none | open, the endpoint exists but this screen does not expose it |
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
| Guest contact email and phone for the host | `StayBusinessProfileCard.tsx`, host contact update | none on this screen | open: the backend `PATCH /api/host/hosts/{hostId}/contact` exists but no host-facing read endpoint exposes the current values |
| Account name, password and language editing | legacy account settings | `apps/host/src/views/settings/SettingsView.tsx`, `hooks/settings/useHostSettings.ts`, `PATCH /api/guest/profile` | partial: first name, last name and preferred language are editable; email, phone and password editing remain open |
| Phone layout | legacy settings sections | one-column account rows, switches wrap below the event label | implemented, 375px runtime check not run |

Settings limits:

- The running API returned two host preferences: reservation held and review submitted. A no-op write of the reservation-held preference with the current values succeeded.
- A no-op profile update with the current name and language values succeeded, so the account edit route is verified without changing data.
- The guest contact form was not built because a blank form would overwrite the current values without a read endpoint.

## Reviews slice

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Property review list with rating, unit, comment, date and status | `components/stays/reviews`, legacy host review screens | `apps/host/src/views/reviews/ReviewsView.tsx`, `hooks/reviews/useHostReviews.tsx`, `GET /api/host/properties/{propertyId}/reviews` | done |
| Status filter for published, pending and hidden reviews | legacy review filters | status selector in the hook | done |
| Reply to a published review | legacy host reply | `components/reviews/ReviewReplyDialog.tsx`, `POST /api/host/reviews/{reviewId}/reply` | done; only `APPROVED` reviews offer the action |
| Review moderation | platform admin | none | open, platform side |
| Guest name on the review | legacy row | none | open: the review contract has no guest or reservation field |
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
| Media upload and ordering | legacy property media | `components/properties/PropertyPhotos.tsx`, `hooks/properties/usePropertyMedia.ts`, host media routes | done: image upload with bilingual alt text, optional cover on upload, set cover, move up and down, remove; missing storage objects render a filename placeholder. Rust media view renamed to `PropertyMediaView` to clear the generated client collision, so the client uses a local type until regeneration |
| Unit type create, update and archive | legacy property units | `components/properties/PropertyUnits.tsx`, `hooks/properties/usePropertyUnits.ts`, host unit-type routes | done: bilingual name, category, inventory model, occupancy, room counts, sellable quantity and descriptions; archive is refused when reservations exist |
| Admin content moderation | legacy admin listing queue | none in the admin app | open: without it a new draft cannot reach `APPROVED`, so publish is only usable for already-approved properties |
| Phone layout | legacy property screens | toolbar stacks, list hides secondary columns, drawer and dialogs are full width | implemented, 375px runtime check not run |

Property limits:

- Property creation, publish, pause and archive were not exercised, so seeded properties are unchanged.
- The list and property payloads were already read against the running API by the rates and reviews screens.
- Unit type list and category reads returned 5 unit types and the active catalog. Unit type writes were not exercised.
- Media list returned 3 rows: a ready cover, a ready photo and a rejected PDF, all without storage URLs, which exercises the placeholder path. Media writes were not exercised.
- The revision list returned one approved revision, and its detail returned the full listing content. Revision create, save, submit and withdraw were not exercised.






