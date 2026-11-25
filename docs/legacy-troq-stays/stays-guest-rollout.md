# Stays on troqzone-web: rollout plan

Decision: the guest side lives inside `troq-front/troqzone-web` under `app/[lang]/stays`. No new Next project. Business app and backend already exist; this plan only adds the guest surface.

Each phase ships on its own and leaves the site working. Do not start a phase before the previous one is verified.

Rules that apply to every phase (see AGENTS.md): no code comments, i18n keys in `en` and `fr`, CDF only, dd/mm/yyyy dates, logic in `use*.ts` hooks, files under 300 lines, server pagination, seed data, `bunx tsc` as the only check.

## Phase 0: entry point on the home screen

Goal: a visitor on the home page sees Stays and lands on `/{lang}/stays`.

1. Route: create `app/[lang]/stays/page.tsx` rendering a minimal landing (title, short text, search form placeholder replaced in phase 1). It must exist before any link points to it.
2. Home section: add `src/components/home/StaysPromoSection.tsx`, placed in `src/components/Home.tsx` right after `HeroSection`. One banner with image, title, subtitle and a single button linking to `/{lang}/stays`. Logic in `src/components/home/hooks/useStaysPromo.ts`.
3. Header link: add a "Stays" item in `src/components/header/mainNav` and in `mobileNav`, same link.
4. Strings: add `stays.json` in `troqzone-app/packages/shared-i18n/locales/{en,fr}/` with promo title, subtitle, button, nav label. Check how `troqzone-web` loads locales before adding the file.
5. Middleware: no change needed, `/stays` is locale-prefixed by the existing redirect.
6. Check: `bunx tsc` in `troqzone-web`, then open home at 375px and desktop, both languages.

Done when: clicking the banner or nav item opens `/fr/stays` and `/en/stays`.

## Phase 1: search and results

Backend ready: `GET /stays/destinations`, `GET /stays/properties`, `GET /stays/amenities`, `GET /stays/accommodation-categories`, `GET /stays/search/catalog`.

1. Port into `troqzone-web/src/components/stays/`: `StayDestinationSelector`, `StayDateRangePicker`, `StayOccupancySelector` from the business app. Move shared types (`stayProperties`, `stayCalendar`) into one shared place instead of copying twice.
2. Search bar on `/stays` and results page `/stays/search` with filters (category, amenities, price in CDF), server pagination, total count, URL as the source of filter state.
3. Server render the results page for SEO.
4. Seed: confirm `stay-service/seeds` gives at least several published properties per city, with media.

## Phase 2: property page

Backend: `GET /stays/properties/{id}`, `/availability`, `/reviews`, `GET /stays/media/{id}/content`.

1. `/stays/[propertyId]`: gallery, description, amenities, unit types, reviews list (paginated), availability calendar.
2. Price quote for the chosen dates and guests via the quotes route; show the breakdown with `StayMoneyBreakdown`.
3. Page metadata and structured data for listings.

## Phase 3: booking and payment

Backend: `POST /stays/reservations` (hold). Open question: no stay reference was found in `payment-service`. Before building checkout, confirm how a hold gets paid (mobile money flow as in `docs/mobile-money-only-resolution.md`) and what confirms the reservation.

1. Decide the contract: hold, pay, confirm, expiry of unpaid holds.
2. Checkout page with guest details (international phone input), quote recap, payment.
3. Confirmation page and notification (`troq-notify/src/stays.rs` already exists).
4. Require login for booking; reuse the auth flow and return to the checkout after sign in.

## Phase 4: guest account

Backend: reservation get, cancel, cancellation quote, messages, review; wishlists; saved searches.

1. `/profile/trips`: list, detail, cancel with refund preview, amendment requests.
2. Messages thread per reservation with the host.
3. Review form after checkout date.
4. Wishlist heart on result cards and property page; saved searches from the results page.

## Phase 5: hardening

1. Empty, error and loading states on every screen; 375px check in English and French.
2. Calendar feed and host notifications verified end to end with the business app.
3. Performance and SEO pass on search and property pages.
4. Release notes listing changed services and apps for the owner to deploy.

## Open questions

- Payment confirmation for stays (phase 3 blocker).
- Whether `/stays` appears on the mobile app later; keep types and API calls in a shared package to make that cheap.
