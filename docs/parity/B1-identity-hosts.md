# B1 identity and hosts parity

This slice follows the D1 separate identity and D2 Mungini-owned hosts defaults. It is incomplete and is not connected to the API. Rust and SQL were read-checked only; neither Cargo nor a migration was run.

| Legacy location or capability | Mungini location | Status |
|---|---|---|
| `auth-service/migrations/00016_stay_operators.sql` host kind, status and member states; `00051_stay_operators_to_businesses.sql` identity change | `backend/migrations/00002_identity.sql` hosts and host_members | open: host management, ownership rules and the full current business model remain to port |
| `auth-service/src/scoped_grants/authorize.rs` scoped grants | `backend/migrations/00002_identity.sql`, `backend/crates/domain/src/identity/directory.rs` | open: read-only access foundation filters expired grants, inactive memberships and suspended hosts; presets, overrides, invitations, property ownership and grant mutations remain |
| `auth-service/src/scoped_grants/member_overrides.rs` `apply_overrides` | `backend/crates/domain/src/identity/overrides.rs`, `directory.rs`, `manage_overrides.rs`, `backend/migrations/00004_identity_overrides.sql` | open: deny then allow effective resolution and focused unit tests added; host-scoped override writes with membership and delegated-permission checks added; preset expansion, property-scoped management, owner-specific access, routes and database verification remain |
| Gateway session revocation and active users | `identity/directory.rs` `revoked_before` and `identity_users` | open: identity lifecycle, logout and token issuance not implemented |
| Auth permissions and memberships exposed to Stay | `identity/directory.rs` `AccessDirectory` implementation | open: unwired; legacy effective-grant resolution and platform staff behavior require full port and verification |
| Users, OTP and sessions | none | open |
| `auth-service/src/scoped_grants/team_lists.rs` `list_my_operators` | `backend/crates/domain/src/hosts/membership.rs` `list_my_hosts` | open: active explicit host memberships, city filtering and SQL pagination implemented; legacy grants-derived membership must be reconciled during data migration, route and integration tests remain |
| `stay-service/src/operators/contact.rs`, `listings/content.rs` contact validation | `backend/crates/domain/src/hosts/contact.rs`, `contact_validation.rs`, `backend/migrations/00003_host_contact.sql` | open: scoped city and active membership checks, legacy validation, transactional update, audit and outbox implemented; route, auth integration and database verification remain |
| Host onboarding, documents, agreements, team and transfers | none | open |
| Host and identity routes, permissions and notifications | none | open |
| Legacy demo host and identity states | `backend/seeds/identity/01-directory.sql`, `02-host-contact.sql`, `03-overrides.sql` | open: two cities and active, removed, suspended and expired examples added; feature seeds remain to port |
| Legacy tests and database tests | none | open |

Property-scoped grants record the intended host, but property ownership is not yet enforced by this directory. Do not wire it into the API until the property ownership contract, effective grant rules and the identity lifecycle are implemented and verified. The new account table is a minimal access record and is not an account provisioning or login implementation.
