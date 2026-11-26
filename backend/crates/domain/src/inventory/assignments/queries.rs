use chrono::NaiveDate;
use kernel::city::City;
use kernel::error::{Error, Result};
use sqlx::{PgPool, Postgres, Transaction};

use super::types::{AssignmentView, ItemContext, UnitOptionView, ASSIGNMENT_COLUMNS};
use crate::inventory::messages::UNIT_OVERLAP;

const OVERLAP_SQLSTATE: &str = "23P01";

pub async fn item_context(pool: &PgPool, item_id: &str) -> Result<Option<ItemContext>> {
    let row = sqlx::query_as::<_, ItemContext>(
        r#"SELECT i.id AS item_id, r.id AS reservation_id, i."propertyId" AS property_id, i."unitTypeId" AS unit_type_id,
                  i."checkIn" AS check_in, i."checkOut" AS check_out, r.state
           FROM stay_reservation_items i
           JOIN stay_reservations r ON r.id = i."reservationId"
           WHERE i.id = $1"#,
    )
    .bind(item_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn lock_item_context(
    tx: &mut Transaction<'_, Postgres>,
    item_id: &str,
) -> Result<Option<ItemContext>> {
    let row = sqlx::query_as::<_, ItemContext>(
        r#"SELECT i.id AS item_id, r.id AS reservation_id, i."propertyId" AS property_id, i."unitTypeId" AS unit_type_id,
                  i."checkIn" AS check_in, i."checkOut" AS check_out, r.state
           FROM stay_reservation_items i
           JOIN stay_reservations r ON r.id = i."reservationId"
           WHERE i.id = $1 FOR UPDATE OF r"#,
    )
    .bind(item_id)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(row)
}

pub async fn assignment_by_id(pool: &PgPool, assignment_id: &str) -> Result<Option<AssignmentView>> {
    let row = sqlx::query_as::<_, AssignmentView>(&format!(
        "SELECT {ASSIGNMENT_COLUMNS} FROM stay_unit_assignments WHERE id = $1"
    ))
    .bind(assignment_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn history(pool: &PgPool, item_id: &str) -> Result<Vec<AssignmentView>> {
    let rows = sqlx::query_as::<_, AssignmentView>(&format!(
        r#"SELECT {ASSIGNMENT_COLUMNS} FROM stay_unit_assignments
           WHERE "reservationItemId" = $1 ORDER BY "createdAt", id"#
    ))
    .bind(item_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn unit_is_compatible(pool: &PgPool, unit_id: &str, unit_type_id: &str) -> Result<bool> {
    let found = sqlx::query_scalar(
        r#"SELECT EXISTS (
               SELECT 1 FROM stay_physical_units u
               JOIN stay_pool_physical_units pu ON pu."physicalUnitId" = u.id
               JOIN stay_inventory_pools p ON p.id = pu."poolId"
               WHERE u.id = $1 AND u."unitTypeId" = $2 AND u.status = 'ACTIVE' AND p."unitTypeId" = $2)"#,
    )
    .bind(unit_id)
    .bind(unit_type_id)
    .fetch_one(pool)
    .await?;
    Ok(found)
}

pub async fn unit_blocked(
    tx: &mut Transaction<'_, Postgres>,
    unit_id: &str,
    from: NaiveDate,
    to: NaiveDate,
    reservation_id: &str,
) -> Result<bool> {
    let blocked = sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM stay_availability_blocks
               WHERE "physicalUnitId" = $1 AND "removedAt" IS NULL AND "startDate" < $3 AND "endDate" > $2)
           OR EXISTS (SELECT 1 FROM stay_reservation_items i
               JOIN stay_reservations r ON r.id = i."reservationId"
               WHERE i."physicalUnitId" = $1 AND i."reservationId" <> $4
                 AND r.state IN ('CONFIRMED', 'CHECKED_IN') AND i."checkIn" < $3 AND i."checkOut" > $2
                 AND NOT EXISTS (SELECT 1 FROM stay_unit_assignments a
                                 WHERE a."reservationItemId" = i.id AND a."releasedAt" IS NULL))"#,
    )
    .bind(unit_id)
    .bind(from)
    .bind(to)
    .bind(reservation_id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(blocked)
}

pub async fn lock_unit(tx: &mut Transaction<'_, Postgres>, unit_id: &str) -> Result<()> {
    sqlx::query("SELECT id FROM stay_physical_units WHERE id = $1 FOR UPDATE")
        .bind(unit_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn insert_assignment(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    id: &str,
    item_id: &str,
    unit_id: &str,
    from: NaiveDate,
    to: NaiveDate,
    reason: &str,
    actor: &str,
) -> Result<()> {
    let outcome = sqlx::query(
        r#"INSERT INTO stay_unit_assignments (id, city_id, "reservationItemId", "physicalUnitId", "startDate", "endDate", reason, "actorUserId")
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
    )
    .bind(id)
    .bind(city.code())
    .bind(item_id)
    .bind(unit_id)
    .bind(from)
    .bind(to)
    .bind(reason)
    .bind(actor)
    .execute(&mut **tx)
    .await;
    match outcome {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(database)) if database.code().as_deref() == Some(OVERLAP_SQLSTATE) => {
            Err(Error::conflict(UNIT_OVERLAP))
        }
        Err(error) => Err(error.into()),
    }
}

pub async fn release_for_move(
    tx: &mut Transaction<'_, Postgres>,
    assignment_id: &str,
    from: NaiveDate,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_unit_assignments
           SET "releasedAt" = CASE WHEN $2::date = "startDate" THEN now() ELSE NULL END,
               "endDate" = CASE WHEN $2::date > "startDate" THEN $2::date ELSE "endDate" END
           WHERE id = $1 AND "releasedAt" IS NULL AND "startDate" = $3 AND "endDate" = $4"#,
    )
    .bind(assignment_id)
    .bind(from)
    .bind(start)
    .bind(end)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn release_for_item(tx: &mut Transaction<'_, Postgres>, item_id: &str) -> Result<()> {
    sqlx::query(
        r#"UPDATE stay_unit_assignments SET "releasedAt" = now()
           WHERE "reservationItemId" = $1 AND "releasedAt" IS NULL"#,
    )
    .bind(item_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn release_for_reservation(
    tx: &mut Transaction<'_, Postgres>,
    reservation_id: &str,
) -> Result<()> {
    sqlx::query(
        r#"UPDATE stay_unit_assignments SET "releasedAt" = now()
           WHERE "releasedAt" IS NULL
             AND "reservationItemId" IN (SELECT id FROM stay_reservation_items WHERE "reservationId" = $1)"#,
    )
    .bind(reservation_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn free_units(
    pool: &PgPool,
    unit_type_id: &str,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<UnitOptionView>> {
    let rows = sqlx::query_as::<_, UnitOptionView>(
        r#"SELECT DISTINCT u.id, u."localCode" AS local_code
           FROM stay_physical_units u
           JOIN stay_pool_physical_units pu ON pu."physicalUnitId" = u.id
           JOIN stay_inventory_pools p ON p.id = pu."poolId"
           WHERE u."unitTypeId" = $1 AND u.status = 'ACTIVE' AND p."unitTypeId" = $1
             AND NOT EXISTS (
                 SELECT 1 FROM stay_unit_assignments a
                 WHERE a."physicalUnitId" = u.id AND a."releasedAt" IS NULL
                   AND daterange(a."startDate", a."endDate", '[)') && daterange($2, $3, '[)'))
           ORDER BY u."localCode""#,
    )
    .bind(unit_type_id)
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
