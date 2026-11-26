# Vision

Mungini is the standalone accommodation booking product that Stay becomes when it leaves TROQ. Hosts list properties, guests search, book and stay, and platform staff moderate, support and bill. It has its own identity, hosts, billing and database. After cutover it shares nothing with TROQ at runtime.

This file describes what Mungini is and what it deliberately is not. Behaviour details are in the other files of this folder. Every statement was checked against the code on `main` (`tzone-rs/stay-service`, `tzone-rs/auth-service`) or against the Mungini tree, unless it is listed under Unverified.

## Audiences

| Audience | Who | Mungini app | Kernel role |
|---|---|---|---|
| Guest | Anyone who searches, books, messages a host, reviews a stay | `apps/web` (Next.js), guest area of `apps/mobile` | `Guest` |
| Host | A person or business that lists properties, and its team members | `apps/host` (Vite, designed at 375px first), host area of `apps/mobile` | `Host` |
| Staff | Platform employees: listing review, support, cases, claims, commission, settings | `apps/admin` (Vite) | `Staff` |

The three roles are the coarse split in `kernel/src/auth/claims.rs`. What a host member or staff member may do is decided by fine-grained permissions (see `host-and-guest-model.md`).

## Cities

Mungini serves two cities, defined in `kernel/src/city.rs`:

| Code | Name | Time zone |
|---|---|---|
| `KIN` | Kinshasa | Africa/Kinshasa |
| `GOMA` | Goma | Africa/Lubumbashi |

The config variable `CITY_CODES` (default `KIN,GOMA`) says which cities an instance serves. A request names its city with the `x-city` header or a `city` query parameter. Adding a city is a code change in the kernel plus seeds, not only configuration. Details in `data-and-cities.md`.

## Money

All money is CDF. Reservations carry a `currency` column that is checked to equal `CDF`. There is no USD, no conversion and no exchange rate. Amounts are stored as `numeric(14,2)`.

## Languages

English and French. Notification titles and bodies are stored in both languages, documents render in `en` or `fr` (French is the default when the locale is not English), and content such as property names and amenities is stored per language. Every UI string goes through the i18n package.

## How booking works

- A property offers unit types sold through rate plans. A guest gets a quote, then books it as a hold.
- A rate plan has a booking mode. `INSTANT` confirms at once. `REQUEST` waits for the host to accept or decline within an approval window.
- The guest pays the property directly. Mungini records what is owed and issues documents. It does not collect, hold or refund guest money (see `payments-and-billing.md`).
- The only money Mungini itself moves is the host's commission, paid from a prepaid credit balance the host holds with the platform.

## What Mungini does not do

Verified as absent in the legacy code and not planned for the first release:

- Online payment of a stay, deposits, card holds, instalments and refunds of guest money.
- A CRM for hosts (default of decision D8, owner to confirm).
- Any shared login, wallet or database with TROQ (default of decision D1).

## Modules today

Backend modules live in `backend/crates/domain/src`. State on 25/11/2026:

| Module | State in the Mungini tree |
|---|---|
| `identity`, `hosts` | partial: access directory, overrides, host contact. No users, OTP or sessions yet |
| `properties` | amenities and media code; property, unit and inventory tables are not yet in `backend/migrations` |
| `inventory` | code present, tables not yet in migrations |
| `pricing` | code and migration `00019_pricing.sql` (policies, rate plans, rules, promotions, settings, quotes) |
| `reservations` | holds, state machine, cancellation, decisions, stay flow, amendments, messages, events. Tables for reservations, cases and tasks exist. Cases, claims and operations code folders are still empty |
| `guests` | discovery, search, wishlists, saved searches, retention; migration `00017_guests.sql` |
| `notifications` | in-app, push, email, documents. WhatsApp not delivered |
| `reviews` | migration `00015_reviews.sql` and seeds |
| `billing` | partial: commission, credits, settings; migration `00018_billing.sql`; admin and HTTP folders empty |

The parity ledgers in `docs/parity/` are the source of truth for what is done.

## Unverified

- Target market, pricing of the platform and brand positioning beyond the facts above.
- Which property types a host may list. The legacy code has accommodation categories and templates, but the list of categories for Mungini is not confirmed.
- Whether more cities than Goma and Kinshasa are planned.
