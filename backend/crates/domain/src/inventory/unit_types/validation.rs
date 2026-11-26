use kernel::error::{Error, Result};
use serde_json::json;

use crate::inventory::messages::{
    UNIT_TYPE_ACCESSIBILITY_INVALID, UNIT_TYPE_CAPACITY_INVALID, UNIT_TYPE_MODEL_INVALID,
    UNIT_TYPE_NAME_INVALID, UNIT_TYPE_QUANTITY_INVALID,
};

const ACCESSIBILITY_CODES: [&str; 5] = [
    "STEP_FREE_ENTRY",
    "GROUND_FLOOR",
    "ACCESSIBLE_BATHROOM",
    "WIDE_DOORWAYS",
    "HEARING_LOOP",
];
const INVENTORY_MODELS: [&str; 2] = ["WHOLE_UNIT", "POOLED_ROOM"];
const NAME_LIMIT: usize = 200;
const MAX_QUANTITY: i32 = 500;

pub struct UnitTypeFields<'a> {
    pub name_en: &'a str,
    pub name_fr: &'a str,
    pub max_adults: i32,
    pub max_children: i32,
    pub bedroom_count: i32,
    pub bed_count: i32,
    pub bathroom_count: i32,
    pub sellable_quantity: i32,
    pub accessibility: &'a [String],
}

pub fn validate_inventory_model(model: &str) -> Result<()> {
    if INVENTORY_MODELS.contains(&model) {
        Ok(())
    } else {
        Err(Error::bad_request(UNIT_TYPE_MODEL_INVALID).with_params(json!({ "allowed": INVENTORY_MODELS })))
    }
}

pub fn validate_unit_type(fields: &UnitTypeFields<'_>) -> Result<()> {
    if fields.name_en.trim().is_empty()
        || fields.name_fr.trim().is_empty()
        || fields.name_en.len() > NAME_LIMIT
        || fields.name_fr.len() > NAME_LIMIT
    {
        return Err(Error::bad_request(UNIT_TYPE_NAME_INVALID).with_params(json!({ "maxLength": NAME_LIMIT })));
    }
    if fields.max_adults < 1
        || fields.max_children < 0
        || fields.bedroom_count < 0
        || fields.bed_count < 0
        || fields.bathroom_count < 0
    {
        return Err(Error::bad_request(UNIT_TYPE_CAPACITY_INVALID));
    }
    if !(1..=MAX_QUANTITY).contains(&fields.sellable_quantity) {
        return Err(Error::bad_request(UNIT_TYPE_QUANTITY_INVALID)
            .with_params(json!({ "min": 1, "max": MAX_QUANTITY })));
    }
    if fields.accessibility.iter().any(|code| !ACCESSIBILITY_CODES.contains(&code.as_str())) {
        return Err(Error::bad_request(UNIT_TYPE_ACCESSIBILITY_INVALID));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields<'a>(accessibility: &'a [String]) -> UnitTypeFields<'a> {
        UnitTypeFields {
            name_en: "Lake room",
            name_fr: "Chambre lac",
            max_adults: 2,
            max_children: 1,
            bedroom_count: 1,
            bed_count: 1,
            bathroom_count: 1,
            sellable_quantity: 3,
            accessibility,
        }
    }

    #[test]
    fn a_complete_unit_type_is_valid() {
        assert!(validate_unit_type(&fields(&["GROUND_FLOOR".to_string()])).is_ok());
    }

    #[test]
    fn names_are_required_in_both_languages() {
        let mut input = fields(&[]);
        input.name_fr = "  ";
        assert!(validate_unit_type(&input).is_err());
    }

    #[test]
    fn capacities_and_quantity_are_bounded() {
        let mut input = fields(&[]);
        input.max_adults = 0;
        assert!(validate_unit_type(&input).is_err());
        let mut input = fields(&[]);
        input.sellable_quantity = 501;
        assert!(validate_unit_type(&input).is_err());
        let mut input = fields(&[]);
        input.sellable_quantity = 0;
        assert!(validate_unit_type(&input).is_err());
    }

    #[test]
    fn unknown_accessibility_codes_are_rejected() {
        assert!(validate_unit_type(&fields(&["JACUZZI".to_string()])).is_err());
    }

    #[test]
    fn inventory_model_is_limited_to_two_values() {
        assert!(validate_inventory_model("WHOLE_UNIT").is_ok());
        assert!(validate_inventory_model("POOLED_ROOM").is_ok());
        assert!(validate_inventory_model("DORM").is_err());
    }
}
