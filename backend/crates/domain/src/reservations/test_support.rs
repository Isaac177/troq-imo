use chrono::NaiveDate;
use kernel::ids::new_event_id;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub const PROPERTY: &str = "seed-prop-goma-lakeside";
pub const HOST: &str = "seed-host-goma-lake";

pub async fn test_pool() -> PgPool {
    let url = std::env::var("MUNGINI_TEST_DATABASE_URL")
        .expect("set MUNGINI_TEST_DATABASE_URL to a database with the Goma seeds loaded");
    PgPoolOptions::new()
        .max_connections(12)
        .connect(&url)
        .await
        .expect("connect to test database")
}

pub fn token() -> String {
    new_event_id().replace('-', "")
}

pub fn spread() -> i64 {
    (u128::from_str_radix(&token()[..16], 16).unwrap() % 10_000) as i64
}

pub fn fresh_night() -> NaiveDate {
    NaiveDate::from_ymd_opt(2040, 1, 1).unwrap() + chrono::Duration::days(spread() * 3)
}

pub async fn insert_reservation(pool: &PgPool, id: &str, state: &str, mode: &str, approval: &str) {
    let reference = format!("T-{}", &token()[..20]);
    let sql = format!(
        r#"INSERT INTO stay_reservations (id, reference, city_id, "propertyId", "operatorId", "guestUserId", state, "bookingMode",
               "totalAmount", "contractSnapshot", "approvalDeadlineAt")
           VALUES ($1, $2, 'GOMA', $3, $4, 'seed-test', $5, $6, 100, '{{}}', {approval})"#
    );
    sqlx::query(&sql)
        .bind(id)
        .bind(reference)
        .bind(PROPERTY)
        .bind(HOST)
        .bind(state)
        .bind(mode)
        .execute(pool)
        .await
        .unwrap();
}
