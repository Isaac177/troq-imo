# Rates, pricing, promotions and policies

Dependencies: D05-D11, D17. All money remains CDF. Policy values below are configurable concepts, not invented approved commercial terms.

## P01: Rate plans and restrictions

Rate plan identifies unit type, occupancy basis, meals/inclusions, cancellation policy version, payment schedule version, booking mode and active sales dates. Unit type capacity is authoritative; a price rule cannot increase capacity.

Manager UI compares plans in rows and edits a plan in sections: inclusions, base price, occupancy, restrictions, cancellation, payment. Show guest-facing preview and all inherited/overridden settings.

## P02: Deterministic price resolution

Document and test one explicit precedence algorithm before implementation. Proposed order: applicable base nightly rate; dated override; occupancy/child/extra-bed charges; eligible length/advance/last-minute promotion; allowed coupon; mandatory fees/tax; optional extras. D07-D11 determine taxable/commissionable bases and whether fees are discounted.

Conflicting rules of equal priority are rejected or resolved by a documented deterministic key, never database row order. Child ages are evaluated at a documented date. Missing rates on any night make that rate plan unbookable; never substitute zero.

Apply rounding at defined calculation boundaries using exact decimal/integer arithmetic. Define whether displayed total is rounded per line or only at aggregation; frontend must not independently recompute financial totals.

## P03: Quote contract

Quote contains ID/version/expiry, property/city, items, dates, per-room occupancy, per-night charge lines, quantities, promotions, required fees, selected extras, total, due-now/later/at-property schedule, currency CDF and policy snapshots.

Bind quote to intended guest/session. Never trust client-submitted price, discount, tax or commission. On expiry or changed inputs regenerate and explicitly ask guest to accept a changed total. Quote does not promise inventory until a hold is obtained.

Receipt presentation separates refundable damage deposit from accommodation total and clearly identifies collected versus due obligations.

## P04: Promotions and automated pricing

Promotion fields: operator/platform owner, applicable properties/plans, booking/stay dates, minimum spend/nights, discount type/value/cap, usage budget, customer eligibility, stacking group and funding source. Reserve/release redemption capacity consistently with booking expiry/cancellation.

Coupons cannot be redeemed twice through retries or multiple payment attempts. Reject promotions outside scope, budget, dates or compatible rate plans. Refunds use original discount allocation, not current campaign settings.

Dynamic pricing is D17-gated: define inputs, floor/ceiling, recommendation versus automatic publication, rule precedence, audit and rollback for future prices. No unverified AI price suggestions or simulated data may drive customer charges.

## P05: Cancellation/payment policies

Versioned policy specifies cancellation cutoff relative to property-local arrival, refundable amount formula, fee treatment, no-show and early-departure behavior, host-cancellation outcome and exceptional approval requirements. Convert accepted deadlines to durable timestamps on reservation.

Payment schedule specifies installments, due times, tolerance if approved and overdue consequences. Damage deposit specifies collection type, claim window, evidence/approval and release. These flows depend on provider capabilities; a deposit is not presumed to be a card authorization.

Policy edits affect future quotes. Guests see concise policy summary plus full terms before acceptance. Store accepted version/time. No hidden fee or cancellation term first appears after payment.

## API and validation

Commands: create/version rate plan, manage dated rules, price preview, issue quote, validate coupon, evaluate cancellation. Price preview shows applied/excluded rule explanations to authorized managers.

Validate negative amounts, invalid percentages, overflow, impossible occupancy, overlapping conflicting overrides, invalid periods and unconfigured policy dependencies. Policy evaluation returns charge/refund breakdown and reasons without executing money movement.

## Seeds and acceptance

Seed base/seasonal/weekend prices, child/extra-guest price, weekly discount, capped coupon, exhausted promotion, refundable and nonrefundable plans, missing-night rate and changed quote.

Test exact sums, rounding boundaries, rule conflict resolution, child age boundaries, coupon race, full/partial cancellation, policy version retention and same quote display across web/mobile. Confirm no USD/conversion path exists in new contracts.
