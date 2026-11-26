use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use super::format::{cdf, date_ddmmyyyy, require_cdf};
use super::kind::{DocumentKind, Locale};
use super::payload_common::{first_str, guest_name, property_name, put, required_str, view_base};

#[derive(Debug, Clone)]
pub struct VoucherPayload {
    pub reservation_id: String,
    pub reference: String,
    pub check_in: String,
    pub check_out: String,
    pub guest_name: String,
    pub property_name: String,
    pub property_address: Option<String>,
    pub total: f64,
    pub locale: Locale,
}

impl VoucherPayload {
    pub fn from_snapshot(value: &Value, locale: Locale) -> Result<Option<Self>> {
        if required_str(value, "state")? != "CONFIRMED" {
            return Ok(None);
        }
        require_cdf(&required_str(value, "currency")?)?;
        let total: f64 = required_str(value, "totalAmount")?.parse().context("stay totalAmount")?;
        Ok(Some(Self {
            reservation_id: required_str(value, "reservationId")?,
            reference: required_str(value, "reference")?,
            check_in: required_str(value, "checkIn")?,
            check_out: required_str(value, "checkOut")?,
            guest_name: guest_name(value)?,
            property_name: property_name(value, locale)?,
            property_address: first_str(value, &["propertyAddress"]),
            total,
            locale,
        }))
    }

    pub fn required_values(&self) -> Vec<String> {
        vec![self.reference.clone(), self.property_name.clone(), cdf(self.total)]
    }

    pub fn view(&self, root: &Path, brand: &str) -> Result<Value> {
        if self.total < 0.0 {
            anyhow::bail!("stay total is negative");
        }
        let mut base = view_base(root, DocumentKind::StayVoucher, self.locale, brand)?;
        let view = &mut base.view;
        put(view, "reference", &self.reference);
        put(view, "guestName", &self.guest_name);
        put(view, "propertyName", &self.property_name);
        put(view, "propertyAddress", self.property_address.clone().unwrap_or(base.missing));
        put(view, "checkInDisplay", date_ddmmyyyy(&self.check_in).context("stay check-in")?);
        put(view, "checkOutDisplay", date_ddmmyyyy(&self.check_out).context("stay check-out")?);
        put(view, "totalDisplay", cdf(self.total));
        Ok(Value::Object(base.view))
    }
}
