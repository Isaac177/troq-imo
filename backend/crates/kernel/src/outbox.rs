use serde_json::{json, Value};
use sqlx::{Postgres, Transaction};

use crate::city::City;
use crate::error::Result;
use crate::ids::new_event_id;

pub const EVENT_VERSION: i32 = 1;

pub async fn enqueue(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    aggregate_type: &str,
    aggregate_id: &str,
    event_type: &str,
    mut data: Value,
) -> Result<String> {
    if let Some(object) = data.as_object_mut() {
        object
            .entry("eventType".to_string())
            .or_insert_with(|| json!(event_type));
        object.insert("city".to_string(), json!(city));
    }
    let event_id = new_event_id();
    let payload = json!({
        "eventId": event_id,
        "aggregateType": aggregate_type,
        "aggregateId": aggregate_id,
        "eventType": event_type,
        "data": data,
        "metadata": {
            "city": city,
            "requestId": event_id,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        },
    });
    sqlx::query(
        r#"INSERT INTO kernel_outbox (id, city_id, aggregate_type, aggregate_id, event_type, event_version, payload)
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
    )
    .bind(&event_id)
    .bind(city.code())
    .bind(aggregate_type)
    .bind(aggregate_id)
    .bind(event_type)
    .bind(EVENT_VERSION)
    .bind(payload)
    .execute(&mut **tx)
    .await?;
    Ok(event_id)
}
