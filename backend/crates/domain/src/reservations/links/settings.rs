use kernel::{City, Result};
use sqlx::PgPool;

use crate::pricing::settings::{get_i64, SettingKey};

pub async fn approval_window_hours(pool: &PgPool, city: City) -> Result<i64> {
    get_i64(pool, city, SettingKey::ApprovalWindowHours).await
}

pub async fn amendment_window_hours(pool: &PgPool, city: City) -> Result<i64> {
    get_i64(pool, city, SettingKey::AmendmentWindowHours).await
}

pub async fn claim_window_days(pool: &PgPool, city: City) -> Result<i64> {
    get_i64(pool, city, SettingKey::ClaimWindowDays).await
}

pub async fn case_response_days(pool: &PgPool, city: City) -> Result<i64> {
    get_i64(pool, city, SettingKey::CaseResponseDays).await
}
