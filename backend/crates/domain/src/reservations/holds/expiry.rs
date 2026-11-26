use kernel::audit::{self, AuditEntry};
use kernel::{Kernel, Result};
use sqlx::PgPool;

use crate::reservations::events::{self, types::EXPIRED};
use crate::reservations::links::billing;
use crate::reservations::links::inventory::{release, ReleaseState};
use crate::reservations::support::{city_of, system_actor};

pub async fn expire_due_holds(kernel: &Kernel, batch: i64) -> Result<usize> {
    sweep(kernel.db.pool(), batch).await
}

pub async fn sweep(pool: &PgPool, batch: i64) -> Result<usize> {
    let mut tx = pool.begin().await?;
    let expired: Vec<(String, String, String)> = sqlx::query_as(
        r#"SELECT r.id, r.state, r.city_id FROM stay_reservations r
           WHERE (r.state IN ('PENDING_PAYMENT', 'REQUESTED')
                  AND EXISTS (
                      SELECT 1 FROM stay_inventory_allocations a
                      WHERE a."referenceType" = 'HOLD' AND a."referenceId" = r.id
                        AND a.state = 'ACTIVE' AND a."expiresAt" <= now()
                  ))
              OR (r.state = 'EXPIRED'
                  AND EXISTS (
                      SELECT 1 FROM stay_inventory_allocations a
                      WHERE a."referenceType" = 'HOLD' AND a."referenceId" = r.id
                        AND a.state = 'ACTIVE'
                  ))
           ORDER BY r.id
           LIMIT $1
           FOR UPDATE OF r SKIP LOCKED"#,
    )
    .bind(batch)
    .fetch_all(&mut *tx)
    .await?;

    for (reservation_id, state, city_code) in &expired {
        let city = city_of(city_code)?;
        let actor = system_actor(city, "reservation-expiry");
        let released = release(&mut tx, "HOLD", reservation_id, ReleaseState::Expired).await?;
        if state == "EXPIRED" {
            audit::record(
                &mut tx,
                &actor,
                AuditEntry {
                    action: "reservation.expired_hold_reconciled",
                    resource_type: "reservation",
                    resource_id: reservation_id,
                    reason: None,
                    change: serde_json::json!({ "releasedNights": released.nights, "releasedAllocations": released.allocations }),
                },
            )
            .await?;
            continue;
        }
        sqlx::query(
            r#"UPDATE stay_reservations SET state = 'EXPIRED', version = version + 1, "updatedAt" = now()
               WHERE id = $1 AND state IN ('PENDING_PAYMENT', 'REQUESTED')"#,
        )
        .bind(reservation_id)
        .execute(&mut *tx)
        .await?;
        billing::release_commission(&mut tx, reservation_id, 0).await?;
        audit::record(
            &mut tx,
            &actor,
            AuditEntry {
                action: "reservation.hold_expired",
                resource_type: "reservation",
                resource_id: reservation_id,
                reason: Some("hold expired before payment or operator approval"),
                change: serde_json::json!({ "state": "EXPIRED", "releasedNights": released.nights, "releasedAllocations": released.allocations }),
            },
        )
        .await?;
        events::reservation(&mut tx, city, reservation_id, EXPIRED).await?;
    }
    tx.commit().await?;
    Ok(expired.len())
}
