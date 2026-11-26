use chrono::NaiveDate;
use kernel::Result;
use sqlx::PgPool;

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
    Ok(sqlx::query_scalar("SELECT stay_room_available($1, $2, $3, $4)")
        .bind(unit_type_id)
        .bind(check_in)
        .bind(check_out)
        .bind(exclude_reservation)
        .fetch_one(pool)
        .await?)
}
