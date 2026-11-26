use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::{Error, Page, PageRequest, Result};
use serde_json::json;
use sqlx::PgPool;

use super::rate_types::{CurrentRate, RateInput, RateRow, RateSet};
use crate::billing::commission::rate::{configured_rate_bps, current_rate_bps};
use crate::billing::messages::RATE_UNCHANGED;
use crate::billing::validators::{check_note, clean, ensure_rate_reason, parse_rate_bps};
use crate::billing::{scope, settings};

pub async fn current(pool: &PgPool, actor: &Actor, host_id: &str) -> Result<CurrentRate> {
    scope::require_staff(actor)?;
    let row: Option<(i32, DateTime<Utc>)> = sqlx::query_as(
        r#"SELECT "rateBps", "effectiveFrom" FROM stay_operator_commission_rates
           WHERE "operatorId" = $1 AND "effectiveFrom" <= now()
           ORDER BY "effectiveFrom" DESC, "createdAt" DESC, id DESC LIMIT 1"#,
    )
    .bind(host_id)
    .fetch_optional(pool)
    .await?;
    let default_bps = settings::default_commission_bps(pool, actor.city).await?;
    Ok(CurrentRate {
        host_id: host_id.to_string(),
        rate_bps: row.as_ref().map_or(default_bps, |value| value.0),
        configured: row.is_some(),
        effective_from: row.map(|value| value.1),
    })
}

pub async fn history(pool: &PgPool, actor: &Actor, host_id: &str, page: PageRequest) -> Result<Page<RateRow>> {
    scope::require_staff(actor)?;
    let page = page.validated()?;
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_operator_commission_rates WHERE "operatorId" = $1 AND city_id = $2"#,
    )
    .bind(host_id)
    .bind(actor.city.code())
    .fetch_one(pool)
    .await?;
    let items = sqlx::query_as::<_, RateRow>(
        r#"SELECT id, "rateBps" AS rate_bps, "effectiveFrom" AS effective_from, "reasonCode" AS reason_code, note,
                  "changedByUserId" AS changed_by_user_id, "createdAt" AS created_at
           FROM stay_operator_commission_rates WHERE "operatorId" = $1 AND city_id = $2
           ORDER BY "effectiveFrom" DESC, "createdAt" DESC, id DESC LIMIT $3 OFFSET $4"#,
    )
    .bind(host_id)
    .bind(actor.city.code())
    .bind(page.limit())
    .bind(page.offset())
    .fetch_all(pool)
    .await?;
    Ok(Page::new(items, total, page))
}

pub async fn set(pool: &PgPool, actor: &Actor, host_id: &str, input: RateInput) -> Result<RateSet> {
    scope::require_staff(actor)?;
    ensure_rate_reason(&input.reason_code)?;
    let rate_bps = parse_rate_bps(&input.rate)?;
    let note = clean(&input.note);
    check_note(note)?;
    scope::ensure_host(pool, host_id, actor.city).await?;
    let previous = current_rate_bps(pool, actor.city, host_id).await?;
    if configured_rate_bps(pool, host_id).await? == Some(rate_bps) {
        return Err(Error::conflict(RATE_UNCHANGED));
    }
    let id = new_id("comrate");
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO stay_operator_commission_rates (id, city_id, "operatorId", "rateBps", "reasonCode", note, "changedByUserId")
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
    )
    .bind(&id)
    .bind(actor.city.code())
    .bind(host_id)
    .bind(rate_bps)
    .bind(&input.reason_code)
    .bind(note)
    .bind(actor.user_id().as_str())
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "commission.rate_set",
            resource_type: "operator",
            resource_id: host_id,
            reason: Some(input.reason_code.as_str()),
            change: json!({ "fromBps": previous, "toBps": rate_bps }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(RateSet { host_id: host_id.to_string(), rate_bps, id })
}
