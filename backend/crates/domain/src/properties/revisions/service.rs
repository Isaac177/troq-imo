use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::outbox;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::{PROPERTIES_READ, PROPERTIES_WRITE};
use serde_json::json;
use sqlx::{PgPool, Postgres, Transaction};

use super::content::ListingContent;
use super::queries;
use super::types::{CreateRevision, RevisionAction, RevisionRow, RevisionSummary, RevisionView, UpdateRevision};
use crate::properties::listings::readiness;
use crate::properties::listings::types::ReadinessView;
use crate::properties::messages::{LISTING_ARCHIVED, LISTING_STATUS_INVALID, LISTING_VERSION_CONFLICT, REVISION_ALREADY_OPEN, REVISION_NOT_FOUND, REVISION_SNAPSHOT_UNREADABLE};
use crate::properties::{amenities, scope};

fn view(row: RevisionRow) -> Result<RevisionView> {
    let content = serde_json::from_value(row.snapshot).map_err(|_| Error::internal(REVISION_SNAPSHOT_UNREADABLE.as_str()))?;
    Ok(RevisionView { summary: row.summary, content })
}

pub async fn list(pool: &PgPool, actor: &Actor, property_id: &str, page: PageRequest) -> Result<Page<RevisionSummary>> {
    scope::property_for(pool, actor, property_id, PROPERTIES_READ).await?;
    queries::list(pool, property_id, page.validated()?).await
}

pub async fn get(pool: &PgPool, actor: &Actor, property_id: &str, revision_id: &str) -> Result<RevisionView> {
    scope::property_for(pool, actor, property_id, PROPERTIES_READ).await?;
    load(pool, property_id, revision_id).await
}

pub(super) async fn load(pool: &PgPool, property_id: &str, revision_id: &str) -> Result<RevisionView> {
    view(queries::find(pool, property_id, revision_id).await?.ok_or_else(|| Error::not_found(REVISION_NOT_FOUND))?)
}

pub async fn create(pool: &PgPool, actor: &Actor, property_id: &str, input: CreateRevision) -> Result<RevisionView> {
    actor.ensure_writable()?;
    scope::property_for(pool, actor, property_id, PROPERTIES_WRITE).await?;
    let content = match input.content {
        Some(content) => {
            let content = content.validated()?;
            amenities::ensure_known(pool, &content.amenities).await?;
            Some(content)
        }
        None => None,
    };
    let mut tx = pool.begin().await?;
    let property = crate::properties::property::queries::lock(&mut tx, property_id).await?
        .ok_or_else(|| Error::not_found(crate::properties::messages::PROPERTY_NOT_FOUND))?;
    if property.status == "ARCHIVED" { return Err(Error::conflict(LISTING_ARCHIVED)); }
    let exists: bool = sqlx::query_scalar(r#"SELECT EXISTS (SELECT 1 FROM stay_property_revisions WHERE "propertyId" = $1 AND "reviewStatus" IN ('DRAFT', 'SUBMITTED', 'CHANGES_REQUESTED'))"#)
        .bind(property_id).fetch_one(&mut *tx).await?;
    if exists { return Err(Error::conflict(REVISION_ALREADY_OPEN)); }
    let content = match content { Some(value) => value, None => ListingContent::from_property(&property)? };
    let snapshot = serde_json::to_value(content).map_err(|error| Error::internal(error.to_string()))?;
    let number: i32 = sqlx::query_scalar(r#"SELECT COALESCE(MAX("revisionNumber"), 0) + 1 FROM stay_property_revisions WHERE "propertyId" = $1"#)
        .bind(property_id).fetch_one(&mut *tx).await?;
    let id = new_id("rev");
    sqlx::query(r#"INSERT INTO stay_property_revisions (id, "propertyId", "revisionNumber", snapshot, "authorUserId") VALUES ($1, $2, $3, $4, $5)"#)
        .bind(&id).bind(property_id).bind(number).bind(snapshot).bind(actor.user_id().as_str()).execute(&mut *tx).await?;
    record(&mut tx, actor, property_id, &id, "create", number).await?;
    tx.commit().await?;
    load(pool, property_id, &id).await
}

pub async fn update(pool: &PgPool, actor: &Actor, property_id: &str, revision_id: &str, input: UpdateRevision) -> Result<RevisionView> {
    actor.ensure_writable()?;
    scope::property_for(pool, actor, property_id, PROPERTIES_WRITE).await?;
    let content = input.content.validated()?;
    amenities::ensure_known(pool, &content.amenities).await?;
    let snapshot = serde_json::to_value(content).map_err(|error| Error::internal(error.to_string()))?;
    let mut tx = pool.begin().await?;
    let revision = queries::lock(&mut tx, property_id, revision_id).await?.ok_or_else(|| Error::not_found(REVISION_NOT_FOUND))?;
    ensure_state(&revision, &["DRAFT", "CHANGES_REQUESTED"], input.expected_version)?;
    sqlx::query(r#"UPDATE stay_property_revisions SET snapshot = $2, version = version + 1, "updatedAt" = now() WHERE id = $1"#)
        .bind(revision_id).bind(snapshot).execute(&mut *tx).await?;
    record(&mut tx, actor, property_id, revision_id, "update", revision.summary.revision_number).await?;
    tx.commit().await?;
    load(pool, property_id, revision_id).await
}

pub async fn withdraw(pool: &PgPool, actor: &Actor, property_id: &str, revision_id: &str, input: RevisionAction) -> Result<RevisionView> {
    actor.ensure_writable()?;
    scope::property_for(pool, actor, property_id, PROPERTIES_WRITE).await?;
    let mut tx = pool.begin().await?;
    let revision = queries::lock(&mut tx, property_id, revision_id).await?.ok_or_else(|| Error::not_found(REVISION_NOT_FOUND))?;
    ensure_state(&revision, &["SUBMITTED"], input.expected_version)?;
    sqlx::query(r#"UPDATE stay_property_revisions SET "reviewStatus" = 'DRAFT', "submittedAt" = NULL, "submittedByUserId" = NULL, version = version + 1, "updatedAt" = now() WHERE id = $1"#)
        .bind(revision_id).execute(&mut *tx).await?;
    record(&mut tx, actor, property_id, revision_id, "withdraw", revision.summary.revision_number).await?;
    tx.commit().await?;
    load(pool, property_id, revision_id).await
}

pub async fn readiness(pool: &PgPool, actor: &Actor, property_id: &str, revision_id: &str) -> Result<ReadinessView> {
    let property = scope::property_for(pool, actor, property_id, PROPERTIES_READ).await?;
    let revision = load(pool, property_id, revision_id).await?;
    let mut conn = pool.acquire().await?;
    readiness::inspect(&mut conn, &property.operator_id, property_id, &revision.content).await
}

pub async fn submit(pool: &PgPool, actor: &Actor, property_id: &str, revision_id: &str, input: RevisionAction) -> Result<RevisionView> {
    actor.ensure_writable()?;
    scope::property_for(pool, actor, property_id, PROPERTIES_WRITE).await?;
    let mut tx = pool.begin().await?;
    let property = crate::properties::property::queries::lock(&mut tx, property_id).await?
        .ok_or_else(|| Error::not_found(crate::properties::messages::PROPERTY_NOT_FOUND))?;
    let revision = queries::lock(&mut tx, property_id, revision_id).await?.ok_or_else(|| Error::not_found(REVISION_NOT_FOUND))?;
    if property.status == "ARCHIVED" { return Err(Error::conflict(LISTING_ARCHIVED)); }
    ensure_state(&revision, &["DRAFT", "CHANGES_REQUESTED"], input.expected_version)?;
    let content: ListingContent = serde_json::from_value(revision.snapshot)
        .map_err(|_| Error::internal(REVISION_SNAPSHOT_UNREADABLE.as_str()))?;
    readiness::ensure_ready(&mut tx, &property.operator_id, property_id, &content).await?;
    sqlx::query(r#"UPDATE stay_property_revisions SET "reviewStatus" = 'SUBMITTED', "submittedAt" = now(), "submittedByUserId" = $2, "reviewedAt" = NULL, "reviewedByUserId" = NULL, "reviewReasons" = '[]'::jsonb, "reviewGuidance" = NULL, version = version + 1, "updatedAt" = now() WHERE id = $1"#)
        .bind(revision_id).bind(actor.user_id().as_str()).execute(&mut *tx).await?;
    let owner = crate::properties::events::owner_user(&mut tx, &property.operator_id).await?;
    outbox::enqueue(&mut tx, actor.city, "property", property_id, "property.revision.submitted", json!({
        "propertyId": property_id, "hostId": property.operator_id, "ownerUserId": owner,
        "propertyName": { "en": content.name_en, "fr": content.name_fr },
        "revisionId": revision_id, "revisionNumber": revision.summary.revision_number,
    })).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "listing.revision.submit", resource_type: "property_revision", resource_id: revision_id,
        reason: None, change: json!({ "propertyId": property_id, "revisionNumber": revision.summary.revision_number }),
    }).await?;
    tx.commit().await?;
    load(pool, property_id, revision_id).await
}

fn ensure_state(revision: &RevisionRow, allowed: &[&str], expected: i32) -> Result<()> {
    if !allowed.contains(&revision.summary.review_status.as_str()) { return Err(Error::conflict(LISTING_STATUS_INVALID)); }
    if revision.summary.version != expected { return Err(Error::conflict(LISTING_VERSION_CONFLICT)); }
    Ok(())
}

async fn record(tx: &mut Transaction<'_, Postgres>, actor: &Actor, property_id: &str, revision_id: &str, action: &str, number: i32) -> Result<()> {
    let audit_action = format!("listing.revision.{action}");
    let event_action = match action { "create" => "created", "update" => "updated", "withdraw" => "withdrawn", _ => action };
    let event_type = format!("property.revision.{event_action}");
    audit::record(tx, &actor.audit(), AuditEntry { action: &audit_action, resource_type: "property_revision", resource_id: revision_id, reason: None, change: json!({ "propertyId": property_id, "revisionNumber": number }) }).await?;
    outbox::enqueue(tx, actor.city, "property", property_id, &event_type, json!({ "propertyId": property_id, "revisionId": revision_id, "revisionNumber": number })).await?;
    Ok(())
}
