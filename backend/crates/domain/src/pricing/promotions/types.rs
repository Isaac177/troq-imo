use chrono::NaiveDate;
use kernel::Money;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePromotionInput {
    pub idempotency_key: Option<String>,
    pub rate_plan_id: Option<String>,
    pub min_nights: i64,
    pub discount_percent: String,
    pub cap_amount: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub priority: i32,
    pub redemption_limit: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePromotionInput {
    pub idempotency_key: Option<String>,
    pub status: Option<String>,
    pub redemption_limit: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FundingInput {
    pub funded_by: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PromotionView {
    pub id: String,
    pub rate_plan_id: Option<String>,
    pub min_nights: i64,
    #[schema(value_type = String)]
    pub discount_percent: Money,
    #[schema(value_type = Option<String>)]
    pub cap_amount: Option<Money>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub priority: i32,
    pub redemption_limit: Option<i32>,
    pub redeemed: i64,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminPromotionView {
    #[serde(flatten)]
    pub promotion: PromotionView,
    pub funded_by: String,
}
