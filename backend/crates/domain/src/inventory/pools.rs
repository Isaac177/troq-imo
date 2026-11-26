use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use sqlx::{Postgres, Transaction};

use super::messages::{POOL_NOT_FOUND, POOL_QUANTITY_INVALID};

async fn add_unit(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    property_id: &str,
    unit_type_id: &str,
    pool_id: &str,
    code: &str,
) -> Result<()> {
    let physical_id = new_id("physical-unit");
    sqlx::query(
        r#"INSERT INTO stay_physical_units (id, city_id, "propertyId", "unitTypeId", "localCode")
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(&physical_id)
    .bind(city.code())
    .bind(property_id)
    .bind(unit_type_id)
    .bind(code)
    .execute(&mut **tx)
    .await?;
    sqlx::query(r#"INSERT INTO stay_pool_physical_units ("poolId", "physicalUnitId") VALUES ($1, $2)"#)
        .bind(pool_id)
        .bind(&physical_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn create_pool(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    property_id: &str,
    unit_type_id: &str,
    quantity: i32,
) -> Result<String> {
    if quantity < 0 {
        return Err(Error::bad_request(POOL_QUANTITY_INVALID));
    }
    let pool_id = new_id("pool");
    sqlx::query(
        r#"INSERT INTO stay_inventory_pools (id, city_id, "propertyId", "unitTypeId", "sellableQuantity")
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(&pool_id)
    .bind(city.code())
    .bind(property_id)
    .bind(unit_type_id)
    .bind(quantity)
    .execute(&mut **tx)
    .await?;
    let suffix = unit_type_id[unit_type_id.len().saturating_sub(8)..].to_uppercase();
    for index in 1..=quantity {
        let code = format!("{suffix}-{index:02}");
        add_unit(tx, city, property_id, unit_type_id, &pool_id, &code).await?;
    }
    Ok(pool_id)
}

pub async fn resize_pool(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    property_id: &str,
    unit_type_id: &str,
    current: i32,
    next: i32,
) -> Result<()> {
    if next < 0 {
        return Err(Error::bad_request(POOL_QUANTITY_INVALID));
    }
    let pool_id: String = sqlx::query_scalar(
        r#"SELECT id FROM stay_inventory_pools WHERE "unitTypeId" = $1 FOR UPDATE"#,
    )
    .bind(unit_type_id)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| Error::not_found(POOL_NOT_FOUND))?;
    if next > current {
        for _ in current..next {
            let code = format!("UNIT-{}", new_id("c")[2..10].to_uppercase());
            add_unit(tx, city, property_id, unit_type_id, &pool_id, &code).await?;
        }
    } else {
        sqlx::query(
            r#"UPDATE stay_physical_units SET status = 'RETIRED', "retiredAt" = now()
               WHERE id IN (SELECT id FROM stay_physical_units WHERE "unitTypeId" = $1 AND status = 'ACTIVE'
               ORDER BY "createdAt" DESC, id LIMIT $2)"#,
        )
        .bind(unit_type_id)
        .bind(i64::from(current - next))
        .execute(&mut **tx)
        .await?;
    }
    sqlx::query(r#"UPDATE stay_inventory_pools SET "sellableQuantity" = $1, version = version + 1 WHERE id = $2"#)
        .bind(next)
        .bind(pool_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn retire_units(tx: &mut Transaction<'_, Postgres>, unit_type_id: &str) -> Result<()> {
    sqlx::query(
        r#"UPDATE stay_physical_units SET status = 'RETIRED', "retiredAt" = now()
           WHERE "unitTypeId" = $1 AND status = 'ACTIVE'"#,
    )
    .bind(unit_type_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
