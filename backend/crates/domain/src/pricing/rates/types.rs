use chrono::NaiveDate;
use kernel::Money;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

fn default_meal_plan() -> String {
    "ROOM_ONLY".into()
}

fn default_booking_mode() -> String {
    "INSTANT".into()
}

fn default_child_amount() -> String {
    "0.00".into()
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct PagingQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

impl PagingQuery {
    pub fn request(&self) -> kernel::PageRequest {
        kernel::PageRequest { page: self.page, page_size: self.page_size }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePlanInput {
    pub idempotency_key: Option<String>,
    pub unit_type_id: String,
    pub occupancy_basis: String,
    #[serde(default = "default_meal_plan")]
    pub meal_plan: String,
    #[serde(default = "default_booking_mode")]
    pub booking_mode: String,
    pub cancellation_policy_version_id: String,
    pub payment_policy_version_id: String,
    #[serde(default = "default_child_amount")]
    pub child_nightly_amount: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePlanInput {
    pub idempotency_key: Option<String>,
    pub expected_version: i32,
    pub status: Option<String>,
    pub booking_mode: Option<String>,
    pub meal_plan: Option<String>,
    pub child_nightly_amount: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRuleInput {
    pub idempotency_key: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub weekdays: Vec<u8>,
    pub adjustment_type: String,
    pub amount: String,
    pub priority: i32,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct RemoveRuleQuery {
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct PolicyQuery {
    pub kind: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlanView {
    pub id: String,
    pub unit_type_id: String,
    pub occupancy_basis: String,
    pub meal_plan: String,
    pub booking_mode: String,
    pub status: String,
    #[schema(value_type = String)]
    pub child_nightly_amount: Money,
    pub cancellation_policy_version_id: String,
    pub payment_policy_version_id: String,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RuleView {
    pub id: String,
    pub rate_plan_id: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub weekdays: Vec<u8>,
    pub adjustment_type: String,
    #[schema(value_type = String)]
    pub amount: Money,
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RemovedRule {
    pub id: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PolicyView {
    pub id: String,
    pub kind: String,
    pub version_number: i32,
    #[schema(value_type = Object)]
    pub content: serde_json::Value,
}

pub fn weekday_mask(weekdays: &[u8]) -> u8 {
    weekdays
        .iter()
        .filter(|day| (1..=7).contains(*day))
        .fold(0u8, |mask, day| mask | (1u8 << (day - 1)))
}

pub fn weekdays_from_mask(mask: u8) -> Vec<u8> {
    (1..=7u8)
        .filter(|day| mask & (1u8 << (day - 1)) != 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weekday_mask_round_trips() {
        assert_eq!(weekday_mask(&[1, 5]), 1 | (1 << 4));
        assert_eq!(weekdays_from_mask(weekday_mask(&[2, 7])), vec![2, 7]);
    }

    #[test]
    fn mask_ignores_days_out_of_range() {
        assert_eq!(weekday_mask(&[0, 8]), 0);
    }
}
