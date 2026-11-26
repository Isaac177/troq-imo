use kernel::storage::{key_in_scopes, Scope};
use kernel::{City, Kernel, Page, PageRequest, Result};
use super::reservation_detail_types::{Image, UnitType};

pub async fn unit_type(kernel: &Kernel, city: City, reservation_id: &str, page: PageRequest) -> Result<Option<UnitType>> {
    let pool = kernel.db.pool();
    let row: Option<(String, String, String)> = sqlx::query_as(r#"
        SELECT t.id, t."nameEn", t."nameFr" FROM stay_reservation_items i
        JOIN stay_reservations r ON r.id = i."reservationId" AND r.city_id = $2
        JOIN stay_unit_types t ON t.id = i."unitTypeId" AND t.city_id = r.city_id
        WHERE r.id = $1 ORDER BY i."checkIn", i.id LIMIT 1"#)
        .bind(reservation_id).bind(city.code()).fetch_optional(pool).await?;
    let Some((id, name_en, name_fr)) = row else { return Ok(None) };
    let count: i64 = sqlx::query_scalar(r#"
        SELECT COUNT(*) FROM stay_template_media m JOIN stay_unit_types u ON u."templateId" = m."templateId"
        WHERE u.id = $1 AND u.city_id = $2 AND m.status = 'READY' AND m."storageKey" LIKE lower($2) || '/%'"#)
        .bind(&id).bind(city.code()).fetch_one(pool).await?;
    let mut images: Vec<Image> = sqlx::query_as(r#"
        SELECT m.id, m."altEn" AS alt_en, m."altFr" AS alt_fr, m."isCover" AS is_cover, m."storageKey" AS storage_key
        FROM stay_template_media m JOIN stay_unit_types u ON u."templateId" = m."templateId"
        WHERE u.id = $1 AND u.city_id = $2 AND m.status = 'READY' AND m."storageKey" LIKE lower($2) || '/%'
        ORDER BY m."isCover" DESC, m."sortOrder", m."createdAt", m.id LIMIT $3 OFFSET $4"#)
        .bind(&id).bind(city.code()).bind(page.limit()).bind(page.offset()).fetch_all(pool).await?;
    for image in &mut images {
        if !key_in_scopes(&image.storage_key, &[Scope::for_city(city)]) {
            return Err(kernel::Error::internal("template media key is outside its city scope"));
        }
        image.url = crate::properties::media::image_url(&kernel.storage, &image.storage_key).await?;
    }
    Ok(Some(UnitType { id, name_en, name_fr, images: Page::new(images, count, page) }))
}
