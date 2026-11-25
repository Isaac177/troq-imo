# Stay inventory in TROQ

Status: measured on 25/11/2026 against `main` at `tzone-rs` 780396f, `troq-front` 6a4aa7a3, `troqzone-app` 2677132. Counts come from a case-insensitive text search for `stay`, so a few hits can be the English word and not the feature. Check each shared file before editing it. The commands at the end regenerate every list.

Legend: PORT means copy into Mungini, then delete from TROQ. EDIT means the file stays in TROQ and only its Stay parts are removed. DELETE means remove from TROQ with nothing to port.

## Totals

| Repo | Files mentioning stay | Stay-only folders |
|---|---|---|
| `tzone-rs` | 472 (excluding `target`) | `stay-service` (422 files: 348 in `src`, 51 migrations, 19 seeds) |
| `troq-front` | web 80, admin 341, business 300, investor 6 | web 63 files, business 256, admin 257 |
| `troqzone-app` | 555 | `packages/merchant/src/stays` (477 files), `apps/mobile/components/stays` and assets (55 files) |
| `troq-pay-sdk` | 0 | none |

`payment-service`, `review-service`, `order-delivery-service` and `catalog-service` have no Stay references.

## tzone-rs

### stay-service (PORT, then DELETE)

- Binary `stay-service`, port 3090, `Dockerfile`, `Makefile`, `README.md`, `Cargo.toml` (depends on `troq-roles` and `troq-storage`).
- 51 migrations, `00001_operators_properties.sql` to `00053_template_media.sql` (numbers 00043 and 00044 do not exist).
- 19 seed files in `seeds/`: `goma*.sql`, `kin*.sql`, `calendar-contracts.sql`.
- Config: `STAY_DATABASE_URL`, `STAY_DATABASE_URL_KIN`, `STAY_SERVICE_PORT`, `STAY_GUEST_RETENTION_DAYS`, plus `NATS_*`.
- Source modules in `src/`: `accommodation_categories`, `accommodation_templates`, `admin_calendar`, `admin_claims`, `admin_commission`, `admin_dashboard`, `admin_guests`, `admin_rate_tools`, `admin_reservations`, `amendments`, `amenities`, `assignments`, `availability`, `blocks`, `calendar`, `cases`, `commission`, `credits`, `listings`, `media`, `operations`, `operator_detail`, `operators`, `platform_settings`, `properties`, `public_discovery`, `quotes`, `rate_admin`, `reports`, `reservations`, `restrictions`, `reviews`, `search_engine`, `sync`, `template_media`, `wishlists`.
- Cross-cutting files in `src/`: `audit.rs`, `change.rs`, `city.rs`, `config.rs`, `consumers.rs`, `content.rs`, `db.rs`, `db_tests.rs`, `engagement.rs`, `envelope.rs`, `error.rs`, `files.rs`, `guest_messages.rs`, `handler.rs`, `main.rs`, `membership.rs`, `migrations.rs`, `nats.rs`, `operator_scope.rs`, `outbox.rs`, `outbox_enrich.rs`, `paging.rs`, `permissions.rs`, `race_tests.rs`, `timezone.rs`.
- Tables (stay-db, per city): `stay_accommodation_categories`, `stay_accommodation_templates`, `stay_amendments`, `stay_amenities`, `stay_amenity_groups`, `stay_audit_records`, `stay_availability_blocks`, `stay_calendar_feed_tokens`, `stay_case_evidence`, `stay_cases`, `stay_claim_decisions`, `stay_claim_statements`, `stay_commission_batch_items`, `stay_commission_batches`, `stay_credit_accounts`, `stay_credit_entries`, `stay_idempotency_keys`, `stay_inbox`, `stay_integration_mappings`, `stay_inventory_allocations`, `stay_inventory_pools`, `stay_night_inventory`, `stay_operations_tasks`, `stay_operator_commission_rates`, `stay_operator_documents`, `stay_operator_status_history`, `stay_operators`, `stay_outbox`, `stay_physical_units`, `stay_platform_settings`, `stay_policy_versions`, `stay_pool_physical_units`, `stay_promotion_funding`, `stay_promotions`, `stay_properties`, `stay_property_media`, `stay_property_revisions`, `stay_quotes`, `stay_rate_plans`, `stay_rate_rules`, `stay_records`, `stay_reservation_guests`, `stay_reservation_items`, `stay_reservation_messages`, `stay_reservations`, `stay_restrictions`, `stay_reviews`, `stay_saved_searches`, `stay_search_events`, `stay_template_media`, `stay_unit_assignments`, `stay_unit_types`, `stay_wishlist_items`, `stay_wishlists`, plus the SQL function for room availability (migrations 00034 and 00035).
- Events: outbox publishes `stay.events.<event>.v<version>` to the `STAY_EVENTS` JetStream stream. The `stay.sync.>` subjects carry operator sync from `auth-service`.
- NATS request/reply subjects follow `stay.<entity>.<action>` (for example `stay.reservation.hold`, `stay.task.transition`, `stay.review.admin.decide`, `stay.wishlist.item.add`) and are listed with their permissions in `stay-service/README.md`. Notification subjects look like `stay.reservation_held.host` and `stay.review_replied.guest`.

### api-gateway (PORT the HTTP shape, then EDIT)

- All 39 files in `src/routes/stays/` (DELETE after port): accommodation categories and templates, admin cases, guests, property, rates, tasks, templates and unit types, amendments, amenities, blocks, calendar, cancellation, cases, credits, engagement routes, feed links, guard, inventory, listings and listing admin, me, media, messages, mine, operators, properties, public discovery, quotes, rate admin, reports, reservations, reviews, tasks, template media.
- Admin routes in `src/routes/admin/`: `stays.rs`, `stay_calendar.rs`, `stay_claims.rs`, `stay_commission.rs`, `stay_dashboard.rs`, `stay_handlers.rs`, `stay_operator_detail.rs`, `stay_rates.rs`, `stay_reservations.rs`, `stay_review_handlers.rs`, `stay_settings.rs`, `stay_team.rs` (DELETE after port).
- Other route files: `routes/public/stays.rs`, `routes/private/customer_stays.rs`, `routes/merchant/stays.rs`, `routes/auth/stay_grants.rs` (DELETE after port).
- `src/middleware/stay_permissions.rs` (DELETE after port).
- EDIT: `src/server.rs` (the `protected_stays_router`, the `/api/stays` and `/api/merchant/stays` mounts and the `stay_business_only` layer), `src/routes/mod.rs`, `src/routes/public/mod.rs`, `src/routes/private/mod.rs`, `src/routes/merchant/mod.rs`, `src/routes/admin/mod.rs`, `src/routes/auth/mod.rs`, `src/routes/auth/team_admin.rs`, `src/routes/merchant/company.rs`, `src/middleware/mod.rs`, `src/middleware/auth.rs`, `src/middleware/authorization.rs`.

### auth-service (PORT, then EDIT and new migration)

- Tables: `stay_operators`, `stay_operator_members`, `stay_operator_sync_outbox`, `stay_property_assignments`, `stay_ownership_transfers`, `stay_agreement_versions`, `stay_agreement_acceptances`, `stay_grant_presets`, `stay_grant_preset_permissions`.
- Stay migrations: 00011 stay permissions, 00012 stay manager role, 00015 grant presets, 00016 operators, 00017 ownership transfer, 00018 operator agreements, 00019 status events, 00020 sync outbox, 00021 onboarding, 00028 key split, 00029 review presets, 00037 company type stay, 00051 stay operators to businesses, 00055 business settings permission, 00056 business delegation, 00057 manager CRM permissions. Applied migrations are never edited. Removal is a new numbered migration that drops the tables and cleans the data.
- Source: `src/company/handler/read_stay_company.rs`, `write_stay_company.rs`, and the Stay parts of `src/scoped_grants/` (`admin_preset_write`, `admin_operator_team`, `authorize`, `permission_catalog`, `admin_business_grants`, `presets`, `handler_admin`, `catalog`, `business_scope`, `operator_sync`). The search output was cut at 30 files, so rerun the grep for the full set.
- NATS: `auth.stay.my_operators`, `auth.stay.permissions`, `auth.stay.business_permissions`.

### messaging-service (PORT, then DELETE)

- `src/stays/{mod,notify}.rs`, `src/notify/copy/stays.rs`.
- Documents: `src/documents/stay_voucher.rs`, `stay_cancellation.rs`, `stay_damage_claim.rs`, `src/documents/payload/{stay,stay_cancellation,stay_damage_claim}.rs`.
- Templates: `templates/documents/stay_{voucher,cancellation,damage_claim}.hbs` and six fixtures `templates/fixtures/stay_*-{en,fr}.json`.
- Migrations 00012, 00013 and 00014 (documents for voucher, cancellation, damage claim). New removal migration needed for rows already inserted.

### crm-service (EDIT, new migration)

- `src/engine/stay_consumer.rs` (DELETE), plus Stay branches in `src/engine/{mod,types,subscriptions,mapping}.rs`, `src/report/validation.rs`, `src/invoice/validation.rs`, `src/budget/{pack_admin,shop_handler}.rs`.
- Migrations 00026 (engine leads), 00061 to 00064 add and later drop `stay_operator_id` and operator tenant columns on audit and webhook tables. Seeds `operator_*.sql` in `seeds/`.
- Hosts used CRM through the business app. Decision D8 covers what Mungini does about it.

### Shared crates and infra

- `troq-notify/src/stays.rs` (PORT, DELETE), plus Stay entries in `src/lib.rs` and `src/roles.rs`.
- `troq-roles/roles.json` and `src/generated.rs`: the `stay_manager` role (EDIT, and every client `roles.ts`).
- `Cargo.toml` workspace member `stay-service` and `Cargo.lock` (EDIT).
- `docker-compose.yml`, `docker-compose.dev.yml`: `stay-db` (container `troq-stay-db`, dev `troqzone-stay-db-dev`, database `stay-db_goma`, volume `stay_db_data`) and `stay-service` (image `ghcr.io/isaac177/troq-stay`). `docker-bake.hcl` target `stay-service`. `Makefile` push line for `troq-stay`. `.env.example`. `.env.enc` and `.env.dev.enc` hold `STAY_*` values and are the owner's to edit.
- `scripts/seed_scoped_grants.sql`, `scripts/seed_preview_accounts.sql` (EDIT).

## troq-front

### troqzone-web (Next.js storefront)

- PORT then DELETE: `app/[lang]/stays/`, `app/api/stays-image/`, `app/api/stays-detail/`, `src/components/stays/`, `src/components/user/stays/`, `src/lib/stays/`, `src/utils/stays/` (63 files).
- EDIT: `next.config.ts`, `src/types/user.ts`, `src/shared/roles.ts`, `src/components/Home.tsx`, `src/components/ProfilePage.tsx`, `src/components/home/StaysPromoSection.tsx` and `home/hooks/useStaysPromo.ts` (DELETE), `src/components/header/mainNav/MainNavDesktop.tsx`, `src/hooks/header/mobileNav/useMobileNav.ts`, `src/features/notifications/lib/{refreshTargets,categoryIcons}.ts`, `src/i18n/i18n.ts`.
- Locales: `src/shared/i18n/locales/{en,fr}/stays.json` (PORT, DELETE), Stay keys in `notificationCenter.json`, `src/i18n/locales/{en/enProfile,fr/frProfile}.json`.

### troqzone-business (host app source)

- PORT then DELETE: `src/components/stays/` (138 files), `src/hooks/stays/` (70), `src/utils/stays/` (48), `src/types/stay*.ts` (accommodation categories, amenities, calendar, cases, credits, media, operators, properties, property create, rates, reservations, reviews, tasks, templates, trips, type images), `src/routes/staysRoutes.ts`, `src/components/layouts/StaysLayout.tsx`, `StaysCalendarLayout.tsx`, `src/components/admin/nav/staysNavigation.ts`, `src/components/settings/cards/StayBusinessProfileCard.tsx`, `src/styles/stays-shell.css`.
- Locales PORT then DELETE: `src/shared/i18n/locales/{en,fr}/stayNavigation.json`, `en/stayCalendarBlocks.json`, `en/stayProperties.json`, `src/i18n/locales/{en/enStays,fr/frStays,en/enStayCredits}.json`.
- EDIT: `src/routes/index.tsx`, `src/routes/crmRoutes.tsx`, `src/shared/{roles,roles.test,businessWorkspace,businessWorkspace.test}.ts`, `src/components/admin/nav/{Adminnav.tsx,hooks/useBusinessSidebar.ts}`, `src/components/auth/hooks/useProtectedRoute.ts`, `src/utils/auth/authRouting.ts`, `src/utils/settings/settingsForm.ts`, `src/hooks/settings/useSettingsPage.ts`, `src/hooks/shell/useBusinessWorkspace.ts`, `src/components/settings/SettingsPage.tsx`, `src/lib/api/config.ts`, `src/i18n/i18n.ts`, `src/features/crm/**` (team presets, access, adapters, breadcrumbs, permission catalog), `src/features/notifications/lib/{refreshTargets,categoryIcons,env}.ts`, Stay keys in `enMerchant.json`, `enCrmTeam.json`, `frCrmTeam.json`, `adminDesign.json`, `notificationCenter.json`.

### troqzone-admin (admin app source)

- PORT then DELETE: `src/components/admin/stays/` (136 files), `src/hooks/admin-stays/` (77), `src/utils/admin-stays/` (44), `src/types/stays-*.ts` (templates, admin dashboard, cases, claims, reservations, reviews, admin, settings, operator detail, and others), plus Stay routes, nav entries, locales and tests found by the grep below.
- EDIT: shared nav, router, role and notification files that mention Stay. The full list is in the grep output (341 files in total, 257 of them in the folders above).

### troqzone-investor

- EDIT only (6 files): `README.md`, `src/shared/roles.ts`, `src/features/notifications/lib/{refreshTargets,categoryIcons}.ts`, `notificationCenter.json` in `en` and `fr`.

## troqzone-app

- PORT then DELETE: `packages/merchant/src/stays/` (477 files) in areas `calendar`, `cases`, `credits`, `home`, `listings`, `navigation`, `notifications`, `rates`, `reservations`, `reviews`, `shared`, `tasks`, `templates`.
- PORT then DELETE, guest side: `apps/mobile/components/stays/` and `apps/mobile/assets/stays/` (55 files), screens `StaysScreen`, `StayTripsScreen`, `StayBookingScreen`, `StayDetailScreen`, `StayFavoritesScreen`, `components/profile/StayManagementCard.tsx`, `hooks/drawer/useStayDrawerLinks.ts`.
- EDIT: `apps/mobile/screens/{MarketplaceHome,ProfileScreen}.tsx`, `apps/mobile/components/app/AppNavigation.tsx`, `components/navigation/MarketplaceDrawerContent.tsx`, `navigation/{types,marketplaceScreens}.ts`, `hooks/drawer/useMarketplaceDrawer.ts`, `hooks/app/useMarketplaceNavVisibility.ts`, `types/roles.ts`, `packages/shared-types/types/roles.ts`, `packages/shared-i18n/index.ts`, `packages/merchant/index.ts`, `packages/merchant/src/navigation/{MerchantNavigator.tsx,merchantSections.ts}`, `packages/merchant/src/hooks/useMerchantDrawer.ts`, `packages/merchant/src/areas/communicationSettings/utils/notificationPreferenceRules.ts`, `packages/merchant/src/areas/productCatalog/utils/aiCreditsAmount.test.ts` (these last two may be false positives).
- Locales PORT then DELETE in `packages/shared-i18n/locales/{en,fr}/`: `stays.json`, `stayCalendarBlocks.json`, `merchantStaysAccount.json`, `merchantStaysCalendar.json`, `merchantStaysCreate.json`, `merchantStaysListings.json`, `merchantStaysOps.json`, `merchantStaysRates.json`, `merchantStaysReservations.json`, `merchantStaysShared.json` (confirm each has an `fr` twin). EDIT the Stay keys in `merchant.json`, `adminDesign.json`, `notificationCenter.json`, `merchantProductCatalog.json`.
- Docs: `docs/MERCHANT_STAYS_AGENT_BRIEF.md` and `docs/MERCHANT_STAYS_MOBILE_PLAN.md` (already copied to `Mungini/docs/legacy-troq-stays/mobile/`, DELETE the originals), `docs/mobile-panelui-refactor-agents.md` and `docs/MERCHANT_BUSINESS_MOBILE_PLAN.md` (EDIT the Stay parts).

## Documents at the TROQ root

Already copied, unedited, to `Mungini/docs/legacy-troq-stays/` as outdated reference (they were written to embed Stay in TROQ). Remove the originals from TROQ when Phase 3 is done:

- `ADMIN_STAYS_ROADMAP.md`, `STAYS_ACCOMMODATION_IMAGES_TASKS.md`, `STAY_COMMISSION_CREDITS_PLAN.md`, `STAY_UI.md`.
- `docs/stays/` (specs 00 to 21 and `README.md`), `docs/stays-guest-rollout.md`.

Mixed documents that mention Stay among other topics (EDIT the Stay parts only): `DESIGN.md`, `HANDOFF.md`, `backend-route-inventory.md`, `client-endpoint-inventories.md`, `panel-components.md`, `routes-refactoring-plan.md`, `web-read-endpoint-payload-audit.md`.

## Coupling that needs a decision before removal

- Role `stay_manager` in the shared role enum, and users who hold it (D2).
- Stay hosts are `auth-service` businesses with `company_type` stay and scoped grants. Mungini `hosts` must reproduce this without the TROQ business table (D2).
- The business app navigation and settings treat a Stay workspace as one of the business workspaces (`businessWorkspace.ts`, `staysNavigation.ts`).
- Notification categories, refresh targets and icons for Stay in all four web apps and mobile.
- Replicated global data: amenities and accommodation categories are owned by `stay-service` and replicated to each city database.

## Regenerate this inventory

Run from `/Users/jogues/Documents/TROQ`. Use these only to read, never to bulk-replace.

```bash
grep -rIil stay tzone-rs --exclude-dir=target --exclude-dir=.git | sort
```

```bash
grep -rIil stay troq-front --exclude-dir=node_modules --exclude-dir=dist --exclude-dir=.next --exclude-dir=.git | sort
```

```bash
grep -rIil stay troqzone-app --exclude-dir=node_modules --exclude-dir=.git | sort
```

```bash
find tzone-rs troq-front troqzone-app \( -name node_modules -o -name target -o -name .git -o -name .next -o -name dist \) -prune -o -type d -iname "*stay*" -print
```

After a removal, the same greps should list only files that are intentionally left (for example the English word "stay" in unrelated text).
