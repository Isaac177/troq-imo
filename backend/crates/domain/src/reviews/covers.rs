use std::collections::HashMap;

use kernel::error::Result;
use kernel::kernel::Kernel;
use sqlx::PgPool;

use super::types::ReviewView;
use crate::properties::media::image_url;

const COVER_SQL: &str = r#"SELECT DISTINCT ON (u.id) u.id AS unit_type_id, m."storageKey" AS storage_key
    FROM stay_unit_types u
    JOIN stay_template_media m ON m."templateId" = u."templateId"
        AND m.status = 'READY' AND m."storageKey" IS NOT NULL
    WHERE u.id = ANY($1)
    ORDER BY u.id, m."isCover" DESC, m."sortOrder", m."createdAt", m.id"#;

async fn cover_keys(pool: &PgPool, unit_type_ids: &[String]) -> Result<HashMap<String, String>> {
    if unit_type_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let rows: Vec<(String, String)> = sqlx::query_as(COVER_SQL)
        .bind(unit_type_ids)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().collect())
}

pub async fn cover_urls(kernel: &Kernel, unit_type_ids: &[String]) -> Result<HashMap<String, String>> {
    let mut urls = HashMap::new();
    for (unit_type_id, key) in cover_keys(kernel.db.pool(), unit_type_ids).await? {
        match image_url(&kernel.storage, &key).await {
            Ok(url) => {
                urls.insert(unit_type_id, url);
            }
            Err(error) => {
                tracing::warn!(unit_type_id = %unit_type_id, error = %error, "review cover unavailable");
            }
        }
    }
    Ok(urls)
}

pub fn unit_type_ids<'a>(ids: impl Iterator<Item = Option<&'a String>>) -> Vec<String> {
    let mut ids: Vec<String> = ids.flatten().cloned().collect();
    ids.sort();
    ids.dedup();
    ids
}

pub async fn attach(kernel: &Kernel, mut views: Vec<ReviewView>) -> Result<Vec<ReviewView>> {
    let ids = unit_type_ids(views.iter().map(|view| view.unit_type_id.as_ref()));
    let covers = cover_urls(kernel, &ids).await?;
    for view in &mut views {
        view.unit_type_cover_url = view.unit_type_id.as_ref().and_then(|id| covers.get(id).cloned());
    }
    Ok(views)
}
