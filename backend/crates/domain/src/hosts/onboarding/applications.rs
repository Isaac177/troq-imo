use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::ids::new_id;
use kernel::outbox;
use kernel::paging::{Page, PageRequest};
use serde_json::json;
use sqlx::{PgPool, Postgres, Transaction};

use super::documents::list_for;
use super::types::{kind, ApplicationInput, ApplicationView, DOCUMENT_KINDS};
use crate::hosts::messages;

type ApplicationRow = (
    String,
    String,
    String,
    String,
    String,
    String,
    Option<String>,
    Option<DateTime<Utc>>,
    Option<DateTime<Utc>>,
    Option<String>,
    DateTime<Utc>,
);

fn view(row: ApplicationRow, documents: Vec<super::types::HostDocumentView>) -> ApplicationView {
    let (id, applicant_user_id, city_id, display_name, kind, status, reason_code, submitted_at, decided_at, host_id, created_at) = row;
    ApplicationView {
        id,
        applicant_user_id,
        city_id,
        display_name,
        kind,
        status,
        reason_code,
        submitted_at,
        decided_at,
        host_id,
        created_at,
        documents,
    }
}

const SELECT: &str = "SELECT id, applicant_user_id, city_id, display_name, kind, status, reason_code, submitted_at, decided_at, host_id, created_at FROM host_applications";

pub async fn create_or_update_draft(pool: &PgPool, actor: &Actor, input: ApplicationInput) -> Result<ApplicationView> {
    actor.ensure_writable()?;
    let display_name = input.display_name.trim();
    if display_name.is_empty() || display_name.chars().count() > 160 {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    let kind = kind(&input.kind).ok_or_else(|| Error::bad_request(messages::KIND_INVALID))?;
    let mut tx = pool.begin().await?;
    let open: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM host_applications WHERE applicant_user_id = $1 AND status IN ('PENDING_REVIEW', 'APPROVED'))",
    )
    .bind(actor.user_id().as_str())
    .fetch_one(&mut *tx)
    .await?;
    if open {
        return Err(Error::conflict(messages::APPLICATION_EXISTS));
    }
    let existing: Option<String> = sqlx::query_scalar(
        "SELECT id FROM host_applications WHERE applicant_user_id = $1 AND status = 'DRAFT' ORDER BY created_at DESC LIMIT 1 FOR UPDATE",
    )
    .bind(actor.user_id().as_str())
    .fetch_optional(&mut *tx)
    .await?;
    let id = match existing {
        Some(id) => {
            sqlx::query("UPDATE host_applications SET display_name = $2, kind = $3, city_id = $4, updated_at = now() WHERE id = $1")
                .bind(&id)
                .bind(display_name)
                .bind(kind)
                .bind(actor.city.code())
                .execute(&mut *tx)
                .await?;
            id
        }
        None => {
            let id = new_id("application");
            sqlx::query(
                "INSERT INTO host_applications (id, applicant_user_id, city_id, display_name, kind) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(&id)
            .bind(actor.user_id().as_str())
            .bind(actor.city.code())
            .bind(display_name)
            .bind(kind)
            .execute(&mut *tx)
            .await?;
            id
        }
    };
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "host.application.saved",
        resource_type: "host_application",
        resource_id: &id,
        reason: None,
        change: json!({ "displayName": display_name, "kind": kind }),
    })
    .await?;
    tx.commit().await?;
    load(pool, &id, true).await
}

async fn load(pool: &PgPool, id: &str, with_documents: bool) -> Result<ApplicationView> {
    let row = sqlx::query_as::<_, ApplicationRow>(&format!("{SELECT} WHERE id = $1"))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    let documents = if with_documents { list_for(pool, id).await? } else { Vec::new() };
    Ok(view(row, documents))
}

pub async fn review_view(pool: &PgPool, id: &str) -> Result<ApplicationView> {
    load(pool, id, true).await
}

pub async fn list_mine(pool: &PgPool, actor: &Actor, request: PageRequest) -> Result<Page<ApplicationView>> {
    let request = request.validated()?;
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM host_applications WHERE applicant_user_id = $1",
    )
    .bind(actor.user_id().as_str())
    .fetch_one(pool)
    .await?;
    let rows = sqlx::query_as::<_, ApplicationRow>(&format!(
        "{SELECT} WHERE applicant_user_id = $1 ORDER BY created_at DESC, id LIMIT $2 OFFSET $3"
    ))
    .bind(actor.user_id().as_str())
    .bind(request.limit())
    .bind(request.offset())
    .fetch_all(pool)
    .await?;
    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let documents = list_for(pool, &row.0).await?;
        items.push(view(row, documents));
    }
    Ok(Page::new(items, total, request))
}

async fn owned_application<'a>(tx: &mut Transaction<'a, Postgres>, actor: &Actor, id: &str, allowed: &[&str]) -> Result<ApplicationRow> {
    let row = sqlx::query_as::<_, ApplicationRow>(&format!(
        "{SELECT} WHERE id = $1 AND applicant_user_id = $2 FOR UPDATE"
    ))
    .bind(id)
    .bind(actor.user_id().as_str())
    .fetch_optional(&mut **tx)
    .await?
    .ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    if !allowed.contains(&row.5.as_str()) {
        return Err(Error::conflict(messages::APPLICATION_STATE).with_params(json!({ "status": row.5 })));
    }
    Ok(row)
}

pub async fn submit(pool: &PgPool, actor: &Actor, id: &str) -> Result<ApplicationView> {
    actor.ensure_writable()?;
    let mut tx = pool.begin().await?;
    let row = owned_application(&mut tx, actor, id, &["DRAFT"]).await?;
    let kinds: Vec<String> = DOCUMENT_KINDS.iter().map(|kind| kind.to_string()).collect();
    let documents: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_application_documents WHERE application_id = $1 AND kind = ANY($2)")
        .bind(id)
        .bind(&kinds)
        .fetch_one(&mut *tx)
        .await?;
    if documents == 0 {
        return Err(Error::bad_request(messages::DOCUMENT_REQUIRED));
    }
    sqlx::query("UPDATE host_applications SET status = 'PENDING_REVIEW', submitted_at = now(), updated_at = now() WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "host.application.submitted",
        resource_type: "host_application",
        resource_id: id,
        reason: None,
        change: json!({ "displayName": row.3, "kind": row.4 }),
    })
    .await?;
    outbox::enqueue(&mut tx, actor.city, "host_application", id, "host.application.submitted", json!({ "applicationId": id })).await?;
    tx.commit().await?;
    load(pool, id, true).await
}

pub async fn withdraw(pool: &PgPool, actor: &Actor, id: &str) -> Result<ApplicationView> {
    actor.ensure_writable()?;
    let mut tx = pool.begin().await?;
    let _ = owned_application(&mut tx, actor, id, &["DRAFT", "PENDING_REVIEW"]).await?;
    sqlx::query("UPDATE host_applications SET status = 'WITHDRAWN', updated_at = now() WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "host.application.withdrawn",
        resource_type: "host_application",
        resource_id: id,
        reason: None,
        change: json!({ "applicationId": id }),
    })
    .await?;
    tx.commit().await?;
    load(pool, id, true).await
}
