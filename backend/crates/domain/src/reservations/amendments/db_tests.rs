use kernel::ids::new_id;

use super::queries::{close_stale_amendments, release_unit_assignment};
use crate::reservations::test_support::{insert_reservation, test_pool, PROPERTY};

#[tokio::test]
#[ignore]
async fn stale_proposals_close_without_replacing_a_current_proposal() {
    let pool = test_pool().await;
    let mut tx = pool.begin().await.unwrap();
    let reservation = new_id("test-amendment-res");
    sqlx::query(
        r#"INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, "bookingMode", "totalAmount", "contractSnapshot")
           VALUES ($1, $2, 'GOMA', $3, 'seed-host-goma-lake', 'seed-test', 'CONFIRMED', 'INSTANT', 100, '{}')"#,
    )
    .bind(&reservation)
    .bind(format!("T-{}", &new_id("t")[2..22]))
    .bind(PROPERTY)
    .execute(&mut *tx)
    .await
    .unwrap();
    for (label, original_version, expiry, expected_state) in [
        ("expired", 1, "now() - interval '1 hour'", "EXPIRED"),
        ("superseded", 2, "now() + interval '1 hour'", "SUPERSEDED"),
        ("current", 1, "now() + interval '1 hour'", "PROPOSED"),
    ] {
        let amendment = new_id(&format!("test-amendment-{label}"));
        let sql = format!(
            r#"INSERT INTO stay_amendments (id, city_id, "reservationId", "originalVersion", "proposedContract", "deltaAmount", state, "expiresAt")
               VALUES ($1, 'GOMA', $2, $3, '{{}}', 0, 'PROPOSED', {expiry})"#
        );
        sqlx::query(&sql).bind(&amendment).bind(&reservation).bind(original_version).execute(&mut *tx).await.unwrap();
        close_stale_amendments(&mut tx, &reservation, 1).await.unwrap();
        let state: String = sqlx::query_scalar(r#"SELECT state FROM stay_amendments WHERE id = $1"#)
            .bind(&amendment)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        assert_eq!(state, expected_state);
    }
    tx.rollback().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn changing_dates_releases_the_previous_room_assignment() {
    let pool = test_pool().await;
    let reservation = new_id("test-amendment-res");
    let item = new_id("test-amendment-item");
    let assignment = new_id("test-amendment-assignment");
    insert_reservation(&pool, &reservation, "CONFIRMED", "INSTANT", "NULL").await;
    sqlx::query(
        r#"INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", "physicalUnitId", quantity, "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
           VALUES ($1, $2, $3, 'seed-unit-type-goma-room', 'seed-rate-goma-room-request', 'seed-physical-goma-room-101', 1, 1, 0, '2200-01-01', '2200-01-02', '[]', 100)"#,
    )
    .bind(&item)
    .bind(&reservation)
    .bind(PROPERTY)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_unit_assignments (id, city_id, "reservationItemId", "physicalUnitId", "startDate", "endDate", reason, "actorUserId")
           VALUES ($1, 'GOMA', $2, 'seed-physical-goma-room-101', '2200-01-01', '2200-01-02', 'ASSIGNED', 'seed-test')"#,
    )
    .bind(&assignment)
    .bind(&item)
    .execute(&pool)
    .await
    .unwrap();
    let mut tx = pool.begin().await.unwrap();
    release_unit_assignment(&mut tx, &item).await.unwrap();
    tx.commit().await.unwrap();
    let room: Option<String> = sqlx::query_scalar(r#"SELECT "physicalUnitId" FROM stay_reservation_items WHERE id = $1"#)
        .bind(&item)
        .fetch_one(&pool)
        .await
        .unwrap();
    let released: bool = sqlx::query_scalar(r#"SELECT "releasedAt" IS NOT NULL FROM stay_unit_assignments WHERE id = $1"#)
        .bind(&assignment)
        .fetch_one(&pool)
        .await
        .unwrap();
    sqlx::query(r#"DELETE FROM stay_unit_assignments WHERE id = $1"#).bind(&assignment).execute(&pool).await.unwrap();
    sqlx::query(r#"DELETE FROM stay_reservation_items WHERE id = $1"#).bind(&item).execute(&pool).await.unwrap();
    sqlx::query(r#"DELETE FROM stay_reservations WHERE id = $1"#).bind(&reservation).execute(&pool).await.unwrap();
    assert!(room.is_none());
    assert!(released);
}
