use chrono::{Duration, NaiveDate};
use kernel::ids::new_id;
use kernel::{City, Error};
use sqlx::{Postgres, Transaction};

use super::rooms::reserve_room;
use crate::reservations::test_support::{insert_reservation, spread, test_pool, PROPERTY};

async fn add_item(tx: &mut Transaction<'_, Postgres>, unit_type: &str, check_in: NaiveDate, check_out: NaiveDate) -> (String, String) {
    let reservation = new_id("test-room-res");
    let item = new_id("test-room-item");
    sqlx::query(
        r#"INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, "bookingMode", "totalAmount", "contractSnapshot")
           VALUES ($1, $2, 'GOMA', $3, 'seed-host-goma-lake', 'seed-test', 'CONFIRMED', 'INSTANT', 100, '{}')"#,
    )
    .bind(&reservation)
    .bind(format!("T-{}", &new_id("t")[2..22]))
    .bind(PROPERTY)
    .execute(&mut **tx)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
           VALUES ($1, $2, $3, $4, 'seed-rate-goma-room-request', 1, 1, 0, $5, $6, '[]', 100)"#,
    )
    .bind(&item)
    .bind(&reservation)
    .bind(PROPERTY)
    .bind(unit_type)
    .bind(check_in)
    .bind(check_out)
    .execute(&mut **tx)
    .await
    .unwrap();
    (reservation, item)
}

#[tokio::test]
#[ignore]
async fn reserve_room_packs_stays_and_rejects_when_no_single_room_is_free() {
    let pool = test_pool().await;
    let mut tx = pool.begin().await.unwrap();
    let suffix = new_id("t");
    let unit_type = format!("test-unit-{suffix}");
    let inventory = format!("test-pool-{suffix}");
    sqlx::query(r#"INSERT INTO stay_unit_types (id, city_id, "propertyId", "inventoryModel", "nameEn", "nameFr", "maxAdults") VALUES ($1, 'GOMA', $2, 'POOLED_ROOM', 'Test', 'Test', 2)"#)
        .bind(&unit_type)
        .bind(PROPERTY)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(r#"INSERT INTO stay_inventory_pools (id, city_id, "propertyId", "unitTypeId", "sellableQuantity") VALUES ($1, 'GOMA', $2, $3, 2)"#)
        .bind(&inventory)
        .bind(PROPERTY)
        .bind(&unit_type)
        .execute(&mut *tx)
        .await
        .unwrap();
    for code in ["a", "b"] {
        let unit = format!("test-room-{code}-{suffix}");
        sqlx::query(r#"INSERT INTO stay_physical_units (id, city_id, "propertyId", "unitTypeId", "localCode") VALUES ($1, 'GOMA', $2, $3, $4)"#)
            .bind(&unit)
            .bind(PROPERTY)
            .bind(&unit_type)
            .bind(format!("{suffix}-{code}"))
            .execute(&mut *tx)
            .await
            .unwrap();
        sqlx::query(r#"INSERT INTO stay_pool_physical_units ("poolId", "physicalUnitId") VALUES ($1, $2)"#)
            .bind(&inventory)
            .bind(&unit)
            .execute(&mut *tx)
            .await
            .unwrap();
    }
    let day = NaiveDate::from_ymd_opt(2200, 1, 1).unwrap() + Duration::days(spread());
    let early_end = day + Duration::days(2);
    let late_end = day + Duration::days(4);

    let (first_res, first_item) = add_item(&mut tx, &unit_type, day, early_end).await;
    reserve_room(&mut tx, City::GOMA, &first_item, &unit_type, &first_res, day, early_end, "system").await.unwrap();
    let (second_res, second_item) = add_item(&mut tx, &unit_type, day + Duration::days(1), late_end).await;
    reserve_room(&mut tx, City::GOMA, &second_item, &unit_type, &second_res, day + Duration::days(1), late_end, "system").await.unwrap();
    let rooms: Vec<String> = sqlx::query_scalar(r#"SELECT "physicalUnitId" FROM stay_unit_assignments WHERE "reservationItemId" = ANY($1) ORDER BY "reservationItemId""#)
        .bind(vec![first_item.clone(), second_item.clone()])
        .fetch_all(&mut *tx)
        .await
        .unwrap();
    assert_eq!(rooms.len(), 2);
    assert_ne!(rooms[0], rooms[1]);

    let available: bool = sqlx::query_scalar("SELECT stay_room_available($1, $2, $3, '')")
        .bind(&unit_type)
        .bind(day + Duration::days(1))
        .bind(early_end)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert!(!available);
    let (third_res, third_item) = add_item(&mut tx, &unit_type, day + Duration::days(1), early_end).await;
    let rejected = reserve_room(&mut tx, City::GOMA, &third_item, &unit_type, &third_res, day + Duration::days(1), early_end, "system").await;
    assert!(matches!(rejected, Err(Error::Conflict(_))));
    tx.rollback().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn a_request_hold_is_found_by_the_decision_lookup() {
    let pool = test_pool().await;
    let id = new_id("test-lookup");
    insert_reservation(&pool, &id, "REQUESTED", "REQUEST", "now() + interval '1 day'").await;
    let row = super::decision_queries::find_for_decision(&pool, City::GOMA, &id).await.unwrap().unwrap();
    let missing = super::decision_queries::find_for_decision(&pool, City::KIN, &id).await.unwrap();
    sqlx::query("DELETE FROM stay_reservations WHERE id = $1").bind(&id).execute(&pool).await.unwrap();
    assert_eq!(row.state, "REQUESTED");
    assert!(missing.is_none());
}

#[tokio::test]
#[ignore]
async fn check_in_selects_only_rooms_in_the_inventory_pool() {
    let pool = test_pool().await;
    let mut tx = pool.begin().await.unwrap();
    let suffix = new_id("t");
    let unit_type = format!("test-unit-{suffix}");
    let inventory = format!("test-pool-{suffix}");
    let outside = format!("test-outside-{suffix}");
    let inside = format!("test-inside-{suffix}");
    sqlx::query(r#"INSERT INTO stay_unit_types (id, city_id, "propertyId", "inventoryModel", "nameEn", "nameFr", "maxAdults") VALUES ($1, 'GOMA', $2, 'POOLED_ROOM', 'Test', 'Test', 2)"#)
        .bind(&unit_type)
        .bind(PROPERTY)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query(r#"INSERT INTO stay_inventory_pools (id, city_id, "propertyId", "unitTypeId", "sellableQuantity") VALUES ($1, 'GOMA', $2, $3, 1)"#)
        .bind(&inventory)
        .bind(PROPERTY)
        .bind(&unit_type)
        .execute(&mut *tx)
        .await
        .unwrap();
    for (id, code) in [(&outside, format!("{suffix}-a")), (&inside, format!("{suffix}-b"))] {
        sqlx::query(r#"INSERT INTO stay_physical_units (id, city_id, "propertyId", "unitTypeId", "localCode") VALUES ($1, 'GOMA', $2, $3, $4)"#)
            .bind(id)
            .bind(PROPERTY)
            .bind(&unit_type)
            .bind(code)
            .execute(&mut *tx)
            .await
            .unwrap();
    }
    sqlx::query(r#"INSERT INTO stay_pool_physical_units ("poolId", "physicalUnitId") VALUES ($1, $2)"#)
        .bind(&inventory)
        .bind(&inside)
        .execute(&mut *tx)
        .await
        .unwrap();
    let start = chrono::Utc::now().date_naive() + Duration::days(70);
    let selected = super::rooms::lock_free_unit(&mut tx, &unit_type, start, start + Duration::days(1), "test-reservation").await.unwrap();
    assert_eq!(selected.as_deref(), Some(inside.as_str()));
    tx.rollback().await.unwrap();
}
