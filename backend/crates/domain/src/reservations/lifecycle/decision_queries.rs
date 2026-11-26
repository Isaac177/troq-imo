use chrono::{DateTime, Utc};
use kernel::{City, Result};
use sqlx::{FromRow, PgPool, Postgres, Transaction};

#[derive(Debug, FromRow)]
pub struct DecisionRow {
    pub operator_id: String,
    pub property_id: String,
    pub state: String,
    pub approval_deadline_at: Option<DateTime<Utc>>,
}

pub async fn find_for_decision(pool: &PgPool, city: City, reservation_id: &str) -> Result<Option<DecisionRow>> {
    let row = sqlx::query_as::<_, DecisionRow>(
        r#"SELECT "operatorId" AS operator_id, "propertyId" AS property_id, state,
                  "approvalDeadlineAt" AS approval_deadline_at
           FROM stay_reservations WHERE id = $1 AND city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn move_request_if_version(
    tx: &mut Transaction<'_, Postgres>,
    reservation_id: &str,
    expected_version: i32,
    new_state: &str,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_reservations
           SET state = $3, "approvalDeadlineAt" = NULL, version = version + 1, "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND state = 'REQUESTED'
             AND ("approvalDeadlineAt" IS NULL OR "approvalDeadlineAt" > now())
             AND EXISTS (SELECT 1 FROM stay_inventory_allocations a
                         WHERE a."referenceType" = 'HOLD' AND a."referenceId" = $1 AND a.state = 'ACTIVE')
             AND NOT EXISTS (SELECT 1 FROM stay_inventory_allocations a
                             WHERE a."referenceType" = 'HOLD' AND a."referenceId" = $1
                               AND a.state = 'ACTIVE' AND a."expiresAt" <= now())"#,
    )
    .bind(reservation_id)
    .bind(expected_version)
    .bind(new_state)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}
