use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::paging::{Page, PageRequest};
use serde_json::json;
use sqlx::PgPool;

use super::types::QueueItem;
use crate::properties::facts::property_facts_sql;
use crate::properties::messages::QUEUE_STATUS_INVALID;

pub const QUEUE_STATUSES: [&str; 4] = ["SUBMITTED", "CHANGES_REQUESTED", "APPROVED", "REJECTED"];
pub const DEFAULT_QUEUE_STATUS: &str = "SUBMITTED";

const CHANGED_FIELDS_SQL: &str = r#"
    ((r.snapshot->>'nameEn' IS DISTINCT FROM p."nameEn")::int
   + (r.snapshot->>'nameFr' IS DISTINCT FROM p."nameFr")::int
   + (r.snapshot->>'descriptionEn' IS DISTINCT FROM p."descriptionEn")::int
   + (r.snapshot->>'descriptionFr' IS DISTINCT FROM p."descriptionFr")::int
   + (r.snapshot->>'neighborhood' IS DISTINCT FROM p.neighborhood)::int
   + (r.snapshot->>'addressLine' IS DISTINCT FROM p."addressLine")::int
   + ((r.snapshot->>'latitude')::numeric IS DISTINCT FROM p.latitude)::int
   + ((r.snapshot->>'longitude')::numeric IS DISTINCT FROM p.longitude)::int
   + (r.snapshot->>'checkInFrom' IS DISTINCT FROM p."checkInFrom")::int
   + (r.snapshot->>'checkInUntil' IS DISTINCT FROM p."checkInUntil")::int
   + (r.snapshot->>'checkOutBy' IS DISTINCT FROM p."checkOutBy")::int
   + (r.snapshot->>'houseRulesEn' IS DISTINCT FROM p."houseRulesEn")::int
   + (r.snapshot->>'houseRulesFr' IS DISTINCT FROM p."houseRulesFr")::int
   + (r.snapshot->>'petsPolicy' IS DISTINCT FROM p."petsPolicy")::int
   + (r.snapshot->>'smokingPolicy' IS DISTINCT FROM p."smokingPolicy")::int
   + (COALESCE(r.snapshot->>'guestContactPhone', '') IS DISTINCT FROM p."guestContactPhone")::int
   + (COALESCE(r.snapshot->>'guestContactEmail', '') IS DISTINCT FROM p."guestContactEmail")::int
   + (r.snapshot->'amenities' IS DISTINCT FROM p.amenities)::int)::bigint"#;

pub fn queue_status(value: Option<String>) -> Result<String> {
    let status = value.unwrap_or_else(|| DEFAULT_QUEUE_STATUS.to_string());
    if QUEUE_STATUSES.contains(&status.as_str()) {
        Ok(status)
    } else {
        Err(Error::bad_request(QUEUE_STATUS_INVALID).with_params(json!({ "allowed": QUEUE_STATUSES })))
    }
}

pub async fn queue(pool: &PgPool, city: City, status: &str, page: PageRequest) -> Result<Page<QueueItem>> {
    let total: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_property_revisions r JOIN stay_properties p ON p.id = r."propertyId"
           WHERE r."reviewStatus" = $1 AND p.city_id = $2"#,
    )
    .bind(status)
    .bind(city.code())
    .fetch_one(pool)
    .await?;
    let sql = format!(
        r#"SELECT r.id AS revision_id, r."propertyId" AS property_id, p."nameEn" AS property_name_en,
             p."operatorId" AS host_id, h.display_name AS host_name, p.city_id AS city, p.neighborhood,
             r."revisionNumber" AS revision_number, r."reviewStatus" AS review_status,
             r."submittedAt" AS submitted_at, (p."publishedRevision" = 0) AS is_first_submission,
             {CHANGED_FIELDS_SQL} AS changed_fields,
             {facts}
           FROM stay_property_revisions r
           JOIN stay_properties p ON p.id = r."propertyId"
           JOIN hosts h ON h.id = p."operatorId"
           WHERE r."reviewStatus" = $1 AND p.city_id = $2
           ORDER BY r."submittedAt" ASC NULLS LAST, r.id
           LIMIT $3 OFFSET $4"#,
        facts = property_facts_sql("p")
    );
    let items = sqlx::query_as::<_, QueueItem>(&sql)
        .bind(status)
        .bind(city.code())
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(pool)
        .await?;
    Ok(Page::new(items, total, page))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_defaults_to_submitted_and_rejects_unknown_statuses() {
        assert_eq!(queue_status(None).unwrap(), "SUBMITTED");
        assert_eq!(queue_status(Some("REJECTED".into())).unwrap(), "REJECTED");
        assert!(queue_status(Some("DRAFT".into())).is_err());
    }
}
