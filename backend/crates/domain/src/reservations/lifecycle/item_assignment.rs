use kernel::Result;
use sqlx::{Postgres, Transaction};

pub async fn assign_physical_unit(
    tx: &mut Transaction<'_, Postgres>,
    item_id: &str,
    unit_id: &str,
) -> Result<()> {
    sqlx::query(r#"UPDATE stay_reservation_items SET "physicalUnitId" = $2 WHERE id = $1"#)
        .bind(item_id)
        .bind(unit_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
