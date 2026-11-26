use kernel::error::{Error, Result};
use kernel::money::Money;
use serde_json::json;

use super::types::TemplateInput;
use crate::inventory::unit_types::{validate_inventory_model, validate_unit_type, UnitTypeFields};
use crate::properties::messages::{TEMPLATE_AMOUNT_INVALID, TEMPLATE_STATUS_INVALID};

const MIN_AMOUNT_MINOR: i64 = 100;
const MAX_AMOUNT_MINOR: i64 = 99_999_999_900;

pub const TEMPLATE_STATUSES: [&str; 2] = ["ACTIVE", "ARCHIVED"];

pub fn amount(value: &str) -> Result<Money> {
    let money = Money::parse_input(value)?;
    if !(MIN_AMOUNT_MINOR..=MAX_AMOUNT_MINOR).contains(&money.minor()) {
        return Err(Error::bad_request(TEMPLATE_AMOUNT_INVALID).with_params(json!({ "min": 1, "max": 999_999_999 })));
    }
    Ok(money)
}

pub fn status(value: &str) -> Result<()> {
    if TEMPLATE_STATUSES.contains(&value) {
        Ok(())
    } else {
        Err(Error::bad_request(TEMPLATE_STATUS_INVALID).with_params(json!({ "allowed": TEMPLATE_STATUSES })))
    }
}

pub fn validate(input: &TemplateInput) -> Result<Money> {
    let money = amount(&input.base_nightly_amount)?;
    validate_inventory_model(&input.inventory_model)?;
    validate_unit_type(&UnitTypeFields {
        name_en: &input.name_en,
        name_fr: &input.name_fr,
        max_adults: input.max_adults,
        max_children: input.max_children,
        bedroom_count: input.bedroom_count,
        bed_count: input.bed_count,
        bathroom_count: input.bathroom_count,
        sellable_quantity: 1,
        accessibility: &input.accessibility,
    })?;
    Ok(money)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amounts_stay_within_the_legacy_whole_franc_range() {
        assert!(amount("95000").is_ok());
        assert!(amount("0.50").is_err());
        assert!(amount("1000000000").is_err());
        assert!(amount("abc").is_err());
    }

    #[test]
    fn status_is_active_or_archived() {
        assert!(status("ACTIVE").is_ok());
        assert!(status("DELETED").is_err());
    }
}
