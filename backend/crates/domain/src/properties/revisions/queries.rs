use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use sqlx::{PgConnection, PgPool};

use super::types::{RevisionRow, RevisionSummary};

const COLUMNS: &str = r#"id, "propertyId" AS property_id, "revisionNumber" AS revision_number,
    "reviewStatus" AS review_status, version, "authorUserId" AS author_user_id,
    "submittedAt" AS submitted_at, "reviewedAt" AS reviewed_at, "reviewReasons" AS review_reasons,
    "reviewGuidance" AS review_guidance, "createdAt" AS created_at, "updatedAt" AS updated_at"#;

pub async fn list(pool: &PgPool, property_id: &str, page: PageRequest) -> Result<Page<RevisionSummary>> {
    let total = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM stay_property_revisions WHERE "propertyId" = $1"#)
        .bind(property_id).fetch_one(pool).await?;
    let query = format!(r#"SELECT {COLUMNS} FROM stay_property_revisions WHERE "propertyId" = $1 ORDER BY "revisionNumber" DESC LIMIT $2 OFFSET $3"#);
    let items = sqlx::query_as::<_, RevisionSummary>(&query)
        .bind(property_id).bind(page.limit()).bind(page.offset()).fetch_all(pool).await?;
    Ok(Page::new(items, total, page))
}

pub async fn find(pool: &PgPool, property_id: &str, revision_id: &str) -> Result<Option<RevisionRow>> {
    let query = format!(r#"SELECT {COLUMNS}, snapshot FROM stay_property_revisions WHERE id = $1 AND "propertyId" = $2"#);
    Ok(sqlx::query_as::<_, RevisionRow>(&query).bind(revision_id).bind(property_id).fetch_optional(pool).await?)
}

pub async fn lock(conn: &mut PgConnection, property_id: &str, revision_id: &str) -> Result<Option<RevisionRow>> {
    let query = format!(r#"SELECT {COLUMNS}, snapshot FROM stay_property_revisions WHERE id = $1 AND "propertyId" = $2 FOR UPDATE"#);
    Ok(sqlx::query_as::<_, RevisionRow>(&query).bind(revision_id).bind(property_id).fetch_optional(&mut *conn).await?)
}
