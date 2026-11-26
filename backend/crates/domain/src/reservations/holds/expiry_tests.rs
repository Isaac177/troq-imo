use chrono::{Duration, NaiveDate};
use kernel::ids::new_id;

use super::expiry::sweep;
use crate::reservations::lifecycle::decision_queries::move_request_if_version;
use crate::reservations::test_support::{insert_reservation, spread, test_pool, PROPERTY};

const POOL: &str = "seed-pool-goma-room";

#[tokio::test]
#[ignore]
async fn expires_every_night_of_a_request_larger_than_the_sweep_batch() {
    let pool = test_pool().await;
    let reservation = new_id("000-test-expiry");
    let item = new_id("test-expiry-item");
    let start = NaiveDate::from_ymd_opt(2100, 1, 1).unwrap() + Duration::days(spread());
    insert_reservation(&pool, &reservation, "REQUESTED", "REQUEST", "now() + interval '1 day'").await;
    sqlx::query(
        r#"INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", "adultCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
           VALUES ($1, $2, $3, 'seed-unit-type-goma-room', 'seed-rate-goma-room-request', 1, $4, $4::date + 120, '[]', 0)"#,
    )
    .bind(&item)
    .bind(&reservation)
    .bind(PROPERTY)
    .bind(start)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount")
           SELECT $1, $2::date + n, 1, 1 FROM generate_series(0, 119) n"#,
    )
    .bind(POOL)
    .bind(start)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_inventory_allocations (id, city_id, "poolId", "referenceType", "referenceId", "startDate", "endDate", quantity, state, "expiresAt")
           SELECT $1 || '-' || n::text, 'GOMA', $2, 'HOLD', $3, $4::date + n, $4::date + n + 1, 1, 'ACTIVE', now() - interval '1 minute'
           FROM generate_series(0, 119) n"#,
    )
    .bind(new_id("test-expiry-allocation"))
    .bind(POOL)
    .bind(&reservation)
    .bind(start)
    .execute(&pool)
    .await
    .unwrap();

    let mut decision = pool.begin().await.unwrap();
    let accepted = move_request_if_version(&mut decision, &reservation, 1, "CONFIRMED").await.unwrap();
    assert!(!accepted);
    decision.rollback().await.unwrap();

    sweep(&pool, 100).await.unwrap();
    let state: String = sqlx::query_scalar("SELECT state FROM stay_reservations WHERE id = $1")
        .bind(&reservation)
        .fetch_one(&pool)
        .await
        .unwrap();
    let active: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM stay_inventory_allocations WHERE "referenceId" = $1 AND state = 'ACTIVE'"#)
        .bind(&reservation)
        .fetch_one(&pool)
        .await
        .unwrap();
    let allocated: i64 = sqlx::query_scalar(
        r#"SELECT SUM("allocatedCount")::bigint FROM stay_night_inventory WHERE "poolId" = $1 AND "stayDate" >= $2 AND "stayDate" < $2::date + 120"#,
    )
    .bind(POOL)
    .bind(start)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(state, "EXPIRED");
    assert_eq!(active, 0);
    assert_eq!(allocated, 0);
    let version: i32 = sqlx::query_scalar("SELECT version FROM stay_reservations WHERE id = $1")
        .bind(&reservation)
        .fetch_one(&pool)
        .await
        .unwrap();

    sqlx::query(r#"UPDATE stay_inventory_allocations SET state = 'ACTIVE', "releasedAt" = NULL WHERE "referenceId" = $1 AND "startDate" = $2"#)
        .bind(&reservation)
        .bind(start)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(r#"UPDATE stay_night_inventory SET "allocatedCount" = 1 WHERE "poolId" = $1 AND "stayDate" = $2"#)
        .bind(POOL)
        .bind(start)
        .execute(&pool)
        .await
        .unwrap();
    sweep(&pool, 100).await.unwrap();
    let reconciled: (i32, i64, i32) = sqlx::query_as(
        r#"SELECT r.version,
                  (SELECT COUNT(*) FROM stay_inventory_allocations WHERE "referenceId" = r.id AND state = 'ACTIVE'),
                  (SELECT "allocatedCount" FROM stay_night_inventory WHERE "poolId" = $2 AND "stayDate" = $3)
           FROM stay_reservations r WHERE r.id = $1"#,
    )
    .bind(&reservation)
    .bind(POOL)
    .bind(start)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(reconciled, (version, 0, 0));

    sqlx::query(r#"DELETE FROM stay_inventory_allocations WHERE "referenceId" = $1"#).bind(&reservation).execute(&pool).await.unwrap();
    sqlx::query(r#"DELETE FROM stay_reservation_items WHERE id = $1"#).bind(&item).execute(&pool).await.unwrap();
    sqlx::query(r#"DELETE FROM stay_reservations WHERE id = $1"#).bind(&reservation).execute(&pool).await.unwrap();
    sqlx::query(r#"DELETE FROM stay_night_inventory WHERE "poolId" = $1 AND "stayDate" >= $2 AND "stayDate" < $2::date + 120"#)
        .bind(POOL)
        .bind(start)
        .execute(&pool)
        .await
        .unwrap();
}
