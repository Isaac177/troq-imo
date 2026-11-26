use std::time::Duration;

use chrono::NaiveDate;
use kernel::audit::AuditActor;
use kernel::city::City;
use kernel::ids::{new_id, UserId};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use super::scope::PropertyRef;

pub const GOMA_PROPERTY: &str = "seed-prop-goma-lakeside";
pub const GOMA_HOST: &str = "seed-host-goma-lake";
pub const STAFF_USER: &str = "seed-user-goma-host";

pub async fn test_pool() -> PgPool {
    let url = std::env::var("MUNGINI_TEST_DATABASE_URL")
        .expect("set MUNGINI_TEST_DATABASE_URL to a database with the Goma seed loaded");
    PgPoolOptions::new()
        .max_connections(12)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&url)
        .await
        .expect("connect to test database")
}

pub fn fresh_night() -> NaiveDate {
    let spread = i64::from_str_radix(&new_id("n")[2..10], 16).unwrap() % 30_000;
    NaiveDate::from_ymd_opt(2040, 1, 1).unwrap() + chrono::Duration::days(spread)
}

pub fn goma_property() -> PropertyRef {
    PropertyRef {
        id: GOMA_PROPERTY.to_string(),
        host_id: GOMA_HOST.to_string(),
        city: City::GOMA,
        timezone: "Africa/Lubumbashi".to_string(),
    }
}

pub fn staff_actor() -> AuditActor {
    AuditActor {
        user_id: UserId::new(STAFF_USER),
        city: City::GOMA,
        permissions: vec!["inventory.write".to_string()],
        request_id: new_id("req"),
    }
}
