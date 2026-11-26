use chrono::{Duration, Utc};
use kernel::city::City;
use kernel::ids::new_id;

use super::assignments::insert_assignment;
use super::nights::{insert_allocation, open_nights, remaining_by_night, take_night};
use super::test_support::{fresh_night, test_pool};

const VILLA_POOL: &str = "seed-pool-goma-villa";
const VILLA_TYPE: &str = "seed-unit-type-goma-villa";
const VILLA_UNIT: &str = "seed-physical-goma-villa";
const CONFIRMED_ITEM: &str = "seed-item-goma-confirmed";
const PROPERTY: &str = "seed-prop-goma-lakeside";

#[tokio::test]
#[ignore]
async fn only_one_of_many_concurrent_buyers_gets_the_last_night() {
    let pool = test_pool().await;
    let night = fresh_night();
    sqlx::query(
        r#"INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount")
           VALUES ($1, $2, 1, 0) ON CONFLICT ("poolId", "stayDate") DO UPDATE SET "allocatedCount" = 0"#,
    )
    .bind(VILLA_POOL)
    .bind(night)
    .execute(&pool)
    .await
    .unwrap();
    let reference = new_id("test-race");
    let mut buyers = Vec::new();
    for _ in 0..8 {
        let pool = pool.clone();
        let reference = reference.clone();
        buyers.push(tokio::spawn(async move {
            let mut tx = pool.begin().await.unwrap();
            match take_night(&mut tx, VILLA_TYPE, night).await.unwrap() {
                Some(pool_id) => {
                    insert_allocation(&mut tx, City::GOMA, &pool_id, "BLOCK", &reference, night, None)
                        .await
                        .unwrap();
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
    let count: i64 = sqlx::query_scalar(
        r#"SELECT "allocatedCount"::bigint FROM stay_night_inventory WHERE "poolId" = $1 AND "stayDate" = $2"#,
    )
    .bind(VILLA_POOL)
    .bind(night)
    .fetch_one(&pool)
    .await
    .unwrap();
    let active: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_inventory_allocations
           WHERE "poolId" = $1 AND "startDate" = $2 AND "referenceId" = $3 AND state = 'ACTIVE'"#,
    )
    .bind(VILLA_POOL)
    .bind(night)
    .bind(&reference)
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query(r#"DELETE FROM stay_inventory_allocations WHERE "referenceId" = $1"#)
        .bind(&reference)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(r#"UPDATE stay_night_inventory SET "allocatedCount" = 0 WHERE "poolId" = $1 AND "stayDate" = $2"#)
        .bind(VILLA_POOL)
        .bind(night)
        .execute(&pool)
        .await
        .unwrap();
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
    let start = chrono::NaiveDate::from_ymd_opt(2027, 4, 5).unwrap();
    let mut tx = pool.begin().await.unwrap();
    insert_assignment(&mut tx, City::GOMA, &first, CONFIRMED_ITEM, VILLA_UNIT, start, start + Duration::days(2), "ASSIGNED", "test")
        .await
        .unwrap();
    let overlapping = insert_assignment(
        &mut tx, City::GOMA, &second, CONFIRMED_ITEM, VILLA_UNIT,
        start + Duration::days(1), start + Duration::days(3), "MOVED", "test",
    )
    .await;
    tx.rollback().await.unwrap();
    let error = overlapping.unwrap_err();
    assert_eq!(error.code().as_str(), "UNIT_OVERLAP");
}

#[tokio::test]
#[ignore]
async fn default_inventory_and_physical_blocks_agree_across_read_and_allocation_paths() {
    let pool = test_pool().await;
    let suffix = new_id("t")[2..].to_string();
    let unit = format!("test-unit-{suffix}");
    let inventory = format!("test-pool-{suffix}");
    let physical = [format!("test-room-a-{suffix}"), format!("test-room-b-{suffix}")];
    let blocks = [format!("test-block-a-{suffix}"), format!("test-block-b-{suffix}")];
    let day = Utc::now().date_naive() + Duration::days(70);
    let end = day + Duration::days(1);
    sqlx::query(
        r#"INSERT INTO stay_unit_types (id, city_id, "propertyId", "inventoryModel", "nameEn", "nameFr", "maxAdults")
           VALUES ($1, 'GOMA', $2, 'POOLED_ROOM', 'Test room', 'Chambre test', 2)"#,
    )
    .bind(&unit)
    .bind(PROPERTY)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_inventory_pools (id, city_id, "propertyId", "unitTypeId", "sellableQuantity", "defaultNightlyCapacity")
           VALUES ($1, 'GOMA', $2, $3, 2, true)"#,
    )
    .bind(&inventory)
    .bind(PROPERTY)
    .bind(&unit)
    .execute(&pool)
    .await
    .unwrap();
    for (index, id) in physical.iter().enumerate() {
        sqlx::query(
            r#"INSERT INTO stay_physical_units (id, city_id, "propertyId", "unitTypeId", "localCode")
               VALUES ($1, 'GOMA', $2, $3, $4)"#,
        )
        .bind(id)
        .bind(PROPERTY)
        .bind(&unit)
        .bind(format!("{suffix}-{index}"))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(r#"INSERT INTO stay_pool_physical_units ("poolId", "physicalUnitId") VALUES ($1, $2)"#)
            .bind(&inventory)
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();
    }
    for index in 0..2 {
        sqlx::query(
            r#"INSERT INTO stay_availability_blocks
               (id, city_id, "propertyId", "physicalUnitId", "startDate", "endDate", reason, origin, "actorUserId")
               VALUES ($1, 'GOMA', $2, $3, $4, $5, 'MAINTENANCE', 'OPERATOR', 'seed-test')"#,
        )
        .bind(&blocks[index])
        .bind(PROPERTY)
        .bind(&physical[index])
        .bind(day)
        .bind(end)
        .execute(&pool)
        .await
        .unwrap();
        let remaining = remaining_by_night(&pool, &unit, day, end).await.unwrap();
        assert_eq!(remaining.get(&day), Some(&(1 - index as i64)));
        let open = open_nights(&pool, &unit, day, end).await.unwrap();
        assert_eq!(open.contains(&day), index == 0);
        let mut tx = pool.begin().await.unwrap();
        let taken = take_night(&mut tx, &unit, day).await.unwrap();
        assert_eq!(taken.is_some(), index == 0);
        tx.rollback().await.unwrap();
    }
    sqlx::query(r#"DELETE FROM stay_availability_blocks WHERE id = ANY($1)"#)
        .bind(&blocks[..])
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(r#"INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity") VALUES ($1, $2, 0)"#)
        .bind(&inventory)
        .bind(day)
        .execute(&pool)
        .await
        .unwrap();
    assert!(open_nights(&pool, &unit, day, end).await.unwrap().is_empty());
    for statement in [
        r#"DELETE FROM stay_night_inventory WHERE "poolId" = $1"#,
        r#"DELETE FROM stay_pool_physical_units WHERE "poolId" = $1"#,
    ] {
        sqlx::query(statement).bind(&inventory).execute(&pool).await.unwrap();
    }
    sqlx::query(r#"DELETE FROM stay_physical_units WHERE "unitTypeId" = $1"#).bind(&unit).execute(&pool).await.unwrap();
    sqlx::query(r#"DELETE FROM stay_inventory_pools WHERE id = $1"#).bind(&inventory).execute(&pool).await.unwrap();
    sqlx::query(r#"DELETE FROM stay_unit_types WHERE id = $1"#).bind(&unit).execute(&pool).await.unwrap();
}
