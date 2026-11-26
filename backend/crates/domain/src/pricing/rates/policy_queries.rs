use kernel::Result;
use sqlx::{FromRow, PgPool};

use super::types::PolicyView;

#[derive(Debug, FromRow)]
struct PolicyRow {
    id: String,
    kind: String,
    version_number: i32,
    content: serde_json::Value,
}

pub async fn list_policies(pool: &PgPool, kind: Option<&str>) -> Result<Vec<PolicyView>> {
    let rows = sqlx::query_as::<_, PolicyRow>(
        r#"SELECT id, kind, "versionNumber" AS version_number, content
           FROM stay_policy_versions
           WHERE "tombstonedAt" IS NULL AND ($1::text IS NULL OR kind = $1)
           ORDER BY kind, "versionNumber", id"#,
    )
    .bind(kind)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|row| PolicyView {
            id: row.id,
            kind: row.kind,
            version_number: row.version_number,
            content: row.content,
        })
        .collect())
}

pub async fn policy_kind(pool: &PgPool, policy_id: &str) -> Result<Option<String>> {
    let kind = sqlx::query_scalar::<_, String>(
        r#"SELECT kind FROM stay_policy_versions WHERE id = $1 AND "tombstonedAt" IS NULL"#,
    )
    .bind(policy_id)
    .fetch_optional(pool)
    .await?;
    Ok(kind)
}

