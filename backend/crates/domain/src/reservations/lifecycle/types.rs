use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::reservations::contract::{DiscountView, NightLine};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GuestInput {
    pub full_name: String,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HoldRequest {
    pub quote_id: String,
    pub guest: GuestInput,
    #[serde(default)]
    pub accepted_policy_version_ids: Vec<String>,
    #[serde(default)]
    pub special_request: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GuestView {
    pub full_name: String,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReservationView {
    pub id: String,
    pub reference: String,
    pub state: String,
    pub property_id: String,
    pub unit_type_id: String,
    pub unit_type_cover_url: Option<String>,
    pub rate_plan_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    pub child_count: i32,
    pub currency: String,
    pub total: String,
    pub nights: Vec<NightLine>,
    pub guest: GuestView,
    pub hold_expires_at: Option<DateTime<Utc>>,
    pub payment_required: bool,
    pub approval_deadline_at: Option<DateTime<Utc>>,
    pub version: i32,
    pub special_request: Option<String>,
    pub special_request_state: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VersionedAction {
    pub expected_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DecisionInput {
    pub action: String,
    pub expected_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpecialRequestDecisionInput {
    pub decision: String,
    pub expected_version: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancelStayInput {
    pub expected_version: i32,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancelledStayView {
    pub reservation_id: String,
    pub state: String,
    pub cancelled_at: DateTime<Utc>,
    pub reason: String,
    pub currency: String,
    pub refund: String,
    pub charge_due_at_property: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancellationView {
    pub reservation_id: String,
    pub currency: String,
    pub total: String,
    pub deadline: DateTime<Utc>,
    pub evaluated_at: DateTime<Utc>,
    pub reason: String,
    pub refund: String,
    pub penalty: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OperatorCancelInput {
    pub expected_version: i32,
    pub reason: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlatformCancelInput {
    pub expected_version: i32,
    pub reason_code: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PendingAmendmentView {
    pub id: String,
    pub check_in: String,
    pub check_out: String,
    pub total: String,
    pub delta: String,
    pub expires_at: DateTime<Utc>,
    pub nights: Vec<NightLine>,
    pub discount: Option<DiscountView>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MineCover {
    pub url: String,
    pub alt_en: String,
    pub alt_fr: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyReservationView {
    pub id: String,
    pub reference: String,
    pub state: String,
    pub property_id: String,
    pub unit_type_id: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    pub child_count: i32,
    pub currency: String,
    pub total: String,
    pub version: i32,
    pub nights: Vec<NightLine>,
    pub discount: Option<DiscountView>,
    pub pending_amendment: Option<PendingAmendmentView>,
    pub property_name_en: String,
    pub property_name_fr: String,
    pub unit_name_en: String,
    pub unit_name_fr: String,
    pub approval_deadline_at: Option<DateTime<Utc>>,
    pub cover: Option<MineCover>,
    pub unit_type_cover_url: Option<String>,
}

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct MyReservationsQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub tab: Option<String>,
}

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct OperatorReservationQuery {
    pub host_id: String,
    pub property_id: Option<String>,
    pub state: Option<String>,
    pub search: Option<String>,
    pub locale: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OperatorReservationView {
    pub id: String,
    pub reference: String,
    pub state: String,
    pub property_id: String,
    pub property_name: String,
    pub unit_type_id: String,
    pub unit_type_name: String,
    pub unit_type_cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Object>)]
    pub unit_type: Option<crate::reservations::links::properties::UnitTypeSummary>,
    pub item_id: String,
    pub assigned_unit_code: Option<String>,
    pub assignment_id: Option<String>,
    pub assignment_start: Option<NaiveDate>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    pub child_count: i32,
    pub currency: String,
    pub total: String,
    pub commission_rate_bps: i32,
    pub commission_amount: String,
    pub net_amount: String,
    pub version: i32,
    pub approval_deadline_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub allowed_actions: Vec<String>,
    pub special_request: Option<String>,
    pub special_request_state: String,
    pub nights: Vec<NightLine>,
    pub guest: Option<GuestView>,
}

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct HostLocaleQuery {
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancelPreview {
    pub reservation_id: String,
    pub state: String,
    pub version: i32,
    pub can_cancel: bool,
    pub total: String,
    pub refund: String,
    pub policy_reason: Option<String>,
    pub policy_refund: Option<String>,
    pub policy_penalty: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
}
