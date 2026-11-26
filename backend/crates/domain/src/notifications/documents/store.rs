use kernel::ids::new_id;
use kernel::storage::{key_for, sanitize_segment, unique_name, Scope, Storage, Visibility};
use kernel::{City, Error, Result};
use sqlx::PgConnection;

use super::kind::DocumentKind;
use super::pipeline::Generated;

pub struct StoreRequest<'a> {
    pub city: City,
    pub kind: DocumentKind,
    pub locale: &'a str,
    pub entity_id: &'a str,
    pub guest_user_id: Option<&'a str>,
    pub host_id: Option<&'a str>,
    pub created_by: &'a str,
    pub source_delivery_id: &'a str,
}

pub fn storage_key(request: &StoreRequest<'_>) -> Result<String> {
    let kind = format!(
        "documents/{}/{}/{}",
        request.kind.as_str(),
        request.kind.entity_type(),
        sanitize_segment(request.entity_id)
    );
    Ok(key_for(&Scope::for_city(request.city), &kind, &unique_name("pdf"))?)
}

pub async fn upload(storage: &Storage, generated: &Generated, request: &StoreRequest<'_>) -> Result<String> {
    let key = storage_key(request)?;
    storage.put(Visibility::Private, &key, generated.bytes.clone(), "application/pdf").await?;
    Ok(key)
}

pub struct Existing {
    pub city: String,
    pub locale: String,
    pub source_delivery_id: Option<String>,
    pub storage_key: String,
    pub file_name: String,
    pub sha256: String,
    pub page_count: i32,
}

pub async fn find(conn: &mut PgConnection, kind: DocumentKind, entity_id: &str) -> Result<Option<Existing>> {
    let row = sqlx::query_as::<_, (String, String, Option<String>, String, String, String, i32)>(
        "SELECT city_id, locale, source_delivery_id, storage_key, file_name, sha256, page_count FROM messaging_document WHERE kind = $1 AND entity_type = $2 AND entity_id = $3",
    ).bind(kind.as_str()).bind(kind.entity_type()).bind(entity_id).fetch_optional(conn).await?;
    Ok(row.map(|(city, locale, source_delivery_id, storage_key, file_name, sha256, page_count)| Existing {
        city, locale, source_delivery_id, storage_key, file_name, sha256, page_count,
    }))
}

pub async fn record(conn: &mut PgConnection, generated: &Generated, storage_key: &str, request: &StoreRequest<'_>) -> Result<String> {
    let id = new_id("doc");
    sqlx::query(
        "INSERT INTO messaging_document (id, city_id, kind, locale, entity_type, entity_id, storage_key, file_name, sha256, page_count, bytes, guest_user_id, host_id, created_by, source_delivery_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)",
    )
    .bind(&id)
    .bind(request.city.code())
    .bind(request.kind.as_str())
    .bind(request.locale)
    .bind(request.kind.entity_type())
    .bind(request.entity_id)
    .bind(storage_key)
    .bind(&generated.file_name)
    .bind(&generated.sha256)
    .bind(generated.page_count as i32)
    .bind(generated.bytes.len() as i32)
    .bind(request.guest_user_id)
    .bind(request.host_id)
    .bind(request.created_by)
    .bind(request.source_delivery_id)
    .execute(&mut *conn)
    .await?;
    record_event(conn, &id, "generated", Some(request.created_by)).await?;
    Ok(id)
}

pub async fn record_event(conn: &mut PgConnection, document_id: &str, event: &str, actor_id: Option<&str>) -> Result<()> {
    sqlx::query("INSERT INTO messaging_document_event (id, document_id, event_type, actor_id) VALUES ($1, $2, $3, $4)")
        .bind(new_id("docevt"))
        .bind(document_id)
        .bind(event)
        .bind(actor_id)
        .execute(conn)
        .await?;
    Ok(())
}

pub fn internal(error: anyhow::Error) -> Error {
    Error::internal(format!("{error:#}"))
}
