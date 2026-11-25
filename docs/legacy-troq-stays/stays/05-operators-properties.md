# Operator onboarding, properties and media

Dependencies: D01-D04, D18; R/B/U. Ownership and media identity must be confirmed before schema changes.

## O01: Onboarding

Steps: account/contact verification; operator type and identity; agreement acceptance; documents; payout setup when applicable; first property; review submission. Each step saves a draft. Completion is server-calculated; client checkmarks are not proof of approval.

Operator profile includes legal/display identity as applicable, contact person, international phone, email, existing company reference if approved, verification evidence references and accepted agreement version/time. Collect no identity document not required by the approved verification policy.

Show review status and field-specific corrections. Submission validates required fields, current agreement, document processing and duplicate identity rules. A user can resume an application without creating duplicate operators.

## O02: Portfolio and ownership

Portfolio rows/cards show property image/name, city, publication state, assigned access, next operational action and appropriately scoped metrics. Property switcher cannot expose inaccessible properties. Filters and server pagination apply.

One operator may have several properties if D02 confirms it. Optional brands are commercial associations, not authorization boundaries. Transferring a property requires explicit treatment of future reservations, liabilities, grants and media licenses; ordinary profile editing cannot do this.

## O03: Property editor

| Section | Required information/validation |
|---|---|
| Identity | en/fr title and description, approved property type, operator, optional brand; stable ID separate from editable slug |
| Location | City, neighborhood, structured address, coordinates, timezone, directions; validate coordinate ranges and city consistency |
| Contact | Guest-facing contact, emergency contact and notification recipients with visibility rules |
| Accommodation | Unit types, physical units/capacity, bed/bath layout and size unit |
| Amenities | Controlled vocabulary, included/paid/limited availability, unit-level overrides |
| Accessibility | Specific verified features, not a generic unsubstantiated accessible badge |
| Policies | Check-in/out windows, pets, smoking, events, child rules, guest requirements and policy versions |
| Media | Cover/gallery order, captions/alt text, property/unit attribution, processing status |
| Sales | Booking mode, availability/rate readiness, accepted rate plans and publication state |

UI uses saved steps and a final guest preview. Required versus optional fields are explicit. Validate on the server and map field errors to localized messages. No fixed arbitrary title/description limits: choose shared bounded constants during contract finalization and test them in both apps.

## O04: Listing lifecycle and revisioning

States: DRAFT, SUBMITTED, CHANGES_REQUESTED, APPROVED, PUBLISHED, PAUSED, SUSPENDED, ARCHIVED. Approval and publication can be separate events. Define allowed transitions and required permission: operators draft/submit/pause; platform approves/rejects/suspends; archival checks future bookings.

Published content points to an approved revision. Edits to moderated fields create a revision rather than silently changing the live listing. Managers see live and proposed versions. Archive retains reservation snapshots and assets required for historical records.

## O05: Media processing

Request an authorized upload, validate MIME/content/size/dimensions, scan supported documents, process image variants, remove unnecessary metadata, then attach only READY media. Display UPLOADING, PROCESSING, READY and REJECTED with retry.

Check media owner and allowed reuse before attachment. Different brands/properties do not automatically share rights. Removing a gallery attachment does not delete media used elsewhere. Retain historical evidence references under D11/D18; public galleries never expose private verification files.

Cover must reference approved/ready media in the property gallery. Reordering is versioned and atomic. Failed uploads cannot count toward publication completeness. Restrict bulk upload size and parallelism.

## O06: Staff and agreements

Team screens follow R05. Agreement updates show version, required acceptance and effect on new sales; do not silently suspend access to servicing existing guests. Payout setup uses the payment contract and restricted permissions, not editable free-text bank details in the public property profile.

## Backend and acceptance

Required commands: create/resume application, update draft, submit revision, attach/reorder media, pause sales, archive, invite/revoke member. Scope every command by operator/property, expectedVersion, and actor.

Fixtures: apartment, hotel with multiple room types, multilingual long content, missing translation, rejected media, duplicate application, property with live and pending revision, multi-property member and inaccessible property.

Verify unauthorized media reuse, stale edits, duplicate submission, interrupted onboarding, reference-safe archive and suspension preserving booked guest access. Property readiness must derive from real valid rates/inventory and approved content, not a manually editable boolean.
