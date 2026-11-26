use chrono::NaiveDate;
use kernel::ids::new_id;
use kernel::{City, Error, Result};
use sqlx::{Postgres, Transaction};

use crate::reservations::codes::NO_SINGLE_ROOM_FREE;
use crate::reservations::links::inventory;

pub async fn lock_free_unit(
    tx: &mut Transaction<'_, Postgres>,
    unit_type_id: &str,
    check_in: NaiveDate,
    check_out: NaiveDate,
    reservation_id: &str,
) -> Result<Option<String>> {
    let candidates = sqlx::query_scalar::<_, String>(
        r#"SELECT u.id FROM stay_physical_units u
           JOIN stay_pool_physical_units pu ON pu."physicalUnitId" = u.id
           JOIN stay_inventory_pools p ON p.id = pu."poolId" AND p."unitTypeId" = u."unitTypeId"
           WHERE u."unitTypeId" = $1 AND u.status = 'ACTIVE'
           ORDER BY u."localCode", u.id
           FOR UPDATE OF u"#,
    )
    .bind(unit_type_id)
    .fetch_all(&mut **tx)
    .await?;
    let mut free = Vec::new();
    for unit in candidates {
        if unit_is_free(tx, &unit, check_in, check_out, reservation_id).await? {
            free.push(unit);
        }
    }
    if free.len() <= 1 {
        return Ok(free.into_iter().next());
    }
    let best: Option<String> = sqlx::query_scalar(
        r#"SELECT u.id FROM stay_physical_units u
           WHERE u.id = ANY($1)
           ORDER BY (SELECT COUNT(*) FROM stay_unit_assignments a
                     WHERE a."physicalUnitId" = u.id AND a."releasedAt" IS NULL
                       AND (a."endDate" = $2 OR a."startDate" = $3)) DESC,
                    u."localCode", u.id
           LIMIT 1"#,
    )
    .bind(&free)
    .bind(check_in)
    .bind(check_out)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(best)
}

pub async fn pool_has_rooms(tx: &mut Transaction<'_, Postgres>, unit_type_id: &str) -> Result<bool> {
    Ok(sqlx::query_scalar(
        r#"SELECT EXISTS (
               SELECT 1 FROM stay_physical_units u
               JOIN stay_pool_physical_units pu ON pu."physicalUnitId" = u.id
               JOIN stay_inventory_pools p ON p.id = pu."poolId" AND p."unitTypeId" = u."unitTypeId"
               WHERE u."unitTypeId" = $1 AND u.status = 'ACTIVE')"#,
    )
    .bind(unit_type_id)
    .fetch_one(&mut **tx)
    .await?)
}

pub async fn reserve_room(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    item_id: &str,
    unit_type_id: &str,
    reservation_id: &str,
    check_in: NaiveDate,
    check_out: NaiveDate,
    actor: &str,
) -> Result<()> {
    if !pool_has_rooms(tx, unit_type_id).await? {
        return Ok(());
    }
    let unit = lock_free_unit(tx, unit_type_id, check_in, check_out, reservation_id)
        .await?
        .ok_or_else(|| Error::conflict(NO_SINGLE_ROOM_FREE))?;
    let inserted = inventory::insert_assignment(
        tx,
        city,
        &new_id("assign"),
        item_id,
        &unit,
        check_in,
        check_out,
        "ASSIGNED",
        actor,
    )
    .await;
    match inserted {
        Ok(()) => Ok(()),
        Err(error) if error.code().as_str() == "UNIT_OVERLAP" => Err(Error::conflict(NO_SINGLE_ROOM_FREE)),
        Err(error) => Err(error),
    }
}

async fn unit_is_free(
    tx: &mut Transaction<'_, Postgres>,
    unit_id: &str,
    check_in: NaiveDate,
    check_out: NaiveDate,
    reservation_id: &str,
) -> Result<bool> {
    let busy = sqlx::query_scalar::<_, bool>(
        r#"SELECT EXISTS (
               SELECT 1 FROM stay_reservation_items i
               JOIN stay_reservations r ON r.id = i."reservationId"
               WHERE i."physicalUnitId" = $1 AND r.state IN ('CONFIRMED', 'CHECKED_IN')
                 AND i."reservationId" <> $4 AND i."checkIn" < $3 AND i."checkOut" > $2
                 AND NOT EXISTS (SELECT 1 FROM stay_unit_assignments a
                                 WHERE a."reservationItemId" = i.id AND a."releasedAt" IS NULL))
           OR EXISTS (
               SELECT 1 FROM stay_availability_blocks b
               WHERE b."physicalUnitId" = $1 AND b."removedAt" IS NULL
                 AND b."startDate" < $3 AND b."endDate" > $2)
           OR EXISTS (
               SELECT 1 FROM stay_unit_assignments a
               WHERE a."physicalUnitId" = $1 AND a."releasedAt" IS NULL
                 AND a."reservationItemId" NOT IN (
                     SELECT id FROM stay_reservation_items WHERE "reservationId" = $4)
                 AND a."startDate" < $3 AND a."endDate" > $2)"#,
    )
    .bind(unit_id)
    .bind(check_in)
    .bind(check_out)
    .bind(reservation_id)
    .fetch_one(&mut **tx)
    .await?;
    if busy {
        return Ok(false);
    }
    let buffered = inventory::turnover_blocked(tx, unit_id, reservation_id, check_in, check_out).await?;
    Ok(buffered.is_none())
}

pub async fn active_unit_for_item(tx: &mut Transaction<'_, Postgres>, item_id: &str, date: NaiveDate) -> Result<Option<String>> {
    let unit = sqlx::query_scalar::<_, String>(
        r#"SELECT "physicalUnitId" FROM stay_unit_assignments
           WHERE "reservationItemId" = $1 AND "releasedAt" IS NULL
             AND "startDate" <= $2 AND "endDate" > $2"#,
    )
    .bind(item_id)
    .bind(date)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(unit)
}

pub async fn has_active_assignments(tx: &mut Transaction<'_, Postgres>, item_id: &str) -> Result<bool> {
    Ok(sqlx::query_scalar(
        r#"SELECT EXISTS (SELECT 1 FROM stay_unit_assignments WHERE "reservationItemId" = $1 AND "releasedAt" IS NULL)"#,
    )
    .bind(item_id)
    .fetch_one(&mut **tx)
    .await?)
}

pub async fn assigned_unit_is_usable(
    tx: &mut Transaction<'_, Postgres>,
    item_id: &str,
    reservation_id: &str,
    unit_id: &str,
    date: NaiveDate,
) -> Result<bool> {
    let segment: Option<(NaiveDate, NaiveDate)> = sqlx::query_as(
        r#"SELECT a."startDate", a."endDate" FROM stay_unit_assignments a
           JOIN stay_reservation_items i ON i.id = a."reservationItemId"
           JOIN stay_physical_units u ON u.id = a."physicalUnitId" AND u."unitTypeId" = i."unitTypeId" AND u.status = 'ACTIVE'
           JOIN stay_pool_physical_units pu ON pu."physicalUnitId" = u.id
           JOIN stay_inventory_pools p ON p.id = pu."poolId" AND p."unitTypeId" = i."unitTypeId"
           WHERE a."reservationItemId" = $1 AND a."physicalUnitId" = $2 AND a."releasedAt" IS NULL
             AND a."startDate" <= $3 AND a."endDate" > $3"#,
    )
    .bind(item_id)
    .bind(unit_id)
    .bind(date)
    .fetch_optional(&mut **tx)
    .await?;
    match segment {
        Some((start, end)) => unit_is_free(tx, unit_id, start, end, reservation_id).await,
        None => Ok(false),
    }
}

pub async fn assign_at_check_in(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    item_id: &str,
    unit_id: &str,
    actor: &str,
) -> Result<()> {
    sqlx::query(r#"UPDATE stay_reservation_items SET "physicalUnitId" = $2 WHERE id = $1"#)
        .bind(item_id)
        .bind(unit_id)
        .execute(&mut **tx)
        .await?;
    sqlx::query(
        r#"INSERT INTO stay_records (id, "reservationItemId", "physicalUnitId", "actualArrivalAt", "authorizedByUserId")
           VALUES ($1, $2, $3, now(), $4)"#,
    )
    .bind(new_id("record"))
    .bind(item_id)
    .bind(unit_id)
    .bind(actor)
    .execute(&mut **tx)
    .await?;
    if has_active_assignments(tx, item_id).await? {
        return Ok(());
    }
    let (check_in, check_out): (NaiveDate, NaiveDate) =
        sqlx::query_as(r#"SELECT "checkIn", "checkOut" FROM stay_reservation_items WHERE id = $1"#)
            .bind(item_id)
            .fetch_one(&mut **tx)
            .await?;
    inventory::insert_assignment(tx, city, &new_id("assign"), item_id, unit_id, check_in, check_out, "CHECK_IN", actor).await
}

pub async fn record_departure(tx: &mut Transaction<'_, Postgres>, item_id: &str) -> Result<()> {
    sqlx::query(r#"UPDATE stay_records SET "actualDepartureAt" = now() WHERE "reservationItemId" = $1"#)
        .bind(item_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
