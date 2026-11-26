# Mungini specs

Fresh specs for Mungini, written 26/11/2026 for slice P0-5. They replace the outdated TROQ Stay docs for architecture. Every statement was checked against the code on `main` and the Mungini tree; anything not checked is in the Unverified section at the end of each file.

## Index

| File | Holds |
|---|---|
| `vision.md` | What Mungini is, audiences, cities, CDF, languages, module state |
| `host-and-guest-model.md` | Identity, hosts, members, roles, permissions, scopes, ownership, agreements, guests |
| `reservations-lifecycle.md` | Quote, hold, states, cancellation, amendments, cases, claims, operations tasks |
| `payments-and-billing.md` | Where money moves, commission, credits, settlements, payment at the property |
| `notifications-and-documents.md` | Events, channels, preferences, voucher and notice documents |
| `data-and-cities.md` | One database with `city_id`, PostGIS, global and per-city tables, seeds |
| `api-contract.md` | API contract notes (other slice) |
| `parity/` | Ledgers proving each legacy feature is done or dropped by decision |
| `legacy-troq-stays/` | Outdated reference, never the spec |

## Decisions in force

D1 separate identity, no shared database. D2 Mungini owns `hosts`; operators and members copied at cutover. D5 `billing` ports commission and credits; provider code copied, never shared. D3 CDF only and D4 one database with `city_id` follow the plan defaults.

## What was dropped from legacy

- Anything written for embedding Stay in TROQ: the shared role enum (`stay_manager`), TROQ businesses as hosts, the business app UI, NATS subjects, the gateway, TROQ CRM links and the replicated global tables.
- Legacy statements the code does not back: online payment, deposits, refunds, settlements, host agreement flows and ownership transfers (schema only), and `PENDING_PAYMENT` as a live state.
- Legacy proposals that were never built, including the damage claim dispute flow.

## Open questions for the owner

1. Host lifecycle. Legacy had `DRAFT`, `PENDING_REVIEW`, `APPROVED`, `SUSPENDED`, `CLOSED` with approval by staff and reason codes. Mungini `hosts` has only `ACTIVE` and `SUSPENDED`. Does Mungini keep host review and documents?
2. Ownership transfer and host agreements exist only as schema in TROQ. Do they ship in the first release, and with what rules?
3. Settlements. The tables exist and nothing uses them. With prepaid credit, what would a settlement pay, and to whom? Port, redesign or drop?
4. Online guest payment. Stay stays pay-at-property forever, or is a provider checkout planned? The refund, payout and `PREPAY` names suggest it was once considered.
5. Cases `REFUND_DISPUTE` and `NO_SHOW_DISPUTE` are refused for lack of payment rules. Define them or drop them.
6. Damage claims. Is a guest dispute flow needed beyond staff deciding?
7. Amendments. Should a guest-proposed amendment need host approval? Today the guest proposes and accepts.
8. Notice gaps. No notice for a declined request, an expired hold, check-in, check-out or no-show. Intentional?
9. Voucher timing. The voucher is issued at hold time even for a request booking that is not yet accepted. Intentional?
10. Presets and delegation. Confirm the final preset list and which permissions hosts may delegate.
11. One user as guest and host member. The token has a single role. May the same account be both?
12. Evidence storage. Keep case evidence in Postgres (legacy) or move to object storage?
13. Property location. PostGIS column or latitude and longitude with a derived point?
14. Property categories available to hosts in the first release.
15. Retention. Default `GUEST_RETENTION_DAYS` value; today unset means data is redacted the day after check-out.
16. Platform cancellation. Which cases may staff cancel and what charge applies?
17. WhatsApp. When is it paid for and enabled as the OTP and notification channel?
18. D8 and D9 are still defaults: CRM out of the first release, a separate Mobile app with its own bundle id.

## Unverified

- Dates of the last code reading: 26/11/2026. Mungini code changes quickly, so the "built" statements are a snapshot. Re-check against the parity ledgers.
- Production behaviour of any job, worker or provider. Nothing was run; the code was only read.
