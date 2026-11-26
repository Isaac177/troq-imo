use chrono::NaiveDate;
use kernel::city::City;
use kernel::error::Result;
use sqlx::{PgPool, Postgres, Transaction};

use super::types::{RestrictionView, RESTRICTION_COLUMNS};

pub async fn list(
    pool: &PgPool,
    property_id: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<RestrictionView>, i64)> {
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_restrictions WHERE "propertyId" = $1 AND "removedAt" IS NULL"#,
    )
    .bind(property_id)
    .fetch_one(pool)
    .await?;
    let items = sqlx::query_as::<_, RestrictionView>(&format!(
        r#"SELECT {RESTRICTION_COLUMNS} FROM stay_restrictions
           WHERE "propertyId" = $1 AND "removedAt" IS NULL
           ORDER BY kind, "startDate" NULLS FIRST, id LIMIT $2 OFFSET $3"#
    ))
    .bind(property_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok((items, total))
}

pub async fn find_active(
    tx: &mut Transaction<'_, Postgres>,
    property_id: &str,
    restriction_id: &str,
) -> Result<Option<RestrictionView>> {
    let row = sqlx::query_as::<_, RestrictionView>(&format!(
        r#"SELECT {RESTRICTION_COLUMNS} FROM stay_restrictions
           WHERE id = $1 AND "propertyId" = $2 AND "removedAt" IS NULL"#
    ))
    .bind(restriction_id)
    .bind(property_id)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(row)
}

pub async fn unit_type_in_property(
    tx: &mut Transaction<'_, Postgres>,
    property_id: &str,
    unit_type_id: &str,
) -> Result<bool> {
    let found = sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM stay_unit_types WHERE id = $1 AND "propertyId" = $2)"#,
    )
    .bind(unit_type_id)
    .bind(property_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(found)
}

pub struct NewRestriction<'a> {
    pub id: &'a str,
    pub city: City,
    pub property_id: &'a str,
    pub unit_type_id: Option<&'a str>,
    pub kind: &'a str,
    pub basis: &'a str,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub value: Option<i32>,
    pub actor: &'a str,
}

pub async fn insert(tx: &mut Transaction<'_, Postgres>, row: NewRestriction<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_restrictions
           (id, city_id, "propertyId", "unitTypeId", kind, basis, "startDate", "endDate", value, "actorUserId")
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
    )
    .bind(row.id)
    .bind(row.city.code())
    .bind(row.property_id)
    .bind(row.unit_type_id)
    .bind(row.kind)
    .bind(row.basis)
    .bind(row.start_date)
    .bind(row.end_date)
    .bind(row.value)
    .bind(row.actor)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn remove(tx: &mut Transaction<'_, Postgres>, restriction_id: &str) -> Result<()> {
    sqlx::query(
        r#"UPDATE stay_restrictions SET "removedAt" = now(), version = version + 1
           WHERE id = $1 AND "removedAt" IS NULL"#,
    )
    .bind(restriction_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn overlapping(
    pool: &PgPool,
    property_id: &str,
    from: NaiveDate,
    to: NaiveDate,
    unit_ids: &[String],
) -> Result<Vec<RestrictionView>> {
    let rows = sqlx::query_as::<_, RestrictionView>(&format!(
        r#"SELECT {RESTRICTION_COLUMNS} FROM stay_restrictions
           WHERE "propertyId" = $1 AND "removedAt" IS NULL
             AND ("startDate" IS NULL OR "startDate" < $3)
             AND ("endDate" IS NULL OR "endDate" > $2)
             AND ("unitTypeId" IS NULL OR "unitTypeId" = ANY($4))
           ORDER BY kind, "startDate" NULLS FIRST, id"#
    ))
    .bind(property_id)
    .bind(from)
    .bind(to)
    .bind(unit_ids)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
