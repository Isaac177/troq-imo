# Stay commission: prepaid credits plan

Owner decision: guests pay at the property, the platform never holds room money. The platform takes its commission when a booking is made, out of a prepaid credit balance each business holds in CDF. A business that cannot cover its commission cannot take bookings, and its listings are disabled in the app until credit is added.

This replaces the earlier settlement batch model (a business owing commission after check-out). Read `AGENTS.md`, `DESIGN.md` and `ADMIN_STAYS_ROADMAP.md` first. All project rules apply: no comments in code, 300 lines per file, no `cargo`, components hold JSX only, all strings in en and fr, money is CDF only, dates dd/mm/yyyy, forward-only migrations, seeds with `seed-` ids, every admin action audited, lists paginate in SQL.

Hard separation: stay credits and stay commission never read or write `payment-service`, the marketplace payment tables, `payment_commission.rs` or marketplace payout endpoints. Stay screens and labels say "stay" so an admin never confuses the two.

## 1. Decisions (confirmed, do not re-ask)

| Topic | Decision |
|---|---|
| Money model | Per business credit balance in CDF. Commission = rate % of the booking total, taken at booking time. |
| Instant bookings | Commission taken when the reservation is created (`create_hold`). |
| Request bookings | Commission taken when the business accepts (`decision.rs` ACCEPT). A decline or expiry never costs credit. |
| Insufficient credit | The booking is rejected. A booking that leaves the balance at exactly 0 is allowed. |
| Reversal | Cancelled, declined, expired: full commission refunded. Late guest cancellation or no-show: commission is kept only on the penalty charge, the rest is refunded. |
| Listings gate | Business with rate above 0% and balance at or below 0 is disabled: hidden from search and discovery, cannot be quoted or booked. Listings stay published, existing confirmed bookings are untouched, and the business re-enables automatically after a top-up. |
| 0% rate | Never gated, nothing is taken. A business with no rate row counts as 0%. First deploy therefore disables nothing until an admin sets a rate. |
| Top-ups | Admin records a top-up (amount, reason, payment reference) for money received outside the platform. Businesses cannot top up online. Not built: online top-up, because it needs a stay payment flow that does not exist. |
| Old batches | `stay_commission_batches` and `stay_commission_batch_items` stay in the database (forward-only), no longer used. Their Rust module files, routes, UI and seeds are removed (rule 18, no dead code). |
| Low balance | Stored as stay platform settings `LOW_CREDIT_CDF` (default 50 000) and `MIN_CREDIT_CDF` (default 0), read through `platform_settings::store`. Add the two keys with a new migration that extends the key check constraint of `stay_platform_settings` and the key list in `platform_settings/keys.rs` (code, default, range, unit CDF), plus their labels in the settings locale files. Open point: confirm the defaults with the owner. |

## 2. Current state (already built, keep)

Backend, `tzone-rs/stay-service`:
- Migrations 00047 (`stay_operator_commission_rates`), 00048 (reservation columns `commissionRateBps`, `commissionAmount`, `operatorNetAmount` plus an insert trigger that defaults net), 00049 (batch tables, now unused).
- `src/commission/` (`snapshot.rs`, `rate.rs`): rate lookup and the commission split. Keep.
- Quote stores the snapshot in its breakdown (`quotes/handler.rs`), `reservations/hold.rs` copies it into the reservation and the contract snapshot, `amendments/queries.rs` recomputes commission at the frozen rate.
- `src/admin_commission/`: `rate_history.rs`, `rate_set.rs`, `reasons.rs`, `access.rs` are kept. `batch_create.rs`, `batch_list.rs`, `batch_transition.rs` and the batch parts of `types.rs`, `subscriptions.rs` are replaced by the credit endpoints below.
- Operator list rows (`operator_detail/overview.rs`) carry `commissionRateBps`. Operator reservation view and admin reservation detail carry rate, commission and net.
- `admin_dashboard` returns `commissionExpected/Earned/Collected` and `payoutsPending/Paid`.
- Gateway `api-gateway/src/routes/admin/stay_commission.rs`: rate routes kept, batch routes replaced.

Admin app, `troq-front/troqzone-admin`:
- Business detail Commission tab (rate, history, change dialog with reason and confirm panel), rate column in the businesses table, commission and net on the reservation sheet, dashboard commission columns and KPIs, page `/stays/commission` (batches, to be replaced), nav entry, locales `*AdminStayCommission.json`.

Seeds: `stay-service/seeds/goma_commission.sql` (rates, reservation snapshots, 6 checked-out stays, batches to be removed).

Roadmap item 14 is built: table `stay_platform_settings` (00050), module `src/platform_settings/`, gateway `stay_settings.rs`, admin page `/stays/settings`, seed `seeds/goma_settings.sql`. Settings that exist: default commission rate, quote hold, request approval window, amendment response window, damage claim window, case response time. The default commission rate is the fallback when a business has no rate row.

Not compiled or run yet: the Rust has only been read through. The seeds have not been applied. No screen has been checked at 375px.

## 3. Two agents, no collisions

The lead (this session) already built the shared foundation so neither agent waits on the other. Do not edit foundation files except where a lane says so.

Foundation (done, read before coding):
- Migration `00051_stay_credits.sql`: tables `stay_credit_accounts`, `stay_credit_entries` (unique COMMISSION and REFUND per reservation), column `stay_reservations."commissionChargedAmount"`.
- Migration `00052_credit_thresholds.sql`: settings keys `MIN_CREDIT_CDF` (0) and `LOW_CREDIT_CDF` (50 000), registered in `platform_settings/keys.rs`, shown on `/stays/settings` in both locales.
- `src/credits/ledger.rs`: `apply(tx, NewEntry)` locks the account row, rejects a COMMISSION that would overdraw (`Conflict("insufficient credit")`), updates the balance and writes the entry in one path, returns the new balance in minor units. `entry_exists`, `next_balance`, `EntryType`.
- `src/credits/account.rs`: `balance_minor(pool, operator_id)` (missing account is 0).
- `src/credits/state.rs`: `disabled_sql(operator_column)` (one SQL boolean, true when the effective rate is above 0 and the balance is at or below `MIN_CREDIT_CDF`), `is_disabled`, `is_low`.
- `src/credits/mod.rs` already declares `charge`, `gate`, `mine` (empty files owned by the lanes below). Nobody edits `mod.rs` or `main.rs`.
- Dead-code warnings from foundation items disappear once the lanes consume them. A lane must not leave any of its own items unused.

Never run `cargo`, never start dev servers (admin 5173, business 5174, storefront 3001 are running), do not commit, do not deploy, do not switch branches (the folder is not a git repository). All project rules in `AGENTS.md` apply.

### Agent 1: booking side (Rust) then business app
Owns, and is the only one to edit:
- `stay-service/src/credits/charge.rs`, `credits/gate.rs`.
- `stay-service/src/reservations/**`, `amendments/**`, `quotes/**`, `search_engine/**`, `public_discovery/**`, and the `db_*_tests.rs` and `race_tests.rs` files that insert reservations.
- `troq-front/troqzone-business/**`.

Work, in order:
1. Section 4.2 items for `charge.rs` (`take_commission`, `release_commission` with the penalty rule, idempotent through `entry_exists`) and `gate.rs` (`ensure_can_book`, a SQL fragment helper built on `state::disabled_sql`).
2. Hooks in `hold.rs`, `decision.rs`, `cancel_guest.rs`, `operator_cancel.rs`, `expiry.rs`, `stay_flow.rs` (no-show), `amendments/respond.rs`, plus writing `cancellationChargeAmount` where missing.
3. Gate in `quotes/handler.rs`, `hold.rs`, `search_engine/`, `public_discovery/` (every list, detail and availability query).
4. Startup and test review from section 4.2 (fresh database, existing database, raw SQL inserts in tests).
5. Business app, section 7. The business endpoints are built by Agent 2 to the contracts in section 5. If an endpoint is not live yet, code to the contract and verify later.

### Agent 2: credits API, dashboard, seeds, admin app
Owns, and is the only one to edit:
- `stay-service/src/credits/mine.rs`, `admin_commission/**` (delete the batch files and batch types, add credit subjects, register them and `stay.credits.mine` in `admin_commission/subscriptions.rs`), `admin_dashboard/**`, `operator_detail/overview.rs`.
- `api-gateway/src/routes/admin/stay_commission.rs` (replace batch routes), a new gateway file for the business read route, and one-line additions to `mod.rs`.
- `stay-service/seeds/goma_commission.sql` and any new credit seed file.
- `troq-front/troqzone-admin/**` and the admin locale files.

Work, in order:
1. Section 4.2 admin module items: accounts list, summary, detail, ledger, top-up and adjust (using `credits::ledger::apply`, `credits::state::disabled_sql`, `platform_settings` thresholds), `stay.credits.mine`, audit entries.
2. Gateway routes from section 5, business route following the existing business stay route pattern.
3. Dashboard backend figures.
4. Seeds (section 4.2), balances equal to their ledger sums, `commissionChargedAmount` set consistently on seeded reservations.
5. Admin app, section 6, then `bunx tsc -p tsconfig.app.json --noEmit`.

### Shared contracts and hand-offs
- The JSON contracts in section 5 are fixed. If a lane must change one, it edits section 5 of this file and tells the other lane through the lead, not by editing the other lane's files.
- Agent 1 relies on nothing from Agent 2 to compile. Agent 2 relies only on foundation files to compile.
- Seeds from Agent 2 must satisfy the unique indexes of 00051 and use existing seed reservations only for COMMISSION and REFUND rows (one each per reservation at most).
- The lead runs the merge checks in section 8 after both finish.

## 4. Backend detail (split between Agent 1 and Agent 2 as in section 3)

The schema (4.1), `account.rs`, `ledger.rs` and the threshold settings are done by the lead. Agent 1 takes `charge.rs`, `gate.rs` and the booking hooks. Agent 2 takes the admin module, `mine.rs`, gateway, dashboard and seeds. Where the text below says `A`, read the lane named in section 3.

Migration numbers: check `ls stay-service/migrations` first, other agents add files. 00050 is taken by `stay_platform_settings` (roadmap item 14, built). 00051 onward is free at the time of writing.

### 4.1 Schema
Migration `stay_credit_accounts_ledger`:
- `stay_credit_accounts`: `operatorId` PK to `stay_operators`, `balance numeric(14,2) NOT NULL DEFAULT 0`, `version int`, `updatedAt`.
- `stay_credit_entries` (ledger, append only): `id`, `operatorId`, `entryType` (TOPUP, COMMISSION, REFUND, ADJUSTMENT), `amount numeric(14,2)` signed (credit positive, debit negative), `balanceAfter`, `reservationId` nullable, `reasonCode`, `paymentRef` nullable, `note` nullable, `createdByUserId` nullable (null means system), `createdAt`.
- Unique partial indexes: one COMMISSION entry per reservation, one REFUND entry per reservation. These make every hook idempotent.
- Index on `(operatorId, createdAt DESC)`.
- Reservation column `commissionChargedAmount numeric(14,2) NOT NULL DEFAULT 0` (what is currently taken for this reservation).
- Backfill: accounts with balance 0 for existing operators is not needed, a missing account counts as balance 0.

Penalty: `stay_reservations."cancellationChargeAmount"` already exists and the admin detail reads it. First verify that `cancel_guest.rs`, `operator_cancel.rs` and the no-show path actually write it. If not, write it there. Commission kept on a penalty = `percent_of(charge, commissionRateBps)`.

### 4.2 Code
New module `src/credits/` (files under 300 lines each):
- `account.rs`: `lock_account(tx, operator_id)` with `SELECT ... FOR UPDATE` after an upsert, `balance(pool, operator_id)`.
- `ledger.rs`: `apply(tx, entry)` writes the entry and updates the balance in one statement path, rejects a negative result for COMMISSION.
- `gate.rs`: `ensure_can_book(pool, operator_id, commission_minor)` for quotes, and `enabled_sql` (one SQL fragment: rate above 0 and balance at or below 0 means disabled) reused by search. Thresholds come from the platform settings above, not constants.
- `charge.rs`: `take_commission(tx, reservation_id)` and `release_commission(tx, reservation_id, keep_minor)`.

Wire the hooks, each inside the existing transaction:
- `reservations/hold.rs`: for INSTANT, `take_commission` in the same transaction as the insert. Reject with `ServiceError::Conflict("insufficient credit")` when the balance cannot cover it.
- `reservations/decision.rs` ACCEPT: `take_commission`. DECLINE: nothing was taken.
- `reservations/cancel_guest.rs`, `operator_cancel.rs`, `expiry.rs`, `stay_flow.rs` (NO_SHOW): `release_commission` with the penalty rule above.
- `amendments/respond.rs` accept: charge or refund the commission delta. Reject the accept when an increase cannot be covered.
- `quotes/handler.rs` (`create_quote`) and `hold.rs`: call `ensure_can_book` so a disabled business cannot be quoted or booked. The error code must be distinguishable so the storefront can say "unavailable".
- Search and discovery: add the gate to `search_engine/` and `public_discovery/` queries so disabled businesses are not listed. Read both before editing and apply the fragment to every list, detail and availability query, not only the main search.

Admin module `src/admin_commission/` (rename `reasons.rs` entries, delete batch files):
- Subjects `stay.admin.credits.accounts.list`, `.summary`, `.detail`, `.ledger`, `.topup`, `.adjust`.
- Top-up reasons: CASH_RECEIVED, MOBILE_MONEY_RECEIVED, BANK_TRANSFER, PROMOTIONAL_CREDIT. A payment reference is required except PROMOTIONAL_CREDIT. Adjustment reasons: ERROR_CORRECTED, GOODWILL, CHARGEBACK_REVERSAL, with direction CREDIT or DEBIT. Amount above 0 and under a sane cap. Every write audited (`credit.topup`, `credit.adjust`).
- Business facing read subject `stay.credits.mine` (balance, rate, enabled, low, ledger page) using the existing operator membership check pattern from `reservations/operator.rs`.

Dashboard (`admin_dashboard`): drop `commissionCollected` and the batch SQL. Replace with, all from SQL aggregates over the ledger in the date window: `commissionTaken` (COMMISSION minus REFUND), `creditsToppedUp` (TOPUP), `creditBalance` (sum of account balances, not windowed), `disabledBusinesses`. Keep `commissionExpected` only if it can be defined from real rows, otherwise remove it. Keep claim payouts as they are.

Gateway: replace the batch routes in `routes/admin/stay_commission.rs` with `/stays/credits/...` (section 5) and add the business read route following the pattern used by the other business stay routes. One-line additions in `mod.rs`.

Seeds: replace the batch part of `seeds/goma_commission.sql`. Keep rates and snapshots. Add accounts and ledger rows for every case: funded business, balance exactly 0, low balance, disabled business (rate above 0, no credit), refunded cancellation, partial refund on a no-show, promotional credit, debit adjustment. Add a second Goma business with a rate and no credit so a disabled listing exists in the demo. Keep ids `seed-*`, never break existing seed rows. Make sure the demo balances equal the sum of their ledger entries.

Startup check (the module must start first, with pay-at-property):
- Read every migration on a fresh database and on one that already ran 00047 to 00049.
- Confirm no startup code, consumer or route depends on `payment-service`.
- Check `db_tests.rs`, `race_tests.rs` and the `db_*_tests.rs` files that insert reservations with raw SQL still work with the new columns and the net trigger.
- Check that no warnings are produced: unused imports, dead fields, unused variables.

## 5. API contracts (Agent 2 builds the endpoints, Agent 1 and Agent 2 consume them)

Admin, base `/api/admin/stays`:
- `GET /credits/accounts?page&pageSize&search&operatorId&status` where `status` is ENABLED, DISABLED or LOW. Item: `{ operatorId, operatorName, rateBps, balance, enabled, low, lastTopUpAt }`. Page shape `{ items, total, page, pageSize }`.
- `GET /credits/summary?operatorId` returns `{ accounts, disabled, low, totalBalance, toppedUp30d, commissionTaken30d, refunded30d }`.
- `GET /credits/operators/{operatorId}` returns `{ operatorId, rateBps, balance, enabled, low, lowThreshold }`.
- `GET /credits/operators/{operatorId}/ledger?page&pageSize&type` returns a page of `{ id, entryType, amount, balanceAfter, reservationId, reservationReference, reasonCode, paymentRef, note, createdByUserId, createdAt }`.
- `POST /credits/operators/{operatorId}/topup` body `{ amount, reasonCode, paymentRef?, note? }`.
- `POST /credits/operators/{operatorId}/adjust` body `{ amount, direction, reasonCode, note? }`.
- Rate routes unchanged: `/commission/operators/{operatorId}/rate`, `/rate/history`.

Business, base `/api/stays` (exact path to be matched to the existing business stay routes by Agent A):
- `GET /credits/mine` returns `{ balance, rateBps, enabled, low, lowThreshold }`.
- `GET /credits/mine/ledger?page&pageSize` returns the ledger page without `createdByUserId`.
- Operator reservation views already carry `commissionRateBps`, `commissionAmount`, `netAmount`.

Amounts are CDF decimals as numbers on read and decimal strings on write, like the existing claims endpoints.

## 6. Admin app (Agent 2)

- Replace the settlement page `/stays/commission` with a credits page. Reuse `StayAdminPanel`, `StayAdminPager`, `StayAdminStatus`, `StayAdminDialog`, `StayAdminConfirm`, `StayCombobox`, `useStayAdminSearch`. Keep the route and nav key, retitle to "Stay credits" in both locales, update `utils/admin-stays/stayCommissionSearch.ts` (statuses ALL, ENABLED, LOW, DISABLED).
- Table: `#` first, business, rate, balance, status dot (enabled success, low amber, disabled destructive), last top-up date. Row opens a detail sheet with balance, ledger (paged) and the top-up and adjust actions. Top-up and adjust dialogs use a reason selector and the confirm panel. Business is never typed, it is picked from the options hook.
- KPI cards two by two on phone: balance total, businesses disabled, low, topped up 30d, commission taken 30d, refunded 30d.
- Business detail Commission tab: add a balance card above the rate card with a top-up button (icon-only below `sm`), the status dot, and the paged ledger.
- Businesses table: add balance and status dot next to the rate column.
- Dashboard: update types, KPIs and by-city and by-business columns to the new figures. Keep KPI cards two by two on phone and an even number of cards.
- Reservation detail sheet: already shows rate, commission and net. Add the commission taken and refunded state if the API exposes it.
- Delete the batch hooks, components, types and locale keys that are no longer used.
- Locale: new keys in the existing `*AdminStayCommission.json` files (rename the namespace only if needed), registered already in `i18n/locales/{en,fr}/admin/index.ts`.
- Finish with `bunx tsc -p tsconfig.app.json --noEmit` in `troqzone-admin`.

## 7. Business app (Agent 1)

Mobile first (AGENTS rule 22): design at 375px, body 12px, labels 11px, titles 15px, controls 36px or less, verb buttons icon-only below `sm`, the page never scrolls sideways.
- First read how the business stay pages, hooks and API client work and match them. Do not guess shared components from their names.
- Credit balance card on the stays home or properties page: balance, rate, status dot, and a low-credit banner when `low`.
- A disabled banner when `enabled` is false that says listings are hidden until credit is added and who to contact. No online top-up button.
- Credit history page: `#` column, type dot, amount, balance after, date, reservation reference. Server paged.
- Reservation views: show commission and net where the merchant sees the total (the fields already exist).
- All strings in en and fr in the business app's own i18n setup. Check each changed screen at 375px in English and French and name the screens in the report.
- Finish with `bunx tsc` in `troqzone-business` using its own tsconfig.

## 8. Order of work

1. Lead: foundation (done).
2. Agent 1 and Agent 2 in parallel, each in its own files (section 3).
3. Agent 1 finishes with the business app against the live endpoints.
4. Lead merge check: the admin type check again after all merges (AGENTS rule 10), then the screen checks last.

## 9. Acceptance

- Booking an instant stay with enough credit reduces the balance by exactly the snapshot commission and writes one COMMISSION entry. Booking again for the same reservation changes nothing.
- A booking that the balance cannot cover is rejected, no reservation, no allocation, no ledger entry.
- Cancel, decline, expire and no-show restore the right amount, once.
- A business with rate above 0 and balance 0 or below: absent from search and discovery, quote and booking rejected, confirmed bookings untouched. A top-up brings it back with no manual step.
- A 0% business is never gated.
- Ledger balances equal account balances for every seed account.
- Admin: credits page, business tab, top-up and adjust with reason and confirm, dashboard figures all from stored data.
- Business app: balance, banners, history, reservation commission and net, checked at 375px in English and French.
- No file over 300 lines, no comments, no warnings in new Rust, admin and business type checks clean.

## 10. Report format

Each agent ends with: files changed, services and apps to restart, migrations and seeds to apply, screens checked at 375px in each language, and anything undone in the form "X: not built, because Y". Never fill a gap with made-up figures.

## 11. Open points for the owner

- `LOW_CREDIT_CDF` and `MIN_CREDIT_CDF` defaults (50 000 and 0).
- Whether a business with a rate above 0 and balance exactly 0 may take one more booking before disabling (current decision: yes, a booking may leave the balance at exactly 0).
- Online top-up for businesses: not in this plan, needs a stay payment flow.
