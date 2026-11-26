use kernel::city::City;
use kernel::error::Result;
use kernel::outbox;
use serde_json::{json, Value};
use sqlx::{Postgres, Row, Transaction};

pub const REVIEW_SUBMITTED: &str = "reservation.review_submitted";
pub const REVIEW_REPLIED: &str = "reservation.review_replied";
const AGGREGATE: &str = "reservation";

const SNAPSHOT_SQL: &str = r#"SELECT r."guestUserId" AS guest_user_id, r."operatorId" AS operator_id,
    (SELECT m.user_id FROM host_members m
      WHERE m.host_id = r."operatorId" AND m.is_owner AND m.status = 'ACTIVE'
      ORDER BY m.created_at LIMIT 1) AS owner_user_id,
    p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr,
    g."fullName" AS guest_name
    FROM stay_reservations r
    LEFT JOIN stay_properties p ON p.id = r."propertyId"
    LEFT JOIN stay_reservation_items i ON i."reservationId" = r.id
    LEFT JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
    WHERE r.id = $1 AND r.city_id = $2
    LIMIT 1"#;

async fn snapshot(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    reservation_id: &str,
) -> Result<Value> {
    let row = sqlx::query(SNAPSHOT_SQL)
        .bind(reservation_id)
        .bind(city.code())
        .fetch_optional(&mut **tx)
        .await?;
    let Some(row) = row else {
        return Ok(json!({}));
    };
    Ok(json!({
        "operatorId": row.try_get::<String, _>("operator_id")?,
        "guestUserId": row.try_get::<String, _>("guest_user_id")?,
        "ownerUserId": row.try_get::<Option<String>, _>("owner_user_id")?,
        "propertyName": {
            "en": row.try_get::<Option<String>, _>("property_name_en")?,
            "fr": row.try_get::<Option<String>, _>("property_name_fr")?,
        },
        "guestName": row.try_get::<Option<String>, _>("guest_name")?,
    }))
}

pub async fn emit(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    reservation_id: &str,
    event_type: &str,
    extra: Value,
) -> Result<()> {
    let mut data = snapshot(tx, city, reservation_id).await?;
    if let (Some(base), Some(additions)) = (data.as_object_mut(), extra.as_object()) {
        base.insert("reservationId".to_string(), json!(reservation_id));
        for (key, value) in additions {
            base.insert(key.clone(), value.clone());
        }
    }
    outbox::enqueue(tx, city, AGGREGATE, reservation_id, event_type, data).await?;
    Ok(())
}
