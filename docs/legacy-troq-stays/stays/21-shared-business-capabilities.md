# Shared business capabilities for stay managers

Status: implementation map. This document maps shared business features onto the stay workspace; it does not authorize financial policy changes or production operations.

## Goal

Give a stay manager the business tools that apply to a real-estate operator while keeping retail workflows and financial records separate. Reuse shared UI and business services where their tenant and authorization models fit. Add stay-specific views and rules where the underlying records differ.

The stay business is a company of type `STAY`. Use its canonical `businessId` and the active city for every shared feature. Do not treat the URL, a client-selected ID, or the legacy `operatorId` as authorization.

## Current implementation evidence

- `useBusinessSidebar` always combines the stay navigation with a retail menu for non-buyer users. The stay manager consequently sees merchant routes that `useProtectedRoute` sends back to `/stays`.
- `useCrmAccess` disables CRM permission loading in the stays workspace, so the existing CRM menu disappears there even though the shared CRM tenant design is now `business_id`.
- Merchant discounts target products, categories, and brands. They are not compatible with stay rate plans or properties.
- Merchant revenue and payment settings use `companyId` and retail payment hooks. Stay finance has separate routes and ledger behavior.
- Stay pricing already has rate-plan promotions. The stay specification defines date windows, minimum nights, priority, redemption limits, and automatic application without coupon codes.
- The stay credit page still calls the legacy `/api/merchant/stays/operators` family. Reconcile that with the business-ID migration and routes before extending finance.
- `docs/crm/38-stay-operator-tenant-cleanup.md` defines CRM, team, grants, and assistant data under `business_id`; it also records migrations that have not yet run. Confirm the actual local database and gateway state before relying on those routes.

## Target navigation

Select navigation from the active business type and scoped grants. A `STAY` business should receive stay operations plus the shared sections below. It must not inherit the retail menu as a fallback.

| Section | Stay manager menu | Treatment |
|---|---|---|
| Portfolio | Properties, accommodation types, calendar, rates, reservations, trips, tasks, cases, reviews, operators | Keep stay routes and stay permissions. |
| Promotions | Rate and stay offers | Keep within rate management initially; use stay rate-plan rules, not retail product discounts. |
| Finance | Stay ledger and statements; payout and refund actions only when their stay contracts are implemented | Share the finance shell and common display components. Keep stay-specific data, permissions, and accounting. |
| CRM | Leads/inquiries, contacts, companies, tasks/activity, pipeline | Reuse CRM screens where semantics fit. Scope all reads and writes to the selected STAY business. Gate reports, automations, exports, and AI tools individually. |
| Team | Members, invitations, scoped permission presets | Reuse business team membership and routes with stay-aware grants and presets. |
| Settings | Shared business profile plus stay policies and defaults | Reuse organization-level fields where valid; keep property, pricing, booking, and payout settings in stay domains. |
| Notifications and audit | Stay reservation, guest, case, review, finance, and team events | Reuse the inbox and audit UI with business and city scoping. Exclude merchant applications and retail-only events. |
| Assistant and AI credits | Only when enabled for the selected STAY business | Treat AI credits as an assistant budget, not as booking finance or a payout balance. |

## Capability mapping

| Existing merchant area | Share with stays? | Stay-specific implementation |
|---|---|---|
| Payments / revenue | Yes, as a finance capability; not by reusing retail revenue queries | Read stay reservation obligations, collections, commissions, ledger, and statements through stay-aware payment contracts. `docs/stays/10-payments-settlements.md` governs refunds, settlements, payout details, and approvals. Under the current pay-at-property decision, do not show online collections or merchant payout claims that do not exist. |
| Billing / AI credits | Partly | Reuse assistant credit UI only after wallet policy and business scoping are resolved. The CRM stay-team inventory marks this work as blocked on wallet policy. Keep AI budget separate from stay funds and merchant billing. |
| Discounts | Yes, as a promotion capability | Reuse concepts such as percentage/fixed amount, validity, and limits where supported. Target properties/rate plans and stay dates. Use the existing stay promotions model in `docs/stays/07-pricing-policies.md`; do not pass `PRODUCT`, `CATEGORY`, or `BRAND` targets into stay APIs. |
| Settings | Yes, for shared organization data | Share company identity, contact, branding, and notification preferences only after confirming each field is valid for `companyType = STAY`. Add stay-specific policy and booking settings under stay routes. Keep merchant catalog attributes and merchant contract signing out of the stay workspace. |
| CRM | Yes | Reuse the business CRM tenant and screens for guest inquiries, contacts, organizations, tasks, and pipeline where useful. Every endpoint, permission lookup, export, realtime subscription, assistant action, and background job must carry and enforce the selected `businessId` and city. |
| Employees / team | Yes, for team administration | Reuse business membership and invitations. Apply stay permission presets and property coverage; do not expose retail employee permissions or imply finance authority from team membership. |
| Statistics / reports | Partly | Reuse chart components and date/filter controls. Build stay metrics from reservations, nights, occupancy, cancellations, and stay ledger records. Do not show retail order or catalog metrics. |
| Notifications / tickets / logs | Partly | Reuse the notification center and audit presentation for stay events. Add only stay-relevant support cases and redacted business audit entries. Hide merchant applications and merchant-only logs. |
| Reviews | Already available as stay-specific capability | Keep `/stays/reviews`; optionally unify the inbox presentation later with an explicit source filter. Do not route stay reviews to merchant product reviews. |
| Locations / employees | Replace with stay concepts | Properties and operator assignments are the location/team model for stays. Do not expose retail locations or employee pages as substitutes. |
| Products, stock, brands, orders | No | Keep retail-only. Accommodation inventory and reservations have their own records and state machines. |
| Merchant settings contract | No, except an explicitly shared organization agreement | Merchant marketplace contracts do not automatically govern stay booking, guest, commission, or payout terms. |

## Routes and access contract

1. Keep stay-manager navigation inside the stay workspace. Add stay route wrappers for shared pages, such as `/stays/finance`, `/stays/crm/...`, `/stays/team`, `/stays/settings`, and `/stays/notifications`. Exact route names can follow current route conventions, but links must not enter `/merchant` and then depend on a redirect.
2. Reuse presentational components and hooks only when their API contract is business-type agnostic. Otherwise add stay-specific data hooks behind the shared page components.
3. Resolve the active `businessId` from trusted auth/business context. If a user has multiple eligible businesses, use an explicit business selector; never silently fall back to a merchant or first operator.
4. Authorize every request using active membership, `companyType = STAY`, city, business scope, and the feature permission. The gateway and owning service must verify scope; hiding a menu item is not authorization.
5. Reuse permission keys only when their meaning matches the stay action. Follow `docs/stays/01-roles-permissions.md` for finance, team, reviews, analytics, and stay operations. Add explicit business-level settings or promotion permissions where the catalog does not yet define them; platform-only configuration grants must not be given to managers.
6. CRM permission lookup must include the selected business context and permit a STAY business. Remove the workspace-path shortcut that disables CRM access; do not broaden CRM access to every role or business.
7. Keep all money in CDF. Keep stay obligation, ledger, refund, and payout states governed by the stay/payment contract; do not relabel merchant history as stay history.

## Delivery sequence

### 1. Align stay identity and routes

- Confirm the state of the business-ID mapping and migrations recorded in `docs/crm/38-stay-operator-tenant-cleanup.md`.
- Reconcile the current `/api/merchant/stays/operators` callers, including stay credits, with the accepted STAY business-ID route contract.
- Verify city and business selection for owners and team members with access to more than one business.

### 2. Make navigation and route guards workspace-aware

- Build separate navigation sets for MERCHANT, BUYER, and STAY business types.
- Keep current merchant links for merchants; show stay operations and permission-enabled shared links for stay businesses.
- Keep direct-route guards aligned with menu access. Shared stay routes must not redirect through merchant routes.

### 3. Reuse CRM and team under business scope

- Re-enable CRM access for a selected STAY business using business-scoped permission lookup.
- Expose only approved modules and actions: leads/inquiries, contacts, companies, tasks/activity, pipeline, and team management as grants allow.
- Review AI assistant, invoices, reports, automation, exports, realtime, and webhooks separately for stay semantics and authorization.

### 4. Connect promotions and settings

- Surface the existing stay rate promotions as the stay manager's discount capability.
- Keep rate plan, date, minimum-night, redemption-limit, and audit rules in stay service.
- Split shared company profile settings from stay booking/policy settings and merchant catalog settings.

### 5. Add stay-aware finance and notifications

- Start with stay ledger and statement reads. Add collection, refund, payout destination, settlement, or approval actions only when the corresponding stay contract and permissions exist.
- Reuse notification and audit presentation with stay event types and tenant filters.
- Keep AI-credit purchases separate from reservation finance and subject to the wallet decision.

## Acceptance criteria

- A STAY business sees no retail-only sidebar items and every shown item has a matching stay route.
- A manager can use shared CRM/team/settings tools only for the selected STAY business and only for granted actions.
- Direct requests with another business ID, wrong city, inactive membership, or revoked permission are rejected by the service.
- Stay promotions affect only eligible stay rate plans and quotes, and match the pricing policy saved on the reservation.
- Finance screens distinguish reservation value, collected funds, commission, operator payable, refunds, and available settlement; no unsupported payment action appears as successful.
- Notifications, CRM activity, exports, and audit history remain tenant-scoped across browser, API, realtime, and background work.
- English and French strings are present. Changed business screens pass type checks and are checked at 375px in both languages.

## Related specifications

- [Stays roles and permissions](01-roles-permissions.md)
- [Stay pricing and promotions](07-pricing-policies.md)
- [Stay payments and settlements](10-payments-settlements.md)
- [Stay messaging, reviews, and support](12-messaging-reviews-support.md)
- [Stay CRM tenant cleanup](../crm/38-stay-operator-tenant-cleanup.md)
- [CRM stay-team implementation tasks](../crm/37-stay-team-route-inventory.md)
