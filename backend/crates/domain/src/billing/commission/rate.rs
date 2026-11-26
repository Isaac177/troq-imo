use kernel::{City, Money, Result};
use sqlx::PgPool;

use super::snapshot::CommissionSnapshot;
use crate::billing::settings;

pub async fn configured_rate_bps(pool: &PgPool, host_id: &str) -> Result<Option<i32>> {
    let rate: Option<i32> = sqlx::query_scalar(
        r#"SELECT "rateBps" FROM stay_operator_commission_rates
           WHERE "operatorId" = $1 AND "effectiveFrom" <= now()
           ORDER BY "effectiveFrom" DESC, "createdAt" DESC, id DESC LIMIT 1"#,
    )
    .bind(host_id)
    .fetch_optional(pool)
    .await?;
    Ok(rate)
}

pub async fn current_rate_bps(pool: &PgPool, city: City, host_id: &str) -> Result<i32> {
    match configured_rate_bps(pool, host_id).await? {
        Some(rate) => Ok(rate),
        None => settings::default_commission_bps(pool, city).await,
    }
}

pub async fn snapshot_for(pool: &PgPool, city: City, host_id: &str, total: Money) -> Result<CommissionSnapshot> {
    CommissionSnapshot::compute(current_rate_bps(pool, city, host_id).await?, total)
}
