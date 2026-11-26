use kernel::Result;
use sqlx::{Postgres, Transaction};

pub async fn archive_for_unit_type(
    tx: &mut Transaction<'_, Postgres>,
    unit_type_id: &str,
) -> Result<()> {
    sqlx::query(r#"UPDATE stay_rate_plans SET status = 'ARCHIVED' WHERE "unitTypeId" = $1"#)
        .bind(unit_type_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
