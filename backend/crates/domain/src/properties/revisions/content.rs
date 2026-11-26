use kernel::error::{Error, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::properties::messages::{CHECK_IN_WINDOW_INVALID, REVISION_SNAPSHOT_UNREADABLE};
use crate::properties::property::types::PropertyView;
use crate::properties::validation::{self, MAX_ADDRESS_LENGTH, MAX_DESCRIPTION_LENGTH, MAX_HOUSE_RULES_LENGTH, MAX_NAME_LENGTH, MAX_NEIGHBORHOOD_LENGTH};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListingContent {
    pub name_en: String,
    pub name_fr: String,
    pub description_en: String,
    pub description_fr: String,
    pub neighborhood: String,
    pub address_line: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub check_in_from: Option<String>,
    pub check_in_until: Option<String>,
    pub check_out_by: Option<String>,
    pub house_rules_en: String,
    pub house_rules_fr: String,
    pub pets_policy: Option<String>,
    pub smoking_policy: Option<String>,
    pub guest_contact_phone: String,
    pub guest_contact_email: String,
    pub amenities: Vec<String>,
}

impl ListingContent {
    pub fn validated(self) -> Result<Self> {
        let coordinates = validation::coordinates(self.latitude, self.longitude)?;
        let check_in_from = validation::clock("checkInFrom", self.check_in_from)?;
        let check_in_until = validation::clock("checkInUntil", self.check_in_until)?;
        if let (Some(from), Some(until)) = (&check_in_from, &check_in_until) {
            if until <= from { return Err(Error::bad_request(CHECK_IN_WINDOW_INVALID)); }
        }
        Ok(Self {
            name_en: validation::required_text("nameEn", &self.name_en, MAX_NAME_LENGTH)?,
            name_fr: validation::required_text("nameFr", &self.name_fr, MAX_NAME_LENGTH)?,
            description_en: validation::bounded("descriptionEn", &self.description_en, MAX_DESCRIPTION_LENGTH)?,
            description_fr: validation::bounded("descriptionFr", &self.description_fr, MAX_DESCRIPTION_LENGTH)?,
            neighborhood: validation::bounded("neighborhood", &self.neighborhood, MAX_NEIGHBORHOOD_LENGTH)?,
            address_line: validation::bounded("addressLine", &self.address_line, MAX_ADDRESS_LENGTH)?,
            latitude: coordinates.map(|value| value.0),
            longitude: coordinates.map(|value| value.1),
            check_in_from,
            check_in_until,
            check_out_by: validation::clock("checkOutBy", self.check_out_by)?,
            house_rules_en: validation::bounded("houseRulesEn", &self.house_rules_en, MAX_HOUSE_RULES_LENGTH)?,
            house_rules_fr: validation::bounded("houseRulesFr", &self.house_rules_fr, MAX_HOUSE_RULES_LENGTH)?,
            pets_policy: validation::choice("petsPolicy", self.pets_policy, &["NOT_ALLOWED", "ALLOWED", "ON_REQUEST"])?,
            smoking_policy: validation::choice("smokingPolicy", self.smoking_policy, &["NOT_ALLOWED", "ALLOWED"])?,
            guest_contact_phone: validation::phone(&self.guest_contact_phone)?,
            guest_contact_email: validation::email(&self.guest_contact_email)?,
            amenities: validation::amenity_codes(self.amenities)?,
        })
    }

    pub fn from_snapshot(snapshot: &serde_json::Value) -> Result<Self> {
        serde_json::from_value(snapshot.clone())
            .map_err(|_| Error::internal(REVISION_SNAPSHOT_UNREADABLE.as_str()))
    }

    pub fn from_property(property: &PropertyView) -> Result<Self> {
        let amenities = serde_json::from_value(property.amenities.clone())
            .map_err(|_| Error::internal(REVISION_SNAPSHOT_UNREADABLE.as_str()))?;
        Ok(Self {
            name_en: property.name_en.clone(), name_fr: property.name_fr.clone(),
            description_en: property.description_en.clone(), description_fr: property.description_fr.clone(),
            neighborhood: property.neighborhood.clone(), address_line: property.address_line.clone(),
            latitude: property.latitude, longitude: property.longitude,
            check_in_from: property.check_in_from.clone(), check_in_until: property.check_in_until.clone(),
            check_out_by: property.check_out_by.clone(), house_rules_en: property.house_rules_en.clone(),
            house_rules_fr: property.house_rules_fr.clone(), pets_policy: property.pets_policy.clone(),
            smoking_policy: property.smoking_policy.clone(), guest_contact_phone: property.guest_contact_phone.clone(),
            guest_contact_email: property.guest_contact_email.clone(), amenities,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn content() -> ListingContent {
        ListingContent {
            name_en: " Lakeside Villa ".into(), name_fr: "Villa du Lac".into(),
            description_en: String::new(), description_fr: String::new(),
            neighborhood: "Lakeshore".into(), address_line: "Avenue du Lac".into(),
            latitude: Some(-1.6792), longitude: Some(29.2228),
            check_in_from: Some("9:05".into()), check_in_until: Some("20:00".into()),
            check_out_by: Some("11:00".into()), house_rules_en: String::new(), house_rules_fr: String::new(),
            pets_policy: Some("ALLOWED".into()), smoking_policy: None, guest_contact_phone: "+243 800 000 001".into(),
            guest_contact_email: "stays@example.com".into(), amenities: vec!["WIFI".into(), " WIFI ".into()],
        }
    }

    #[test]
    fn trims_content_and_normalizes_time_and_amenities() {
        let valid = content().validated().unwrap();
        assert_eq!(valid.name_en, "Lakeside Villa");
        assert_eq!(valid.check_in_from.as_deref(), Some("09:05"));
        assert_eq!(valid.amenities, vec!["WIFI"]);
    }

    #[test]
    fn rejects_invalid_time_window_and_coordinates() {
        let mut invalid = content();
        invalid.check_in_until = Some("08:00".into());
        assert_eq!(invalid.validated().unwrap_err().code().as_str(), "CHECK_IN_WINDOW_INVALID");
        let mut invalid = content();
        invalid.longitude = None;
        assert!(invalid.validated().is_err());
    }
}
