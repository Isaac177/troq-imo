use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::reservations::support::parse_minor;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NightLine {
    pub date: NaiveDate,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiscountView {
    pub promotion_id: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommissionSnap {
    pub rate_bps: i32,
    pub amount: String,
    pub net: String,
}

impl CommissionSnap {
    pub fn without_commission(total: &str) -> kernel::Result<Self> {
        let total = parse_minor(total)?;
        Ok(Self {
            rate_bps: 0,
            amount: crate::reservations::support::wire_minor(0),
            net: crate::reservations::support::wire_minor(total),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredBreakdown {
    pub unit_type_id: String,
    pub rate_plan_id: String,
    pub nights: Vec<NightLine>,
    pub policies: Value,
    #[serde(default)]
    pub subtotal: Option<String>,
    #[serde(default)]
    pub discount: Option<DiscountView>,
    #[serde(default)]
    pub cancellation_deadline: Option<DateTime<Utc>>,
    #[serde(default)]
    pub commission: Option<CommissionSnap>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractSnapshot {
    #[serde(default)]
    pub policies: Vec<PolicyEntry>,
    #[serde(default)]
    pub cancellation_deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PolicyEntry {
    pub kind: String,
    pub content: Value,
}

pub fn nights_from(value: Value) -> kernel::Result<Vec<NightLine>> {
    if value.is_null() {
        return Ok(Vec::new());
    }
    serde_json::from_value(value).map_err(|error| kernel::Error::internal(format!("nightly charges unreadable: {error}")))
}

pub fn discount_from(value: Option<Value>) -> kernel::Result<Option<DiscountView>> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(value) => serde_json::from_value(value)
            .map(Some)
            .map_err(|error| kernel::Error::internal(format!("discount unreadable: {error}"))),
    }
}
