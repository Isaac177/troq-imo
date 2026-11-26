# Host and guest model

Defaults accepted by the owner: D1 (separate identity, no shared database) and D2 (Mungini owns a `hosts` entity; stay operators and their members are copied at cutover). Sources: `tzone-rs/auth-service` and `stay-service` on `main`, and the Mungini tree (`kernel/src/auth`, `kernel/src/permissions`, `domain/src/identity`, `domain/src/hosts`, migrations `00002` to `00004`).

## Identity

- Table `identity_users`: `id`, `home_city_id`, `status` (`ACTIVE` or `SUSPENDED`), `revoked_before`, `created_at`. It is a minimal access record. Profile fields (name, phone, email, language) are not defined yet.
- Tokens carry `sub`, `role` (`Guest`, `Host` or `Staff`), optional `authCity` and `cityAccessMode`, `iat` and `exp`. A token issued before `revoked_before` is rejected as a revoked session.
- A user whose `authCity` differs from the requested city gets view-only access there: reads work, writes fail with `CITY_VIEW_ONLY`.
- Login by one-time code. `AUTH_OTP_CHANNEL` defaults to email; the WhatsApp code is kept for later. User creation, OTP and sessions are not implemented in Mungini yet (ledger B1).
- Role split: the role is coarse (who you are), permissions are fine (what you may do).

## Hosts

- Table `hosts`: `id`, `city_id`, `display_name`, `kind` (`INDIVIDUAL` or `BUSINESS`), `status` (`ACTIVE` or `SUSPENDED`), `guest_contact_phone`, `guest_contact_email`.
- The guest contact fields are what a guest sees for the host. Validation and update (with audit and outbox event) exist in `hosts/contact.rs`.
- Legacy, on `main` today: a stay host is a TROQ business (migration `auth-service/00051` copied `stay_operators` into `companies` and `businesses`, `00052` dropped the operator tables). `stay-service` keeps a mirror table `stay_operators` with status `DRAFT`, `PENDING_REVIEW`, `APPROVED`, `SUSPENDED`, `CLOSED`.
- Legacy staff decisions on a host: `APPROVE` (from `DRAFT` or `PENDING_REVIEW`), `SUSPEND` (from `APPROVED`), `REINSTATE` (from `SUSPENDED`), each with a mandatory reason code from a fixed list and a row in the status history. Reasons: approve `DOCUMENTS_VERIFIED`, `IDENTITY_VERIFIED`, `PROPERTY_VISITED`; suspend `POLICY_VIOLATION`, `FRAUD_SUSPECTED`, `GUEST_SAFETY`, `UNPAID_FEES`, `DOCUMENTS_EXPIRED`, `OWNER_REQUEST`; reinstate `ISSUE_RESOLVED`, `APPEAL_ACCEPTED`, `DOCUMENTS_UPDATED`, `ERROR_CORRECTED`.
- Legacy host documents: types `BUSINESS_REGISTRATION`, `TAX_CERTIFICATE`, `OWNER_IDENTITY`, `PROPERTY_PROOF`, `INSURANCE`, `SAFETY_CERTIFICATE`; status `PENDING`, `VERIFIED`, `REJECTED`, `EXPIRED`; up to 10 MiB.
- The Mungini `hosts` table has no review lifecycle yet. Which lifecycle Mungini keeps is an open question (see README).
- A host is tied to one city (`hosts.city_id`).

## Members

- Table `host_members`: `host_id`, `user_id`, `is_owner`, `status` (`ACTIVE` or `REMOVED`), `removed_at`. A user has at most one active membership per host. `list_my_hosts` lists active memberships by city with SQL pagination.
- `is_owner` marks the owner. The schema does not force a single owner; notifications to "the host" resolve the active owner member.
- Legacy invitations carry an expiry set in days when the invitation is created; only `PENDING` and unexpired invitations can be accepted.

## Permissions

Fine-grained, named `domain.action`, defined in `kernel/src/permissions/names.rs` (30 names): `properties.read|write|configure|moderate`, `inventory.write|integrations`, `pricing.write|campaigns`, `reservations.read|write|guest_details`, `operations.checkin|housekeeping|maintenance`, `messages.write`, `cases.write`, `reviews.write|moderate`, `billing.read|request_refunds|approve_refunds|payout_details|settlements|approve_payouts`, `analytics.read|export`, `hosts.team|settings|review_applications`, `audit.read`.

Legacy names are `STAY_*` (for example `STAY_VIEW_PROPERTIES`, `STAY_MANAGE_RESERVATIONS`) and map one to one through `permissions::from_legacy`, which is needed for the cutover copy of grants.

## Scopes and host scoping

- A grant has a scope: `PLATFORM`, `HOST` or `PROPERTY`. Legacy used `BUSINESS` for the host scope; the copy renames it.
- `PLATFORM` covers every host. `HOST` covers one host and its properties. `PROPERTY` covers one property.
- The kernel enforces scoping on the server. `Access::ensure_host_scope` first checks membership: a host the user is not a member of is `NOT_FOUND`, not forbidden. Then the permission must be granted at a scope that covers the host or property.
- List endpoints use `scoped_access`, which returns the hosts the user may see in full and the single properties they may see.
- Directory reads ignore expired grants, removed memberships and suspended hosts. Property-scoped grants and overrides are held closed until property ownership can be verified (ledger B1).

## Overrides

Per member, a permission can be explicitly `ALLOW`ed or `DENY`ed at a scope (`identity_member_overrides`). Resolution applies deny, then allow. Writing an override needs team management at the host scope. Clearing a `DENY` needs the underlying permission at the host scope too; this is stricter than legacy on purpose.

## Delegation

Legacy rule: a grantor needs the team permission covering the target and must hold the permission being granted. Permissions marked platform-only in the catalog cannot be delegated to hosts. Delegable set in migration `00030` (later changed by `00056`, final set unverified): property, availability, rates, reservations, guest details, check-in, housekeeping, maintenance, messages, reviews, finance view, refund requests, payout details, analytics, export, team, integrations, audit. Not in that set: moderation, settlements, payout approval, refund approval, application review.

## Presets

Legacy presets (from `auth-service/00015`, extended by `00028` and `00029`): host side `OPERATOR_OWNER`, `PROPERTY_MANAGER`, `RESERVATIONS_DESK`, `REVENUE_SPECIALIST`, `HOUSEKEEPING`, `MAINTENANCE`, `OPERATOR_ACCOUNTANT`; platform side `PLATFORM_REVIEWER`, `SUPPORT`, `PLATFORM_FINANCE`. Mungini has no presets table yet.

## Ownership transfer and agreements

On `main` the tables `stay_ownership_transfers` (statuses `PENDING`, `ACCEPTED`, `CANCELLED`, one pending per host) and `stay_agreement_versions` and `stay_agreement_acceptances` (versions `DRAFT`, `PUBLISHED`, `RETIRED`, one published per code) exist as schema. No Rust code in `auth-service`, `stay-service` or the gateway reads or writes them. So the code defines no transfer flow and no host agreement flow. Mungini must specify both before building them (open questions).

Guests accept terms through a general user terms table (`user_terms_acceptances`, immutable rows with version and locale). At booking time the guest also accepts the policy versions (cancellation and payment) attached to the rate plan; see `reservations-lifecycle.md`.

## Guest accounts

- A guest is any authenticated user with role `Guest`. Reservations store `guestUserId`.
- Per reservation the primary guest has a name and at least a phone or an email; additional guests need a name only. Each guest row carries a retention date.
- Retention: after `check_out + GUEST_RETENTION_DAYS` (unset means the check-out date) a job replaces the name with `REDACTED` and clears phone and email, with an audit record and an event. Search events are purged after 6 months.
- Guests also have wishlists and saved searches, and write reviews.

## Unverified

- Mungini user profile fields and the account provisioning flow.
- Final preset contents after the later legacy business presets (`00034`, `00035`, `00057` add CRM permissions that Mungini drops under D8).
- Whether a host may have members in more than one city.
- Whether a single user may be a guest and a host member at once (the token has one role).
- Invitation expiry length.
