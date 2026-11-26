use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::reservations::contract::{DiscountView, NightLine};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProposeInput {
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AmendmentProposal {
    pub id: String,
    pub reservation_id: String,
    pub state: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub nights: Vec<NightLine>,
    pub total: String,
    pub delta: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AmendmentListItem {
    pub id: String,
    pub state: String,
    pub check_in: String,
    pub check_out: String,
    pub total: String,
    pub delta: String,
    pub nights: Vec<NightLine>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct AmendmentListQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredAmendment {
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub nights: Vec<NightLine>,
    pub total: String,
    #[serde(default)]
    pub subtotal: Option<String>,
    #[serde(default)]
    pub discount: Option<DiscountView>,
    #[serde(default)]
    pub cancellation_deadline: Option<DateTime<Utc>>,
}
