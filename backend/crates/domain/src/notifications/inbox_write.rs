use kernel::ids::new_id;
use kernel::{City, Result};
use serde_json::{Map, Value};
use sqlx::PgConnection;

pub struct NewNotification<'a> {
    pub city: City,
    pub user_id: &'a str,
    pub db_type: &'a str,
    pub priority: &'a str,
    pub title_en: &'a str,
    pub title_fr: &'a str,
    pub body_en: &'a str,
    pub body_fr: &'a str,
    pub deep_link: &'a str,
    pub event_key: &'a str,
    pub category: &'a str,
    pub audience: &'a str,
    pub params: &'a Map<String, Value>,
    pub entity_type: Option<&'a str>,
    pub entity_id: Option<&'a str>,
    pub actor_id: Option<&'a str>,
    pub dedupe_key: Option<&'a str>,
    pub source_event_id: &'a str,
}

pub async fn insert(conn: &mut PgConnection, input: &NewNotification<'_>) -> Result<Option<String>> {
    let id = new_id("notif");
    let inserted = sqlx::query_scalar::<_, String>(
        "INSERT INTO notifications (id, city_id, user_id, type, title, title_en, title_fr, body, body_en, body_fr, data, status, priority, deep_link, event_key, category, audience, params, entity_type, entity_id, actor_id, dedupe_key, source_event_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'SENT', $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22) ON CONFLICT DO NOTHING RETURNING id",
    )
    .bind(&id)
    .bind(input.city.code())
    .bind(input.user_id)
    .bind(input.db_type)
    .bind(input.title_fr)
    .bind(input.title_en)
    .bind(input.title_fr)
    .bind(input.body_fr)
    .bind(input.body_en)
    .bind(input.body_fr)
    .bind(serde_json::json!({ "eventKey": input.event_key, "deepLink": input.deep_link }))
    .bind(input.priority)
    .bind(input.deep_link)
    .bind(input.event_key)
    .bind(input.category)
    .bind(input.audience)
    .bind(Value::Object(input.params.clone()))
    .bind(input.entity_type)
    .bind(input.entity_id)
    .bind(input.actor_id)
    .bind(input.dedupe_key)
    .bind(input.source_event_id)
    .fetch_optional(conn)
    .await?;
    Ok(inserted)
}

pub async fn recent_duplicates(conn: &mut PgConnection, user_ids: &[String], dedupe_key: &str) -> Result<Vec<String>> {
    let ids = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT user_id FROM notifications WHERE user_id = ANY($1) AND dedupe_key = $2 AND created_at > now() - interval '10 minutes'",
    )
    .bind(user_ids)
    .bind(dedupe_key)
    .fetch_all(conn)
    .await?;
    Ok(ids)
}
