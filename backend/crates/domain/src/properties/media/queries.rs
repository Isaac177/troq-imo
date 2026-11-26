use kernel::error::Result;
use sqlx::{PgPool, Postgres, Transaction};

use super::kind::MediaKind;
use super::types::{media_columns, MediaChanges, PropertyMediaView};

pub async fn list(pool: &PgPool, kind: MediaKind, owner_id: &str) -> Result<Vec<PropertyMediaView>> {
    let sql = format!(
        r#"SELECT {} FROM {} WHERE {owner} = $1 AND status <> 'REMOVED' ORDER BY "sortOrder", "createdAt", id"#,
        media_columns(kind),
        kind.table(),
        owner = kind.owner_column()
    );
    Ok(sqlx::query_as::<_, PropertyMediaView>(&sql).bind(owner_id).fetch_all(pool).await?)
}

pub async fn find(pool: &PgPool, kind: MediaKind, media_id: &str) -> Result<Option<PropertyMediaView>> {
    let sql = format!(
        "SELECT {} FROM {} WHERE id = $1 AND status <> 'REMOVED'",
        media_columns(kind),
        kind.table()
    );
    Ok(sqlx::query_as::<_, PropertyMediaView>(&sql).bind(media_id).fetch_optional(pool).await?)
}

pub async fn find_by_client_key(
    pool: &PgPool,
    kind: MediaKind,
    owner_id: &str,
    client_key: &str,
) -> Result<Option<PropertyMediaView>> {
    let sql = format!(
        r#"SELECT {} FROM {} WHERE {owner} = $1 AND "clientKey" = $2"#,
        media_columns(kind),
        kind.table(),
        owner = kind.owner_column()
    );
    Ok(sqlx::query_as::<_, PropertyMediaView>(&sql).bind(owner_id).bind(client_key).fetch_optional(pool).await?)
}

pub async fn storage_key(pool: &PgPool, kind: MediaKind, media_id: &str, ready_only: bool) -> Result<Option<String>> {
    let filter = if ready_only { " AND status = 'READY'" } else { "" };
    let sql = format!(r#"SELECT "storageKey" FROM {} WHERE id = $1{filter}"#, kind.table());
    let key: Option<Option<String>> = sqlx::query_scalar(&sql).bind(media_id).fetch_optional(pool).await?;
    Ok(key.flatten())
}

pub async fn lock_owner(tx: &mut Transaction<'_, Postgres>, kind: MediaKind, owner_id: &str) -> Result<()> {
    let sql = format!("SELECT id FROM {} WHERE id = $1 FOR UPDATE", kind.owner_table());
    sqlx::query(&sql).bind(owner_id).execute(&mut **tx).await?;
    Ok(())
}

pub async fn next_sort_order(tx: &mut Transaction<'_, Postgres>, kind: MediaKind, owner_id: &str) -> Result<i32> {
    let sql = format!(
        r#"SELECT COALESCE(MAX("sortOrder"), 0) + 1 FROM {} WHERE {owner} = $1 AND status <> 'REMOVED'"#,
        kind.table(),
        owner = kind.owner_column()
    );
    Ok(sqlx::query_scalar::<_, i32>(&sql).bind(owner_id).fetch_one(&mut **tx).await?)
}

pub async fn has_cover(tx: &mut Transaction<'_, Postgres>, kind: MediaKind, owner_id: &str) -> Result<bool> {
    let sql = format!(
        r#"SELECT EXISTS (SELECT 1 FROM {} WHERE {owner} = $1 AND "isCover")"#,
        kind.table(),
        owner = kind.owner_column()
    );
    Ok(sqlx::query_scalar::<_, bool>(&sql).bind(owner_id).fetch_one(&mut **tx).await?)
}

pub async fn clear_covers_except(
    tx: &mut Transaction<'_, Postgres>,
    kind: MediaKind,
    owner_id: &str,
    media_id: &str,
) -> Result<()> {
    let sql = format!(
        r#"UPDATE {} SET "isCover" = FALSE WHERE {owner} = $1 AND "isCover" AND id <> $2"#,
        kind.table(),
        owner = kind.owner_column()
    );
    sqlx::query(&sql).bind(owner_id).bind(media_id).execute(&mut **tx).await?;
    Ok(())
}

pub async fn promote_first_ready(tx: &mut Transaction<'_, Postgres>, kind: MediaKind, owner_id: &str) -> Result<()> {
    let sql = format!(
        r#"UPDATE {table} SET "isCover" = TRUE WHERE id = (SELECT id FROM {table}
           WHERE {owner} = $1 AND status = 'READY' ORDER BY "sortOrder", "createdAt", id LIMIT 1)"#,
        table = kind.table(),
        owner = kind.owner_column()
    );
    sqlx::query(&sql).bind(owner_id).execute(&mut **tx).await?;
    Ok(())
}

pub struct NewMedia<'a> {
    pub id: &'a str,
    pub owner_id: &'a str,
    pub status: &'a str,
    pub sort_order: i32,
    pub is_cover: bool,
    pub alt_en: &'a str,
    pub alt_fr: &'a str,
    pub file_name: &'a str,
    pub mime_type: &'a str,
    pub storage_key: Option<&'a str>,
    pub size_bytes: usize,
    pub rejection_reason: Option<&'a str>,
    pub client_key: &'a str,
    pub actor: &'a str,
}

pub async fn insert(tx: &mut Transaction<'_, Postgres>, kind: MediaKind, media: NewMedia<'_>) -> Result<()> {
    let sql = format!(
        r#"INSERT INTO {} (id, {}, status, "sortOrder", "isCover", "altEn", "altFr", "fileName", "mimeType",
           "sizeBytes", "rejectionReason", "clientKey", "addedByUserId", "storageKey")
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
        kind.table(),
        kind.owner_column()
    );
    sqlx::query(&sql)
        .bind(media.id)
        .bind(media.owner_id)
        .bind(media.status)
        .bind(media.sort_order)
        .bind(media.is_cover)
        .bind(media.alt_en)
        .bind(media.alt_fr)
        .bind(media.file_name)
        .bind(media.mime_type)
        .bind(media.size_bytes as i32)
        .bind(media.rejection_reason)
        .bind(media.client_key)
        .bind(media.actor)
        .bind(media.storage_key)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn update_if_version(
    tx: &mut Transaction<'_, Postgres>,
    kind: MediaKind,
    media_id: &str,
    expected_version: i32,
    changes: &MediaChanges,
) -> Result<bool> {
    let sql = format!(
        r#"UPDATE {} SET "altEn" = COALESCE($3, "altEn"), "altFr" = COALESCE($4, "altFr"),
           "isCover" = CASE WHEN $7 THEN FALSE ELSE COALESCE($5, "isCover") END,
           "sortOrder" = COALESCE($6, "sortOrder"),
           status = CASE WHEN $7 THEN 'REMOVED' ELSE status END,
           "sizeBytes" = CASE WHEN $7 THEN 0 ELSE "sizeBytes" END,
           "storageKey" = CASE WHEN $7 THEN NULL ELSE "storageKey" END,
           version = version + 1, "updatedAt" = now()
           WHERE id = $1 AND version = $2 AND status <> 'REMOVED'"#,
        kind.table()
    );
    let result = sqlx::query(&sql)
        .bind(media_id)
        .bind(expected_version)
        .bind(changes.alt_en.as_deref())
        .bind(changes.alt_fr.as_deref())
        .bind(changes.is_cover)
        .bind(changes.sort_order)
        .bind(changes.remove)
        .execute(&mut **tx)
        .await?;
    Ok(result.rows_affected() == 1)
}
