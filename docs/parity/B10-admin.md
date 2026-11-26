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
| Admin reservation detail | `stay-service/src/admin_reservations/detail.rs` | `admin/reservation_detail*.rs` | done: child collections paginated and CDF strings |
| Admin reservation export | `stay-service/src/admin_reservations/list.rs` | `admin/reservation_list.rs`, `reservation_handlers.rs` | done: SQL-paginated with a total |
| Admin calendar | `stay-service/src/admin_calendar/` | none | open: inventory exposes admin calendar data, but a dedicated admin read model is not ported |
| Host analytics summary report | `stay-service/src/reports/` | `admin/report*.rs` | done: host and property scoped aggregates |
| Other reports | `stay-service/src/reports/` | none | open |
| Host applications and hosts | new in Mungini (D2) | `hosts/onboarding/admin.rs` | done: application list, detail, decision, document review, host list and status change; live requests verified |
| Admin seeds | `stay-service/seeds/` | B5, B7, B8, B9 and B12 seeds | done for dashboard, reservations, guests, applications and hosts |
| Legacy unit, database and concurrency tests | `stay-service/src/admin_guests/`, `admin_dashboard/`, `admin_reservations/` | unit tests and live checks | partial: SQLx tests for the domain modules run; no dedicated admin DB tests |

## Open

- Admin calendar read model and the remaining report screens are not ported.
- Dashboard night counts depend on inventory rows inside the selected range; seeded nights outside the period produce zero nights sold, which is correct but makes the occupancy figure look empty in short demos.
- No dedicated admin database tests exist; the current verification is compile, unit tests and live requests against the seeded dev database.
