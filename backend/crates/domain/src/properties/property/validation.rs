use kernel::error::Result;

use super::types::CreatePropertyInput;
use crate::properties::validation::{
    bounded, coordinates, required_text, timezone, MAX_ADDRESS_LENGTH, MAX_DESCRIPTION_LENGTH,
    MAX_NAME_LENGTH,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedProperty {
    pub host_id: String,
    pub timezone: String,
    pub name_en: String,
    pub name_fr: String,
    pub description_en: String,
    pub description_fr: String,
    pub address_line: String,
    pub coordinates: Option<(f64, f64)>,
    pub idempotency_key: Option<String>,
}

pub fn validate_create(input: CreatePropertyInput) -> Result<ValidatedProperty> {
    Ok(ValidatedProperty {
        host_id: required_text("hostId", &input.host_id, 64)?,
        timezone: timezone(&input.timezone)?,
        name_en: required_text("nameEn", &input.name_en, MAX_NAME_LENGTH)?,
        name_fr: required_text("nameFr", &input.name_fr, MAX_NAME_LENGTH)?,
        description_en: bounded("descriptionEn", &input.description_en, MAX_DESCRIPTION_LENGTH)?,
        description_fr: bounded("descriptionFr", &input.description_fr, MAX_DESCRIPTION_LENGTH)?,
        address_line: bounded("addressLine", &input.address_line, MAX_ADDRESS_LENGTH)?,
        coordinates: coordinates(input.latitude, input.longitude)?,
        idempotency_key: crate::properties::validation::optional_text("idempotencyKey", input.idempotency_key, 128)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> CreatePropertyInput {
        CreatePropertyInput {
            host_id: " seed-host-goma-lake ".into(),
            timezone: "Africa/Lubumbashi".into(),
            name_en: " Goma House ".into(),
            name_fr: "Maison Goma".into(),
            description_en: String::new(),
            description_fr: String::new(),
            address_line: "Avenue du Lac".into(),
            latitude: Some(-1.67),
            longitude: Some(29.23),
            idempotency_key: Some("  ".into()),
        }
    }

    #[test]
    fn trims_and_accepts_valid_input() {
        let validated = validate_create(input()).unwrap();
        assert_eq!(validated.host_id, "seed-host-goma-lake");
        assert_eq!(validated.name_en, "Goma House");
        assert_eq!(validated.idempotency_key, None);
    }

    #[test]
    fn rejects_unknown_timezone_half_coordinates_and_blank_names() {
        let mut bad = input();
        bad.timezone = "Europe/Paris".into();
        assert!(validate_create(bad).is_err());
        let mut half = input();
        half.longitude = None;
        assert!(validate_create(half).is_err());
        let mut blank = input();
        blank.name_fr = "   ".into();
        assert!(validate_create(blank).is_err());
    }
}
