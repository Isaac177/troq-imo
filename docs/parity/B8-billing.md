# B8 billing parity

D5 default applies: Mungini has its own `billing` module. D1 and D2 defaults: legacy operators are Mungini hosts (`hosts`), and legacy business ids in routes become `hostId`. Rust and SQL were read-checked only; cargo, sqlx and the migration were not run, and the ported tests were not run.

| Legacy location | Mungini location | Status |
|---|---|---|
| `stay-service/src/commission/snapshot.rs` `CommissionSnapshot` and tests | `billing/commission/snapshot.rs`, amounts are kernel `Money` (wire decimal string) | done |
| `commission/rate.rs` `configured_rate_bps`, `current_rate_bps`, `snapshot_for` | `billing/commission/rate.rs`, now takes the city for the default-rate setting | done |
| `credits/ledger.rs` `apply`, `next_balance`, `entry_exists`, tests | `billing/credits/ledger.rs`, writes `city_id` on accounts and entries | done |
| `credits/charge.rs` `take_commission`, `release_commission`, `release_for_penalty`, `sync_amended_commission`, `commission_kept`, test | `billing/credits/charge.rs`, same signatures B5 calls through `reservations/links/billing.rs`; city read from the reservation row | done |
| `credits/gate.rs` `ensure_can_book`, `enabled_sql` | `billing/credits/gate.rs`, `billing/credits/state.rs`; the city is read from `hosts.city_id` so the signature is unchanged | done |
| `credits/state.rs` `disabled_sql`, `is_disabled`, `is_low`, tests | `billing/credits/state.rs`, `disabled_sql` takes a city column because settings are per city | done |
| `credits/account.rs` | `billing/credits/account.rs` | done |
| `credits/mine.rs` ledger query | `billing/credits/entries.rs` | done |
| `credits/mine.rs` summary and ledger, NATS `stay.credits.mine`, `stay.credits.mine.ledger`, gateway `routes/stays/credits.rs`, merchant routes `/credits/mine`, `/credits/mine/ledger` | `billing/credits/host_view.rs`, `GET /api/host/credits?hostId=`, `GET /api/host/credits/ledger?hostId=&page=&pageSize=` | done |
| Permission `STAY_VIEW_FINANCE` on host reads | `billing.read` (kernel `LEGACY` map), membership check first (NOT_FOUND) then host-scoped grant | done |
| `admin_commission/rate_history.rs` `current`, `history`; NATS `stay.admin.commission.rate.current`, `.rate.history`; gateway `GET /stays/commission/operators/{id}/rate`, `/rate/history` | `billing/admin/rates.rs`, `GET /api/admin/commission/hosts/:hostId/rate`, `.../rate/history` | done |
| `admin_commission/rate_set.rs`; NATS `stay.admin.commission.rate.set`; gateway `POST .../rate` | `billing/admin/rates.rs`, `POST /api/admin/commission/hosts/:hostId/rate`, audit `commission.rate_set` | done |
| `admin_commission/credit_accounts.rs` list, summary, detail; NATS `stay.admin.credits.accounts.list`, `.summary`, `.detail`; gateway `/stays/credits/accounts`, `/summary`, `/operators/{id}` | `billing/admin/accounts.rs`, `GET /api/admin/credits/accounts`, `/summary`, `/hosts/:hostId`; filtered to the route city in SQL | done |
| `admin_commission/credit_ledger.rs`; NATS `stay.admin.credits.accounts.ledger`; gateway `/operators/{id}/ledger` | `billing/admin/ledger.rs`, `GET /api/admin/credits/hosts/:hostId/ledger?type=` | done |
| `admin_commission/credit_write.rs` topup and adjust; NATS `.topup`, `.adjust`; gateway `POST /operators/{id}/topup`, `/adjust` | `billing/admin/credit_writes.rs`, `POST /api/admin/credits/hosts/:hostId/topup`, `/adjust`, audits `credit.topup`, `credit.adjust` | done |
| `admin_commission/reasons.rs` reasons, `parse_rate_bps`, `parse_amount_minor`, tests | `billing/validators.rs`; failures are typed error codes (see below) | done |
| Admin gate `ensure_platform_moderator` (`STAY_MODERATE_LISTINGS`) | `scope::require_staff`, platform grant `properties.moderate`, same as B4 settings | open: owner decision whether admin billing should move to `billing.read` and `billing.settlements` (both exist in kernel) once staff presets carry them |
| `admin_commission/subscriptions.rs` NATS registration | none, replaced by HTTP routes | dropped by D5 (no NATS) |
| Gateway `ensure_writable` on admin writes | kernel `Actor` extractor rejects writes in view-only city mode | done |
| Migration `00047_commission_rates.sql` | `backend/migrations/00018_billing.sql` | done |
| Migration `00048_reservation_commission.sql` reservation columns and net trigger | already in `00012_reservations.sql` (B5) | done by B5, not duplicated |
| Migration `00049_commission_settlements.sql` batches and items | `00018_billing.sql` | done for schema; open for behaviour: legacy has no code and no seed that creates, approves, pays or fails a batch |
| Migration `00051_stay_credits.sql` accounts, entries, indexes | `00018_billing.sql`; `commissionChargedAmount` already in `00012_reservations.sql` | done |
| Migration `00052_credit_thresholds.sql` settings keys | `00019_pricing.sql` (B4 owns `stay_platform_settings`) | done by B4, billing only reads it |
| Seeds `goma_commission.sql`, `kin_commission.sql`, `goma_admin_rates.sql`, `goma_credits.sql` | `backend/seeds/billing/01` to `04`, listed in `00-order.txt` | done; ids renamed to `seed-host-*`, `seed-res-bill-*` |
| Test `reservations/db_credit_tests.rs` | `billing/credits/db_tests.rs` using `sqlx::test` and the migrator | done, not run |
| Tests in `snapshot.rs`, `ledger.rs`, `state.rs`, `charge.rs`, `reasons.rs` | same files in Mungini, plus `amounts.rs` and `validators.rs` | done, not run |
| Reservation event subscription | none | not needed: legacy takes and releases commission by direct calls inside the reservation transaction; B5 does the same through `reservations/links/billing.rs` |
| Events emitted | none (legacy emitted none) | done |
| Notifications for low or disabled credit | none in legacy billing code | done |
| Quote-time commission and `ensure_can_book` for B4 | `QuoteChecks` is a trait in `pricing/quotes/checks.rs`; billing exposes `commission::rate::snapshot_for` and `credits::gate::ensure_can_book` | open: the `QuoteChecks` implementation that calls them is not written (it needs B4 and B5 wiring) |

## Differences from legacy

- Amounts on the wire are decimal strings (kernel rule). Legacy returned floats. Ledger amounts and balances can be negative, and kernel `Money::to_wire` formats negatives wrongly, so billing has its own signed formatter (`billing/amounts.rs`). Kernel `Money` also cannot parse negative values, so only magnitudes are accepted as input.
- Legacy messages became error codes: `HOST_REQUIRED`, `HOST_NOT_FOUND`, `RESERVATION_NOT_FOUND`, `RATE_INVALID`, `RATE_REASON_UNKNOWN`, `RATE_UNCHANGED`, `TOPUP_REASON_UNKNOWN`, `ADJUST_REASON_UNKNOWN`, `DIRECTION_UNKNOWN`, `ENTRY_TYPE_UNKNOWN`, `CREDIT_STATUS_UNKNOWN`, `NOTE_TOO_LONG`, `PAYMENT_REF_TOO_LONG`, `PAYMENT_REF_REQUIRED`, `CREDIT_LISTING_UNAVAILABLE`, `CREDIT_INSUFFICIENT`, plus kernel `AMOUNT_INVALID`.
- One database now serves both cities, so admin reads and writes filter by `hosts.city_id` of the route city. The admin ledger now returns NOT_FOUND for a host in another city; legacy had no such case.
- Routes use `hosts` and `hostId` instead of `operators` and `businessId`. The admin account list returns `hostId` and `hostName`.
- Bug kept as is: `set` rate checks only the configured rate for equality, so setting the rate to the platform default when none is configured is accepted. Rate changes and balance reads have no lock between check and insert.
- Bug kept as is: `credits::gate::ensure_can_book` and the account list read the balance without locking, so concurrent bookings are only guarded by the locked `ledger::apply` at confirmation time.
- Host reads also check that the host belongs to the route city (not in legacy).

## Schema contract

All tables keep legacy names and quoted camelCase columns, add `city_id text NOT NULL CHECK (city_id IN ('GOMA','KIN'))`, and reference `hosts(id)` in place of `stay_operators(id)`. Migration `00018_billing.sql`.

| Table | Columns | Key and checks |
|---|---|---|
| `stay_operator_commission_rates` | `id`, `city_id`, `operatorId`, `rateBps int`, `effectiveFrom timestamptz default now()`, `reasonCode varchar(32)`, `note text null`, `changedByUserId`, `createdAt` | pk `id`; `rateBps` 0 to 10000; note at most 500 chars; index (`operatorId`, `effectiveFrom desc`, `createdAt desc`) |
| `stay_commission_batches` | `id`, `city_id`, `reference varchar(24) unique`, `operatorId`, `periodFrom date`, `periodTo date`, `reservationCount int`, `commissionTotal numeric(14,2)`, `state` in PENDING, APPROVED, PAID, FAILED, `paymentRef null`, `reasonCode null`, `createdByUserId`, `stateChangedByUserId null`, `version`, `createdAt`, `updatedAt` | PAID requires `paymentRef`; `periodTo >= periodFrom`; count at least 1 |
| `stay_commission_batch_items` | `id`, `city_id`, `batchId`, `reservationId unique`, `operatorId`, `rateBps`, `baseAmount`, `commissionAmount`, `earnedAt` | fk batch and `stay_reservations` |
| `stay_credit_accounts` | `operatorId` pk, `city_id`, `balance numeric(14,2)`, `version`, `updatedAt` | fk `hosts` |
| `stay_credit_entries` | `id`, `city_id`, `operatorId`, `entryType` in TOPUP, COMMISSION, REFUND, ADJUSTMENT, `amount`, `balanceAfter`, `reservationId null`, `reasonCode null`, `paymentRef null`, `note null`, `createdByUserId null`, `createdAt` | sign per type; reservation set exactly for COMMISSION and REFUND; one COMMISSION and one REFUND per reservation (partial unique indexes); index (`operatorId`, `createdAt desc`, `id desc`) |

Read dependencies owned elsewhere: `hosts` (B1), `stay_reservations` with `commissionRateBps`, `commissionAmount`, `commissionChargedAmount`, `cancellationChargeAmount` (B5, `00012`), `stay_platform_settings` keys `DEFAULT_COMMISSION_BPS`, `MIN_CREDIT_CDF`, `LOW_CREDIT_CDF` (B4, `00019`).

## Seeds

`backend/seeds/billing/`: `01-hosts-rates.sql` (rate history with a past, current and future rate, a zero rate, both cities), `02-reservations.sql` (minimal reservations `seed-res-bill-*`; their `propertyId` values are placeholders because no foreign key exists yet), `03-credits.sql` (TOPUP in all four reasons, COMMISSION, full and partial REFUND, ADJUSTMENT credit, debit and amendment, accounts that are healthy, low, zero and without a row), `04-batches.sql` (PENDING, APPROVED, PAID, FAILED in both cities).
