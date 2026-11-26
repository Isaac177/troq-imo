use chrono::NaiveDate;
use kernel::audit::AuditActor;
use kernel::ids::{new_event_id, new_id, UserId};
use kernel::City;
use serde_json::json;
use sqlx::PgPool;

use super::cancel_guest::cancel_stay_in;
use super::types::CancelStayInput;
use crate::reservations::test_support::{fresh_night, test_pool, HOST, PROPERTY};

const POOL: &str = "seed-pool-goma-room";
const GUEST: &str = "seed-guest-goma-1";

fn guest(user: &str) -> AuditActor {
    AuditActor { user_id: UserId::new(user), city: City::GOMA, permissions: Vec::new(), request_id: new_event_id() }
}

fn input(version: i32, key: Option<String>) -> CancelStayInput {
    CancelStayInput { expected_version: version, idempotency_key: key }
}

async fn confirmed_stay(pool: &PgPool, night: NaiveDate, deadline: &str, penalty_percent: i64) -> String {
    let reservation = new_id("cancel-res");
    let reference = format!("CNL-{}", &new_id("c")[2..10].to_uppercase());
    let contract = json!({
        "policies": [{
            "id": "seed-policy-cancel-flex-v1",
            "kind": "CANCELLATION",
            "content": { "freeUntilHours": 48, "penaltyPercent": penalty_percent }
        }],
        "cancellationDeadline": deadline,
    });
    sqlx::query(
        r#"INSERT INTO stay_inventory_allocations (id, city_id, "poolId", "referenceType", "referenceId", "startDate", "endDate", quantity, state)
           VALUES ($1, 'GOMA', $2, 'RESERVATION', $3, $4, $5, 1, 'ACTIVE')"#,
    )
    .bind(new_id("cancel-alloc"))
    .bind(POOL)
    .bind(&reservation)
    .bind(night)
    .bind(night + chrono::Duration::days(1))
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount") VALUES ($1, $2, 1, 1)
           ON CONFLICT ("poolId", "stayDate") DO UPDATE SET "sellableCapacity" = 1, "allocatedCount" = 1"#,
    )
    .bind(POOL)
    .bind(night)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, "bookingMode",
                                          "totalAmount", "contractSnapshot")
           VALUES ($1, $2, 'GOMA', $3, $4, $5, 'CONFIRMED', 'INSTANT', 300000.00, $6)"#,
    )
    .bind(&reservation)
    .bind(&reference)
    .bind(PROPERTY)
    .bind(HOST)
    .bind(GUEST)
    .bind(&contract)
    .execute(pool)
    .await
    .unwrap();
    reservation
}

async fn version_of(pool: &PgPool, reservation: &str) -> i32 {
    sqlx::query_scalar(r#"SELECT version FROM stay_reservations WHERE id = $1"#).bind(reservation).fetch_one(pool).await.unwrap()
}

async fn allocated_count(pool: &PgPool, night: NaiveDate) -> i32 {
    sqlx::query_scalar(r#"SELECT "allocatedCount" FROM stay_night_inventory WHERE "poolId" = $1 AND "stayDate" = $2"#)
        .bind(POOL)
        .bind(night)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[tokio::test]
#[ignore]
async fn guest_cancel_releases_nights_and_refuses_a_second_cancel() {
    let pool = test_pool().await;
    let night = fresh_night();
    let reservation = confirmed_stay(&pool, night, "2040-01-01T00:00:00Z", 0).await;
    let before = allocated_count(&pool, night).await;
    let version = version_of(&pool, &reservation).await;
    let view = cancel_stay_in(&pool, &guest(GUEST), &reservation, input(version, None)).await.unwrap();
    assert_eq!(view.state, "CANCELLED");
    assert_eq!(view.reason, "FREE_CANCELLATION");
    assert_eq!(view.charge_due_at_property, "0.00");
    assert_eq!(view.refund, "0.00");
    assert_eq!(allocated_count(&pool, night).await, before - 1);
    let released: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM stay_inventory_allocations WHERE "referenceId" = $1 AND state = 'RELEASED'"#)
        .bind(&reservation)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(released, 1);
    let second = cancel_stay_in(&pool, &guest(GUEST), &reservation, input(version + 1, None)).await;
    assert!(second.is_err());
}

#[tokio::test]
#[ignore]
async fn late_guest_cancel_keeps_the_penalty_due_at_the_property() {
    let pool = test_pool().await;
    let reservation = confirmed_stay(&pool, fresh_night(), "2000-01-01T00:00:00Z", 50).await;
    let version = version_of(&pool, &reservation).await;
    let view = cancel_stay_in(&pool, &guest(GUEST), &reservation, input(version, None)).await.unwrap();
    assert_eq!(view.reason, "PENALTY_APPLIES");
    assert_eq!(view.charge_due_at_property, "150000.00");
    assert_eq!(view.refund, "0.00");
}

#[tokio::test]
#[ignore]
async fn cancel_with_one_key_replays_and_other_guests_cannot_cancel() {
    let pool = test_pool().await;
    let reservation = confirmed_stay(&pool, fresh_night(), "2040-01-01T00:00:00Z", 0).await;
    let version = version_of(&pool, &reservation).await;
    let key = new_id("cancel-key");
    let other = cancel_stay_in(&pool, &guest("seed-guest-goma-2"), &reservation, input(version, None)).await;
    assert!(other.is_err());
    let first = cancel_stay_in(&pool, &guest(GUEST), &reservation, input(version, Some(key.clone()))).await.unwrap();
    let replayed = cancel_stay_in(&pool, &guest(GUEST), &reservation, input(version, Some(key))).await.unwrap();
    assert_eq!(first.cancelled_at, replayed.cancelled_at);
}
