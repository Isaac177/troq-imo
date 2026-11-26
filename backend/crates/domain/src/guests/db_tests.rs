use chrono::NaiveDate;
use kernel::city::City;
use kernel::ids::new_id;
use kernel::paging::PageRequest;
use kernel::storage::{Storage, StorageConfig};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use super::search::{run_search, RawSearch, SearchQuery, SearchResponse};
use super::wishlists::{
    add_wishlist_item, create_wishlist, list_items, list_wishlists, remove_wishlist_item,
    CreateWishlist, ItemsQuery,
};
use super::saved_searches::{create_saved_search, delete_saved_search, list_saved_searches, CreateSavedSearch};

const LAKESIDE: &str = "seed-prop-goma-lakeside";
const PAUSED: &str = "seed-prop-goma-paused";
const VILLA: &str = "seed-unit-type-goma-villa";

async fn test_pool() -> PgPool {
    let url = std::env::var("MUNGINI_TEST_DATABASE_URL")
        .expect("set MUNGINI_TEST_DATABASE_URL to a database with the guests, properties and inventory seeds");
    PgPoolOptions::new().max_connections(8).connect(&url).await.expect("connect to test database")
}

fn storage() -> Storage {
    Storage::new(StorageConfig::from_env().expect("storage environment"))
}

fn guest() -> String {
    new_id("guest-test")
}

fn new_list(name: &str) -> CreateWishlist {
    CreateWishlist { name: name.into(), visibility: Some("SHARED".into()), idempotency_key: None }
}

fn first_page() -> PageRequest {
    PageRequest::default()
}

#[tokio::test]
#[ignore]
async fn a_list_belongs_to_its_owner_only() {
    let pool = test_pool().await;
    let (owner, other) = (guest(), guest());
    let list = create_wishlist(&pool, City::GOMA, &owner, new_list("Lake trip")).await.unwrap();
    assert!(add_wishlist_item(&pool, City::GOMA, &other, &list.id, LAKESIDE).await.is_err());
    assert_eq!(list_wishlists(&pool, City::GOMA, &other, first_page()).await.unwrap().total, 0);
    assert_eq!(list_wishlists(&pool, City::GOMA, &owner, first_page()).await.unwrap().total, 1);
    assert_eq!(list_wishlists(&pool, City::KIN, &owner, first_page()).await.unwrap().total, 0);
}

#[tokio::test]
#[ignore]
async fn only_listed_properties_can_be_saved_and_removed_again() {
    let pool = test_pool().await;
    let owner = guest();
    let list = create_wishlist(&pool, City::GOMA, &owner, new_list("Lake trip")).await.unwrap();
    assert!(add_wishlist_item(&pool, City::GOMA, &owner, &list.id, PAUSED).await.is_err());
    add_wishlist_item(&pool, City::GOMA, &owner, &list.id, LAKESIDE).await.unwrap();
    let again = add_wishlist_item(&pool, City::GOMA, &owner, &list.id, LAKESIDE).await.unwrap();
    assert_eq!(again.item_count, 1, "adding twice keeps one entry");
    let items = list_items(&pool, City::GOMA, &owner, &list.id, &ItemsQuery::default()).await.unwrap();
    assert_eq!(items.items[0].property_id, LAKESIDE);
    let filtered = ItemsQuery { property_ids: Some("nothing".into()), ..ItemsQuery::default() };
    assert_eq!(list_items(&pool, City::GOMA, &owner, &list.id, &filtered).await.unwrap().total, 0);
    let emptied = remove_wishlist_item(&pool, City::GOMA, &owner, &list.id, LAKESIDE).await.unwrap();
    assert_eq!(emptied.item_count, 0);
}

#[tokio::test]
#[ignore]
async fn creating_a_list_twice_with_one_key_returns_the_first() {
    let pool = test_pool().await;
    let owner = guest();
    let key = Some(new_id("key"));
    let input = || CreateWishlist { name: "Once".into(), visibility: None, idempotency_key: key.clone() };
    let first = create_wishlist(&pool, City::GOMA, &owner, input()).await.unwrap();
    let second = create_wishlist(&pool, City::GOMA, &owner, input()).await.unwrap();
    assert_eq!(first.id, second.id);
    assert_eq!(list_wishlists(&pool, City::GOMA, &owner, first_page()).await.unwrap().total, 1);
}

#[tokio::test]
#[ignore]
async fn saved_searches_are_deleted_only_by_their_owner() {
    let pool = test_pool().await;
    let (owner, other) = (guest(), guest());
    let input = CreateSavedSearch {
        parameters: json!({ "city": "GOMA", "adults": 2 }),
        alert_consent: true,
        idempotency_key: None,
    };
    let saved = create_saved_search(&pool, City::GOMA, &owner, input).await.unwrap();
    assert!(delete_saved_search(&pool, City::GOMA, &other, &saved.id).await.is_err());
    assert_eq!(list_saved_searches(&pool, City::GOMA, &owner, first_page()).await.unwrap().total, 1);
    delete_saved_search(&pool, City::GOMA, &owner, &saved.id).await.unwrap();
    assert_eq!(list_saved_searches(&pool, City::GOMA, &owner, first_page()).await.unwrap().total, 0);
}

fn fresh_window() -> (NaiveDate, NaiveDate) {
    let spread = (uuid_spread() % 200) as i64;
    let check_in = chrono::Utc::now().date_naive() + chrono::Duration::days(60 + spread);
    (check_in, check_in + chrono::Duration::days(2))
}

fn uuid_spread() -> u128 {
    chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default() as u128
}

async fn open_inventory(pool: &PgPool, check_in: NaiveDate, check_out: NaiveDate) {
    for (pool_id, sellable) in [("seed-pool-goma-villa", 1), ("seed-pool-goma-room", 1), ("seed-pool-goma-triple", 3)] {
        for night in check_in.iter_days().take_while(|day| *day < check_out) {
            sqlx::query(
                r#"INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount")
                   VALUES ($1, $2, $3, 0)
                   ON CONFLICT ("poolId", "stayDate") DO UPDATE SET "sellableCapacity" = $3, "allocatedCount" = 0"#,
            )
            .bind(pool_id)
            .bind(night)
            .bind(sellable)
            .execute(pool)
            .await
            .unwrap();
        }
    }
}

fn window_search(check_in: NaiveDate, check_out: NaiveDate) -> RawSearch {
    RawSearch {
        check_in: Some(check_in),
        check_out: Some(check_out),
        adults: Some(2),
        children: Some(0),
        ..RawSearch::default()
    }
}

async fn run(pool: &PgPool, raw: RawSearch, consent: bool) -> SearchResponse {
    let query = SearchQuery::from_raw(raw).unwrap();
    run_search(pool, &storage(), City::GOMA, "en", query, consent).await.unwrap()
}

#[tokio::test]
#[ignore]
async fn search_prices_the_lakeside_with_offers_and_filters() {
    let pool = test_pool().await;
    let (check_in, check_out) = fresh_window();
    open_inventory(&pool, check_in, check_out).await;
    let response = run(&pool, window_search(check_in, check_out), false).await;
    let hit = response.items.iter().find(|hit| hit.id == LAKESIDE).expect("lakeside is found");
    assert!(hit.offers.len() >= 3);
    assert!(hit.best.total.is_some());
    assert_eq!(response.stats.result_count, response.total);
    let instant = run(&pool, RawSearch { instant_booking: true, ..window_search(check_in, check_out) }, false).await;
    let hit = instant.items.iter().find(|hit| hit.id == LAKESIDE).unwrap();
    assert!(hit.offers.iter().all(|offer| offer.booking_mode == "INSTANT"));
    let step_free = run(&pool, RawSearch { accessibility: vec!["STEP_FREE_ENTRY".into()], ..window_search(check_in, check_out) }, false).await;
    let hit = step_free.items.iter().find(|hit| hit.id == LAKESIDE).unwrap();
    assert!(hit.offers.iter().all(|offer| offer.unit_type_id == VILLA));
}

#[tokio::test]
#[ignore]
async fn distance_filter_uses_the_search_point() {
    let pool = test_pool().await;
    let (check_in, check_out) = fresh_window();
    open_inventory(&pool, check_in, check_out).await;
    let near = RawSearch {
        latitude: Some(-1.6792),
        longitude: Some(29.2228),
        max_distance_km: Some(1.0),
        sort: Some("distance".into()),
        ..window_search(check_in, check_out)
    };
    assert!(run(&pool, near, false).await.items.iter().any(|hit| hit.id == LAKESIDE));
    let far = RawSearch { latitude: Some(0.0), longitude: Some(0.0), max_distance_km: Some(1.0), ..window_search(check_in, check_out) };
    assert!(run(&pool, far, false).await.items.iter().all(|hit| hit.id != LAKESIDE));
}

#[tokio::test]
#[ignore]
async fn analytics_need_consent_and_never_keep_child_ages() {
    let pool = test_pool().await;
    let (check_in, check_out) = fresh_window();
    open_inventory(&pool, check_in, check_out).await;
    let raw = RawSearch { children: Some(1), child_ages: vec![5], adults: Some(3), ..window_search(check_in, check_out) };
    let fingerprint = SearchQuery::from_raw(raw.clone()).unwrap().fingerprint();
    let count = |pool: PgPool, fingerprint: String| async move {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM stay_search_events WHERE fingerprint = $1")
            .bind(fingerprint)
            .fetch_one(&pool)
            .await
            .unwrap()
    };
    let before = count(pool.clone(), fingerprint.clone()).await;
    assert!(!run(&pool, raw.clone(), false).await.meta.analytics_recorded);
    assert!(run(&pool, raw, true).await.meta.analytics_recorded);
    assert_eq!(count(pool.clone(), fingerprint.clone()).await, before + 1);
    let stored: serde_json::Value = sqlx::query_scalar(
        r#"SELECT query FROM stay_search_events WHERE fingerprint = $1 ORDER BY "createdAt" DESC LIMIT 1"#,
    )
    .bind(&fingerprint)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(stored["childAges"], json!([]));
    assert_eq!(stored["children"], json!(1));
}
