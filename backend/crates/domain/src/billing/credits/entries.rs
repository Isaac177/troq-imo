use kernel::{PageRequest, Result};
use sqlx::PgPool;

use super::types::LedgerRecord;
use crate::billing::validators::ensure_entry_type;

pub async fn ledger_page(
    pool: &PgPool,
    host_id: &str,
    entry_type: Option<&str>,
    page: PageRequest,
) -> Result<(Vec<LedgerRecord>, i64)> {
    if let Some(entry_type) = entry_type {
        ensure_entry_type(entry_type)?;
    }
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_credit_entries e
           WHERE e."operatorId" = $1 AND ($2::text IS NULL OR e."entryType" = $2)"#,
    )
    .bind(host_id)
    .bind(entry_type)
    .fetch_one(pool)
    .await?;
    let items = sqlx::query_as::<_, LedgerRecord>(
        r#"SELECT e.id, e."entryType" AS entry_type, (e.amount * 100)::bigint AS amount_minor,
                  (e."balanceAfter" * 100)::bigint AS balance_after_minor,
                  e."reservationId" AS reservation_id, r.reference AS reservation_reference,
                  e."reasonCode" AS reason_code, e."paymentRef" AS payment_ref, e.note,
                  e."createdByUserId" AS created_by_user_id, e."createdAt" AS created_at
             FROM stay_credit_entries e
             LEFT JOIN stay_reservations r ON r.id = e."reservationId"
            WHERE e."operatorId" = $1 AND ($2::text IS NULL OR e."entryType" = $2)
            ORDER BY e."createdAt" DESC, e.id DESC LIMIT $3 OFFSET $4"#,
    )
    .bind(host_id)
    .bind(entry_type)
    .bind(page.limit())
    .bind(page.offset())
    .fetch_all(pool)
    .await?;
    Ok((items, total))
}
