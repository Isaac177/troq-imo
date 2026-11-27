use chrono::{DateTime, Utc};
use kernel::{Error, Page, PageRequest, Result};
use serde_json::Value;
use sqlx::PgPool;

use super::codes::{LOCALE_INVALID, NOTIFICATION_NOT_FOUND};
use super::render::pick;
use super::types::{MarkedCount, NotificationView};

#[derive(sqlx::FromRow)]
struct Row {
    id: String,
    kind: String,
    title_en: String,
    title_fr: String,
    body_en: String,
    body_fr: String,
    priority: String,
    deep_link: Option<String>,
    event_key: Option<String>,
    category: Option<String>,
    audience: Option<String>,
    params: Option<Value>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    read_at: Option<DateTime<Utc>>,
    seen_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

const COLUMNS: &str = "id, type AS kind, title_en, title_fr, body_en, body_fr, priority, deep_link, event_key, category, audience, params, entity_type, entity_id, read_at, seen_at, created_at";

pub fn locale(value: Option<&str>) -> Result<&'static str> {
    match value.map(|text| text.trim().to_ascii_lowercase()).as_deref() {
        None | Some("") | Some("en") => Ok("en"),
        Some("fr") => Ok("fr"),
        Some(_) => Err(Error::bad_request(LOCALE_INVALID)),
    }
}

fn view(row: Row, language: &str) -> NotificationView {
    NotificationView {
        title: pick(language, &row.title_en, &row.title_fr).to_string(),
        body: pick(language, &row.body_en, &row.body_fr).to_string(),
        id: row.id,
        r#type: row.kind,
        priority: row.priority,
        deep_link: row.deep_link,
        event_key: row.event_key,
        category: row.category,
        audience: row.audience,
        params: row.params,
        entity_type: row.entity_type,
        entity_id: row.entity_id,
        read: row.read_at.is_some(),
        read_at: row.read_at,
        seen_at: row.seen_at,
        created_at: row.created_at,
    }
}

pub async fn list(pool: &PgPool, user_id: &str, audience: &str, unread_only: bool, language: &str, page: PageRequest) -> Result<Page<NotificationView>> {
    let page = page.validated()?;
    let sql = format!(
        "SELECT {COLUMNS} FROM notifications WHERE user_id = $1 AND audience = $2 AND deleted_at IS NULL AND (expires_at IS NULL OR expires_at > now()) AND (NOT $3 OR read_at IS NULL) ORDER BY created_at DESC, id LIMIT $4 OFFSET $5"
    );
    let rows = sqlx::query_as::<_, Row>(&sql)
        .bind(user_id)
        .bind(audience)
        .bind(unread_only)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(pool)
        .await?;
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND audience = $2 AND deleted_at IS NULL AND (expires_at IS NULL OR expires_at > now()) AND (NOT $3 OR read_at IS NULL)",
    ).bind(user_id).bind(audience).bind(unread_only).fetch_one(pool).await?;
    Ok(Page::new(rows.into_iter().map(|row| view(row, language)).collect(), total, page))
}

pub async fn unread_count(pool: &PgPool, user_id: &str, audience: &str) -> Result<i64> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND audience = $2 AND read_at IS NULL AND deleted_at IS NULL AND (expires_at IS NULL OR expires_at > now())",
    )
    .bind(user_id)
    .bind(audience)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn mark_read(pool: &PgPool, user_id: &str, audience: &str, id: &str) -> Result<()> {
    let updated = sqlx::query(
        "UPDATE notifications SET read_at = COALESCE(read_at, now()), seen_at = COALESCE(seen_at, now()), status = 'READ', updated_at = now() WHERE id = $1 AND user_id = $2 AND audience = $3 AND deleted_at IS NULL AND (expires_at IS NULL OR expires_at > now())",
    )
    .bind(id)
    .bind(user_id)
    .bind(audience)
    .execute(pool)
    .await?
    .rows_affected();
    if updated == 0 {
        return Err(Error::not_found(NOTIFICATION_NOT_FOUND));
    }
    Ok(())
}

pub async fn mark_all_read(pool: &PgPool, user_id: &str, audience: &str) -> Result<MarkedCount> {
    let updated = sqlx::query(
        "UPDATE notifications SET read_at = now(), seen_at = COALESCE(seen_at, now()), status = 'READ', updated_at = now() WHERE user_id = $1 AND audience = $2 AND read_at IS NULL AND deleted_at IS NULL AND (expires_at IS NULL OR expires_at > now())",
    )
    .bind(user_id)
    .bind(audience)
    .execute(pool)
    .await?
    .rows_affected();
    Ok(MarkedCount { updated: updated as i64 })
}

#[cfg(test)]
mod tests {
    use super::*;

    static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../migrations");

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn inbox_operations_respect_audience_and_expiry(pool: PgPool) {
        sqlx::query(
            "INSERT INTO notifications (id, city_id, user_id, type, title, title_en, title_fr, body, body_en, body_fr, audience, expires_at) VALUES ('seed-host', 'GOMA', 'seed-user', 'INFO', 'Title', 'Title', 'Titre', 'Body', 'Body', 'Texte', 'host', NULL), ('seed-guest', 'GOMA', 'seed-user', 'INFO', 'Title', 'Title', 'Titre', 'Body', 'Body', 'Texte', 'guest', NULL), ('seed-expired', 'GOMA', 'seed-user', 'INFO', 'Title', 'Title', 'Titre', 'Body', 'Body', 'Texte', 'host', now() - interval '1 day'), ('seed-other-user', 'GOMA', 'seed-other', 'INFO', 'Title', 'Title', 'Titre', 'Body', 'Body', 'Texte', 'host', NULL)",
        ).execute(&pool).await.unwrap();
        let host = list(&pool, "seed-user", "host", true, "en", PageRequest::default()).await.unwrap();
        assert_eq!(host.total, 1);
        assert_eq!(host.items[0].id, "seed-host");
        assert_eq!(unread_count(&pool, "seed-user", "host").await.unwrap(), 1);
        assert!(mark_read(&pool, "seed-user", "host", "seed-guest").await.is_err());
        assert!(mark_read(&pool, "seed-user", "host", "seed-expired").await.is_err());
        assert_eq!(mark_all_read(&pool, "seed-user", "host").await.unwrap().updated, 1);
        assert_eq!(unread_count(&pool, "seed-user", "guest").await.unwrap(), 1);
        let expired_read: Option<DateTime<Utc>> = sqlx::query_scalar("SELECT read_at FROM notifications WHERE id = 'seed-expired'")
            .fetch_one(&pool).await.unwrap();
        assert!(expired_read.is_none());
    }

    #[test]
    fn locale_defaults_to_english_and_rejects_unknown() {
        assert_eq!(locale(None).unwrap(), "en");
        assert_eq!(locale(Some(" FR ")).unwrap(), "fr");
        assert!(locale(Some("sw")).is_err());
    }
}
