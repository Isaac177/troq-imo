# B2 properties and media

Partial port. D2 follows the Mungini-owned host default. Legacy `nameEn` and `nameFr` fields need per-locale content tables under the Mungini multilingual rules; the property schema and migration are still open. Public amenity endpoints are wired but await compile and runtime verification. Global amenity demo rows cover active and retired records; they are shared across both cities. Rust is read-checked only.

| Legacy location | Mungini location | Status |
|---|---|---|
| `stay-service/src/media/validation.rs`: PNG and JPEG signatures, 5 MB limit, alt-text limit | `backend/crates/domain/src/properties/media/validation.rs` | implemented; typed rejection mapping to persisted media status and translated client codes remains open |
| `stay-service/src/media/storage.rs` and `template_media/storage.rs`: city-scoped deterministic private object keys, upload, signed read, delete | `backend/crates/domain/src/properties/media/storage.rs` | implemented as storage primitives; authorization, persistence and transaction cleanup remain open |
| `stay-service/src/media/handler.rs`, `queries.rs`, `access.rs`, `admin_content.rs`, `change.rs` | properties media services and routes | open: upload validation and rejected state, scoping, list pagination, cover, ordering, revisions, audit, cleanup, legacy content backfill |
| `stay-service/src/template_media/` | properties template media services and routes | open: template-specific authorization, storage ownership, content and states |
| `stay-service/src/properties/` | properties services and routes | open: property ownership, validation, per-locale content, revisions, permissions, schema and seed |
| `stay-service/src/amenities/queries.rs`: admin group and amenity lists, SQL filtering, search and totals | `properties/amenities/`, `00005_property_amenities.sql` | partial: translated paginated reads and seed written, not executed; endpoints and public and admin route permissions remain open |
| `stay-service/src/amenities/handler.rs`: catalog reads and admin create/update | `properties/amenities/catalog.rs`, `group_writes.rs`, `amenity_writes.rs` | partial: paginated active catalog, platform permission, single-locale writes and transactional audit and outbox implemented; owner compile and database verification, active-language validation, API routes and translations for error codes remain open |
| `stay-service/src/amenities/handler.rs` `ensure_known` | `properties/amenities/known.rs` | partial: validates known active or retired codes; not yet connected to property creation or updates; owner database verification open |
| `stay-service/src/amenities/`: legacy groups and amenities | `backend/seeds/properties/01-amenities.sql` | open: demo records only; legacy canonical taxonomy and migration/copy parity still required |
| `stay-service/src/accommodation_categories/`, `accommodation_templates/` | properties catalog | open: categories, templates, content, admin writes, permissions and seed |
| `stay-service/src/listings/` | properties listings | open: lifecycle, publication checks, permissions and seed |
| Gateway public amenity catalog | `backend/crates/api/src/amenities.rs` | partial: paginated active group and amenity routes with handler-derived OpenAPI; legacy nested response shape differs, owner compile and runtime verification open |
| Gateway property, media, admin amenity, category, template and listing routes | API and OpenAPI | open |
| Legacy tests and realistic Goma and Kinshasa seeds | B2 tests and seeds | open: validation, key and database catalog tests are written but not run; full behavior and status coverage remain |

Amenity and group change events use the acting city as outbox provenance; the catalog itself is global. Job dispatch and any event consumers remain open under B11.
