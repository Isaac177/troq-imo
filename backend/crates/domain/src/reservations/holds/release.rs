use kernel::Result;
use sqlx::{Postgres, Transaction};

use crate::reservations::links::inventory::{release, ReleaseState};

pub async fn release_holds(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<()> {
    release(tx, "HOLD", reservation_id, ReleaseState::Released).await?;
    Ok(())
}

pub async fn release_reservation_nights(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<()> {
    release(tx, "RESERVATION", reservation_id, ReleaseState::Released).await?;
    Ok(())
}

pub async fn convert_holds_to_reservations(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<()> {
    sqlx::query(
        r#"UPDATE stay_inventory_allocations
           SET "referenceType" = 'RESERVATION', "expiresAt" = NULL
           WHERE "referenceType" = 'HOLD' AND "referenceId" = $1 AND state = 'ACTIVE'"#,
    )
    .bind(reservation_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
