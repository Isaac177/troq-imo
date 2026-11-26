use kernel::error::{Error, Result};
use sqlx::PgConnection;

use crate::properties::revisions::content::ListingContent;

pub async fn apply_content(
    conn: &mut PgConnection,
    property_id: &str,
    content: &ListingContent,
    published_revision: Option<i32>,
) -> Result<()> {
    let amenities = serde_json::to_value(&content.amenities).map_err(|error| Error::internal(error.to_string()))?;
    sqlx::query(
        r#"UPDATE stay_properties SET "nameEn" = $2, "nameFr" = $3, "descriptionEn" = $4, "descriptionFr" = $5,
           neighborhood = $6, "addressLine" = $7, latitude = $8::float8::numeric, longitude = $9::float8::numeric,
           "checkInFrom" = $10, "checkInUntil" = $11, "checkOutBy" = $12, "houseRulesEn" = $13,
           "houseRulesFr" = $14, "petsPolicy" = $15, "smokingPolicy" = $16, "guestContactPhone" = $17,
           "guestContactEmail" = $18, amenities = $19,
           status = CASE WHEN $20::int IS NOT NULL AND status = 'DRAFT' THEN 'PUBLISHED' ELSE status END,
           "publishedRevision" = COALESCE($20, "publishedRevision"), version = version + 1, "updatedAt" = now()
           WHERE id = $1"#,
    )
    .bind(property_id)
    .bind(&content.name_en)
    .bind(&content.name_fr)
    .bind(&content.description_en)
    .bind(&content.description_fr)
    .bind(&content.neighborhood)
    .bind(&content.address_line)
    .bind(content.latitude)
    .bind(content.longitude)
    .bind(&content.check_in_from)
    .bind(&content.check_in_until)
    .bind(&content.check_out_by)
    .bind(&content.house_rules_en)
    .bind(&content.house_rules_fr)
    .bind(&content.pets_policy)
    .bind(&content.smoking_policy)
    .bind(&content.guest_contact_phone)
    .bind(&content.guest_contact_email)
    .bind(amenities)
    .bind(published_revision)
    .execute(&mut *conn)
    .await?;
    Ok(())
}
