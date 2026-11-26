use chrono::NaiveDate;
use kernel::city::City;
use kernel::error::Result;
use sqlx::{PgPool, Postgres, Transaction};

use super::types::{BlockView, BLOCK_COLUMNS};

pub async fn find(pool: &PgPool, block_id: &str) -> Result<Option<BlockView>> {
    let row = sqlx::query_as::<_, BlockView>(&format!(
        r#"SELECT {BLOCK_COLUMNS} FROM stay_availability_blocks WHERE id = $1"#
    ))
    .bind(block_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn list(
    pool: &PgPool,
    property_id: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<BlockView>, i64)> {
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_availability_blocks WHERE "propertyId" = $1 AND "removedAt" IS NULL"#,
    )
    .bind(property_id)
    .fetch_one(pool)
    .await?;
    let items = sqlx::query_as::<_, BlockView>(&format!(
        r#"SELECT {BLOCK_COLUMNS} FROM stay_availability_blocks
           WHERE "propertyId" = $1 AND "removedAt" IS NULL
           ORDER BY "startDate", id LIMIT $2 OFFSET $3"#
    ))
    .bind(property_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok((items, total))
}

pub struct NewBlock<'a> {
    pub id: &'a str,
    pub city: City,
    pub property_id: &'a str,
    pub unit_type_id: &'a str,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: &'a str,
    pub origin: &'a str,
    pub actor: &'a str,
}

pub async fn insert(tx: &mut Transaction<'_, Postgres>, block: NewBlock<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_availability_blocks
           (id, city_id, "propertyId", "unitTypeId", "startDate", "endDate", reason, origin, "actorUserId")
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
    )
    .bind(block.id)
    .bind(block.city.code())
    .bind(block.property_id)
    .bind(block.unit_type_id)
    .bind(block.start_date)
    .bind(block.end_date)
    .bind(block.reason)
    .bind(block.origin)
    .bind(block.actor)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn remove_if_version(
    tx: &mut Transaction<'_, Postgres>,
    block_id: &str,
    expected_version: i32,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_availability_blocks
           SET "removedAt" = now(), version = version + 1
           WHERE id = $1 AND version = $2 AND "removedAt" IS NULL"#,
    )
    .bind(block_id)
    .bind(expected_version)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}
