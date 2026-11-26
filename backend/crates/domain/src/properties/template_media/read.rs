use std::collections::HashMap;

use kernel::error::Result;
use kernel::storage::Storage;
use sqlx::PgPool;

use crate::properties::media::image_url;

pub async fn covers_for_templates(
    pool: &PgPool,
    storage: &Storage,
    template_ids: &[String],
) -> Result<HashMap<String, String>> {
    let mut covers = HashMap::new();
    if template_ids.is_empty() {
        return Ok(covers);
    }
    let rows: Vec<(String, String)> = sqlx::query_as(
        r#"SELECT DISTINCT ON ("templateId") "templateId", "storageKey" FROM stay_template_media
           WHERE "templateId" = ANY($1) AND status = 'READY' AND "storageKey" IS NOT NULL
           ORDER BY "templateId", "isCover" DESC, "sortOrder", "createdAt", id"#,
    )
    .bind(template_ids)
    .fetch_all(pool)
    .await?;
    for (template_id, key) in rows {
        match image_url(storage, &key).await {
            Ok(url) => {
                covers.insert(template_id, url);
            }
            Err(error) => tracing::warn!(template_id = %template_id, error = %error, "template cover unavailable"),
        }
    }
    Ok(covers)
}
