use kernel::error::{Error, Result};
use sqlx::{Postgres, Transaction};

use crate::inventory::messages::INVENTORY_INCONSISTENT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseState {
    Released,
    Expired,
}

impl ReleaseState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Released => "RELEASED",
            Self::Expired => "EXPIRED",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReleasedInventory {
    pub allocations: i64,
    pub nights: i64,
}

pub async fn release(
    tx: &mut Transaction<'_, Postgres>,
    reference_type: &str,
    reference_id: &str,
    state: ReleaseState,
) -> Result<ReleasedInventory> {
    let (allocations, nights, updated): (i64, i64, i64) = sqlx::query_as(
        r#"WITH released AS (
               UPDATE stay_inventory_allocations
               SET state = $3, "releasedAt" = now()
               WHERE "referenceType" = $1 AND "referenceId" = $2 AND state = 'ACTIVE'
               RETURNING "poolId", "startDate", "endDate", quantity
           ), totals AS (
               SELECT r."poolId", r."startDate" + d.day_offset AS stay_date,
                      SUM(r.quantity)::integer AS quantity
               FROM released r
               CROSS JOIN LATERAL generate_series(0, r."endDate" - r."startDate" - 1) AS d(day_offset)
               GROUP BY r."poolId", r."startDate" + d.day_offset
           ), decremented AS (
               UPDATE stay_night_inventory n
               SET "allocatedCount" = n."allocatedCount" - totals.quantity, version = n.version + 1
               FROM totals
               WHERE n."poolId" = totals."poolId" AND n."stayDate" = totals.stay_date
                 AND n."allocatedCount" >= totals.quantity
               RETURNING n."poolId"
           )
           SELECT (SELECT COUNT(*) FROM released), (SELECT COUNT(*) FROM totals),
                  (SELECT COUNT(*) FROM decremented)"#,
    )
    .bind(reference_type)
    .bind(reference_id)
    .bind(state.as_str())
    .fetch_one(&mut **tx)
    .await?;
    if nights != updated {
        return Err(Error::internal(INVENTORY_INCONSISTENT.as_str()));
    }
    if reference_type == "HOLD" || reference_type == "RESERVATION" {
        sqlx::query(
            r#"UPDATE stay_unit_assignments SET "releasedAt" = now()
               WHERE "releasedAt" IS NULL
                 AND "reservationItemId" IN (SELECT id FROM stay_reservation_items WHERE "reservationId" = $1)"#,
        )
        .bind(reference_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(ReleasedInventory { allocations, nights })
}
