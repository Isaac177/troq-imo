# F2 guest web parity: trips and trip actions

Status: open. Written 26/11/2026 for the guest trip actions slice. This ledger is not closed and parity is not complete.

Legacy reference: `troq-front/troqzone-web/src/components/user/stays` and `src/shared/i18n/locales/{en,fr}/stays.json` on the TROQ repository, read-only.

Mungini locations: `apps/web/src/{app,components,hooks,services,types}/trips`, `packages/i18n/locales/{en,fr}/trips.ts`.

## Screens

| # | Legacy feature | Legacy location | Mungini location | Status | Notes |
|---|---|---|---|---|---|
| 1 | Guest stays list | `components/user/stays/StaysTab.tsx` | `components/trips/TripsView.tsx`, `hooks/trips/useTrips.ts` | done | Paginated list with city and status filter, loading, error and empty text. |
| 2 | Status tabs with counts (all, pending, upcoming, active, past, cancelled) | `hooks/useMyStays.ts`, `StaysTab.tsx` | `components/trips/TripsView.tsx`, `reservations/lifecycle/mine_counts.rs` | open | Tabs use city-scoped SQL aggregates and URL state; Rust and SQL read-checked only. API contract regenerated; runtime and phone verification remain. |
| 3 | Total count label | `StaysTab.tsx` | `components/trips/TripsView.tsx` | done | Header shows the total returned by the server. |
| 4 | Reference highlight from `?reference=` | `hooks/useMyStays.ts` | `components/trips/TripsView.tsx`, `hooks/trips/useTrips.ts` | done | The matching row is highlighted and labelled; when the reference is on another page or filter, a translated note points it out. |
| 5 | Explore stays call to action on the empty list | `StaysTab.tsx` | `components/trips/TripsView.tsx` | done | Links to the discovery home. |
| 6 | Stay row: number, photo, property, unit, dates, nights, guests, reference, total, status dot and label | `MyStayRow.tsx` | `components/trips/TripsView.tsx`, `hooks/trips/useTrips.ts` | done | Cover photo, row number, property, unit, dates, nights, guest counts, reference, total and a status dot with the state label are shown. |
| 7 | Approval deadline on a requested row | `MyStayRow.tsx`, `hooks/useMyStays.ts` | `hooks/trips/useTrips.ts` | done | Rendered in the city time zone when present. |
| 8 | Trip detail sheet with photos, status, facts and pricing | `MyStayDetailSheet.tsx`, `hooks/useMyStayDetail.ts` | `components/trips/TripDetailView.tsx`, `hooks/trips/useTripDetail.ts` | open | Facts, state guidance and a unit photo gallery are implemented; API contract regenerated; visual verification remains. |
| 9 | State information text per reservation state | `hooks/useMyStayDetail.ts` | `hooks/trips/useTripDetail.ts`, `TripDetailView.tsx` | done | Each state has a translated explanation in English and French. |
| 10 | Night price lines and discount | `hooks/useMyStayDetail.ts` | `reservations/lifecycle/view.rs`, `TripDetailView.tsx` | open | Stored contract discount is exposed and rendered with the night lines; API contract regenerated; verification remains. Seed `09-discounted-trip.sql` covers a discounted stay. |
| 11 | Guest name and contact details on the detail | `useMyStayDetail.ts` | `useTripDetail.ts` | done | Name, email and phone with mail and phone links. |
| 12 | Nights count, approval deadline and hold expiry on the detail | `useMyStayDetail.ts` | `useTripDetail.ts` | done | Deadlines use the city time zone from `/api/public/cities`. |
| 13 | Unit photos on the detail | `useMyStayDetail.ts` | `reservations/lifecycle/view.rs`, `components/trips/TripGallery.tsx` | open | Ordered ready unit photos and localized alt text are returned for the reservation, with a cover fallback; existing media seeds have storage keys but require actual objects. API contract regenerated; visual verification remains. |
| 14 | View listing link | `MyStayDetailSheet.tsx` | `TripDetailView.tsx` | done | Links to `/stays/{propertyId}`. |

## Trip actions

| # | Legacy feature | Legacy location | Mungini location | Status | Notes |
|---|---|---|---|---|---|
| 15 | Cancel a confirmed trip | `MyStayCancel.tsx`, `hooks/useMyStayCancel.ts` | `components/trips/TripActions.tsx`, `hooks/trips/useTripActions.ts`, `services/trips/trips.api.ts` | done | Shown only for `CONFIRMED`, behind a dialog and a confirm button. |
| 16 | Cancellation preview: total, refund, penalty, deadline | `hooks/useMyStayCancel.ts` | `hooks/trips/useTripActions.ts` | done | Loaded from `GET /api/guest/reservations/{reservationId}/cancellation`. Amounts in CDF through the shared formatter. Deadline as dd/mm/yyyy HH:mm in the city time zone from `/api/public/cities`. |
| 17 | Penalty rendered as "no charge" when zero | `MyStayCancel.tsx` | `TripActions.tsx` | done | |
| 18 | Cancel with expected version | `hooks/useMyStayCancel.ts` | `services/trips/trips.api.ts` | done | Sends `expectedVersion` and a generated idempotency key. Refreshes the trip and shows the charge due at the property. |
| 19 | Withdraw a requested trip | `MyStayCancel.tsx`, `useMyStayCancel.ts` | `TripActions.tsx`, `useTripActions.ts` | done | Shown for `REQUESTED` and `PENDING_PAYMENT`, behind a confirm dialog, sends `expectedVersion`. |
| 20 | Withdraw a `PENDING_PAYMENT` trip | `useMyStayCancel.ts` | `TripActions.tsx`, `useTripActions.ts` | done | The withdraw action is offered for both `REQUESTED` and `PENDING_PAYMENT`. |
| 21 | Cancel hidden once the stay has started | `useMyStayCancel.ts` | `useTripActions.ts`, `TripDetailView.tsx` | done | Cancel is hidden and blocked when the property-city date is on or after the check-in date, and the detail explains why. |
| 22 | Cancelling and withdraw success text | `MyStayCancel.tsx` | `TripActions.tsx` | done | Success notice with charge due at the property and the refund when non-zero. |
| 23 | Action error text | `useMyStayCancel.ts` | `useTripActions.ts` | done | Maps `CANNOT_CANCEL`, `NO_CANCELLATION_DEADLINE`, `NO_CANCELLATION_POLICY`, `RESERVATION_VERSION_CONFLICT`, `REQUEST_IN_PROGRESS`, `CANNOT_WITHDRAW`, `APPROVAL_WINDOW_CLOSED` and a generic fallback to translated messages. |
| 24 | Close the dialog after a completed action | `MyStayCancel.tsx` | `TripActions.tsx` | done | Both dialogs close on success; the success notice stays on the page. |

## Endpoints used

| # | Endpoint | Legacy source | Mungini location | Status |
|---|---|---|---|---|
| 25 | `GET /api/guest/reservations/mine` | `lib/stays/stayClient.ts` | `services/trips/trips.api.ts` | done |
| 26 | `GET /api/guest/reservations/{reservationId}` | `lib/stays/stayClient.ts` | `services/trips/trips.api.ts` | done |
| 27 | `GET /api/guest/reservations/{reservationId}/cancellation` | `lib/stays/stayClient.ts` | `services/trips/trips.api.ts` | done |
| 28 | `POST /api/guest/reservations/{reservationId}/cancel` | `lib/stays/stayClient.ts` | `services/trips/trips.api.ts` | done |
| 29 | `POST /api/guest/reservations/{reservationId}/withdraw` | `lib/stays/stayClient.ts` | `services/trips/trips.api.ts` | done |

## States, seeds and translations

| # | Legacy item | Legacy location | Mungini location | Status | Notes |
|---|---|---|---|---|---|
| 30 | State labels for requested, pending payment, confirmed, checked in, checked out, no show, completed, cancelled, expired, declined, held | `stays.json` `stays.mine.state` | `packages/i18n/locales/{en,fr}/trips.ts` | done | |
| 31 | Cancellation, withdraw and error strings in English and French | `stays.json` `stays.mine.booking` | `packages/i18n/locales/{en,fr}/trips.ts` | done | French satisfies the English type. |
| 32 | Demo trips for every state | `stay-service/seeds` | `backend/seeds/reservations/01-reservations-goma.sql`, `06-guest-trips.sql` | done | Existing seeds cover `CONFIRMED`, `REQUESTED`, `PENDING_PAYMENT`, checked in, checked out, cancelled, declined, expired and no show for the seeded guest. |
| 33 | Demo cancelled trip with a charge due at the property | legacy seeds | `backend/seeds/reservations/08-charged-cancellation.sql` | done in seed: a confirmed stay cloned from the seeded confirmed trip with a past cancellation deadline and a 50 percent penalty, so the cancellation preview shows a charge. The seed must be applied by the owner |
| 34 | Demo trip with an approved cancellation on a past deadline | legacy seeds | none | open | The seeded confirmed trip is still inside its free window. |
| 35 | Deadline formatting in the city time zone | `useMyStays.ts`, `useMyStayDetail.ts` | `useTripActions.ts` | done | Time zone read from `CityView.timezone` returned by `/api/public/cities`, not a hardcoded map. |

## Notification inbox

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Guest notification inbox | legacy notification center | `apps/web/src/app/notifications/page.tsx`, `components/notifications/NotificationsView.tsx`, `hooks/notifications/useNotifications.ts`, `GET /api/guest/notifications` | done: server pagination, all/unread tabs, unread summary, mark all read, skeleton, error and empty states |
| Rows with type, priority, city-time timestamp and read state | legacy rows | `components/notifications/NotificationRow.tsx` | done; 15 types and 4 priorities translated, unknown values fall back |
| Mark read and deep-link navigation | legacy notification targets | hook marks unread rows read and follows internal links only | done; external and protocol-relative links are ignored |
| Filter and page in the URL | legacy query state | `?filter=unread` and `?page=` kept with `history.replaceState` | done |
| Header entry | legacy bell | `components/notifications/NotificationsLink.tsx` in `SiteHeader` | done |
| Phone and dark mode | legacy notification center | rows stack, controls within 36px | implemented, 375px and dark-mode checks not run |

Inbox limits:

- Unread count expiry filtering is implemented in the backend and awaits owner compilation and live verification.
- Audience filtering is implemented for inbox operations and awaits owner compilation and verification; historical rows with no audience need a cutover decision.

## Verification

- `bunx tsc --noEmit` in `apps/web`: passed.
- `bunx tsc --noEmit` in `packages/i18n`: passed.
- Runtime, seed load, live API calls and 375px checks in English and French: not run.

## Guest profile language

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Profile language preference | `troqzone-web/src/components/user/ProfileEditModal.tsx` | `GET /api/public/languages`, `PATCH /api/guest/profile`, `components/account/ProfileForm.tsx`, `hooks/account/useProfileForm.ts` | open: supported languages come from the identity profile language catalog; saving updates the session and active site language. Existing identity seeds include English and French. Rust read-checked only; API contract regenerated; phone verification remains open. |
