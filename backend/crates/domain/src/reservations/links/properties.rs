use std::collections::HashMap;

use kernel::storage::Visibility;
use kernel::{Kernel, Result};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Postgres, Transaction};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImageView {
    pub id: String,
    pub url: String,
    pub alt_en: String,
    pub alt_fr: String,
    pub is_cover: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnitTypeSummary {
    pub id: String,
    pub name_en: String,
    pub name_fr: String,
    pub images: Vec<ImageView>,
}

pub struct PropertyCover {
    pub url: String,
    pub alt_en: String,
    pub alt_fr: String,
}

#[derive(Debug, FromRow)]
struct UnitImageRow {
    unit_type_id: String,
    id: String,
    alt_en: String,
    alt_fr: String,
    is_cover: bool,
    storage_key: String,
}

const IMAGE_SQL: &str = r#"SELECT u.id AS unit_type_id, m.id, m."altEn" AS alt_en, m."altFr" AS alt_fr,
    m."isCover" AS is_cover, m."storageKey" AS storage_key
    FROM stay_unit_types u
    JOIN stay_template_media m ON m."templateId" = u."templateId" AND m.status = 'READY' AND m."storageKey" IS NOT NULL
    WHERE u.id = ANY($1)
    ORDER BY u.id, m."isCover" DESC, m."sortOrder", m."createdAt", m.id"#;

pub async fn lock_property(tx: &mut Transaction<'_, Postgres>, property_id: &str) -> Result<()> {
    sqlx::query("SELECT id FROM stay_properties WHERE id = $1 FOR UPDATE")
        .bind(property_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

async fn presign(kernel: &Kernel, key: &str, media_id: &str) -> Option<String> {
    match kernel.storage.presign_get(Visibility::Private, key, None).await {
        Ok(url) => Some(url),
        Err(error) => {
            tracing::warn!(media_id = %media_id, error = %error, "stay media unavailable");
            None
        }
    }
}

pub async fn images_for_unit_types(kernel: &Kernel, unit_type_ids: &[String]) -> Result<HashMap<String, Vec<ImageView>>> {
    let mut images: HashMap<String, Vec<ImageView>> = HashMap::new();
    if unit_type_ids.is_empty() {
        return Ok(images);
    }
    let rows = sqlx::query_as::<_, UnitImageRow>(IMAGE_SQL).bind(unit_type_ids).fetch_all(kernel.db.pool()).await?;
    for row in rows {
        if let Some(url) = presign(kernel, &row.storage_key, &row.id).await {
            images.entry(row.unit_type_id).or_default().push(ImageView {
                id: row.id,
                url,
                alt_en: row.alt_en,
                alt_fr: row.alt_fr,
                is_cover: row.is_cover,
            });
        }
    }
    Ok(images)
}

pub async fn covers_for_unit_types(kernel: &Kernel, unit_type_ids: &[String]) -> Result<HashMap<String, String>> {
    let images = images_for_unit_types(kernel, unit_type_ids).await?;
    Ok(images
        .into_iter()
        .filter_map(|(unit_type_id, mut list)| (!list.is_empty()).then(|| (unit_type_id, list.remove(0).url)))
        .collect())
}

pub async fn unit_type_summary(kernel: &Kernel, unit_type_id: &str) -> Result<Option<UnitTypeSummary>> {
    let names: Option<(String, String)> = sqlx::query_as(r#"SELECT "nameEn", "nameFr" FROM stay_unit_types WHERE id = $1"#)
        .bind(unit_type_id)
        .fetch_optional(kernel.db.pool())
        .await?;
    let Some((name_en, name_fr)) = names else {
        return Ok(None);
    };
    let mut images = images_for_unit_types(kernel, &[unit_type_id.to_string()]).await?;
    Ok(Some(UnitTypeSummary { id: unit_type_id.to_string(), name_en, name_fr, images: images.remove(unit_type_id).unwrap_or_default() }))
}

pub async fn property_covers(kernel: &Kernel, property_ids: &[String]) -> Result<HashMap<String, PropertyCover>> {
    if property_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let rows: Vec<(String, String, String, String, String)> = sqlx::query_as(
        r#"SELECT DISTINCT ON ("propertyId") "propertyId", id, "altEn", "altFr", "storageKey"
           FROM stay_property_media
           WHERE "propertyId" = ANY($1) AND status = 'READY' AND "storageKey" IS NOT NULL
           ORDER BY "propertyId", "isCover" DESC, "sortOrder", "createdAt", id"#,
    )
    .bind(property_ids)
    .fetch_all(kernel.db.pool())
    .await?;
    let mut covers = HashMap::new();
    for (property_id, media_id, alt_en, alt_fr, key) in rows {
        if let Some(url) = presign(kernel, &key, &media_id).await {
            covers.insert(property_id, PropertyCover { url, alt_en, alt_fr });
        }
    }
    Ok(covers)
}
