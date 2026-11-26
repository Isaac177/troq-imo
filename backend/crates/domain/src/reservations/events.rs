use kernel::outbox;
use kernel::{City, Result};
use serde_json::{json, Value};
use sqlx::{Postgres, Row, Transaction};

const OWNER_SQL: &str = r#"(SELECT m.user_id FROM host_members m
    WHERE m.host_id = r."operatorId" AND m.is_owner AND m.status = 'ACTIVE' LIMIT 1)"#;

fn insert_missing(data: &mut Value, key: &str, value: Value) {
    if let Some(object) = data.as_object_mut() {
        object.entry(key.to_string()).or_insert(value);
    }
}

fn set_authoritative(data: &mut Value, key: &str, value: Value) {
    if let Some(object) = data.as_object_mut() {
        object.insert(key.to_string(), value);
    }
}

async fn enrich_reservation(tx: &mut Transaction<'_, Postgres>, reservation_id: &str, data: &mut Value) -> Result<()> {
    let sql = format!(
        r#"SELECT r."guestUserId", r."operatorId" AS operator_id, {OWNER_SQL} AS owner_user_id,
                  p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr, g."fullName" AS guest_name
           FROM stay_reservations r
           LEFT JOIN stay_properties p ON p.id = r."propertyId"
           LEFT JOIN stay_reservation_items i ON i."reservationId" = r.id
           LEFT JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
           WHERE r.id = $1
           LIMIT 1"#
    );
    let Some(row) = sqlx::query(&sql).bind(reservation_id).fetch_optional(&mut **tx).await? else {
        return Ok(());
    };
    set_authoritative(data, "operatorId", json!(row.try_get::<String, _>("operator_id")?));
    insert_missing(data, "guestUserId", json!(row.try_get::<String, _>("guestUserId")?));
    insert_missing(data, "ownerUserId", json!(row.try_get::<Option<String>, _>("owner_user_id")?));
    insert_missing(
        data,
        "propertyName",
        json!({
            "en": row.try_get::<Option<String>, _>("property_name_en")?,
            "fr": row.try_get::<Option<String>, _>("property_name_fr")?,
        }),
    );
    insert_missing(data, "guestName", json!(row.try_get::<Option<String>, _>("guest_name")?));
    Ok(())
}

async fn enrich_case(tx: &mut Transaction<'_, Postgres>, data: &mut Value) -> Result<()> {
    let Some(host_id) = data.get("operatorId").and_then(Value::as_str).map(str::to_string) else {
        return Ok(());
    };
    let owner: Option<String> = sqlx::query_scalar(
        r#"SELECT user_id FROM host_members WHERE host_id = $1 AND is_owner AND status = 'ACTIVE' LIMIT 1"#,
    )
    .bind(host_id)
    .fetch_optional(&mut **tx)
    .await?;
    insert_missing(data, "ownerUserId", json!(owner));
    Ok(())
}

pub async fn emit(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    aggregate_type: &str,
    aggregate_id: &str,
    event_type: &str,
    mut data: Value,
) -> Result<String> {
    match aggregate_type {
        "reservation" => enrich_reservation(tx, aggregate_id, &mut data).await?,
        "case" => enrich_case(tx, &mut data).await?,
        _ => {}
    }
    outbox::enqueue(tx, city, aggregate_type, aggregate_id, event_type, data).await
}

async fn snapshot(tx: &mut Transaction<'_, Postgres>, reservation_id: &str) -> Result<Value> {
    let row = sqlx::query(
        r#"SELECT r.reference, r.state, r."operatorId", r."propertyId", r.city_id, r.currency,
                  r."totalAmount"::text AS total, i."checkIn"::text AS check_in, i."checkOut"::text AS check_out,
                  g."fullName", g."contactPhone", g."contactEmail", p."addressLine" AS property_address
           FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
           LEFT JOIN stay_properties p ON p.id = r."propertyId"
           WHERE r.id = $1
           ORDER BY i."checkIn" ASC
           LIMIT 1"#,
    )
    .bind(reservation_id)
    .fetch_optional(&mut **tx)
    .await?;
    let Some(row) = row else {
        return Ok(json!({ "reservationId": reservation_id }));
    };
    Ok(json!({
        "reservationId": reservation_id,
        "reference": row.try_get::<String, _>("reference")?,
        "state": row.try_get::<String, _>("state")?,
        "operatorId": row.try_get::<String, _>("operatorId")?,
        "propertyId": row.try_get::<String, _>("propertyId")?,
        "propertyAddress": row.try_get::<Option<String>, _>("property_address")?,
        "currency": row.try_get::<String, _>("currency")?,
        "totalAmount": row.try_get::<String, _>("total")?,
        "checkIn": row.try_get::<String, _>("check_in")?,
        "checkOut": row.try_get::<String, _>("check_out")?,
        "guest": {
            "fullName": row.try_get::<String, _>("fullName")?,
            "contactPhone": row.try_get::<Option<String>, _>("contactPhone")?,
            "contactEmail": row.try_get::<Option<String>, _>("contactEmail")?,
        },
    }))
}

pub async fn reservation_with(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    reservation_id: &str,
    event_type: &str,
    extra: Value,
) -> Result<String> {
    let mut data = snapshot(tx, reservation_id).await?;
    if let (Some(base), Some(additions)) = (data.as_object_mut(), extra.as_object()) {
        for (key, value) in additions {
            base.insert(key.clone(), value.clone());
        }
    }
    emit(tx, city, "reservation", reservation_id, event_type, data).await
}

pub async fn reservation(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    reservation_id: &str,
    event_type: &str,
) -> Result<String> {
    reservation_with(tx, city, reservation_id, event_type, json!({})).await
}

pub mod types {
    pub const HELD: &str = "reservation.held";
    pub const REQUEST_ACCEPTED: &str = "reservation.request_accepted";
    pub const REQUEST_DECLINED: &str = "reservation.request_declined";
    pub const CANCELLED: &str = "reservation.cancelled";
    pub const EXPIRED: &str = "reservation.expired";
    pub const CHECKED_IN: &str = "reservation.checked_in";
    pub const CHECKED_OUT: &str = "reservation.checked_out";
    pub const MARKED_NO_SHOW: &str = "reservation.marked_no_show";
    pub const SPECIAL_REQUEST_DECIDED: &str = "reservation.special_request_decided";
    pub const AMENDMENT_PROPOSED: &str = "reservation.amendment_proposed";
    pub const AMENDMENT_ACCEPTED: &str = "reservation.amendment_accepted";
    pub const AMENDMENT_DECLINED: &str = "reservation.amendment_declined";
    pub const MESSAGE_SENT: &str = "reservation.message_sent";
    pub const DAMAGE_CLAIMED: &str = "reservation.damage_claimed";
    pub const CASE_OPENED: &str = "case.opened";
    pub const CASE_CLOSED: &str = "case.closed";
    pub const CASE_ESCALATED: &str = "case.escalated";
    pub const CASE_EVIDENCE_ADDED: &str = "case.evidence_added";
    pub const CLAIM_DECIDED: &str = "claim.decided";
    pub const TASK_CREATED: &str = "operations.task.created";
    pub const TASK_UPDATED: &str = "operations.task.updated";
}
