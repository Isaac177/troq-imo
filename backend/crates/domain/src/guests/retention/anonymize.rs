use chrono::Utc;
use kernel::audit::{self, AuditActor, AuditEntry};
use kernel::city::City;
use kernel::error::Result;
use kernel::ids::{new_id, UserId};
use kernel::{outbox, Kernel};
use serde_json::json;

use super::dates::REDACTED;

const SYSTEM_ACTOR: &str = "system:retention";

fn system_actor(city: City) -> AuditActor {
    AuditActor {
        user_id: UserId::new(SYSTEM_ACTOR),
        city,
        permissions: Vec::new(),
        request_id: new_id("req"),
    }
}

pub async fn anonymize_expired_guests(kernel: &Kernel, batch: i64) -> Result<usize> {
    if kernel.config.guest_retention_days.is_none() {
        return Ok(0);
    }
    let mut tx = kernel.db.pool().begin().await?;
    let rows: Vec<(String, String, String)> = sqlx::query_as(
        r#"SELECT g.id, i."reservationId", r.city_id
           FROM stay_reservation_guests g
           JOIN stay_reservation_items i ON i.id = g."reservationItemId"
           JOIN stay_reservations r ON r.id = i."reservationId"
           WHERE g."anonymizedAt" IS NULL AND g."retainUntil" < $1
           ORDER BY g."retainUntil", g.id
           LIMIT $2
           FOR UPDATE OF g SKIP LOCKED"#,
    )
    .bind(Utc::now().date_naive())
    .bind(batch)
    .fetch_all(&mut *tx)
    .await?;
    for (guest_id, reservation_id, city_code) in &rows {
        let city = City::from_code(city_code)?;
        sqlx::query(
            r#"UPDATE stay_reservation_guests
               SET "fullName" = $2, "contactPhone" = NULL, "contactEmail" = NULL, "anonymizedAt" = now()
               WHERE id = $1"#,
        )
        .bind(guest_id)
        .bind(REDACTED)
        .execute(&mut *tx)
        .await?;
        audit::record(
            &mut tx,
            &system_actor(city),
            AuditEntry {
                action: "guest.anonymize",
                resource_type: "reservation",
                resource_id: reservation_id,
                reason: Some("guest data passed its retention date"),
                change: json!({ "guestId": guest_id }),
            },
        )
        .await?;
        outbox::enqueue(
            &mut tx,
            city,
            "reservation",
            reservation_id,
            "reservation.guest_anonymized",
            json!({ "reservationId": reservation_id }),
        )
        .await?;
    }
    tx.commit().await?;
    Ok(rows.len())
}
