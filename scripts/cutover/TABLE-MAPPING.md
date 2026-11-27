# Stay to Mungini table mapping

Read-only planning reference for the cutover copy scripts. It maps the legacy Stay tables to the Mungini schema as written in `backend/migrations/`. It is not a copy script, does not connect to a database and does not prove that a copy is complete.

This mapping is reviewed against `tzone-rs/stay-service/migrations`, `tzone-rs/auth-service/migrations`, `tzone-rs/messaging-service/migrations` and the Mungini migrations on 27/11/2026. Recheck it before writing a copy script.

## Rules

- The legacy Stay database is per city. Every copied business row takes the destination `city_id` from the source database, `GOMA` or `KIN`, not from a column that may be absent on older rows.
- Copy parents before children inside one transaction per city and table group.
- `stay_*` table names are retained where the Mungini module ported the table as is. Do not rename columns during the copy.
- Global reference tables are copied once, not per city. Mungini owns its own amenities and categories, so the copy must deduplicate by code.
- Seed rows are recognizable by `seed-` ids. A rehearsal target may contain seeds, so extra target rows are expected and must not be deleted automatically.
- Media rows reference storage keys. Copy metadata only after the object is present in Mungini storage under the same key.
- Keep personal data out of reports and logs.

## Global reference tables

| Legacy table | Mungini table | Notes |
|---|---|---|
| `stay_accommodation_categories` | `stay_accommodation_categories` | One shared row per code. Deduplicate across city databases. |
| `stay_amenities` | `property_amenities` plus `property_amenity_content` | The Mungini catalog splits the code from its localized copy. |
| `stay_amenity_groups` | `property_amenity_groups` plus `property_amenity_group_content` | Same split as amenities. |

## City-owned tables copied one to one

Copy each table from both city databases, adding `city_id` from the source database where the column is missing.

`stay_accommodation_templates`, `stay_amendments`, `stay_availability_blocks`, `stay_calendar_feed_tokens`, `stay_case_evidence`, `stay_cases`, `stay_claim_decisions`, `stay_claim_statements`, `stay_commission_batch_items`, `stay_commission_batches`, `stay_credit_accounts`, `stay_credit_entries`, `stay_inventory_allocations`, `stay_inventory_pools`, `stay_night_inventory`, `stay_operations_tasks`, `stay_operator_commission_rates`, `stay_physical_units`, `stay_platform_settings`, `stay_policy_versions`, `stay_pool_physical_units`, `stay_promotion_funding`, `stay_promotions`, `stay_properties`, `stay_property_media`, `stay_property_revisions`, `stay_quotes`, `stay_rate_plans`, `stay_rate_rules`, `stay_records`, `stay_reservation_guests`, `stay_reservation_items`, `stay_reservation_messages`, `stay_reservations`, `stay_restrictions`, `stay_reviews`, `stay_saved_searches`, `stay_search_events`, `stay_template_media`, `stay_unit_assignments`, `stay_unit_types`, `stay_wishlist_items`, `stay_wishlists`.

Foreign keys require this order within a city:

1. `stay_accommodation_templates` and `stay_accommodation_categories`.
2. `stay_properties`, `stay_property_media`, `stay_property_revisions`.
3. `stay_unit_types`, `stay_inventory_pools`, `stay_pool_physical_units`, `stay_physical_units`, `stay_night_inventory`.
4. `stay_policy_versions`, `stay_rate_plans`, `stay_rate_rules`, `stay_promotions`, `stay_promotion_funding`, `stay_platform_settings`.
5. `stay_quotes`, `stay_reservations`, `stay_reservation_items`, `stay_reservation_guests`, `stay_reservation_messages`.
6. `stay_amendments`, `stay_unit_assignments`, `stay_records`, `stay_availability_blocks`, `stay_restrictions`, `stay_calendar_feed_tokens`.
7. `stay_cases`, `stay_case_evidence`, `stay_claim_statements`, `stay_claim_decisions`, `stay_operations_tasks`.
8. `stay_reviews`, `stay_wishlists`, `stay_wishlist_items`, `stay_saved_searches`, `stay_search_events`.
9. `stay_credit_accounts`, `stay_credit_entries`, `stay_operator_commission_rates`, `stay_commission_batches`, `stay_commission_batch_items`.

## Renamed or restructured targets

| Legacy table | Mungini table | Notes |
|---|---|---|
| `stay_operators` | `hosts` | `operatorId` becomes `hostId` in every copied child row. Host city comes from the source database. |
| `stay_operator_documents` | `host_application_documents` | Mungini has application documents only. Confirm whether legacy operator documents belong to an application before copying; do not invent an application. |
| `stay_operator_status_history` | `host_status_history` | Keep the original action, reason and timestamp. |
| `stay_audit_records` | `kernel_audit_records` | Actor and resource ids stay as text. |
| `stay_idempotency_keys` | `kernel_idempotency_keys` | Copy only keys that must survive cutover; expired keys can be left behind. |
| `stay_inbox` | `kernel_inbox` | In-process outbox delivery deduplication. |
| `stay_outbox` | `kernel_outbox` | Do not replay already delivered events; use the cutover event boundary. |

## Auth-service tables

| Legacy table | Mungini target | Notes |
|---|---|---|
| User accounts referenced by Stay | `identity_users` | Copy the account id, email, phone, preferred language, status and city. Password hashes must be verified compatible before copy. |
| `stay_operators` | `hosts` | As above. |
| `stay_operator_members` | `host_members` | Map business owner to `is_owner`, keep membership status and removed timestamp. |
| `stay_grant_presets`, `stay_grant_preset_permissions` | code presets in `identity/presets.rs` | Mungini presets are code, not tables. Map by preset code; review any preset that does not exist in Mungini. |
| `stay_property_assignments` | none yet | Property-scoped grants are held closed by identity until ownership verification. Do not copy them as host grants. |
| `stay_ownership_transfers` | none yet | Open in B1. Do not copy as host ownership. |
| `stay_agreement_versions`, `stay_agreement_acceptances` | none yet | Open in B1. Do not fabricate acceptance. |
| `stay_operator_sync_outbox` | none | NATS sync is replaced by the in-process outbox. |

## Messaging and CRM

| Legacy item | Mungini target | Notes |
|---|---|---|
| Issued Stay documents | `messaging_document` | Keep kind, language, entity id and stored bytes reference. |
| Document events | `messaging_document_event` | Keep the event timeline where it is needed for audit. |
| Notification deliveries and attempts | `notification_deliveries`, `notification_delivery_attempts` | Copy historic delivery state; do not resend from history. |
| Notification templates and copy | notification catalog and locale files | Templates are code and locales, not database rows. |
| CRM Stay leads and `stay_operator_id` columns | none | CRM is dropped by decision D8. Do not copy CRM leads into Mungini. |
| CRM contact details used for notifications | `identity_users` and identity contacts | Copy only the contact values needed to deliver notifications; keep the consent basis. |

## Deliberately not carried over

- `stay_integration_mappings` (external channel sync mappings). Mungini inventory has no external channel mapping table. The owner must confirm that channel sync is not required before these rows are dropped.
- `stay_operator_sync_outbox` and the NATS Stay subjects. Replaced by the in-process outbox.
- `stay_idempotency_keys` older than the cutover window.
- CRM leads and CRM audit columns, by decision D8.

## Media

- `stay_property_media`, `stay_template_media` and `stay_case_evidence` metadata cannot be used until the object exists under the same storage key in Mungini storage.
- Copy or rekey the object first, then the row. A missing object must be reported, not silently replaced.

## Verification per table

Use `scripts/cutover/reconcile-cli.ts` with exported identifier snapshots. Export source and target from consistent snapshots and include every table above.

```sql
SELECT json_build_object('table', 'stay_reservations', 'city', 'GOMA', 'key', json_build_array(id))
FROM public.stay_reservations;
```

```sql
SELECT json_build_object('table', 'stay_reservations', 'city', city_id, 'key', json_build_array(id))
FROM stay_reservations
WHERE city_id IN ('GOMA', 'KIN');
```

Repeat for each parent and child table. A clean report proves only that the supplied snapshots agree on keys. It does not prove column values, foreign keys, media objects, auth contacts or notification state.

## Open questions for the owner

1. Are property-scoped assignments copied as `identity_grants` with `PROPERTY` scope once identity opens that scope, or are they rebuilt from `host_members`?
2. Are ownership transfers and agreements copied as history or dropped with a note?
3. Is external channel sync required, which decides whether `stay_integration_mappings` is copied?
4. Are legacy operator documents copied into an application record, or kept as a separate host document table?
5. Which historic notifications and documents must be visible to users after cutover?
