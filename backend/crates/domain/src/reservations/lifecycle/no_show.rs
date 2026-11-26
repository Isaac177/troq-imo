use chrono::Utc;
use kernel::Result;
use serde_json::Value;
use sqlx::{Postgres, Transaction};

use super::cancellation::{cancellation_terms, read_contract};
use crate::reservations::links::{billing, pricing};
use crate::reservations::support::{parse_minor, wire_minor};

pub async fn record_charge_and_release(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<()> {
    let (total, contract): (String, Value) =
        sqlx::query_as(r#"SELECT "totalAmount"::text, "contractSnapshot" FROM stay_reservations WHERE id = $1"#)
            .bind(reservation_id)
            .fetch_one(&mut **tx)
            .await?;
    let snapshot = read_contract(contract)?;
    let penalty_minor = match (cancellation_terms(&snapshot.policies), snapshot.cancellation_deadline) {
        (Ok(terms), Some(deadline)) => pricing::evaluate(terms, deadline, Utc::now(), parse_minor(&total)?).penalty_minor,
        _ => 0,
    };
    sqlx::query(r#"UPDATE stay_reservations SET "cancellationChargeAmount" = $2::text::numeric WHERE id = $1"#)
        .bind(reservation_id)
        .bind(wire_minor(penalty_minor))
        .execute(&mut **tx)
        .await?;
    billing::release_for_penalty(tx, reservation_id).await
}
