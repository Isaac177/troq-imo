# Security and privacy implementation requirements

Dependencies: R, all API contracts, decisions D11/D18/D19. This is a feature specification, not a generic warning backlog.

## SEC01: Authorization and trust boundaries

Authenticate at the gateway and enforce domain authorization in the owning service. Internal service calls also need authenticated trusted context. Validate user/session, scoped permission, current membership, resource owner/city and allowed state. Never accept actor role/operator scope solely from request body or forwarded untrusted headers.

Use deny-by-default object and field access. An authorized list filter cannot be widened with another property/operator ID. Validate nested resources belong to the parent, including units, media, guest records, quote, refund and export. Public serializers must explicitly allowlist fields rather than serialize database rows and hide them in UI.

Reference: [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html).

## SEC02: Session, privilege and staff changes

Follow existing verified token/session architecture. Distinguish access and refresh tokens, validate issuer/audience/type as applicable, reject invalid/expired tokens and revoke disabled accounts. Do not extend a frontend enum and treat it as server authority.

Sensitive reads/writes resolve current grants or a version-validated cache. Revocation invalidation must survive lost notification through bounded revalidation. Financial commands require current authority, not an old JWT permission list. Step-up/multi-factor and dual approval thresholds are D19 decisions.

Prevent privilege escalation through invitations/presets, grants outside the grantor's scope, modifying another operator's member and removing the last owner. Deny cross-city privilege borrowing.

## SEC03: Personal data and files

Separate public listing media, private verification documents, guest registration documents, financial documents and case evidence. Apply category-specific authorization, retention and storage paths. Short-lived signed URLs are issued only after permission checks. Do not store private files under public CDN paths.

Validate upload content, type, size, decompression dimensions and filenames; scan supported risky documents; process images in bounded jobs and strip location metadata. Escape captions and text. Attachment download permission is checked independently from knowing its ID.

Collect only approved fields. Housekeeping sees unit/task information; accounting sees necessary financial details; support gets only case-relevant guest data. Define export/deletion/anonymization and legally required retention with owner-provided D11 rules. Logs/traces/analytics redact personal and payment data.

## SEC04: Payment and business integrity

Verify callback signature, trusted provider account, timestamp/replay controls where supported, reservation obligation reference, amount and CDF currency. Provider redirect success parameters alone never confirm payment.

Rate-limit holds, quote creation, payment attempts, coupons, messaging and exports by appropriate identity/network/property limits. Bound pending inventory per guest/session to reduce hoarding. Detect repeated coupon/reward abuse and route suspicious exceptions to permitted review without automatic arbitrary penalties.

Policy/price/capacity evaluation occurs server-side. Invalid state transitions, negative/refund-over-collected amounts and stale approvals fail before side effects. Idempotency keys are not authorization credentials.

## SEC05: Network and application boundaries

Use parameterized SQL, bounded input schemas, allowlisted sort fields and explicit request/body limits. CORS follows approved origins; cookie-authenticated writes use appropriate CSRF protection. Render untrusted content safely; avoid executable rich HTML without a proven sanitizer.

Calendar/webhook import cannot fetch private infrastructure. Validate DNS/IP at connection time, redirects and resolved destinations; restrict schemes and resource budgets. Store integration credentials in existing secret mechanisms, not UI-visible configuration or logs.

Minimize public map/address disclosure according to D18. Access codes use restricted storage, timed release and revocation/rotation policy; no analytics, calendar export or broadly distributed event may contain them.

## SEC06: Audit and verification

Sensitive changes record actor, effective scope, correlation, resource/version, decision/reason and redacted before/after facts. Protect audit modification through application/database privileges and retention policy. Reading sensitive documents and exporting personal data are auditable actions.

Security tests must attempt direct API abuse: cross-operator/city ID substitution, nested-resource mismatch, stale membership, customer accessing manager route, manager invoking platform approval, export after revocation, forged payment callbacks, replay, unauthorized media reuse, private URL fetch and oversized payloads.

No actual credentials or personal guest documents belong in fixtures. Use recognizable synthetic seed identities. Report real discovered secret problems once in chat according to repository rules; do not add warning lists to commits or backlog.
