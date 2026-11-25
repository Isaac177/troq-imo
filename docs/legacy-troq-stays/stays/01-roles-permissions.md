# Roles, memberships and permissions

Dependencies: D02, D03, D12, D19. Applies to every other feature.

## Repository evidence

- Auth migration 00001_types.sql defines UserRole and Permission enums; later migrations add roles such as affiliate_partner.
- auth-service/src/jwt.rs carries one role, a permission list and companyId. A single company claim does not model several property memberships.
- Business src/types/user.ts and shared-types/types/user.ts currently expose different role vocabularies. Do not copy either as a complete canonical model.
- Gateway auth middleware already has auth-city/active-city context. Extend its verified behavior; do not create an independent client-trusted city authorization system.

## R01: Minimal role change

Add stay_manager as the only planned new UserRole value using a forward migration after confirming all enum consumers. Do not add front-desk, finance, cleaner, host-owner, or city-admin enum roles.

New accommodation-only manager accounts can receive stay_manager. Existing merchant/admin accounts retain their role and gain scoped memberships. Guest access follows account ownership, not stay_manager. Staff accounts can share stay_manager while receiving narrower grants. The enum grants no operator data access on its own.

A second new role is not required by this design. Do not add one without a demonstrated need and owner agreement.

## R02: Scoped grants

Authorization requires active user/session, valid permission, active membership, matching resource scope, and valid resource state. Scope is PLATFORM, CITY, OPERATOR or PROPERTY; grant fields include subjectId, permission, scopeType, scopeId, status, grantor, grantedAt and revokedAt. Reject invalid combinations. Resolve scopes from authoritative resource ownership.

Operator membership and property assignment are first-class records owned by auth; stay service owns property/operator business data. Define synchronized scope references and fail closed on unverifiable scope. Do not duplicate independent grants in each app.

Owner is an operator membership designation, not an enum role. Ownership transfer is a separate audited workflow, not a normal permission toggle. Prevent removal of the final active owner before transfer.

## R03: Permission catalog

These target enum names must be propagated consistently across database, Rust validation, gateway and affected clients. Fine-grained server checks must not fall back to broad legacy MANAGE_PAYMENTS or MANAGE_BUSINESSES.

| Permission | Actions | Permitted scope |
|---|---|---|
| STAY_VIEW_PROPERTIES | Private property and unit details | Operator/property |
| STAY_MANAGE_PROPERTIES | Edit property content, units, media; submit revisions | Operator/property |
| STAY_MANAGE_AVAILABILITY | Inventory, restrictions and blocks | Operator/property |
| STAY_MANAGE_RATES | Rate plans, price rules and operator promotions | Operator/property |
| STAY_VIEW_RESERVATIONS | Scoped reservation summaries | Operator/property/city/platform |
| STAY_MANAGE_RESERVATIONS | Create, accept, decline, propose amendments/cancellation | Operator/property/city/platform |
| STAY_VIEW_GUEST_DETAILS | Required contact and arrival fields | Operator/property/city/platform |
| STAY_MANAGE_CHECKIN | Assign rooms, check in/out, no-show operations | Operator/property |
| STAY_MANAGE_HOUSEKEEPING | Assign and complete housekeeping work | Operator/property |
| STAY_MANAGE_MAINTENANCE | Maintenance work; inventory blocks additionally require availability permission | Operator/property |
| STAY_MANAGE_MESSAGES | Scoped guest conversations | Operator/property/city/platform |
| STAY_MANAGE_REVIEWS | Operator replies and reports | Operator/property |
| STAY_VIEW_FINANCE | Scoped statements and financial summaries | Operator/property/city/platform |
| STAY_REQUEST_REFUNDS | Initiate evaluated refund request | Operator/property/city/platform |
| STAY_APPROVE_REFUNDS | Approve policy/threshold-permitted refunds | City/platform |
| STAY_MANAGE_PAYOUT_DETAILS | Change operator payout destination with step-up | Operator |
| STAY_MANAGE_SETTLEMENTS | Prepare/reconcile settlements | City/platform |
| STAY_APPROVE_PAYOUTS | Authorize payouts within approved limit | City/platform |
| STAY_VIEW_ANALYTICS | Scoped aggregates | Operator/property/city/platform |
| STAY_EXPORT_DATA | Approved export plus underlying view permission | Operator/property/city/platform |
| STAY_MANAGE_TEAM | Invite, assign and revoke delegable operator grants | Operator |
| STAY_MANAGE_INTEGRATIONS | External calendar/channel configuration | Operator/property |
| STAY_REVIEW_APPLICATIONS | Verification application decisions | City/platform |
| STAY_MODERATE_LISTINGS | Approve, reject, suspend listing revisions | City/platform |
| STAY_MODERATE_REVIEWS | Review moderation and appeals | City/platform |
| STAY_MANAGE_CASES | Support/dispute case handling | City/platform |
| STAY_MANAGE_CONFIGURATION | Taxonomies, policy versions and module settings | Platform |
| STAY_MANAGE_CAMPAIGNS | Platform collections and funded promotions | City/platform |
| STAY_VIEW_AUDIT | Scoped redacted audit history | Operator/city/platform |

## R04: Presets, not roles

Presets are editable bundles of explicit grants. Applying a preset snapshots grants; later preset changes never silently increase existing access.

| Preset | Default access |
|---|---|
| Operator owner | Operator management, team, finance, payout details; no platform moderation or payout approval |
| Property manager | Assigned property content, rates, availability, reservations, guest details, check-in, messages, tasks, analytics |
| Reservations desk | Reservation view/manage, guest details, check-in and messages; no rate configuration, team or payout access |
| Revenue specialist | Property view, availability, rates and analytics; reservation data only in aggregate unless separately granted |
| Housekeeping | Assigned task execution and readiness; no guest contact, reservation finance or access codes |
| Maintenance | Assigned maintenance jobs; no automatic permission to block inventory |
| Operator accountant | Finance view, refund requests and explicitly allowed exports |
| Platform reviewer | Assigned city application/listing moderation only |
| Support | Assigned city cases, reservation view, minimum guest details, messages; refund request separately granted |
| Platform finance | Finance view and settlement preparation; refund/payout approval granted independently |

Task assignment further limits task-oriented access. STAY_MANAGE_HOUSEKEEPING does not imply viewing every operator task when the membership is restricted to assigned tasks.

## R05: Team UI and safeguards

Team table shows member, invitation status, property scope, preset label and effective permissions. Detail drawer groups permissions by feature, shows scope explicitly, and previews changes. Invitations expire and can be revoked; accepting requires the intended verified identity.

Grantors may delegate only explicitly delegable permissions within their own scope. Platform-only permissions cannot be granted by operators. Self-approval of financial requests is prohibited where separation of duties applies. Require an ownership-transfer workflow for changing owner identity.

Use a membership/grant version to invalidate authorization caches. Writes and sensitive reads must observe revocation without waiting for a 15-minute token expiry. Define fail-closed behavior for unavailable authorization infrastructure.

## Acceptance and fixtures

- seed-stay-owner, seed-stay-desk, seed-stay-cleaner and seed-stay-accountant share a manager role but have different grants.
- A merchant with stay membership retains merchant access; revoking stay membership does not remove merchant capability.
- Owner of operator A cannot inspect property B by substituting IDs, city headers or export parameters.
- A desk user cannot change payout details, rates, team grants or refund approval.
- Revoked invitations, disabled users and stale tokens cannot mutate data.
- Public DTOs never contain private grant, guest, payout or verification fields.
- Test direct backend calls as well as hidden/disabled UI actions.
