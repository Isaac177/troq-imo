# Client discovery, detail and saved stays

Dependencies: U, O, I, P; D12 for cross-city behavior. Applies to Next.js public web and Expo mobile.

## C01: Entry and destination search

Add a Stays module entry through existing super-app navigation. Preserve retail navigation and authenticated account context. Home includes destination/date/occupancy search, approved curated collections, recent searches and saved stays. Do not show fake recommendations when data is empty.

Destination selector searches supported cities and neighborhoods and allows manual choice when geolocation is unavailable or denied. Location permission is optional for browsing. Distinguish user's location from destination and property-authoritative city.

## C02: Search input and results

Inputs: city/destination, checkIn/checkOut, rooms, adults, children with ages, pets where relevant and optional flexibility. Validate capacity and dates before search. Multi-room occupancy is explicit per room, not an ambiguous total count.

Results include property ID/city, localized title, approved cover image, approximate location, verified rating/count, matching unit/rate summary, cancellation/payment summary and full stay total. Without dates display a clearly qualified starting rate or request dates; never label it an exact stay quote.

Sort choices: relevance with documented factors, total stay price, guest rating with count and distance to selected point. Use stable tie-breaking. Filters include property type, required amenities, accessibility details, price, rating, instant booking, cancellation, meals and payment options. Run filters/sort in SQL or the approved server-side search projection, not on one client page.

UI: URL-backed search on web, equivalent deep-link state on mobile; filter sheet/sidebar; active filter chips; reset; count; server pagination. Avoid jumping results during refetch. Stale responses for an older search cannot overwrite a newer search.

## C03: Map and privacy

Map/list selection stays synchronized. Cluster public approximate coordinates as appropriate; select marker focuses result without losing search state. Area-search is an explicit bounded request. Map provider choice/cost/privacy requirements need approval before integration.

Private exact address and access instructions follow D18. Network payloads must not include hidden exact coordinates merely because UI displays an approximate point. Accessible results list provides equivalent discovery without map use.

## C04: Property detail

Sections: gallery; identity/location; verified rating; capacity/amenities/accessibility; unit/rate options; description; rules and check-in/out; cancellation/payment terms; operator profile; reviews; nearby context if real data exists.

Gallery supports keyboard/swipe, alt text, thumbnails, responsive variants and stable layout. Unit comparison shows beds/capacity, meals, cancellation, due-now amount and total. Available quantity comes from authoritative search/availability response and is not fabricated urgency.

Booking panel persists selected dates/occupancy. Selecting a rate requests a current quote. Sold-out responses offer changed dates or eligible alternatives while preserving the search. Property unavailable/unpublished deep links show an appropriate unavailable state without exposing private draft content.

## C05: Saved content and alerts

Authenticated wishlists support named collections, add/remove, pagination and explicit private/shared visibility. Shared links are revocable and reveal only public property data. Saved search stores parameters and notification consent; alerts must deduplicate and respect quiet/unsubscribe preferences.

Recently viewed storage has bounded retention and account/privacy behavior. Comparison, if enabled, uses consistent total prices for identical search inputs and explains non-comparable terms. Do not compare a starting rate with a full-stay total.

## C06: Public indexing and analytics

Public published property pages have localized metadata/canonical routing and truthful structured data where supported. Private booking/account pages are not publicly indexable. Search URLs need an explicit indexing policy to avoid unbounded combinations.

Track consent-appropriate search, property view, quote and conversion events with correlation IDs; never send private guest/document/payment content into analytics. Ranking changes need explainable factors and sponsored placement labeling.

## API and acceptance

Public routes return public DTOs only. Queries are bounded and rate-limited. Client never receives internal costs, private media, other guests or owner verification documents.

Fixtures cover no matches, unavailable dates, incomplete translation, no reviews, long descriptions, large galleries and stale prices. Test server filters/totals, back/deep-link state, list/map parity, denied geolocation, flexible-date distinction, inaccessible drafts and responsive bilingual UI.
