TASK: Build the guest-facing stays UI for TROQ. This covers specs 08 (client discovery) and 09 (checkout, reservations, my stays) and the guest policy summary from spec 07. Use the owner decisions in docs/stays/00-decisions.md, including the section "Owner instruction: use the proposed defaults".

READ FIRST, in this order, and follow the rules exactly:
1. /Users/jogues/Documents/TROQ/AGENTS.md (repository rules; the hard rules apply to every change)
2. /Users/jogues/Documents/TROQ/HANDOFF.md
3. /Users/jogues/Documents/TROQ/docs/stays/README.md, 00-decisions.md, 03-ui-system.md, 07-pricing-policies.md (sections P03 and P05 only), 08-client-discovery.md, 09-checkout-reservations.md
4. The AGENTS.md or README of each app you touch. For mobile: /Users/jogues/Documents/TROQ/troqzone-app/AGENTS.md. For web: the README in troq-front/troqzone-web.

Before writing code, reply with a plan that lists every screen (S01 to S15 below) and where it will live in each app. Then build.

==================================================
1. TARGETS AND SCOPE
==================================================
Web (public storefront): troq-front/troqzone-web. Next.js 16, React 18, app router, routes under app/[lang]/. Keep retail navigation working. Add a Stays entry through the existing navigation. Do not change retail pages except to add that entry.

Mobile: troqzone-app/apps/mobile. Expo 54, React Native 0.81. Shared code goes in troqzone-app/packages: shared-api, shared-hooks, shared-i18n, shared-state, shared-storage, shared-theme, shared-types, shared-ui, shared-utils.

Build both platforms. Every screen must work in English and French, on phone, tablet and desktop (web).

Out of scope, do not build any of these:
- Manager and operator screens (already built in troq-front/troqzone-business).
- Admin screens.
- Reviews and messaging (spec 12).
- Online payment, card entry, receipts for payments. Payment is pay at the property only (decision D06).
- Damage deposits and installments (D09 and D06).
- Dynamic pricing (D17).
- Group and multi-room booking (D13). One reservation holds one room.
- Exact address and access codes (D18). Show only the approximate location.
- Map view. No map provider is approved yet. Results are a list only.

==================================================
2. API AVAILABLE TODAY
==================================================
All routes are under /api/stays. Responses use the envelope { success, data }. Errors use { success: false, error: { message } }. Check the envelope against an existing client before you rely on it: troq-front/troqzone-business/src/hooks/stays/useStayTasksPage.ts reads response.data.data, and stayFailureMessage in src/utils/stays/apiErrors.ts reads error.message. Authentication uses the existing retail session in each app.

Usable by a signed-in guest:
- GET /availability?propertyId&unitTypeId&checkIn&checkOut
  Returns the remaining rooms and any violated rules for each night.
- POST /quotes  Body: propertyId, unitTypeId, ratePlanId, checkIn (YYYY-MM-DD), checkOut, adultCount, childCount.
  Returns a quote: id, propertyId, unitTypeId, ratePlanId, checkIn, checkOut, adultCount, childCount, currency ("CDF"), subtotal, discount ({promotionId, amount} or null), total, nights ([{date, amount}]), policies ([{id, kind, content}]), cancellationDeadline (or null for quotes made before the policy snapshot), paymentSchedule ({dueNow, dueLater, dueAtProperty}), expiresAt, expired.
- GET /quotes/{quoteId}  Same shape as the quote above.
- POST /reservations  Body: quoteId, guest ({fullName, contactPhone, contactEmail}).
  Returns a reservation: id, reference, state (CONFIRMED for instant rate plans, REQUESTED for request rate plans), propertyId, unitTypeId, ratePlanId, checkIn, checkOut, adultCount, childCount, currency, total, nights, guest, holdExpiresAt, paymentRequired, approvalDeadlineAt.
- GET /reservations/mine?page&pageSize  The signed-in guest's own reservations, paged on the server.
- GET /reservations/{reservationId}  The guest's own reservation. Returns 404 for other users.
- GET /reservations/{reservationId}/cancellation  Preview of a cancellation: reservationId, currency, total, deadline, evaluatedAt, reason (FREE_CANCELLATION or PENALTY_APPLIES), refund ("0.00" under pay at property), penalty.
- POST /reservations/{reservationId}/cancel  Body: expectedVersion (required), idempotencyKey (optional).
  Returns: reservationId, state ("CANCELLED"), cancelledAt, reason, currency, refund, chargeDueAtProperty. Releases the nights and unit assignments.
- POST /amendments/{amendmentId}/accept and POST /amendments/{amendmentId}/decline  Return the reservation.
- GET /cases/mine and POST /cases (optional; "report a problem" only if the owner asks for it).

BACKEND STATUS (updated after the backend pass). The gaps below are still open unless marked DONE.
- DONE G1. Public (anonymous) routes exist: GET /api/stays/public/destinations, GET /api/stays/public/properties (search with dates, guests, instantBooking, sort, page, pageSize), GET /api/stays/public/properties/{propertyId}, GET /api/stays/public/properties/{propertyId}/availability, GET /api/stays/public/media/{mediaId}/content.
- DONE G2. The search engine (docs/stays/20-search-engine.md) returns hits with the best offer and all offers, facets and stats. Filters: dates, guests, instantBooking, propertyTypes, mealPlans, paymentCollections, accessibility, amenities, priceMin/priceMax, minFreeCancellationHours, maxPenaltyPercent, distance. Sorts: relevance, name, price, distance. Rating filters are not built (reviews are spec 12). The filter list is at GET /api/stays/public/search/catalog.
- OPEN G3. The public detail has no exact or approximate coordinates, no neighborhood, no verified rating and no review count (reviews are spec 12). Gallery variants are not generated.
- OPEN G4. Child ages are not sent. The quote and search take only the number of children. Spec 08 needs an age for each child, and the age cut-offs are an owner decision.
- DONE G5. Special requests are stored (up to 500 characters) on the hold as a request, not a guarantee. Operators accept or decline them.
- DONE G6. The hold takes acceptedPolicyVersionIds, which must match the quote exactly. The acceptance time is stored.
- DONE G7. The reservation view and the my-reservations list carry version.
- DONE G8. The my-reservations list carries pendingAmendment, and GET /api/stays/reservations/{id}/amendments lists the guest's amendments.
- OPEN G9. No guest reservation timeline.
- OPEN G10. No voucher document or voucher data endpoint.
- DONE G11. Wishlists (named, PRIVATE or SHARED, add and remove, paged items) and saved searches exist at /api/stays/me/wishlists and /api/stays/me/saved-searches. Share links and alerts are not built.
- DONE G12. My reservations filter by tab: pending, upcoming, active, past, cancelled.
- OPEN. Cancellation preview as POST (the GET preview exists), amendment previews (POST /reservations/{id}/amendment-previews), and the operations endpoint for retry recovery.

==================================================
3. SCREENS TO BUILD
==================================================
For each screen, build all of these states: initial loading, refresh, loaded, empty, validation failure, service error, permission denied (a 403 is not an empty list), stale data, and for mutations: submitting, ambiguous response and recovery.

S01 STAYS ENTRY (web: [lang]/stays; mobile: a Stays entry in the existing module navigation)
- A search card: destination selector, check-in and check-out dates, occupancy (rooms, adults, children with ages). Search submits to S02.
- Recent searches stored locally, bounded (last 5), with a clear action.
- Saved stays only when the API exists (G11). Approved collections only when real data exists.
- No invented recommendations. When there is no data, say so.

S02 SEARCH RESULTS (blocked on G1 and G2 for real data; build the shell and states against fixtures only if the owner approves fixtures)
- Web: search state in the URL (destination, dates, occupancy, sort, filters, page). Mobile: the same state as a deep link.
- Result card: approved cover image, localized title, approximate city or neighborhood, verified rating and count (or "No reviews yet"), matching unit and rate summary, cancellation and payment summary, and the full stay total in CDF. When there are no dates, show a clearly labeled starting rate, never a quote.
- Sort: relevance (state the factors), total price, guest rating, distance from the selected point. Use a stable tie-breaker.
- Filters: property type, required amenities, accessibility, price range, rating, instant booking, cancellation terms, meals, payment (pay at property). Filters apply on the server.
- Active filter chips, a reset action, a result count, and server pagination with page, pageSize and total.
- Keep the previous results on screen during a refetch, with a small loading indicator. Results must not jump.
- A response for an older search must never overwrite a newer search.
- No map (see scope).

S03 PROPERTY DETAIL (blocked on G3 for real data)
- Gallery: keyboard and swipe navigation, alt text, thumbnails, stable layout.
- Identity and approximate location. Verified rating and count. Capacity, amenities, accessibility details.
- Unit comparison: beds and capacity, meals, cancellation summary, due at property, total.
- Description (long text can expand), house rules, check-in and check-out times, cancellation and payment terms (the policy summary, see S04), operator profile.
- Reviews section is not built. Do not show it.
- Booking panel: sticky on desktop, a bottom action bar on mobile above the safe area and the keyboard. It keeps the selected dates and occupancy. Choosing a rate requests a quote (POST /quotes).
- If the rate has sold out, offer alternative dates from the same search and keep the search state.
- Unavailable or unpublished property: a clear unavailable page. Never show draft content.

S04 QUOTE AND POLICY REVIEW (checkout step 1)
- Show the property, city, local dates, nights, rooms, occupancy, inclusions, cancellation deadline (with the property's timezone), and the full money breakdown in CDF: nightly lines, discount if any, total, due now (0), due later (0), due at property.
- Show the policy versions in full: the cancellation policy (free until N hours before arrival, then a penalty percentage) and the payment policy (pay at the property).
- Show the quote expiry as information only. The server's expiresAt decides. When a quote expires, request a new quote.
- If the total changes on refresh, the guest must explicitly accept the new total before continuing.

S05 CHECKOUT FORM (checkout steps 2 and 3)
- Step 2, guest details: full name (required), contact phone (international input with a country selector and a geo-detected default, with manual override), contact email (optional). Special requests are hidden until G5 exists.
- Step 3, policy acceptance: one explicit checkbox per policy version, with the version shown. Record the time of acceptance on the client and send it once G6 exists.
- Require a signed-in account. Pre-fill the name and contact from the account.
- Draft: keep non-sensitive fields in local storage with an expiry. Never store payment data or identity documents.
- Submit: disabled while submitting. Each attempt carries an idempotency key once the API accepts one (G6). Repeated taps must not create duplicate holds.

S06 OUTCOME
- Instant (CONFIRMED): confirmation with the reference, dates, total, the statement "pay at the property", and a voucher link once G10 exists.
- Request (REQUESTED): "requested" page with the approval deadline in the property's local time and what happens next.
- Failure: a clear error. If the response was ambiguous (network failure after submit), show "we are checking your booking" and look the reservation up by its id. Never show a booking as confirmed until the server says so.
- Expired hold: explain it and return the guest to a new quote.

S07 MY STAYS (list)
- Tabs: pending (REQUESTED), upcoming (CONFIRMED with a future arrival), active (CHECKED_IN), past (CHECKED_OUT, NO_SHOW, COMPLETED), cancelled (CANCELLED, EXPIRED, DECLINED). Put the active tab in the URL.
- Each row: reference, property, dates, a state label (text plus an icon, never color alone), total in CDF.
- Server pagination, empty states per tab, and a signed-out state.

S08 RESERVATION DETAIL
- Separate status blocks: the reservation state, the stay phase (upcoming, arrival due, in stay, completed, derived from the dates and state), and the payment (pay at the property, with the amount due there).
- Sections: dates and nights, rooms and occupancy, guest details, the contract (policy versions and the cancellation deadline), the money breakdown.
- Actions depend on state. Cancel: only for CONFIRMED stays, opens S09. Voucher: once G10 exists. Amendment: when G8 exists and one is pending, opens S10. Contact the property: hidden until messaging exists.
- No timeline (G9).

S09 CANCELLATION CONFIRMATION
- Preview from GET /reservations/{id}/cancellation: the deadline, whether the cancellation is free, the amount due at the property if it is not, refund "0.00" (nothing was collected), and the effect (nights released, cannot be undone).
- An explicit action label such as "Cancel this stay". A confirmation step. A pending state while it runs.
- On success: the cancelled state with the reason.
- On conflict (the reservation changed meanwhile): reload the preview.
- On an ambiguous response: reload the reservation and show its actual state.

S10 AMENDMENT REVIEW (blocked on G8)
- Proposed dates, the new total, the price difference (paid at the property), the expiry.
- Accept and decline, each behind a confirmation panel. Declining keeps the original booking.

S11 VOUCHER (blocked on G10)
- A printable and downloadable confirmation, separate from a receipt. Localized. Shows only authorized fields. Shows "last refreshed" time. An offline copy is marked with its last refreshed time.

S12 SAVED STAYS AND WISHLISTS (blocked on G11)
- Named collections; add and remove; pagination; private by default; a shared link that is revocable and shows only public property data.
- Recently viewed: local, bounded (last 20), with a clear action.

S13 PUBLIC PROPERTY PAGE SEO (web only, after G1)
- Localized metadata and canonical URLs. Structured data only where it is true for a published property.
- Reservation, my stays and checkout pages are noindex. Search result URLs are noindex unless the owner approves an indexing policy.

S14 ANALYTICS (consent-aware)
- Events: search, property view, quote, conversion, each with a correlation id. Never send a guest's name, contact details, documents or payment information.

S15 STATE PAGES
- No matches. Dates not available. Property unpublished or missing. Signed out. Permission denied. Service error. Stale data. Slow network. Offline with the last refreshed state.

==================================================
4. SHARED PRIMITIVES (build once; never copy them per app)
==================================================
- Destination selector: search city and neighborhood, show the selected label, handle loading, error and empty states, keyboard selection, clear. Geolocation is optional and must work when it is denied. Always allow a manual choice. Keep the user's location separate from the destination.
- Stay date picker: two dates, unavailable nights marked, minimum-stay explanations, keyboard navigation, dates in the property's local calendar. Mobile uses a full-screen calendar.
- Occupancy selector: rooms, adults, children with ages (per room), capacity validation, steppers styled to the design system.
- Money breakdown: CDF throughout, nightly lines, discount, total, due now, due later, due at property.
- Policy summary and full-terms view: shows the policy version and the accepted version.
- Status badge and timeline: text labels with an icon. Show the actor and time only where the guest is allowed to see them.
- Confirmation panel: consequence preview, exact amount and date, an explicit action label, pending state, error recovery.
- Server-paged list with URL state (web) or a deep link (mobile).
- International phone input: country selector, geo-detected default, manual override. Never a bare type="tel".

==================================================
5. RULES (from AGENTS.md; a violation fails review)
==================================================
- No comments in code: no //, no /* */, no JSX comments, no docstrings, no section separators.
- No hardcoded UI text. Every string goes through shared-i18n (locales en and fr), including errors, statuses, empty states, validation messages and placeholders. Store and send machine codes (CANCELLED, FREE_CANCELLATION), and show localized labels.
- React components contain JSX only. State, effects, handlers, derived values, labels, formatting, fetching and validation go in custom hooks (use*.ts or use*.tsx) next to the component, or in the feature's existing hooks folder.
- No file over 300 lines. Split it.
- Lists paginate on the server: page, pageSize and total come from the API. Never fetch every row and paginate in the client.
- Money is CDF only. No USD, no conversion text. Use the existing CDF formatter.
- Dates display as dd/mm/yyyy. Show the property's timezone wherever a deadline could be ambiguous. Never show raw ISO strings.
- Form controls use the design system. No raw radio, checkbox or select elements on web.
- No decorative accent bars or colored borders on cards. No small eyebrow labels above section titles. No nested cards. No Unicode arrows as UI glyphs (use the icon library). Separate sections with spacing and dividers.
- Interactive UI animates: panels, drawers, expand and collapse, selection changes, layout shifts at 200 to 300 ms, ease-in-out. Honor reduced motion.
- Repeated taps must not create duplicate actions. Send an idempotency key wherever the API accepts one.
- Never show a booking as confirmed, a payment as successful, a refund as issued or a cancellation as complete before the server says so.
- Never make the user type an ID, slug, code or reference. Pick from lists fetched from the API.
- Seed realistic demo data for every feature so no page is empty. The stay seeds are in tzone-rs/stay-service/seeds/goma.sql and kin.sql. Keep ids that start with seed-. Never break an existing seed row.
- No placeholder pages, fake data or stubs. If the API cannot support a screen, leave the screen out and report it.
- Use bun for every JavaScript workspace (bun install, bun run, bunx, bun test). Never npm, npx, yarn or pnpm.
- Do not deploy, push images, run production compose, or commit unless asked.
- Do not add Co-Authored-By lines or any AI attribution to commits or PRs.
- Do not add security warnings to commits, backlogs or status lists.

==================================================
6. ACCESSIBILITY AND QUALITY
==================================================
- Every action reachable by keyboard, in a logical order. Focus returns to the trigger after a dialog or drawer closes.
- Labels linked to fields. Errors announced. Status changes announced politely to screen readers.
- Touch targets are large enough. Contrast checked in light and dark (where the app supports dark). No color-only status.
- Test narrow phone, tablet and desktop. English and French. Slow network. Offline shows the last refreshed state.
- Edge cases to test: long names, long descriptions, large galleries, no matches, no reviews, a missing translation, a stale price, an expired quote, an expired hold, a cancellation after the deadline.

==================================================
7. VERIFICATION BEFORE YOU REPORT DONE
==================================================
- Web: run bun run typecheck and bun run build in troq-front/troqzone-web. Open each screen in the browser, in English and French, at narrow and desktop widths.
- Mobile: run the typecheck in troqzone-app. Run the app in the iOS simulator and check the core flows: search, property detail, quote, checkout, my stays, cancel.
- Screenshots are evidence. They do not replace functional checks.

==================================================
8. REPORT BACK WITH
==================================================
- Files changed, per app.
- Screens delivered, by ID (S01 to S15).
- Screens blocked, by ID, with the gap number (G1 to G12).
- Shared primitives built.
- Commands run and their results.
- The manual steps the owner must run.
