use kernel::City;
use sqlx::PgPool;

use super::access::ensure_host_city;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

#[sqlx::test(migrator = "MIGRATOR")]
async fn template_host_must_belong_to_requested_city(pool: PgPool) {
    sqlx::query("INSERT INTO hosts (id, city_id, display_name, kind) VALUES ('seed-host-cross-city', 'KIN', 'River Rooms', 'BUSINESS')")
        .execute(&pool)
        .await
        .unwrap();
    let mut tx = pool.begin().await.unwrap();
    assert!(ensure_host_city(&mut tx, "seed-host-cross-city", City::KIN).await.is_ok());
    let error = ensure_host_city(&mut tx, "seed-host-cross-city", City::GOMA).await.unwrap_err();
    assert_eq!(error.code().as_str(), "HOST_NOT_FOUND");
    let missing = ensure_host_city(&mut tx, "seed-host-missing", City::KIN).await.unwrap_err();
    assert_eq!(missing.code().as_str(), "HOST_NOT_FOUND");
    tx.rollback().await.unwrap();
}
