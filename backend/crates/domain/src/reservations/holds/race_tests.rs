use chrono::NaiveDate;
use kernel::audit::AuditActor;
use kernel::ids::{new_event_id, new_id, UserId};
use kernel::City;
use sqlx::PgPool;

use super::expiry::sweep;
use crate::reservations::test_support::{fresh_night, test_pool, HOST, PROPERTY};
use crate::reservations::lifecycle::decision::apply_decision;
use crate::reservations::lifecycle::decision_queries::find_for_decision;

const ROOM_POOL: &str = "seed-pool-goma-room";
const STAFF: &str = "seed-user-goma-host";

fn actor() -> AuditActor {
    AuditActor {
        user_id: UserId::new(STAFF),
        city: City::GOMA,
        permissions: vec!["reservations.write".into()],
        request_id: new_event_id(),
    }
}

async fn expired_request_fixture(pool: &PgPool, night: NaiveDate) -> String {
    let reservation = new_id("race-res");
    let reference = format!("RACE-{}", &new_id("r")[2..10].to_uppercase());
    sqlx::query(
        r#"INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount") VALUES ($1, $2, 1, 1)
           ON CONFLICT ("poolId", "stayDate") DO UPDATE SET "sellableCapacity" = 1, "allocatedCount" = 1"#,
    )
    .bind(ROOM_POOL)
    .bind(night)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, "bookingMode",
                                          "totalAmount", "contractSnapshot", "approvalDeadlineAt")
           VALUES ($1, $2, 'GOMA', $3, $4, 'seed-guest-goma-1', 'REQUESTED', 'REQUEST', 0, '{}', now() + interval '1 day')"#,
    )
    .bind(&reservation)
    .bind(&reference)
    .bind(PROPERTY)
    .bind(HOST)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", "adultCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
           VALUES ($1, $2, $3, 'seed-unit-type-goma-room', 'seed-rate-goma-room-request', 1, $4, $5, '[]', 0)"#,
    )
    .bind(new_id("race-item"))
    .bind(&reservation)
    .bind(PROPERTY)
    .bind(night)
    .bind(night + chrono::Duration::days(1))
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_inventory_allocations (id, city_id, "poolId", "referenceType", "referenceId", "startDate", "endDate", quantity, state, "expiresAt")
           VALUES ($1, 'GOMA', $2, 'HOLD', $3, $4, $5, 1, 'ACTIVE', now() - interval '1 minute')"#,
    )
    .bind(new_id("race-alloc"))
    .bind(ROOM_POOL)
    .bind(&reservation)
    .bind(night)
    .bind(night + chrono::Duration::days(1))
    .execute(pool)
    .await
    .unwrap();
    reservation
}

async fn night_is_consistent(pool: &PgPool, night: NaiveDate) -> bool {
    let (count, active): (i32, i64) = sqlx::query_as(
        r#"SELECT n."allocatedCount", (SELECT COUNT(*) FROM stay_inventory_allocations a
               WHERE a."poolId" = n."poolId" AND a."startDate" = n."stayDate" AND a.state = 'ACTIVE')
           FROM stay_night_inventory n WHERE n."poolId" = $1 AND n."stayDate" = $2"#,
    )
    .bind(ROOM_POOL)
    .bind(night)
    .fetch_one(pool)
    .await
    .unwrap();
    i64::from(count) == active
}

#[tokio::test]
#[ignore]
async fn hold_expiry_and_acceptance_race_leaves_one_consistent_outcome() {
    let pool = test_pool().await;
    for round in 0..12 {
        let night = fresh_night() + chrono::Duration::days(round);
        let reservation = expired_request_fixture(&pool, night).await;
        let row = find_for_decision(&pool, City::GOMA, &reservation).await.unwrap().unwrap();
        let sweeper = {
            let pool = pool.clone();
            tokio::spawn(async move { sweep(&pool, 100).await })
        };
        let accepter = {
            let pool = pool.clone();
            let reservation = reservation.clone();
            tokio::spawn(async move { apply_decision(&pool, &actor(), &reservation, "ACCEPT", 1, row).await.is_ok() })
        };
        let _ = sweeper.await.unwrap();
        let accepted = accepter.await.unwrap();

        let state: String = sqlx::query_scalar(r#"SELECT state FROM stay_reservations WHERE id = $1"#)
            .bind(&reservation)
            .fetch_one(&pool)
            .await
            .unwrap();
        let active_for_reservation: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM stay_inventory_allocations WHERE "referenceId" = $1 AND state = 'ACTIVE'"#,
        )
        .bind(&reservation)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert!(state == "CONFIRMED" || state == "EXPIRED", "round {round}: unexpected state {state} (accepted={accepted})");
        if state == "CONFIRMED" {
            assert_eq!(active_for_reservation, 1, "round {round}: confirmed stay must hold its night");
        } else {
            assert_eq!(active_for_reservation, 0, "round {round}: expired stay must hold nothing");
        }
        assert!(night_is_consistent(&pool, night).await, "round {round}: night count must match active allocations");
    }
}
