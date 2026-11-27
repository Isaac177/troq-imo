use chrono::{Duration, NaiveDate};
use kernel::audit::AuditActor;
use kernel::ids::{new_event_id, new_id, UserId};
use kernel::{City, PageRequest};
use serde_json::json;
use sqlx::PgPool;

use super::mine::{list, tab_predicate};
use super::mine_counts::counts;
use super::special_request::apply_special_request;
use super::types::SpecialRequestDecisionInput;
use crate::reservations::amendments::list as amendment_list;
use crate::reservations::test_support::{spread, test_pool, HOST, PROPERTY};

const UNIT_TYPE: &str = "seed-unit-type-goma-villa";
const RATE_PLAN: &str = "seed-rate-goma-villa-instant";

fn future_stay() -> (NaiveDate, NaiveDate) {
    let check_in = NaiveDate::from_ymd_opt(2040, 1, 1).unwrap() + Duration::days(spread() * 3);
    (check_in, check_in + Duration::days(2))
}

fn past_stay() -> (NaiveDate, NaiveDate) {
    let check_in = NaiveDate::from_ymd_opt(2020, 1, 10).unwrap();
    (check_in, check_in + Duration::days(2))
}

fn page() -> PageRequest {
    PageRequest { page: 1, page_size: 50 }
}

fn staff() -> AuditActor {
    AuditActor {
        user_id: UserId::new("seed-user-goma-host"),
        city: City::GOMA,
        permissions: vec!["reservations.write".into()],
        request_id: new_event_id(),
    }
}

async fn stay(pool: &PgPool, guest: &str, state: &str, dates: (NaiveDate, NaiveDate), special: Option<&str>) -> String {
    let reservation = new_id("guest-res");
    let item = new_id("guest-item");
    let special_state = if special.is_some() { "REQUESTED" } else { "NONE" };
    sqlx::query(
        r#"INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, "bookingMode",
                                          "totalAmount", "contractSnapshot", "specialRequest", "specialRequestState")
           VALUES ($1, $2, 'GOMA', $3, $4, $5, $6, 'INSTANT', 200.00, '{}', $7, $8)"#,
    )
    .bind(&reservation)
    .bind(format!("GST-{}", &new_id("g")[2..10].to_uppercase()))
    .bind(PROPERTY)
    .bind(HOST)
    .bind(guest)
    .bind(state)
    .bind(special)
    .bind(special_state)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_reservation_items (id, "reservationId", "propertyId", "unitTypeId", "ratePlanId", quantity,
                                               "adultCount", "childCount", "checkIn", "checkOut", "nightlyCharges", "subtotalAmount")
           VALUES ($1, $2, $3, $4, $5, 1, 2, 0, $6, $7, '[]', 200.00)"#,
    )
    .bind(&item)
    .bind(&reservation)
    .bind(PROPERTY)
    .bind(UNIT_TYPE)
    .bind(RATE_PLAN)
    .bind(dates.0)
    .bind(dates.1)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"INSERT INTO stay_reservation_guests (id, "reservationItemId", "fullName", "contactEmail", role, "consentBasis", "retainUntil")
           VALUES ($1, $2, 'Test Guest', 'guest.fixture@example.test', 'PRIMARY', 'CONTRACT', '2050-01-01')"#,
    )
    .bind(new_id("guest-row"))
    .bind(&item)
    .execute(pool)
    .await
    .unwrap();
    reservation
}

#[tokio::test]
#[ignore]
async fn my_stays_tabs_split_upcoming_pending_and_past() {
    let pool = test_pool().await;
    let guest = new_id("guest-tabs");
    stay(&pool, &guest, "CONFIRMED", future_stay(), None).await;
    stay(&pool, &guest, "REQUESTED", future_stay(), None).await;
    stay(&pool, &guest, "CHECKED_OUT", past_stay(), None).await;
    let count = |tab: Option<&'static str>| {
        let pool = pool.clone();
        let guest = guest.clone();
        async move { list(&pool, City::GOMA, &guest, page(), tab_predicate(tab).unwrap()).await.unwrap().1 }
    };
    assert_eq!(count(None).await, 3);
    assert_eq!(count(Some("upcoming")).await, 1);
    assert_eq!(count(Some("pending")).await, 1);
    assert_eq!(count(Some("past")).await, 1);
    assert_eq!(count(Some("cancelled")).await, 0);
    let summary = counts(&pool, City::GOMA, &guest).await.unwrap();
    assert_eq!((summary.all, summary.pending, summary.upcoming, summary.active, summary.past, summary.cancelled), (3, 1, 1, 0, 1, 0));
    assert!(tab_predicate(Some("nonsense")).is_err());
}

#[tokio::test]
#[ignore]
async fn guest_sees_the_pending_amendment_with_its_contract() {
    let pool = test_pool().await;
    let guest = new_id("guest-amend");
    let reservation = stay(&pool, &guest, "CONFIRMED", future_stay(), None).await;
    let proposed = json!({
        "checkIn": "2041-01-02",
        "checkOut": "2041-01-04",
        "total": "200.00",
        "nights": [{ "date": "2041-01-02", "amount": "100.00" }, { "date": "2041-01-03", "amount": "100.00" }]
    });
    sqlx::query(
        r#"INSERT INTO stay_amendments (id, city_id, "reservationId", "originalVersion", "proposedContract", "deltaAmount", state, "expiresAt")
           VALUES ($1, 'GOMA', $2, 1, $3, 0.00, 'PROPOSED', now() + interval '1 day')"#,
    )
    .bind(new_id("guest-amend"))
    .bind(&reservation)
    .bind(&proposed)
    .execute(&pool)
    .await
    .unwrap();
    let (items, total) = amendment_list::list(&pool, &reservation, page()).await.unwrap();
    assert_eq!(total, 1);
    assert_eq!(items[0].state, "PROPOSED");
    assert_eq!(items[0].check_in, "2041-01-02");
    assert_eq!(items[0].nights.len(), 2);
}

#[tokio::test]
#[ignore]
async fn operator_accepts_a_special_request_once() {
    let pool = test_pool().await;
    let guest = new_id("guest-special");
    let reservation = stay(&pool, &guest, "CONFIRMED", future_stay(), Some("late arrival after 22:00")).await;
    let input = |decision: &str| SpecialRequestDecisionInput { decision: decision.into(), expected_version: 1 };
    apply_special_request(&pool, &staff(), &reservation, &input("ACCEPT")).await.unwrap();
    let (state, version): (String, i32) =
        sqlx::query_as(r#"SELECT "specialRequestState", version FROM stay_reservations WHERE id = $1"#)
            .bind(&reservation)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(state, "ACCEPTED");
    assert_eq!(version, 2);
    let stale = apply_special_request(&pool, &staff(), &reservation, &input("DECLINE")).await;
    assert!(stale.is_err());
}
