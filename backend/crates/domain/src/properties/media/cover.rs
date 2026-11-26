use kernel::storage::{key_in_scopes, Scope, Storage};
use kernel::{City, Result};
use sqlx::PgPool;

use super::storage::image_url;

pub async fn cover_for_unit_type(
    pool: &PgPool,
    storage: &Storage,
    city: City,
    unit_type_id: &str,
) -> Result<Option<String>> {
    let key: Option<String> = sqlx::query_scalar(
        r#"SELECT m."storageKey" FROM stay_unit_types u
           JOIN stay_properties p ON p.id = u."propertyId" AND p.city_id = u.city_id
           JOIN stay_template_media m ON m."templateId" = u."templateId" AND m.status = 'READY'
           WHERE u.id = $1 AND u.city_id = $2 AND m."storageKey" IS NOT NULL
           ORDER BY m."isCover" DESC, m."sortOrder", m."createdAt", m.id LIMIT 1"#,
    )
    .bind(unit_type_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    let Some(key) = key else { return Ok(None) };
    if !key_in_scopes(&key, &[Scope::for_city(city)]) {
        tracing::warn!(unit_type_id, "template media key is outside its city scope");
        return Ok(None);
    }
    match image_url(storage, &key).await {
        Ok(url) => Ok(Some(url)),
        Err(error) => {
            tracing::warn!(unit_type_id, error = %error, "template cover unavailable");
            Ok(None)
        }
    }
}
