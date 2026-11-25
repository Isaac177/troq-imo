# Slice 1 contract proposals

Status: proposal for owner review. Not approved, not implemented. Schema, enum migrations and money behavior stay blocked until D01-D03, D12 (identity) and D06-D11, D19 (money) are answered in [00-decisions.md](00-decisions.md).

Sources checked: `tzone-rs/auth-service/migrations/00001_types.sql` (UserRole, Permission), `00008_affiliate_program.sql`, `auth-service/src/jwt.rs`, `payment-service/migrations/00007_affiliate_program.sql`, payment-service `order_id` usage.

## P-R03: Permission enum additions

Current `Permission` values are all coarse (MANAGE_PAYMENTS, MANAGE_BUSINESSES, VIEW_ANALYTICS, and so on). None of the stay permissions in [01-roles-permissions.md](01-roles-permissions.md) R03 exist.

Proposal:

- Add each R03 name as a new `Permission` value through one forward migration in `auth-service/migrations/00011_*.sql`, using `ALTER TYPE ... ADD VALUE IF NOT EXISTS`.
- Do not reuse MANAGE_PAYMENTS, MANAGE_BUSINESSES or MANAGE_ORDERS as fallbacks for stay checks (R03 requires this).
- Postgres does not let a new enum value be used in the same transaction that adds it (B05). The migration only adds values. Any code using them ships in a later deploy.
- Propagate names to the Rust validation list, the gateway, and the TS types that mirror the enum. Two TS vocabularies already disagree (R intro), so pick one source and generate the other.

## P-R01: UserRole addition

- Add `stay_manager` to `UserRole` in `auth-service/migrations/00012_*.sql`, after `affiliate_partner` (00008 added that value the same way).
- Existing consumers that check roles by string (for example `stats_company.rs` uses buyer_manager, buyer_employee, merchant_manager) are unaffected.
- stay_manager grants no data access on its own (R01). Access comes only from membership grants (P-R02).

## P-R02: Grant storage and token claims

- New auth table `scoped_grants`: id, subject_user_id, permission, scope_type (PLATFORM | CITY | OPERATOR | PROPERTY), scope_id (nullable for PLATFORM), status, granted_by, granted_at, revoked_at, grant_version. Unique on active (subject, permission, scope_type, scope_id).
- JWT stays small. `Claims` keeps role and company_id for existing consumers. Stay authorization is resolved from `scoped_grants` on each sensitive request, with a version check against a cache (R05). The current `permissions` claim is not extended to carry stay scopes, because one list cannot express multiple property scopes.

## P-F01: Stay payment reference (contract extension)

Payment-service today is keyed on `order_id` (about 137 references in its source). Proposal:

- Add `resource_type` (default `ORDER`) and `resource_id` to the payment-intent path, plus `city`. Retail rows keep `ORDER` and their current `order_id`. No fake orders are created for stays.
- New tables in `payment-service/migrations/00011_*.sql`:
  - `payment_obligations`: id, resource_type, resource_id, city, payer_user_id, beneficiary_operator_id, kind, amount_cdf numeric(14,2), currency CHECK ('CDF'), due_at, accepted_contract_version, idempotency_key UNIQUE, status.
  - `payment_attempts`: id, obligation_id FK, provider, provider_reference, status (CREATED | PENDING | SUCCEEDED | FAILED | CANCELLED | UNKNOWN), created_at. Retries create attempts, never obligations.
- Success is recorded only after server-side provider verification (amount, currency, reference). Redirect parameters never confirm payment (SEC04).
- Blocked on D06 (schedules) and D19 (who may refund). Only the shape is proposed here.

## P-F03: Ledger extension

Existing data: `affiliate_earnings_ledger` is single-entry (one signed amount per row). It cannot prove balance for accommodation money, so a separate double-entry set is proposed rather than reusing it.

- `ledger_accounts`: code (CASH_CLEARING, GUEST_LIABILITY, OPERATOR_PAYABLE, PLATFORM_COMMISSION, REFUND_PAYABLE, DEPOSIT_LIABILITY), normal_side.
- `ledger_journals`: id, source_type, source_id, reverses_journal_id (nullable), posted_at, idempotency_key UNIQUE.
- `ledger_lines`: id, journal_id, account_id, direction (DEBIT | CREDIT), amount_cdf numeric(14,2) CHECK (amount_cdf > 0).
- Balance: enforced inside the posting transaction (sum of debits equals sum of credits per journal), and rechecked by a nightly reconciliation (F06).
- Corrections are reversal journals. Historical rows are never updated.
- Posting and the outbox row are written in one transaction (F03).
- Amounts match the existing numeric(12,2) convention, widened for totals. Exact CDF minor-unit handling is still open; confirm before schema.

## Open questions for the owner

1. Which identity model applies (D02, D03). Grant scope fields depend on it.
2. Whether accommodation money stays inside payment-service, as proposed, or moves to its own ledger service (B01 says payment-service).
3. Whether `Permission` values should be generated from one shared list in Rust and TS (proposed) or edited by hand in each place.
