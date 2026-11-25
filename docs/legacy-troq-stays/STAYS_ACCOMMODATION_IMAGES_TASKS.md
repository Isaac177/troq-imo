# Accommodation type images: shared task file

Goal: every accommodation type (template, `stay_accommodation_templates`) gets multiple images. Images follow the type everywhere it appears: the business app type editor, the listing editor, the storefront and mobile room picker, and every booking/reservation table or detail that names an accommodation type.

Follow `/Users/jogues/Documents/TROQ/AGENTS.md` (no comments, 300 line files, hooks hold logic, i18n en+fr in `troqzone-app/packages/shared-i18n/locales/`, seeds, no cargo, `bunx tsc` only, mobile first for the business app). Do not run cargo or any build.

## Findings

- Business app route `/stays/accommodation-types` renders `troqzone-business/src/components/stays/templates/` (Page, Filters, List, Pager, Sheet). Types in `src/types/stayTemplates.ts`.
- Admin app has the same module: `troqzone-admin/src/types/stays-templates.ts`, `hooks/admin-stays/useAdminStayTemplatesPage.ts`, routes in `routes/admin/stays.ts`.
- Backend: `tzone-rs/stay-service/src/accommodation_templates/` (types, queries, handler, subscriptions, admin_subscriptions). A listing's rooms are `stay_unit_types` rows with `templateId` (migration 00031). Seeds in `tzone-rs/stay-service/seeds/`.
- Property images already exist: `stay-service/src/media/` (table `stay_property_media`, base64 upload, MinIO via `media/storage.rs`, `ensure_url`, validation in `media/validation.rs`, cover and sortOrder, optimistic `version`). Gateway: `api-gateway/src/routes/stays/media.rs`, `admin_property.rs`, `merchant/stays.rs`, `admin/stays.rs`, `public/stays.rs`. Business UI for it: `useStayMedia*.ts`, `useStayPropertyCover.ts`. Reuse these patterns for templates.
- Public detail: `stay-service/src/public_discovery/handler.rs` builds property `media` and `unitTypes`; the storefront reads it in `troqzone-web/src/lib/stays/publicDetail.ts`, `components/stays/detail/StayRooms.tsx`, `hooks/useStayChoice.ts`. Mobile: `troqzone-app/apps/mobile/components/stays/hooks/useStayDetail.ts`, `useStayBookingChoice.ts`, `stayBookingTypes.ts`.
- Reservation surfaces naming a unit type: stay-service `reservations/`, `admin_reservations/`, `calendar`, `rate_admin`; business `components/stays/reservations/`, `trips/`, `calendar/`; admin `components/admin/stays/reservations/`, `calendar/`; storefront trips; mobile booking and trips.

## Contract (all agents code against this)

Table, new migration `00053_template_media.sql` in `stay-service/migrations/`:
`stay_template_media` with the same columns as `stay_property_media` but `"templateId"` (FK `stay_accommodation_templates(id)`) instead of `"propertyId"`, plus `"storageKey"`. Unique cover per template, unique `("templateId","clientKey")`.

Media view returned everywhere (same shape as the property one, with `templateId`; `storageKey` is internal and never returned):
`{ id, templateId, status, sortOrder, isCover, altEn, altFr, fileName, mimeType, sizeBytes, rejectionReason, version, createdAt, updatedAt }`

Gateway routes (merchant mount, same auth as `/accommodation-templates`; admin mount under `/stays/` with `admin_` handlers):
Merchant mount uses `{id}` as the template path param (matches the existing `PATCH /accommodation-templates/{id}`); the URLs are identical for clients.
- `GET /accommodation-templates/{templateId}/media`
- `POST /accommodation-templates/{templateId}/media` body `{ clientKey, fileName, mimeType, contentBase64, altEn, altFr }`
- `PATCH /accommodation-templates/{templateId}/media/{mediaId}` body `{ expectedVersion, altEn?, altFr?, isCover?, sortOrder? }`
- `DELETE /accommodation-templates/{templateId}/media/{mediaId}` body `{ expectedVersion }`
- Content: `GET /media/{mediaId}/content` (existing) must also resolve template media.

Admin mount: `GET|POST /stays/accommodation-templates/{templateId}/media`, `PATCH|DELETE /stays/accommodation-templates/{templateId}/media/{mediaId}`, content via the existing `GET /stays/media/{mediaId}/content`. Public content via the existing `GET /stays/media/{mediaId}/content` (public mount) also resolves template media that belongs to a unit type of a published property. Content responses for template media are `{ media, url }` like property media (merchant/admin) and `{ mediaId, mimeType, altEn, altFr, url }` (public).

NATS subjects: `stay.template.media.list|add|update|remove` and `stay.admin.template.media.list|add|update|remove`.

Template list/detail view gains: `coverUrl: string | null`, `imageCount: number`.

Public detail `unitTypes[]` (and merchant/admin unit type lists) gain:
`images: [{ id, url, altEn, altFr, isCover }]` ordered cover first then `sortOrder`, and `coverUrl: string | null`. A unit type with no template images falls back to an empty list, never to property images.

Implemented field placement (backend):
- Template list/create/update views: `coverUrl`, `imageCount`.
- Public detail `unitTypes[]`, merchant and admin `GET .../unit-types` list items: `images`, `coverUrl`. Create and update unit type responses return `images: []`, `coverUrl: null`; reload the list for images.
- Guest reservation view (hold, get, decision responses): `unitTypeCoverUrl`. Quote view: `unitTypeCoverUrl`. Guest trips list (`/reservations/mine`) items: `unitTypeCoverUrl`.
- Merchant reservations list and detail items: `unitTypeCoverUrl`; detail also `unitType: { id, nameEn, nameFr, images[] }` (absent on list items).
- Calendar (merchant and admin): each `units[]` entry and each `reservations[]` entry has `unitTypeCoverUrl`.
- Admin reservations list rows: `unitTypeId`, `unitTypeCoverUrl`. Admin detail: `items[].unitTypeId`, `items[].unitTypeCoverUrl`, and top level `unitType: { id, nameEn, nameFr, images[] }` for the first item.

Reservation, quote, trip and calendar payloads that already carry `unitTypeId` or a unit type name gain `unitTypeCoverUrl: string | null`. Merchant and admin reservation detail also gain `unitType: { id, nameEn, nameFr, images[] }`.

Seed: every seeded template gets 2 to 4 `seed-tmplmedia-*` images (small real JPEG/PNG bytes or storage keys the existing seeding uses for property media), one cover each, covering a template with zero images too.

## Workstreams

### Agent 1: backend (Rust, `tzone-rs/`)
Owns `stay-service` and `api-gateway` only.
1. Migration 00053 and a `template_media` module (types, queries, validation reuse, handler, subscriptions, admin subscriptions) wired in `main.rs`; split files under 300 lines; reuse `media/storage.rs`, `validation.rs`.
2. Template views: `coverUrl`, `imageCount`; deleting or archiving rules unchanged.
3. Public detail, merchant and admin unit type lists: `images`, `coverUrl`.
4. Reservation, quote, trip, calendar, admin reservation payloads: `unitTypeCoverUrl`, plus `unitType` object on merchant and admin reservation detail.
5. Gateway routes for merchant and admin, content endpoint resolving template media.
6. Seeds for all cities in `stay-service/seeds/` and the seed README if present.
7. Report the final field names if anything differs from the contract; update the Contract section of this file.

### Agent 2: business app and admin app (`troq-front/troqzone-business`, `troq-front/troqzone-admin`)
1. Business accommodation type sheet: multi image uploader (drop, multiple files, reorder, cover, alt text en/fr, remove), cover thumbnail and image count in the list, pagination and row number unchanged. Reuse `useStayMedia*` patterns, new hooks in `hooks/`. Check at 375px in en and fr.
2. Business listing editor (`StayUnitTypesEditor.tsx`, create flow review and pricing steps): show each added accommodation type with its cover.
3. Business reservations list, detail rows, trips, calendar: show the chosen accommodation type with its cover thumbnail; merchant detail shows the type gallery.
4. Admin templates page, listing room editor (`stays-rooms`), admin reservations table and detail, admin calendar: same image management and display.
5. Types in `src/types/`, API clients, i18n keys en+fr in `shared-i18n`.
6. `bunx tsc` in both apps.

### Agent 3 (me): storefront and mobile (`troq-front/troqzone-web`, `troqzone-app`)
1. `publicDetail.ts` types, `StayRooms.tsx` and `useStayChoice.ts`: room cards with image carousel and lightbox per accommodation type, selected room image in the booking panel and reserve dialog.
2. Storefront trips and confirmation views: show the chosen type image.
3. Mobile stay detail and booking choice, booking summary and trips: same.
4. i18n keys en+fr, `bunx tsc` in web and mobile.

## Rules for parallel work

- Agents touch only their own folders. Shared i18n files are edited by appending keys under a `stays.accommodationImages` namespace (en and fr), re-reading the file right before each edit.
- If the contract must change, edit this file first and tell the others via SendMessage.
- Status log below: each agent appends one line per finished item.

## Status log

- Agent 3 (storefront + mobile): room image gallery in StayRooms/StayRoomImages, lightbox, booking panel and reserve dialog covers, My stays and mobile trips use unitTypeCoverUrl, mobile room gallery. tsc clean web and mobile.
- Agent 1 (backend) items 1 and 5 done: migration `00053_template_media.sql`, module `stay-service/src/template_media/`, subscriptions wired in `main.rs` (plus backfill task), gateway routes in `routes/stays/template_media.rs`, merchant and admin mounts, content endpoints resolve template media (merchant, admin, public).
- Agent 1 (backend) items 2, 3 and 4 done: template views, unit type lists and public detail, reservation, quote, trips, calendar and admin reservation payloads carry the image fields listed in the Contract section.
- Agent 1 (backend) item 6 done: `stay-service/seeds/goma_template_media.sql` and `kin_template_media.sql`, README updated. Nothing compiled or run (no cargo). Owner steps: restart stay-service (migration 00053 runs at startup), restart api-gateway, then apply the two seed files to the local city databases.
- Agent 2 item 1 done: business type sheet multi image editor (useStayMediaEditor now takes a scope; templateMediaScope), list cover and image count column.
- Agent 2 item 2 done: business StayUnitTypesEditor, create flow pricing and review steps show type covers (UnitTypeDraft.coverUrl, StayUnitType.images and coverUrl).
- Agent 2 item 3 done: business reservations list and detail (cover, type gallery from unitType.images), trips (unitTypeCoverUrl), calendar grid and list view (unit.unitTypeCoverUrl).
- Agent 2 item 4 done: admin templates page (table cover and count, sheet image manager via StayPhotosManager scope), rooms table and sheet (cover, count, read-only gallery), reservations table and detail, calendar grid.
- Agent 2 item 5 done: types in business types/stayTypeImages.ts and admin types/stays-type-images.ts; i18n keys added under stays.accommodationImages in the business and admin local locale files (those apps do not read shared-i18n); the business also has tpl_* additions in stayProperties.json.
- Agent 2 item 6 done: bunx tsc -p tsconfig.app.json --noEmit clean in troqzone-business and troqzone-admin. Not checked in a browser (backend routes not live yet).
- Agent 2 contract assumptions: coverUrl and images[].url are used directly as img src; calendar unit rows read unit.unitTypeCoverUrl; merchant trips only read unitTypeCoverUrl; template create response must return the created template in data.
