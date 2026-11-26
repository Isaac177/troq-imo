use kernel::error::Result;
use serde_json::Value;
use sqlx::{PgConnection, PgPool};

use super::types::ReadinessFacts;
use crate::properties::facts::{READINESS_FACTS_SQL, UPCOMING_STAYS_SQL};
use crate::properties::revisions::types::RevisionRow;

const REVISION_COLUMNS: &str = r#"id, "propertyId" AS property_id, "revisionNumber" AS revision_number,
    "reviewStatus" AS review_status, version, "authorUserId" AS author_user_id,
    "submittedAt" AS submitted_at, "reviewedAt" AS reviewed_at, "reviewReasons" AS review_reasons,
    "reviewGuidance" AS review_guidance, "createdAt" AS created_at, "updatedAt" AS updated_at"#;

pub async fn find_revision(pool: &PgPool, revision_id: &str) -> Result<Option<RevisionRow>> {
    let sql = format!("SELECT {REVISION_COLUMNS}, snapshot FROM stay_property_revisions WHERE id = $1");
    Ok(sqlx::query_as::<_, RevisionRow>(&sql).bind(revision_id).fetch_optional(pool).await?)
}

pub async fn revision_property_id(pool: &PgPool, revision_id: &str) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(r#"SELECT "propertyId" FROM stay_property_revisions WHERE id = $1"#)
        .bind(revision_id)
        .fetch_optional(pool)
        .await?)
}

pub async fn set_revision_decision(
    conn: &mut PgConnection,
    revision_id: &str,
    status: &str,
    reviewer: &str,
    reasons: &[String],
    guidance: Option<&str>,
) -> Result<()> {
    let reasons: Value = serde_json::to_value(reasons).map_err(|error| kernel::Error::internal(error.to_string()))?;
    sqlx::query(
        r#"UPDATE stay_property_revisions SET "reviewStatus" = $2, "reviewedAt" = now(), "reviewedByUserId" = $3,
           "reviewReasons" = $4, "reviewGuidance" = $5, version = version + 1, "updatedAt" = now()
           WHERE id = $1"#,
    )
    .bind(revision_id)
    .bind(status)
    .bind(reviewer)
    .bind(reasons)
    .bind(guidance)
    .execute(&mut *conn)
    .await?;
    Ok(())
}

pub async fn set_property_status(conn: &mut PgConnection, property_id: &str, status: &str) -> Result<()> {
    sqlx::query(r#"UPDATE stay_properties SET status = $2, version = version + 1, "updatedAt" = now() WHERE id = $1"#)
        .bind(property_id)
        .bind(status)
        .execute(&mut *conn)
        .await?;
    Ok(())
}

pub async fn upcoming_stay_count(conn: &mut PgConnection, property_id: &str, timezone: &str) -> Result<i64> {
    Ok(sqlx::query_scalar::<_, i64>(UPCOMING_STAYS_SQL)
        .bind(property_id)
        .bind(timezone)
        .fetch_one(&mut *conn)
        .await?)
}

pub async fn host_status(conn: &mut PgConnection, host_id: &str) -> Result<Option<String>> {
    Ok(sqlx::query_scalar("SELECT status FROM hosts WHERE id = $1")
        .bind(host_id)
        .fetch_optional(&mut *conn)
        .await?)
}

pub async fn readiness_facts(conn: &mut PgConnection, property_id: &str) -> Result<ReadinessFacts> {
    Ok(sqlx::query_as::<_, ReadinessFacts>(READINESS_FACTS_SQL)
        .bind(property_id)
        .fetch_one(&mut *conn)
        .await?)
}
