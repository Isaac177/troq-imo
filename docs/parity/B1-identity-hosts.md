# B1 identity and hosts parity

D1 separate identity and D2 Mungini-owned hosts are followed. The module is wired into the API: public registration and login routes, authenticated session routes, host team routes, onboarding routes and admin host routes are registered, and the fail-closed API directory is replaced by `IdentityDirectory`.

Verification on 26/11/2026: `cargo check --workspace --all-targets` and `cargo test --workspace --lib` pass (309 domain, 18 api, 60 kernel tests). Migrations 00001 to 00030 and every seed file were applied to the local `mungini-db-dev` database after a dump in `backups/`. Live requests verified login, session, host membership listing, team invitation and acceptance, member permission replacement, override set and clear, member removal, owner protection, registration OTP verification and account creation, onboarding draft, document upload, submission, document review and approval, and admin host application, host and guest read models.

| Legacy location or capability | Mungini location | Status |
|---|---|---|
| `auth-service/migrations/00016_stay_operators.sql` host kind, status and member states; `00051_stay_operators_to_businesses.sql` identity change | `backend/migrations/00002_identity.sql` hosts and host_members | done: hosts own the operational ACTIVE/SUSPENDED status and the review lifecycle lives in `host_applications` |
| `auth-service/src/scoped_grants/authorize.rs` scoped grants | `identity/directory.rs`, `kernel/src/permissions` | done: active memberships, unexpired grants, suspended-host exclusion; owner memberships expand to the delegable set |
| `auth-service/src/scoped_grants/member_overrides.rs` `apply_overrides` | `identity/overrides.rs`, `directory.rs`, `manage_overrides.rs`, `clear_override.rs`, migration `00004` | done: deny then allow resolution; host-scoped writes require team management; clearing a DENY requires the underlying permission |
| Gateway session revocation and active users | `identity/sessions.rs`, `session_tokens.rs`, migration `00025` | done: signed access and refresh tokens, rotation with the legacy 30-second grace, logout-all advances the revocation cutoff |
| Auth permissions and memberships exposed to Stay | `identity/directory.rs`, `kernel/src/auth/extract.rs` | done: `Actor` uses the identity directory; unauthenticated calls fail with `AUTH_REQUIRED` |
| Users, OTP and sessions | `identity/account.rs`, `contacts.rs`, `registration_otp.rs`, `password_login.rs`, migrations `00024`, `00026`, `00028`, `00029` | done: email and WhatsApp OTP issuance, single-use proof, guest account provisioning, Argon2/bcrypt password login, lockout, profile update |
| `auth-service/src/scoped_grants/team_lists.rs` `list_my_operators` | `hosts/membership.rs`, `identity/team.rs` | done: paginated active memberships and paginated member listing with effective permissions and overrides |
| `stay-service/src/operators/contact.rs` contact validation | `hosts/contact.rs`, `migration 00003` | done: scoped city and active membership checks, audit and outbox |
| Host-scoped guest contact read | `hosts/contact.rs`, `identity/handlers_host.rs` | open: active city and membership checks and host-scoped property permission; Rust read-checked only, generated contract and verification pending |
| Host onboarding, documents, agreements, team and transfers | `hosts/onboarding/`, migrations `00029`; `identity/team.rs`, `identity/invitations.rs` | partial: applications, document upload and review, approve/reject, suspend/reinstate, team invitations and membership management done; agreements and ownership transfers remain open because no legacy flow exists to port |
| Host and identity routes, permissions and notifications | `identity/routes.rs`, `hosts/routes.rs`; `api/src/registry.rs`, `api/src/main.rs` | partial: routes and permissions implemented; invitation event queues bilingual email for existing or new invitees, runtime verification and inbox UI remain open |
| Legacy demo host and identity states | `seeds/identity/01` to `09` | done: two cities, active, removed, suspended and expired examples, pending/accepted/expired/revoked invitations, draft/pending/approved/rejected applications, every document status, platform staff accounts and host status history |
| Legacy tests and database tests | `identity/session_tests.rs`, unit tests per module | done: unit and SQLx tests run locally; host application and invitation database paths were exercised through live requests |

## Open

- Invitation emails: subscriber and queued delivery implemented; owner compilation, provider delivery and job runtime verification remain. Events already published without a subscriber require a separate replay decision. Guest invitation inbox UI remains open.
- Agreements and ownership transfers: schema not created because the legacy code defines no flow. A Mungini specification is required before building them.
- Network-level login and OTP rate limiting: account lockout exists; Redis-based per-network limits remain open.
- Property-scoped grants and overrides stay closed in the directory until property ownership verification exists (B2 contract).
- Contact migration from TROQ auth is part of cutover C1.

- Registration and invitation cities: a verified email may have pending invitations in several cities. Registration must not be blocked or have its home city selected by an unsolicited invitation. Show a warning after OTP verification listing eligible invitation cities while preserving the user's choice; the response contract, generated client and guest UI remain open.
