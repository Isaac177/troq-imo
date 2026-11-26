use chrono::{DateTime, NaiveDate, Utc};
use kernel::{Error, Money, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use super::schedule::PaymentSchedule;
use crate::pricing::messages::{GUEST_COUNT_INVALID, STAY_DATES_INVALID, STAY_TOO_LONG};

pub const MAX_STAY_NIGHTS: i64 = 365;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    pub property_id: String,
    pub unit_type_id: String,
    pub rate_plan_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    #[serde(default)]
    pub child_count: i32,
}

impl QuoteRequest {
    pub fn validated(self) -> Result<Self> {
        let nights = (self.check_out - self.check_in).num_days();
        if nights < 1 {
            return Err(Error::bad_request(STAY_DATES_INVALID));
        }
        if nights > MAX_STAY_NIGHTS {
            return Err(Error::bad_request(STAY_TOO_LONG)
                .with_params(serde_json::json!({ "max": MAX_STAY_NIGHTS })));
        }
        if self.adult_count < 1 || self.child_count < 0 {
            return Err(Error::bad_request(GUEST_COUNT_INVALID));
        }
        Ok(self)
    }

    pub fn nights(&self) -> Vec<NaiveDate> {
        self.check_in
            .iter_days()
            .take_while(|day| *day < self.check_out)
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NightLine {
    pub date: NaiveDate,
    #[schema(value_type = String)]
    pub amount: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiscountView {
    pub promotion_id: String,
    #[schema(value_type = String)]
    pub amount: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommissionQuote {
    pub rate_bps: i32,
    #[schema(value_type = String)]
    pub amount: Money,
    #[schema(value_type = String)]
    pub net: Money,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredBreakdown {
    pub unit_type_id: String,
    pub rate_plan_id: String,
    pub nights: Vec<NightLine>,
    pub policies: Value,
    #[serde(default)]
    pub subtotal: Option<Money>,
    #[serde(default)]
    pub discount: Option<DiscountView>,
    #[serde(default)]
    pub cancellation_deadline: Option<DateTime<Utc>>,
    #[serde(default)]
    pub commission: Option<CommissionQuote>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QuoteView {
    pub id: String,
    pub property_id: String,
    pub unit_type_id: String,
    pub unit_type_cover_url: Option<String>,
    pub rate_plan_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    pub child_count: i32,
    pub currency: &'static str,
    #[schema(value_type = Option<String>)]
    pub subtotal: Option<Money>,
    pub discount: Option<DiscountView>,
    #[schema(value_type = String)]
    pub total: Money,
    pub nights: Vec<NightLine>,
    #[schema(value_type = Object)]
    pub policies: Value,
    pub cancellation_deadline: Option<DateTime<Utc>>,
    pub payment_schedule: PaymentSchedule,
    pub expires_at: DateTime<Utc>,
    pub expired: bool,
}
