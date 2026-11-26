use kernel::{City, Error, Money, Result};
use sqlx::{FromRow, Postgres, Transaction};

use super::ledger::{self, EntryType, NewEntry};
use crate::billing::messages::{CREDIT_INSUFFICIENT, RESERVATION_NOT_FOUND};

const BOOKING_REASON: &str = "BOOKING";
const RELEASE_REASON: &str = "RELEASE";
const AMENDMENT_REASON: &str = "AMENDMENT";

#[derive(Debug, FromRow)]
struct ChargeRow {
    operator_id: String,
    city_id: String,
    rate_bps: i32,
    commission_minor: i64,
    charged_minor: i64,
    penalty_minor: i64,
}

async fn load(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<ChargeRow> {
    sqlx::query_as::<_, ChargeRow>(
        r#"SELECT "operatorId" AS operator_id, city_id, "commissionRateBps" AS rate_bps,
                  ("commissionAmount" * 100)::bigint AS commission_minor,
                  ("commissionChargedAmount" * 100)::bigint AS charged_minor,
                  ("cancellationChargeAmount" * 100)::bigint AS penalty_minor
           FROM stay_reservations WHERE id = $1"#,
    )
    .bind(reservation_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))
}

async fn set_charged(tx: &mut Transaction<'_, Postgres>, reservation_id: &str, charged_minor: i64) -> Result<()> {
    sqlx::query(r#"UPDATE stay_reservations SET "commissionChargedAmount" = $2::bigint::numeric / 100 WHERE id = $1"#)
        .bind(reservation_id)
        .bind(charged_minor)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub fn commission_kept(rate_bps: i32, penalty_minor: i64) -> i64 {
    Money::from_minor(penalty_minor).percent_of(i64::from(rate_bps)).minor()
}

pub async fn take_commission(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<()> {
    let row = load(tx, reservation_id).await?;
    if row.commission_minor <= 0 || ledger::entry_exists(tx, reservation_id, EntryType::Commission).await? {
        return Ok(());
    }
    ledger::apply(
        tx,
        NewEntry {
            city: City::from_code(&row.city_id)?,
            host_id: &row.operator_id,
            entry_type: EntryType::Commission,
            amount_minor: -row.commission_minor,
            reservation_id: Some(reservation_id),
            reason_code: Some(BOOKING_REASON),
            payment_ref: None,
            note: None,
            created_by: None,
        },
    )
    .await?;
    set_charged(tx, reservation_id, row.commission_minor).await
}

async fn release_row(
    tx: &mut Transaction<'_, Postgres>,
    reservation_id: &str,
    row: &ChargeRow,
    keep_minor: i64,
) -> Result<()> {
    if !ledger::entry_exists(tx, reservation_id, EntryType::Commission).await?
        || ledger::entry_exists(tx, reservation_id, EntryType::Refund).await?
    {
        return Ok(());
    }
    let keep = keep_minor.clamp(0, row.charged_minor);
    let refund = row.charged_minor - keep;
    if refund > 0 {
        ledger::apply(
            tx,
            NewEntry {
                city: City::from_code(&row.city_id)?,
                host_id: &row.operator_id,
                entry_type: EntryType::Refund,
                amount_minor: refund,
                reservation_id: Some(reservation_id),
                reason_code: Some(RELEASE_REASON),
                payment_ref: None,
                note: None,
                created_by: None,
            },
        )
        .await?;
    }
    set_charged(tx, reservation_id, keep).await
}

pub async fn release_commission(tx: &mut Transaction<'_, Postgres>, reservation_id: &str, keep_minor: i64) -> Result<()> {
    let row = load(tx, reservation_id).await?;
    release_row(tx, reservation_id, &row, keep_minor).await
}

pub async fn release_for_penalty(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<()> {
    let row = load(tx, reservation_id).await?;
    let keep = commission_kept(row.rate_bps, row.penalty_minor);
    release_row(tx, reservation_id, &row, keep).await
}

pub async fn sync_amended_commission(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<()> {
    let row = load(tx, reservation_id).await?;
    if !ledger::entry_exists(tx, reservation_id, EntryType::Commission).await? {
        return Ok(());
    }
    let delta = row.commission_minor - row.charged_minor;
    if delta == 0 {
        return Ok(());
    }
    if delta > 0 {
        let balance = sqlx::query_scalar::<_, i64>(
            r#"SELECT (balance * 100)::bigint FROM stay_credit_accounts WHERE "operatorId" = $1 FOR UPDATE"#,
        )
        .bind(&row.operator_id)
        .fetch_optional(&mut **tx)
        .await?
        .unwrap_or(0);
        if balance < delta {
            return Err(Error::conflict(CREDIT_INSUFFICIENT));
        }
    }
    let note = format!("Stay amendment {reservation_id}");
    ledger::apply(
        tx,
        NewEntry {
            city: City::from_code(&row.city_id)?,
            host_id: &row.operator_id,
            entry_type: EntryType::Adjustment,
            amount_minor: -delta,
            reservation_id: None,
            reason_code: Some(AMENDMENT_REASON),
            payment_ref: None,
            note: Some(&note),
            created_by: None,
        },
    )
    .await?;
    set_charged(tx, reservation_id, row.commission_minor).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commission_is_kept_only_on_the_penalty_share() {
        assert_eq!(commission_kept(1_000, 9_000_000), 900_000);
        assert_eq!(commission_kept(1_250, 0), 0);
        assert_eq!(commission_kept(0, 9_000_000), 0);
    }
}
