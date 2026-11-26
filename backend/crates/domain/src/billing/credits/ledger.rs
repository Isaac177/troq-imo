use kernel::ids::new_id;
use kernel::{City, Error, Result};
use sqlx::{Postgres, Transaction};

use crate::billing::messages::CREDIT_INSUFFICIENT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Topup,
    Commission,
    Refund,
    Adjustment,
}

impl EntryType {
    pub fn code(self) -> &'static str {
        match self {
            Self::Topup => "TOPUP",
            Self::Commission => "COMMISSION",
            Self::Refund => "REFUND",
            Self::Adjustment => "ADJUSTMENT",
        }
    }
}

pub struct NewEntry<'a> {
    pub city: City,
    pub host_id: &'a str,
    pub entry_type: EntryType,
    pub amount_minor: i64,
    pub reservation_id: Option<&'a str>,
    pub reason_code: Option<&'a str>,
    pub payment_ref: Option<&'a str>,
    pub note: Option<&'a str>,
    pub created_by: Option<&'a str>,
}

pub fn next_balance(current_minor: i64, entry_type: EntryType, amount_minor: i64) -> Result<i64> {
    let next = current_minor
        .checked_add(amount_minor)
        .ok_or_else(|| Error::internal("credit balance overflow"))?;
    if entry_type == EntryType::Commission && next < 0 {
        return Err(Error::conflict(CREDIT_INSUFFICIENT));
    }
    Ok(next)
}

pub async fn entry_exists(
    tx: &mut Transaction<'_, Postgres>,
    reservation_id: &str,
    entry_type: EntryType,
) -> Result<bool> {
    let found: Option<String> =
        sqlx::query_scalar(r#"SELECT id FROM stay_credit_entries WHERE "reservationId" = $1 AND "entryType" = $2"#)
            .bind(reservation_id)
            .bind(entry_type.code())
            .fetch_optional(&mut **tx)
            .await?;
    Ok(found.is_some())
}

pub async fn apply(tx: &mut Transaction<'_, Postgres>, entry: NewEntry<'_>) -> Result<i64> {
    sqlx::query(r#"INSERT INTO stay_credit_accounts ("operatorId", city_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"#)
        .bind(entry.host_id)
        .bind(entry.city.code())
        .execute(&mut **tx)
        .await?;
    let current: i64 = sqlx::query_scalar(
        r#"SELECT (balance * 100)::bigint FROM stay_credit_accounts WHERE "operatorId" = $1 FOR UPDATE"#,
    )
    .bind(entry.host_id)
    .fetch_one(&mut **tx)
    .await?;
    let next = next_balance(current, entry.entry_type, entry.amount_minor)?;
    sqlx::query(
        r#"UPDATE stay_credit_accounts SET balance = $2::bigint::numeric / 100, version = version + 1, "updatedAt" = now()
           WHERE "operatorId" = $1"#,
    )
    .bind(entry.host_id)
    .bind(next)
    .execute(&mut **tx)
    .await?;
    sqlx::query(
        r#"INSERT INTO stay_credit_entries (id, city_id, "operatorId", "entryType", amount, "balanceAfter", "reservationId",
                                            "reasonCode", "paymentRef", note, "createdByUserId")
           VALUES ($1, $2, $3, $4, $5::bigint::numeric / 100, $6::bigint::numeric / 100, $7, $8, $9, $10, $11)"#,
    )
    .bind(new_id("credit"))
    .bind(entry.city.code())
    .bind(entry.host_id)
    .bind(entry.entry_type.code())
    .bind(entry.amount_minor)
    .bind(next)
    .bind(entry.reservation_id)
    .bind(entry.reason_code)
    .bind(entry.payment_ref)
    .bind(entry.note)
    .bind(entry.created_by)
    .execute(&mut **tx)
    .await?;
    Ok(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commission_may_leave_the_balance_at_exactly_zero() {
        assert_eq!(next_balance(50_000, EntryType::Commission, -50_000).unwrap(), 0);
    }

    #[test]
    fn commission_that_overdraws_is_rejected() {
        assert!(next_balance(49_999, EntryType::Commission, -50_000).is_err());
    }

    #[test]
    fn adjustments_may_go_negative_and_refunds_add_back() {
        assert_eq!(next_balance(1_000, EntryType::Adjustment, -5_000).unwrap(), -4_000);
        assert_eq!(next_balance(0, EntryType::Refund, 2_500).unwrap(), 2_500);
    }
}
