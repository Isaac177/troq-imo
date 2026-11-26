use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use super::format::{cdf, date_ddmmyyyy, require_cdf};
use super::kind::{DocumentKind, Locale};
use super::payload_common::{first_str, guest_name, property_name, put, required_str, view_base};

#[derive(Debug, Clone)]
pub struct DamageClaimPayload {
    pub case_id: String,
    pub reference: String,
    pub check_out: String,
    pub dispute_deadline: String,
    pub reason: String,
    pub guest_name: String,
    pub property_name: String,
    pub property_address: Option<String>,
    pub amount: f64,
    pub locale: Locale,
}

impl DamageClaimPayload {
    pub fn from_event(value: &Value, locale: Locale) -> Result<Option<Self>> {
        if required_str(value, "state")? != "CHECKED_OUT" {
            return Ok(None);
        }
        require_cdf(&required_str(value, "currency")?)?;
        let amount: f64 = required_str(value, "claimAmount")?.parse().context("claimAmount")?;
        Ok(Some(Self {
            case_id: required_str(value, "caseId")?,
            reference: required_str(value, "reference")?,
            check_out: required_str(value, "checkOut")?,
            dispute_deadline: required_str(value, "disputeDeadline")?,
            reason: required_str(value, "claimReason")?,
            guest_name: guest_name(value)?,
            property_name: property_name(value, locale)?,
            property_address: first_str(value, &["propertyAddress"]),
            amount,
            locale,
        }))
    }

    pub fn required_values(&self) -> Vec<String> {
        vec![self.reference.clone(), self.reason.clone(), cdf(self.amount)]
    }

    pub fn view(&self, root: &Path, brand: &str) -> Result<Value> {
        let mut base = view_base(root, DocumentKind::StayDamageClaim, self.locale, brand)?;
        let view = &mut base.view;
        put(view, "reference", &self.reference);
        put(view, "caseId", &self.case_id);
        put(view, "guestName", &self.guest_name);
        put(view, "propertyName", &self.property_name);
        put(view, "propertyAddress", self.property_address.clone().unwrap_or(base.missing));
        put(view, "checkOutDisplay", date_ddmmyyyy(&self.check_out).context("stay check-out")?);
        put(view, "disputeDeadlineDisplay", date_ddmmyyyy(&self.dispute_deadline).context("dispute deadline")?);
        put(view, "claimReason", &self.reason);
        put(view, "amountDisplay", cdf(self.amount));
        Ok(Value::Object(base.view))
    }
}
