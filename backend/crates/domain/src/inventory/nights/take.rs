use chrono::{DateTime, NaiveDate, Utc};
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use sqlx::{Postgres, Transaction};

use crate::inventory::messages::DATE_RANGE_INVALID;

pub async fn take_night(
    tx: &mut Transaction<'_, Postgres>,
    unit_type_id: &str,
    night: NaiveDate,
) -> Result<Option<String>> {
    let inventory = sqlx::query_as::<_, (String, i32, bool)>(
        r#"SELECT id, "sellableQuantity", "defaultNightlyCapacity"
           FROM stay_inventory_pools WHERE "unitTypeId" = $1 FOR UPDATE"#,
    )
    .bind(unit_type_id)
    .fetch_optional(&mut **tx)
    .await?;
    let Some((pool_id, capacity, default_capacity)) = inventory else {
        return Ok(None);
    };
    if default_capacity && capacity > 0 {
        sqlx::query(
            r#"INSERT INTO stay_night_inventory ("poolId", "stayDate", "sellableCapacity", "allocatedCount")
               VALUES ($1, $2, $3, 0) ON CONFLICT ("poolId", "stayDate") DO NOTHING"#,
        )
        .bind(&pool_id)
        .bind(night)
        .bind(capacity)
        .execute(&mut **tx)
        .await?;
    }
    let taken = sqlx::query_scalar::<_, String>(
        r#"UPDATE stay_night_inventory
           SET "allocatedCount" = "allocatedCount" + 1, version = version + 1
           WHERE "poolId" = $1 AND "stayDate" = $2
             AND "allocatedCount" + stay_unallocated_physical_blocks("poolId", "stayDate") < "sellableCapacity"
           RETURNING "poolId""#,
    )
    .bind(&pool_id)
    .bind(night)
    .fetch_optional(&mut **tx)
    .await?;
    Ok(taken)
}

pub async fn insert_allocation(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    pool_id: &str,
    reference_type: &str,
    reference_id: &str,
    night: NaiveDate,
    expires_at: Option<DateTime<Utc>>,
) -> Result<String> {
    let allocation_id = new_id("alloc");
    sqlx::query(
        r#"INSERT INTO stay_inventory_allocations (id, city_id, "poolId", "referenceType", "referenceId",
               "startDate", "endDate", quantity, state, "expiresAt")
           VALUES ($1, $2, $3, $4, $5, $6, $6 + 1, 1, 'ACTIVE', $7)"#,
    )
    .bind(&allocation_id)
    .bind(city.code())
    .bind(pool_id)
    .bind(reference_type)
    .bind(reference_id)
    .bind(night)
    .bind(expires_at)
    .execute(&mut **tx)
    .await?;
    Ok(allocation_id)
}

pub async fn take_range(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    unit_type_id: &str,
    reference_type: &str,
    reference_id: &str,
    start: NaiveDate,
    end: NaiveDate,
    expires_at: Option<DateTime<Utc>>,
) -> Result<Vec<NaiveDate>> {
    let mut unavailable = Vec::new();
    let mut night = start;
    while night < end {
        match take_night(tx, unit_type_id, night).await? {
            Some(pool_id) => {
                insert_allocation(tx, city, &pool_id, reference_type, reference_id, night, expires_at)
                    .await?;
            }
            None => unavailable.push(night),
        }
        night = night
            .succ_opt()
            .ok_or_else(|| Error::bad_request(DATE_RANGE_INVALID))?;
    }
    Ok(unavailable)
}
