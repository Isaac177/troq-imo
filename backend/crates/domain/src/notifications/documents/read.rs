use chrono::{DateTime, Utc};
use kernel::ids::HostId;
use kernel::storage::Visibility;
use kernel::{Error, Kernel, Page, PageRequest, Result};
use sqlx::PgPool;

use super::store::record_event;
use crate::notifications::codes::{DOCUMENT_KIND_INVALID, DOCUMENT_NOT_FOUND};
use crate::notifications::types::DocumentView;
use super::kind::DocumentKind;

pub enum DocumentScope<'a> {
    Admin,
    Guest(&'a str),
    Host(&'a [HostId]),
}

impl DocumentScope<'_> {
    fn guest(&self) -> Option<&str> {
        match self {
            Self::Guest(user_id) => Some(user_id),
            _ => None,
        }
    }

    fn hosts(&self) -> Option<Vec<String>> {
        match self {
            Self::Host(ids) => Some(ids.iter().map(|id| id.as_str().to_string()).collect()),
            _ => None,
        }
    }
}

type Row = (String, String, String, String, String, Option<String>, String, String, i32, i32, DateTime<Utc>, i64);

pub async fn list(
    pool: &PgPool,
    scope: &DocumentScope<'_>,
    kind: Option<&str>,
    entity_id: Option<&str>,
    page: PageRequest,
) -> Result<Page<DocumentView>> {
    let page = page.validated()?;
    if kind.is_some_and(|value| DocumentKind::parse(value).is_none()) {
        return Err(Error::bad_request(DOCUMENT_KIND_INVALID));
    }
    let rows = sqlx::query_as::<_, Row>(
        "SELECT id, kind, locale, entity_type, entity_id, document_number, file_name, sha256, page_count, bytes, created_at, COUNT(*) OVER() AS total FROM messaging_document WHERE ($1::text IS NULL OR guest_user_id = $1) AND ($2::text[] IS NULL OR host_id = ANY($2)) AND ($3::text IS NULL OR kind = $3) AND ($4::text IS NULL OR entity_id = $4) ORDER BY created_at DESC, id LIMIT $5 OFFSET $6",
    )
    .bind(scope.guest())
    .bind(scope.hosts())
    .bind(kind)
    .bind(entity_id)
    .bind(page.limit())
    .bind(page.offset())
    .fetch_all(pool)
    .await?;
    let total = rows.first().map(|row| row.11).unwrap_or(0);
    let items = rows
        .into_iter()
        .map(|row| DocumentView {
            id: row.0,
            kind: row.1,
            locale: row.2,
            entity_type: row.3,
            entity_id: row.4,
            document_number: row.5,
            file_name: row.6,
            sha256_prefix: row.7.chars().take(12).collect(),
            page_count: row.8,
            bytes: row.9,
            created_at: row.10,
        })
        .collect();
    Ok(Page::new(items, total, page))
}

pub async fn fetch(kernel: &Kernel, scope: &DocumentScope<'_>, id: &str, actor_id: &str) -> Result<(String, Vec<u8>)> {
    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT storage_key, file_name FROM messaging_document WHERE id = $1 AND ($2::text IS NULL OR guest_user_id = $2) AND ($3::text[] IS NULL OR host_id = ANY($3))",
    )
    .bind(id)
    .bind(scope.guest())
    .bind(scope.hosts())
    .fetch_optional(kernel.db.pool())
    .await?;
    let Some((storage_key, file_name)) = row else {
        return Err(Error::not_found(DOCUMENT_NOT_FOUND));
    };
    let bytes = kernel.storage.get_bytes(Visibility::Private, &storage_key).await?;
    let mut conn = kernel.db.pool().acquire().await?;
    record_event(&mut conn, id, "downloaded", Some(actor_id)).await?;
    Ok((file_name, bytes.to_vec()))
}
