use std::collections::HashMap;

use chrono::NaiveDate;
use kernel::error::Result;
use sqlx::PgPool;

pub async fn remaining_by_night(
    pool: &PgPool,
    unit_type_id: &str,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<HashMap<NaiveDate, i64>> {
    let rows: Vec<(NaiveDate, i64)> = sqlx::query_as(
        r#"SELECT ($2::date + d.day_offset),
                  SUM(COALESCE(n."sellableCapacity", CASE WHEN p."defaultNightlyCapacity" THEN p."sellableQuantity" ELSE 0 END) - COALESCE(n."allocatedCount", 0) - stay_unallocated_physical_blocks(p.id, ($2::date + d.day_offset)))::bigint
           FROM stay_inventory_pools p
           CROSS JOIN LATERAL generate_series(0, $3::date - $2::date - 1) AS d(day_offset)
           LEFT JOIN stay_night_inventory n ON n."poolId" = p.id AND n."stayDate" = ($2::date + d.day_offset)
           WHERE p."unitTypeId" = $1
           GROUP BY d.day_offset"#,
    )
    .bind(unit_type_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().collect())
}

pub async fn open_nights(
    pool: &PgPool,
    unit_type_id: &str,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Result<Vec<NaiveDate>> {
    let rows: Vec<(NaiveDate,)> = sqlx::query_as(
        r#"SELECT ($2::date + d.day_offset)
           FROM stay_inventory_pools p
           CROSS JOIN LATERAL generate_series(0, $3::date - $2::date - 1) AS d(day_offset)
           LEFT JOIN stay_night_inventory n ON n."poolId" = p.id AND n."stayDate" = ($2::date + d.day_offset)
           WHERE p."unitTypeId" = $1
             AND COALESCE(n."allocatedCount", 0) + stay_unallocated_physical_blocks(p.id, ($2::date + d.day_offset)) < COALESCE(n."sellableCapacity", CASE WHEN p."defaultNightlyCapacity" THEN p."sellableQuantity" ELSE 0 END)"#,
    )
    .bind(unit_type_id)
    .bind(check_in)
    .bind(check_out)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(date,)| date).collect())
}

pub async fn room_available(
    pool: &PgPool,
    unit_type_id: &str,
    check_in: NaiveDate,
    check_out: NaiveDate,
    exclude_reservation: &str,
) -> Result<bool> {
    let available = sqlx::query_scalar("SELECT stay_room_available($1, $2, $3, $4)")
        .bind(unit_type_id)
        .bind(check_in)
        .bind(check_out)
        .bind(exclude_reservation)
        .fetch_one(pool)
        .await?;
    Ok(available)
}
