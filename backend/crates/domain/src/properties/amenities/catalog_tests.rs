use kernel::paging::PageRequest;
use sqlx::PgPool;

use super::{ensure_known, list_active_amenities, list_active_groups, list_amenities, AmenityFilters};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

async fn seed(pool: &PgPool) {
    sqlx::raw_sql(include_str!("../../../../../seeds/properties/01-amenities.sql"))
        .execute(pool)
        .await
        .unwrap();
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn public_catalog_filters_states_and_paginates(pool: PgPool) {
    seed(&pool).await;
    let first = list_active_amenities(&pool, "fr", None, PageRequest { page: 1, page_size: 1 }).await.unwrap();
    let second = list_active_amenities(&pool, "fr", None, PageRequest { page: 2, page_size: 1 }).await.unwrap();
    assert_eq!(first.total, 3);
    assert_eq!(second.total, 3);
    assert_eq!(first.items[0].code, "SEED_WIFI");
    assert_eq!(second.items[0].name, "Eau chaude");
    let groups = list_active_groups(&pool, "fr", PageRequest::default()).await.unwrap();
    assert_eq!(groups.total, 2);
    assert_eq!(groups.items[0].name, "Essentiels");
    let filtered = list_active_amenities(&pool, "en", Some("SEED_OUTDOOR"), PageRequest::default()).await.unwrap();
    assert_eq!(filtered.total, 1);
    assert_eq!(filtered.items[0].code, "SEED_LAKE_VIEW");
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn search_and_translation_fallback_follow_stored_content(pool: PgPool) {
    seed(&pool).await;
    let filters = AmenityFilters { search: Some("eau chaude".into()), ..Default::default() };
    let found = list_amenities(&pool, "en", &filters, PageRequest::default()).await.unwrap();
    assert_eq!(found.total, 1);
    assert_eq!(found.items[0].name, "Hot water");
    let fallback = list_active_amenities(&pool, "es", Some("SEED_OUTDOOR"), PageRequest::default()).await.unwrap();
    assert_eq!(fallback.items[0].name, "Lake view");
    let filters = AmenityFilters { search: Some("%".into()), ..Default::default() };
    assert_eq!(list_amenities(&pool, "en", &filters, PageRequest::default()).await.unwrap().total, 0);
}

#[sqlx::test(migrator = "MIGRATOR")]
async fn known_validation_includes_retired_amenities(pool: PgPool) {
    seed(&pool).await;
    assert!(ensure_known(&pool, &[]).await.is_ok());
    assert!(ensure_known(&pool, &["SEED_WIFI".into(), "SEED_RETIRED_FAN".into()]).await.is_ok());
    let error = ensure_known(&pool, &["SEED_WIFI".into(), "MISSING".into()]).await.unwrap_err();
    assert_eq!(error.code().as_str(), "AMENITY_UNKNOWN");
}
