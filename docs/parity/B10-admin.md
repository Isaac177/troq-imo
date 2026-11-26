# B10 admin read models

Sources are `tzone-rs` main at `780396f`. Mungini uses one database, so city filters, totals and pagination run in SQL. These read models use platform-scoped permissions and return CDF amounts as decimal strings. Rust and SQL are read-checked only; nothing has been compiled or run.

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Guest booking history | `stay-service/src/admin_guests/bookings.rs`, `api-gateway/src/routes/stays/admin_guests.rs` | `backend/crates/domain/src/admin/guest_bookings.rs`, `handlers.rs` | open: query and API registry wired; database and authorization verification required |
| Guest list and account profiles | `stay-service/src/admin_guests/list.rs`, `profiles.rs` | none | open: identity profile and searchable join required |
| Dashboard summary and city series | `stay-service/src/admin_dashboard/{summary,figures,credit_figures,series}.rs` | `backend/crates/domain/src/admin/dashboard*.rs` | open: aggregate endpoint and daily series implemented, owner compile and database verification required |
| Host dashboard breakdown | `stay-service/src/admin_dashboard/businesses.rs` | none | open |
| Admin reservations list and summary | `stay-service/src/admin_reservations/{list,summary,filters}.rs` | `backend/crates/domain/src/admin/reservation_*.rs` | open: city-scoped SQL list, summary, cover lookup and filters implemented; owner compile, database and route verification required |
| Admin reservation detail | `stay-service/src/admin_reservations/detail.rs` | `admin/reservation_detail*.rs` | open: route and core, child collections, unit images and CDF strings implemented; owner compile and database verification required; cancel and preview belong to B5 |
| Admin reservation export | `stay-service/src/admin_reservations/list.rs`, `api-gateway/src/routes/admin/stay_reservations.rs` | `admin/reservation_list.rs`, `reservation_handlers.rs` | open: wired and SQL-paginated; owner compile and database verification required |
| Admin calendar | `stay-service/src/admin_calendar/` | none | open |
| Host analytics summary report | `stay-service/src/reports/{handler,queries,types}.rs`, `api-gateway/src/routes/stays/reports.rs` | `admin/report*.rs` | open: scoped host/property and city SQL aggregates wired; owner compile and database verification required |
| Other reports | `stay-service/src/reports/` and gateway routes | none | open |
| Admin guest, reservation and dashboard seeds | `stay-service/seeds/` | B5 reservation seeds | open: existing B5 seeds support guest history, but full B10 coverage is not audited |
| Legacy unit, database and concurrency tests | `stay-service/src/admin_guests/`, `admin_dashboard/`, `admin_reservations/`, `admin_calendar/`, `reports/` | none | open |

The guest history requires `properties.moderate` at platform scope, matching the existing Mungini admin case handlers' platform authorization. Its explicit city filter defaults to the actor's city, and it rejects a city the server does not serve. Guest account detail remains unavailable until identity supplies it. The endpoint is registered in the API, but the current API identity directory is closed.

Dashboard summary uses the configured cities, SQL aggregates, per-city local-day grouping and CDF decimal strings. The original cross-city `operatorId` filter is exposed as `hostId`; host breakdown remains open. Existing B5, B8 and inventory seed files supply representative dashboard data; complete B10 seed parity remains open.

Reservation list and summary follow legacy filters, with `hostId` replacing `operatorId`, city scoping and CDF decimal strings. Summary ignores the state filter as in legacy. B5 reservation seeds supply examples; complete B10 seed parity remains open.

Admin export returns the same filtered rows without cover URLs, as the legacy export does. It uses `page` and `pageSize` and returns a total instead of fetching a fixed maximum of 5,000 rows, to meet the server-pagination rule; clients must request successive pages. Existing reservation seed data covers export rows.

Admin reservation detail paginates each embedded child collection and unit-type images using `page` and `pageSize`, with separate totals, instead of the legacy unbounded arrays. It preserves payment state, CDF amounts, audit, guest, item, amendment and case fields. Existing B5 seed data supplies example detail records; template media must be present in storage for signed images. Owner compilation and database, signed URL and API integration tests remain open.

The host analytics summary uses host membership plus scoped `analytics.read`, verifies that any property belongs to the host and requested city, and applies legacy check-in date filters in SQL. Gross revenue remains a CDF decimal string. Existing B5 and B7 seeds provide reservation and review cases; owner compilation and database checks remain open.
