# B10 admin read models

Sources are `tzone-rs` main at `780396f`. Mungini uses one database, so city filters, totals and pagination run in SQL. Platform-scoped permissions are enforced and CDF amounts are decimal strings.

Verification on 26/11/2026: the workspace compiles and the domain, api and kernel unit tests pass. Admin dashboard, host breakdown, guest list, host application list, host list and host status change were requested against the local seeded database.

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Guest booking history | `stay-service/src/admin_guests/bookings.rs`, gateway `admin_guests.rs` | `admin/guest_bookings.rs`, `admin/handlers.rs` | done for the current booking model; live request verified |
| Guest list and account profiles | `stay-service/src/admin_guests/list.rs`, `profiles.rs` | `admin/guests.rs`, `admin/guest_handlers.rs` | done: SQL aggregate list with search, flag filters and pagination, identity profile join, detail route; live request verified |
| Dashboard summary and city series | `stay-service/src/admin_dashboard/{summary,figures,credit_figures,series}.rs` | `admin/dashboard*.rs` | done: aggregate endpoint and daily series; live request verified |
| Host dashboard breakdown | `stay-service/src/admin_dashboard/businesses.rs` | `admin/dashboard_hosts.rs` | done: per-host figures with pagination; live request verified |
| Host dashboard for a host member | new in Mungini | `admin/dashboard.rs::host_summary`, `GET /api/host/dashboard` | done: host-scoped `analytics.read`, city-scoped; live request verified |
| Admin reservations list and summary | `stay-service/src/admin_reservations/{list,summary,filters}.rs` | `admin/reservation_*.rs` | done; live list request verified through the admin route |
| Admin reservation detail | `stay-service/src/admin_reservations/detail.rs` | `admin/reservation_detail*.rs`, `apps/admin/src/components/reservations/ReservationDetails.tsx` | done on the API; the admin drawer now lists stays, guests, cases, amendments and timeline with translated case types, case states, amendment states and audit actions. Child collections share one page, so the pager advances all of them together. Live detail requests verified for reservations with stays, guests, amendments and a case |
| Admin reservation export | `stay-service/src/admin_reservations/list.rs` | `admin/reservation_list.rs`, `reservation_handlers.rs` | done: SQL-paginated with a total |
| Admin calendar | `stay-service/src/admin_calendar/`, `stay-service/src/calendar/handler.rs` | `inventory/calendar/{handler,service,view,queries}.rs`, `GET /api/admin/calendar` | implemented: platform and city scoped, paginated units with reservations, blocks, restrictions and prices; database and live API verification remain open; inventory response type renamed `InventoryCalendarView` to eliminate a collision with guest `CalendarView`, generated client awaits owner rebuild |
| Host analytics summary report | `stay-service/src/reports/` | `admin/report*.rs` | done: host and property scoped aggregates |
| Other reports | `stay-service/src/reports/` | none | open |
| Host applications and hosts | new in Mungini (D2) | `hosts/onboarding/admin.rs` | done: application list, detail, decision, document review, host list and status change; live requests verified |
| Admin review moderation | `stay-service/src/reviews/admin_list.rs`, `moderation.rs`, gateway `admin/stay_review_handlers.rs` | `reviews/admin_list.rs`, `reviews/moderation.rs`, `apps/admin/src/{views,components,hooks,services,types}/reviews` | implemented: paginated city reviews with status, rating and search filters, guest and reservation columns, publish or hide with a reason code. Live list and filter requests verified; decision writes and 375px checks remain open |
| Admin cases | `stay-service/src/cases/admin.rs`, `admin_actions.rs`, gateway `admin/stay_claims.rs` | `reservations/cases/admin.rs`, `admin_actions.rs`, `apps/admin/src/{views,components,hooks,services,types}/cases` | implemented: paginated city cases with state, type, overdue and search filters; drawer with reason, deadlines, claim amount, escalation, resolution and evidence preview; close and escalate actions. Live list, filter, detail and evidence requests verified. Assign, claim decisions, payouts, action writes and 375px checks remain open |
| Admin seeds | `stay-service/seeds/` | B5, B7, B8, B9 and B12 seeds; `reservations/07-reservation-history.sql` for timeline rows | done for dashboard, reservations, guests, applications, hosts and timeline; the timeline seed must be applied by the owner |
| Legacy unit, database and concurrency tests | `stay-service/src/admin_guests/`, `admin_dashboard/`, `admin_reservations/` | unit tests and live checks | partial: SQLx tests for the domain modules run; no dedicated admin DB tests |

## Open

- Admin calendar read model is implemented in inventory; database and live API verification remain open. Remaining reports are not ported.
- Cancel and claim actions on a case are still platform screens outside this drawer; the reservation detail links are display only.
- Reservation timeline demo rows are added by `backend/seeds/reservations/07-reservation-history.sql`, which the owner must apply before the timeline shows seeded activity.
- Dashboard night counts depend on inventory rows inside the selected range; seeded nights outside the period produce zero nights sold, which is correct but makes the occupancy figure look empty in short demos.
- No dedicated admin database tests exist; the current verification is compile, unit tests and live requests against the seeded dev database.

The admin calendar screen is implemented against the regenerated `InventoryCalendarView` contract. Property search and unit pages are server-paginated; date range is limited to 93 nights. Live API and screen behavior and 375px English/French layouts remain unverified.
