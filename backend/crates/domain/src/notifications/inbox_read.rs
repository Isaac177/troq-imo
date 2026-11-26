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
    total: i64,
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

pub async fn list(pool: &PgPool, user_id: &str, unread_only: bool, language: &str, page: PageRequest) -> Result<Page<NotificationView>> {
    let page = page.validated()?;
    let sql = format!(
        "SELECT {COLUMNS}, COUNT(*) OVER() AS total FROM notifications WHERE user_id = $1 AND deleted_at IS NULL AND (expires_at IS NULL OR expires_at > now()) AND (NOT $2 OR read_at IS NULL) ORDER BY created_at DESC, id LIMIT $3 OFFSET $4"
    );
    let rows = sqlx::query_as::<_, Row>(&sql)
        .bind(user_id)
        .bind(unread_only)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(pool)
        .await?;
    let total = rows.first().map(|row| row.total).unwrap_or(0);
    Ok(Page::new(rows.into_iter().map(|row| view(row, language)).collect(), total, page))
}

pub async fn unread_count(pool: &PgPool, user_id: &str) -> Result<i64> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND read_at IS NULL AND deleted_at IS NULL",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn mark_read(pool: &PgPool, user_id: &str, id: &str) -> Result<()> {
    let updated = sqlx::query(
        "UPDATE notifications SET read_at = COALESCE(read_at, now()), seen_at = COALESCE(seen_at, now()), status = 'READ', updated_at = now() WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?
    .rows_affected();
    if updated == 0 {
        return Err(Error::not_found(NOTIFICATION_NOT_FOUND));
    }
    Ok(())
}

pub async fn mark_all_read(pool: &PgPool, user_id: &str) -> Result<MarkedCount> {
    let updated = sqlx::query(
        "UPDATE notifications SET read_at = now(), seen_at = COALESCE(seen_at, now()), status = 'READ', updated_at = now() WHERE user_id = $1 AND read_at IS NULL AND deleted_at IS NULL",
    )
    .bind(user_id)
    .execute(pool)
    .await?
    .rows_affected();
    Ok(MarkedCount { updated: updated as i64 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_defaults_to_english_and_rejects_unknown() {
        assert_eq!(locale(None).unwrap(), "en");
        assert_eq!(locale(Some(" FR ")).unwrap(), "fr");
        assert!(locale(Some("sw")).is_err());
    }
}
