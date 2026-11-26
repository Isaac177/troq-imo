use kernel::error::{Error, Result};
use serde_json::json;
use sqlx::PgConnection;

use super::queries;
use super::types::ReadinessView;
use crate::properties::messages::LISTING_NOT_READY;
use crate::properties::revisions::content::ListingContent;

pub fn content_blockers(content: &ListingContent) -> Vec<&'static str> {
    let mut blockers = Vec::new();
    let checks = [
        (content.name_en.is_empty(), "MISSING_NAME_EN"),
        (content.name_fr.is_empty(), "MISSING_NAME_FR"),
        (content.description_en.is_empty(), "MISSING_DESCRIPTION_EN"),
        (content.description_fr.is_empty(), "MISSING_DESCRIPTION_FR"),
        (content.neighborhood.is_empty(), "MISSING_NEIGHBORHOOD"),
        (content.address_line.is_empty(), "MISSING_ADDRESS"),
        (content.latitude.is_none() || content.longitude.is_none(), "MISSING_COORDINATES"),
        (content.check_in_from.is_none(), "MISSING_CHECK_IN_TIME"),
        (content.check_out_by.is_none(), "MISSING_CHECK_OUT_TIME"),
    ];
    for (missing, code) in checks {
        if missing {
            blockers.push(code);
        }
    }
    blockers
}

pub async fn blockers(
    conn: &mut PgConnection,
    host_id: &str,
    property_id: &str,
    content: &ListingContent,
) -> Result<Vec<&'static str>> {
    let mut blockers = content_blockers(content);
    if queries::host_status(&mut *conn, host_id).await?.as_deref() != Some("ACTIVE") {
        blockers.push("HOST_NOT_ACTIVE");
    }
    let facts = queries::readiness_facts(&mut *conn, property_id).await?;
    if !facts.has_sellable_unit {
        blockers.push("NO_SELLABLE_UNIT");
    }
    if !facts.has_base_rate {
        blockers.push("NO_BASE_RATE");
    }
    if !facts.has_cover_photo {
        blockers.push("NO_COVER_PHOTO");
    }
    Ok(blockers)
}

pub async fn inspect(
    conn: &mut PgConnection,
    host_id: &str,
    property_id: &str,
    content: &ListingContent,
) -> Result<ReadinessView> {
    let blockers = blockers(conn, host_id, property_id, content).await?;
    Ok(ReadinessView { ready: blockers.is_empty(), blockers: blockers.into_iter().map(String::from).collect() })
}

pub async fn ensure_ready(
    conn: &mut PgConnection,
    host_id: &str,
    property_id: &str,
    content: &ListingContent,
) -> Result<()> {
    let blockers = blockers(conn, host_id, property_id, content).await?;
    if blockers.is_empty() {
        Ok(())
    } else {
        Err(Error::conflict(LISTING_NOT_READY).with_params(json!({ "blockers": blockers })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete() -> ListingContent {
        ListingContent {
            name_en: "Lakeside".into(),
            name_fr: "Lac".into(),
            description_en: "Villa".into(),
            description_fr: "Villa".into(),
            neighborhood: "Lakeshore".into(),
            address_line: "Avenue du Lac".into(),
            latitude: Some(-1.6),
            longitude: Some(29.2),
            check_in_from: Some("15:00".into()),
            check_in_until: None,
            check_out_by: Some("11:00".into()),
            house_rules_en: String::new(),
            house_rules_fr: String::new(),
            pets_policy: None,
            smoking_policy: None,
            guest_contact_phone: String::new(),
            guest_contact_email: String::new(),
            amenities: Vec::new(),
        }
    }

    #[test]
    fn complete_content_has_no_blockers() {
        assert!(content_blockers(&complete()).is_empty());
    }

    #[test]
    fn missing_fields_are_reported_by_code_in_order() {
        let mut bare = complete();
        bare.description_fr = String::new();
        bare.check_out_by = None;
        bare.latitude = None;
        assert_eq!(
            content_blockers(&bare),
            vec!["MISSING_DESCRIPTION_FR", "MISSING_COORDINATES", "MISSING_CHECK_OUT_TIME"]
        );
    }
}
