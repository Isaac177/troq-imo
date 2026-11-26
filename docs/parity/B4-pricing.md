# Parity ledger B4, pricing

Source on main: `tzone-rs/stay-service/src/{rate_admin,admin_rate_tools,quotes,platform_settings}`, gateway `routes/stays/{rate_admin,quotes,admin_rates,admin_unit_types}.rs`, `routes/admin/{stay_rates,stay_settings}.rs`, `routes/merchant/stays.rs`, migrations 00016, 00017, 00032, 00042, 00050 (and 00001 to 00003, 00052 for the tables). Target: `backend/crates/domain/src/pricing`, migration `00019_pricing.sql`, seeds `backend/seeds/pricing`. Status values: done, dropped by decision D#, open. Rust is read-checked only, never compiled; tests are written, not run.

Defaults followed: D1 and D2 (Actor scope `Host`, host id is the legacy `"operatorId"` column, hosts table is `hosts`), D3 CDF only, D4 one database with `city_id`. Amounts are integer hundredths in Rust (`Money` at the edges, `i64` minor inside the pure engines) and decimal strings on the wire and in SQL (`::text::numeric`), unchanged from legacy.

## Endpoints

HTTP shape changes: NATS subject plus gateway route became one axum handler. Hosts use `/api/host`, platform staff `/api/admin`, guests `/api/guest`. Legacy `POST .../update` and `POST .../remove` admin routes became `PATCH` and `DELETE`, the admin funding `POST` became `PUT`. Legacy admin routes at `/stays/rates/{propertyId}/...` now share the `/properties/{propertyId}/...` paths of the host side under `/api/admin`. Success responses use the kernel envelope, errors use typed codes (`messages.rs`).

| Legacy (gateway route, NATS subject) | Mungini | Status |
|---|---|---|
| `GET /merchant/stays/properties/{id}/rate-plans`, `stay.rate.plans.list` | `GET /api/host/properties/{propertyId}/rate-plans`, `rates/plans.rs list_plans` | done |
| `POST` same path, `stay.rate.plan.create` | `POST /api/host/properties/{propertyId}/rate-plans`, `create_plan` | done |
| `PATCH .../rate-plans/{id}`, `stay.rate.plan.update` | `PATCH /api/host/.../rate-plans/{ratePlanId}`, `update_plan` | done |
| `GET .../rate-plans/{id}/rules`, `stay.rate.rules.list` | `GET /api/host/.../rules`, `rules.rs list_rules` | done |
| `POST` same, `stay.rate.rule.create` | `POST /api/host/.../rules`, `create_rule` | done |
| `DELETE .../rules/{ruleId}?idempotencyKey`, `stay.rate.rule.remove` | `DELETE /api/host/.../rules/{ruleId}`, `remove_rule` | done |
| `GET /merchant/stays/properties/{id}/promotions`, `stay.promotion.list` | `GET /api/host/properties/{propertyId}/promotions`, `promotions/service.rs` | done |
| `POST` same, `stay.promotion.create` | `POST /api/host/.../promotions` | done |
| `PATCH .../promotions/{id}`, `stay.promotion.update` | `PATCH /api/host/.../promotions/{promotionId}` | done |
| `GET /merchant/stays/rate-policies`, `stay.policy.list` | `GET /api/host/rate-policies`, `rates/policies.rs` | done |
| `POST /merchant/stays/price-preview`, `stay.pricing.preview` | `POST /api/host/price-preview`, `quotes/preview.rs` | done |
| `POST /stays/quotes`, `stay.quote.create` (customer and merchant mounts) | `POST /api/guest/quotes` and `/api/host/quotes`, `quotes/create.rs` | done for pricing logic; the restrictions check, commission snapshot, credit gate and cover url are open, see below |
| `GET /stays/quotes/{id}`, `stay.quote.get` | `GET /api/guest/quotes/{quoteId}` and `/api/host/quotes/{quoteId}`, `quotes/read.rs` | done (cover url open, see below) |
| admin `GET /stays/rates/{p}/plans`, `stay.admin.rate.plans.list` | `GET /api/admin/properties/{propertyId}/rate-plans` | done |
| admin `POST /stays/properties/{p}/rate-plans`, `stay.admin.rate.plan.create` | `POST /api/admin/.../rate-plans` | done |
| admin `POST .../plans/{id}/update`, `stay.admin.rate.plan.update` | `PATCH /api/admin/.../rate-plans/{ratePlanId}` | done |
| admin `GET .../plans/{id}/rules`, `stay.admin.rate.rules.list` | `GET /api/admin/.../rules` | done |
| admin `POST /stays/properties/{p}/rate-plans/{id}/rules`, `stay.admin.rate.rule.create` | `POST /api/admin/.../rules` | done |
| admin `POST .../rules/{id}/remove`, `stay.admin.rate.rule.remove` | `DELETE /api/admin/.../rules/{ruleId}` | done |
| admin `GET /stays/rates/{p}/promotions`, `stay.admin.promotions.list` (with `fundedBy`) | `GET /api/admin/.../promotions`, `promotions/funding.rs list_for_admin` | done |
| admin `POST .../promotions/{id}/update`, `stay.admin.promotion.update` | `PATCH /api/admin/.../promotions/{promotionId}` | done |
| admin `POST .../promotions/{id}/funding`, `stay.admin.promotion.funding.set` | `PUT /api/admin/.../promotions/{promotionId}/funding`, `set_funding` | done |
| admin `GET /stays/rate-policies`, `stay.admin.policy.list` | `GET /api/admin/rate-policies` | done |
| admin `POST /stays/rates/{p}/preview`, `stay.admin.pricing.preview` | `POST /api/admin/properties/{propertyId}/price-preview` | done |
| admin `GET /stays/settings`, `stay.admin.settings.list` | `GET /api/admin/settings`, `settings/service.rs list` | done |
| admin `POST /stays/settings/{key}`, `stay.admin.settings.update` | `PATCH /api/admin/settings/{key}`, `update` | done |
| `GET/POST/PATCH/DELETE .../unit-types` (host and admin, `stay.unit_types.list`, `stay.unit_type.create`, `.update`, `.archive`, `stay.admin.*` twins, `rate_admin/unit_types.rs`, `unit_type_updates.rs`, `unit_type_archive.rs`, `unit_type_validation.rs`) | not ported here | open: the architecture assigns unit types, pools and physical units to `inventory` (B3), but its source list omits `rate_admin/unit_types*`. These handlers write `stay_unit_types`, `stay_inventory_pools`, `stay_physical_units`, so porting them in pricing would break the owned-writes rule. Coordinator to confirm that B3 takes them. Note `accommodation_templates` imports `validate_unit_type` from this file |
| `stay_audit_records`, `stay_outbox`, `stay_idempotency_keys` use | kernel `audit::record`, `idempotency` (scopes in `pricing/idempotency.rs`, same names as legacy) | done |

## Cross-module steps of quote creation (open until wired)

`create_quote` calls a `quotes::QuoteChecks` trait, because the implementations live in modules that have no public surface yet. The route reads `Extension<Arc<dyn QuoteChecks>>`; until the api crate adds that layer, `POST /quotes` and `GET /quotes/{id}` answer 500.

| Legacy call | Needed from | Status |
|---|---|---|
| `restrictions::check::enforce` (min stay, closed days, lead time) | `inventory::restrictions::check::enforce(pool, property_id, &StayRequest)` exists in the B3 folder but is not exported | implemented in `inventory::restrictions::enforce_for_city` and API `DomainQuoteChecks`; owner compile and integration verification open |
| `commission::rate::snapshot_for` | `billing::commission::rate::snapshot_for(pool, city, host_id, Money)` exists, not exported. Its `CommissionSnapshot` (rate_bps, amount, net) has the same wire shape as `quotes::CommissionQuote` | implemented in API `DomainQuoteChecks` with host-city check; owner compile and integration verification open |
| `credits::gate::ensure_can_book` | `billing::credits::gate::ensure_can_book(pool, host_id, commission_minor)` exists, not exported | implemented in API `DomainQuoteChecks` with host-city check; owner compile and integration verification open |
| `template_media::read::cover_for_unit_type` | properties template media (B2) | implemented in `properties::media::cover_for_unit_type` and API `DomainQuoteChecks`; owner compile and integration verification open |

## Behaviour notes

| Item | Mungini | Status |
|---|---|---|
| host status gate `operator_status == APPROVED` | hosts table has `ACTIVE` and `SUSPENDED` (B1), so the gate is `host_status == ACTIVE`, code `HOST_NOT_ACTIVE` | done (adapted, D2) |
| `property_for` / `property_for_actor` (membership via NATS, `require_scoped`) | `scope.rs`: reads the property, `Actor::ensure_city`, then `Actor::ensure_host_scope` (non member is `NOT_FOUND`) or, for admin routes, `require_platform(properties.moderate)` | done |
| `ensure_platform_moderator` (NATS `auth.stay.platform_moderator`) | `Actor::require_platform(PROPERTIES_MODERATE)` | done |
| permissions `STAY_VIEW_PROPERTIES`, `STAY_MANAGE_RATES`, `STAY_MANAGE_CAMPAIGNS` | `properties.read`, `pricing.write`, `pricing.campaigns` | done |
| view-only city mode blocks writes (gateway `ensure_writable`) | kernel `Actor` extractor | done |
| free text errors | typed codes in `messages.rs`, params carry the field, limits or ids | done |
| page validation (`restrictions::validation::validate_page`) | kernel `PageRequest::validated` (`PAGE_INVALID`, `PAGE_SIZE_INVALID`) | done |
| quote `session_ref` is the user id | `actor.user_id()` | done |
| legacy seed quote `seed-quote-goma-confirmed` has breakdown `{"nights": 2}`, which `GET /quotes/{id}` cannot read (bug, would answer 500) | ported behaviour as is; the seed now carries a full breakdown so the demo works | done (bug noted, seed corrected) |
| money helpers `quotes/money.rs` | kernel `Money` (`parse`, `to_wire`, `percent_of`); `pricing/amounts.rs` holds the i64 bridge | done |
| `lock_budget` used by reservations | `pricing::promotions::lock_budget`, public | done |
| race test `rate_admin/db_race_tests.rs` (6 holds against a budget of 1, 6 writers on one idempotency key) | not ported: it needs reservations and the kernel idempotency table with a live database, and the budget test inserts `stay_reservations` rows (B5). Re-port with B5 | open |
| unit tests (`validation` 6, `pricing` 7, `charges` 4, `promotion` 6, `cancellation` 6, `explain` 2, `schedule` 1, `keys` 3, `rate_admin/types` weekday mask) | next to the code in `rates/`, `promotions/`, `quotes/`, `settings/` | done (not run) |

## Public surface for other modules

- `pricing::rates::{nightly_minor, Adjustment, RateRule, load_rules(pool, city, plan_id), rules_for_plans(conn, city, plan_ids)}`
- `pricing::promotions::{Promotion, Discount, best_length_of_stay, load_active(pool, city, property_id, plan_id, in, out), active_for_properties(conn, city, property_ids, in, out), lock_budget(tx, promotion_id)}`
- `pricing::quotes::{QuoteRequest, QuoteView, NightLine, DiscountView, StoredBreakdown, CommissionQuote, MAX_STAY_NIGHTS, ensure_bookable, guest_factor, QuoteContext}`, `quotes::charges::{price_stay, StayTerms, PricedStay}`, `quotes::cancellation::{CancellationTerms, free_cancellation_deadline, evaluate, RefundEvaluation}`, `quotes::schedule::pay_at_property`, `quotes::availability::{open_nights, room_available}`, `quotes::context::load_context`
- `pricing::settings::{SettingKey, get(pool, city, key), get_i64}` for approval, amendment, claim, case, credit and commission settings
- error codes: `pricing::*` constants (`messages.rs`)
- routes: `public_routes` (empty), `guest_routes`, `host_routes`, `admin_routes`, `openapi()`. Wired in `domain/src/lib.rs` and `api/src/registry.rs`.

## Notifications and seeds

No notification or outbox event is sent by the legacy pricing code, so none is ported. Seeds: `backend/seeds/pricing/01-policies.sql` to `05-quotes.sql`, listed in `backend/seeds/00-order.txt`, covering every plan status (DRAFT, ACTIVE, PAUSED, ARCHIVED), both booking modes and both occupancy bases, OVERRIDE, AMOUNT and PERCENTAGE rules, the gap and conflict plan, active, paused and budget-limited promotions with both fundings, a retired policy, changed and default settings for Goma and Kinshasa, and expired and live quotes. Plans and quotes reference properties, hosts and unit types seeded by B1, B2 and B3 (no foreign keys, so load order does not break the insert, but the pages need those seeds).

## Schema contract

Migration `00019_pricing.sql`. Legacy table and column names (camelCase quoted) kept, `city_id` added.

| Table | Change versus legacy |
|---|---|
| `stay_policy_versions` | global (no `city_id`, same rows in both cities), otherwise identical |
| `stay_rate_plans` | `city_id`; `childNightlyAmount` folded into the create; foreign keys to unit types and properties dropped (not owned, add when those tables land); index `(city_id, "propertyId")` |
| `stay_rate_rules` | `city_id` |
| `stay_promotions` | `city_id`; `redemptionLimit` folded in |
| `stay_promotion_funding` | `city_id` |
| `stay_platform_settings` | `city_id`, primary key `(city_id, key)`, the 00052 key list and ranges folded in; no default rows inserted (code defaults apply, seeds set values) |
| `stay_quotes` | legacy `city` column is `city_id`; foreign key to properties dropped |

Tables and functions read but owned elsewhere: `stay_properties` (`id`, `city_id`, `"operatorId"`, `timezone`, `status`), `hosts` (`id`, `status`), `stay_unit_types` (`id`, `"propertyId"`, `city_id`, `status`, `"maxAdults"`, `"maxChildren"`), `stay_reservations` (`"contractSnapshot"`, `state`), `stay_inventory_pools`, `stay_night_inventory`, SQL functions `stay_room_available`, `stay_unallocated_physical_blocks`. B2 and B3 migrations must keep these names. Translatable content: the pricing tables hold no translatable text (policy content is structured JSON), so no per-locale tables are needed; the Mungini shape does not differ.

`DomainQuoteChecks` is attached to the API router. Its implementation verifies the property and unit city before restriction checks, checks host city before billing checks, and resolves ready template media from the property-owned storage module. Rust is read-checked only. A ready media key outside the property city scope is not signed. As in legacy, an unavailable cover is omitted without failing the quote. Database, signed URL, permission and API integration tests remain open. The API identity directory remains fail-closed.

Unit-type archive now calls `pricing::archive_for_unit_type` in its existing transaction. This keeps rate-plan writes in the pricing module without changing the legacy archive update. Owner compilation and database verification remain open.
