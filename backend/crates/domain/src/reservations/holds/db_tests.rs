use chrono::NaiveDate;
use kernel::ids::new_id;
use kernel::City;
use sqlx::PgPool;

use crate::reservations::test_support::{fresh_night, test_pool};
use crate::reservations::links::inventory::{insert_allocation, take_night};

const VILLA_POOL: &str = "seed-pool-goma-villa";
const VILLA_UNIT_TYPE: &str = "seed-unit-type-goma-villa";
const TEST_ITEM: &str = "seed-item-goma-confirmed";
const TEST_UNIT: &str = "seed-physical-goma-villa";

async fn reset_night(pool: &PgPool, night: NaiveDate) {
    sqlx::query(
        r#"INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount") VALUES ($1, $2, 1, 0)
           ON CONFLICT ("poolId", "stayDate") DO UPDATE SET "allocatedCount" = 0, "sellableCapacity" = 1"#,
    )
    .bind(VILLA_POOL)
    .bind(night)
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
#[ignore]
async fn only_one_of_many_concurrent_buyers_gets_the_last_night() {
    let pool = test_pool().await;
    let night = fresh_night();
    reset_night(&pool, night).await;
    let prefix = new_id("test-race");

    let mut buyers = Vec::new();
    for index in 0..8 {
        let pool = pool.clone();
        let reference = format!("{prefix}-{index}");
        buyers.push(tokio::spawn(async move {
            let mut tx = pool.begin().await.unwrap();
            match take_night(&mut tx, VILLA_UNIT_TYPE, night).await.unwrap() {
                Some(pool_id) => {
                    insert_allocation(&mut tx, City::GOMA, &pool_id, "BLOCK", &reference, night, None).await.unwrap();
                    tx.commit().await.unwrap();
                    true
                }
                None => {
                    tx.rollback().await.unwrap();
                    false
                }
            }
        }));
    }
    let mut winners = 0;
    for buyer in buyers {
        if buyer.await.unwrap() {
            winners += 1;
        }
    }
    let count: i64 = sqlx::query_scalar(r#"SELECT "allocatedCount"::bigint FROM stay_night_inventory WHERE "poolId" = $1 AND "stayDate" = $2"#)
        .bind(VILLA_POOL)
        .bind(night)
        .fetch_one(&pool)
        .await
        .unwrap();
    let active: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_inventory_allocations WHERE "poolId" = $1 AND "startDate" = $2 AND "referenceId" LIKE $3 AND state = 'ACTIVE'"#,
    )
    .bind(VILLA_POOL)
    .bind(night)
    .bind(format!("{prefix}-%"))
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query(r#"DELETE FROM stay_inventory_allocations WHERE "referenceId" LIKE $1"#)
        .bind(format!("{prefix}-%"))
        .execute(&pool)
        .await
        .unwrap();
    reset_night(&pool, night).await;

    assert_eq!(winners, 1);
    assert_eq!(count, 1);
    assert_eq!(active, 1);
}

#[tokio::test]
#[ignore]
async fn a_unit_cannot_hold_two_overlapping_assignments() {
    let pool = test_pool().await;
    let first = new_id("test-assign");
    let second = new_id("test-assign");
    let start = NaiveDate::from_ymd_opt(2027, 4, 5).unwrap();
    let insert = r#"INSERT INTO stay_unit_assignments (id, city_id, "reservationItemId", "physicalUnitId", "startDate", "endDate", reason, "actorUserId")
                    VALUES ($1, 'GOMA', $2, $3, $4, $5, $6, 'test')"#;

    sqlx::query(insert)
        .bind(&first)
        .bind(TEST_ITEM)
        .bind(TEST_UNIT)
        .bind(start)
        .bind(start + chrono::Duration::days(2))
        .bind("ASSIGNED")
        .execute(&pool)
        .await
        .unwrap();
    let overlapping = sqlx::query(insert)
        .bind(&second)
        .bind(TEST_ITEM)
        .bind(TEST_UNIT)
        .bind(start + chrono::Duration::days(1))
        .bind(start + chrono::Duration::days(3))
        .bind("MOVED")
        .execute(&pool)
        .await;
    let code = match &overlapping {
        Err(sqlx::Error::Database(database)) => database.code().map(|code| code.to_string()),
        _ => None,
    };
    sqlx::query("DELETE FROM stay_unit_assignments WHERE id = $1 OR id = $2")
        .bind(&first)
        .bind(&second)
        .execute(&pool)
        .await
        .unwrap();

    assert!(overlapping.is_err());
    assert_eq!(code.as_deref(), Some("23P01"));
}
