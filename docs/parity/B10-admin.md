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
| Admin reservation export | `stay-service/src/admin_reservations/list.rs` | `admin/reservation_list.rs`, `reservation_handlers.rs`, `apps/admin/src/{views,hooks,services,utils}/reservations` | implemented: SQL-paginated API with a total, and the admin toolbar export builds the CSV in the browser from the filtered, paged results. Type-checked only; not exercised against a live API request or checked at 375px |
| Admin calendar | `stay-service/src/admin_calendar/`, `stay-service/src/calendar/handler.rs` | `inventory/calendar/{handler,service,view,queries}.rs`, `GET /api/admin/calendar` | implemented: platform and city scoped, paginated units with reservations, blocks, restrictions and prices; database and live API verification remain open; inventory response type renamed `InventoryCalendarView` to eliminate a collision with guest `CalendarView`, generated client awaits owner rebuild |
| Host analytics summary report | `stay-service/src/reports/` | `admin/report*.rs` | done: host and property scoped aggregates |
| Other reports | `stay-service/src/reports/` | platform summary below | open: additional report coverage not yet assessed |
| Platform city report | host summary report adapted for platform staff | `admin/platform_report.rs`, `apps/admin/src/{views,hooks,services,types}/reports` | open: SQL aggregates and check-in window, permission-gated admin page and bilingual labels implemented; existing reservation and review seeds cover report states. API contract regenerated; verification remains; Rust and SQL read-checked only |
| Host applications and hosts | new in Mungini (D2) | `hosts/onboarding/admin.rs` | done: application list, detail, decision, document review, host list and status change; live requests verified |
| Admin review moderation | `stay-service/src/reviews/admin_list.rs`, `moderation.rs`, gateway `admin/stay_review_handlers.rs` | `reviews/admin_list.rs`, `reviews/moderation.rs`, `apps/admin/src/{views,components,hooks,services,types}/reviews` | implemented: paginated city reviews with status, rating and search filters, guest and reservation columns, publish or hide with a reason code. Live list and filter requests verified; decision writes are implemented, runtime and 375px checks remain open |
| Admin cases | `stay-service/src/cases/admin.rs`, `admin_actions.rs`, gateway `admin/stay_claims.rs` | `reservations/cases/admin.rs`, `admin_actions.rs`, `apps/admin/src/{views,components,hooks,services,types}/cases` | implemented: paginated city cases with state, type, overdue and search filters; drawer with reason, deadlines, claim amount, escalation, resolution and evidence preview; close and escalate actions. Live list, filter, detail and evidence requests verified. Assignment, close and escalate actions are implemented; claim decisions, payouts, runtime and 375px checks remain open |
| Case assignee directory and validation | new for the admin assignment selector | `reservations/cases/assignees.rs`, `http/admin.rs`, `cases/admin_actions.rs`, `apps/admin/src/{components,hooks,services,types}/cases` | open: paginated searchable eligible platform staff directory, assignment validation and selector implemented; API contract regenerated; verification remains open; Rust and SQL read-checked only |
| Admin listing moderation | `stay-service/src/listings`, gateway `admin/stays.rs` listing routes | `properties/listings/admin_handler.rs`, `review.rs`, `apps/admin/src/{views,components,hooks,services,types}/listings` | implemented: submission queue with status filter, live versus proposed content comparison, and approve, request-changes or reject decisions with reason codes and guidance. Live queue and detail requests verified. The generated client uses `ListingDecisionInput`; decision writes are implemented, runtime and 375px checks remain open |
| Admin damage claims | `stay-service/src/admin_claims`, gateway `admin/stay_claims.rs` claim routes | `reservations/claims/{list,detail,decide,payout,statements}.rs`, `apps/admin/src/{views,components,hooks,services,types}/claims` | implemented: CDF summary, paginated list with status and search, detail with evidence and statements, statement capture for host or guest, approve, partial-approve and reject decisions with reason codes, and payout state with reference. Live summary, list and detail requests verified. Decision, payout and statement writes are implemented; runtime and 375px checks remain open |
| Admin properties | `stay-service/src/properties`, `admin_rate_tools`, gateway `admin/stays.rs` property routes | `properties/listings/admin_handler.rs`, `apps/admin/src/{views,components,hooks,services,types}/properties` | implemented: paginated property list with status and search, detail drawer with content, host, counts, amenity names and guest contact, suspend with reason and reinstate. Live list and detail requests verified. Suspend and reinstate writes are implemented; direct content editing, media and unit type admin screens, runtime and 375px checks remain open |
| Admin notifications | messaging-service admin inbox | `notifications` routes, `apps/admin/src/{views,components,hooks,services,types}/notifications` | implemented: paginated admin inbox with unread filter and count, mark one/all read, translated types and priorities, city-time timestamps and internal deep links. Type-checked only; live requests, runtime and 375px checks remain open |
| Admin seeds | `stay-service/seeds/` | B5, B7, B8, B9 and B12 seeds; `reservations/07-reservation-history.sql` for timeline rows | done for dashboard, reservations, guests, applications, hosts and timeline; the timeline seed must be applied by the owner |
| Legacy unit, database and concurrency tests | `stay-service/src/admin_guests/`, `admin_dashboard/`, `admin_reservations/` | unit tests and live checks | partial: SQLx tests for the domain modules run; no dedicated admin DB tests |

## Open

- Admin calendar read model is implemented in inventory; database and live API verification remain open. The platform report summary is implemented; additional report coverage remains open.
- Cancel and claim actions on a case are still platform screens outside this drawer; the reservation detail links are display only.
- Reservation timeline demo rows are added by `backend/seeds/reservations/07-reservation-history.sql`, which the owner must apply before the timeline shows seeded activity.
- Dashboard night counts depend on inventory rows inside the selected range; seeded nights outside the period produce zero nights sold, which is correct but makes the occupancy figure look empty in short demos.
- No dedicated admin database tests exist; the current verification is compile, unit tests and live requests against the seeded dev database.

The admin calendar screen is implemented against the regenerated `InventoryCalendarView` contract. Property search and unit pages are server-paginated; date range is limited to 93 nights. Live API and screen behavior and 375px English/French layouts remain unverified.

## Operations tasks

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Platform task list with host, property, state, type, priority, assignee and search filters | `troqzone-admin/src/components/admin/stays/tasks/StayTasksTable.tsx` and legacy admin task pages | `apps/admin/src/views/tasks/TasksView.tsx`, `hooks/tasks/useAdminTasks.tsx`, `GET /api/admin/tasks` | done against the contract; runtime not checked |
| Task creation with property, type, priority and checklist | `StayTaskCreateSheet.tsx` | `components/tasks/TaskCreateDialog.tsx`, `POST /api/admin/tasks` | done; property is API-driven and required |
| Start, complete and cancel a task | `StayTaskStatus.tsx`, legacy admin task actions | `components/tasks/TaskActionsCell.tsx`, `POST /api/admin/tasks/{taskId}/transition` | done; platform staff are not offered assign-to-me |
| Row number, property and host, task type, unit or reservation, state, priority, assignee and city-time date | `StayTasksTable.tsx` | table columns in `useAdminTasks.tsx` | done; secondary columns hide below `sm` |
| Permission gating | legacy platform task role | module and every route require `properties.moderate` | done |
| Seed states | `stay-service/seeds` tasks | `backend/seeds/reservations/04-cases-tasks-kin.sql`, `05-tasks-goma.sql` | done; every state and priority is represented |

Tasks limits:

- Type-checked only. Live requests, mutations, 375px and dark-mode checks were not run.
- The assignee filter offers all or unassigned only because the contract exposes no paginated platform staff directory for tasks.
- Registry entry and route were wired by the integrator after the slice was written.

## Credits and commission

| Legacy feature | Legacy location | Mungini location | Status |
|---|---|---|---|
| Credit summary cards | `troqzone-admin/src/components/admin/stays/credits/StayCreditsKpis.tsx`, `hooks/admin-stays/useAdminStayCredits.ts` | `apps/admin/src/components/credits/CreditsSummary.tsx`, `GET /api/admin/credits/summary` | done against the contract; runtime not checked |
| Host credit account list with search, status filter and pagination | `StayCreditsTable.tsx`, `StayCreditsToolbar.tsx` | `apps/admin/src/views/credits/CreditsView.tsx`, `hooks/credits/useAdminCredits.tsx`, `GET /api/admin/credits/accounts` | done |
| Host credit drawer with balance, threshold, state and rate | `StayCreditSheet.tsx`, `StayCreditBalance.tsx` | `components/credits/CreditDetailDrawer.tsx`, `hooks/credits/useCreditDetail.ts` | done |
| Credit ledger with type, amount, balance after, note and reservation reference | `StayCreditLedger.tsx`, `hooks/admin-stays/useStayCreditLedger.ts` | `components/credits/CreditLedger.tsx`, `GET /api/admin/credits/hosts/{hostId}/ledger` | done |
| Top up and adjust a host balance with a reason | `StayCreditEntryDialog.tsx`, `hooks/admin-stays/useStayCreditEntryDialog.ts` | `components/credits/CreditEntryDialog.tsx`, `hooks/credits/useCreditEntryDialog.ts` | done; reason codes match the backend validator |
| Current commission rate and rate history | `hooks/admin-stays/useStayOperatorCommission.ts`, `useStayCommissionRateDialog.ts` | `components/credits/CommissionRateHistory.tsx`, `hooks/credits/useCommissionRateHistory.tsx` | done |
| Change the commission rate with a reason | legacy rate dialog | `components/credits/CommissionRateDialog.tsx`, `hooks/credits/useCommissionRateDialog.ts`, `POST /api/admin/commission/hosts/{hostId}/rate` | done |
| Permission gating | legacy platform finance role | every route requires platform `properties.moderate`; the module uses the same permission | done |
| Phone layout | legacy credit screens | summary two by two, host and balance stay visible, secondary columns hide below `sm` | implemented, 375px runtime check not run |
| Seed data | `stay-service/seeds` credits and commission | `backend/seeds/billing/01-hosts-rates.sql`, `03-credits.sql` | done; enabled, low, disabled and zero-balance hosts plus rate history |

Credits limits:

- Type-checked only. Live requests, mutations, 375px and dark-mode checks were not run.
- The summary is read together with the account page, so a failed summary also shows the list error state.
