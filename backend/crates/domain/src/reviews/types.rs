use chrono::{DateTime, Utc};
use kernel::paging::PageRequest;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub const PENDING: &str = "PENDING";
pub const APPROVED: &str = "APPROVED";
pub const HIDDEN: &str = "HIDDEN";
pub const CHECKED_OUT: &str = "CHECKED_OUT";

pub const REVIEW_STATUSES: [&str; 3] = [PENDING, APPROVED, HIDDEN];
pub const MODERATION_STATUSES: [&str; 2] = [APPROVED, HIDDEN];
pub const HIDE_REASON_CODES: [&str; 5] = ["ABUSIVE", "SPAM", "OFF_TOPIC", "PRIVACY", "OTHER"];

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReviewInput {
    pub rating: i16,
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReplyReviewInput {
    pub reply: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DecideReviewInput {
    pub status: String,
    #[serde(default)]
    pub reason_code: Option<String>,
}

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct ReviewListQuery {
    pub status: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct PublicReviewQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

pub fn page_request(page: Option<u32>, page_size: Option<u32>) -> PageRequest {
    let default = PageRequest::default();
    PageRequest {
        page: page.unwrap_or(default.page),
        page_size: page_size.unwrap_or(default.page_size),
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReviewView {
    pub id: String,
    pub property_id: String,
    pub rating: i16,
    pub comment: Option<String>,
    pub status: String,
    pub operator_reply: Option<String>,
    pub operator_replied_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub unit_type_id: Option<String>,
    pub unit_type_name_en: Option<String>,
    pub unit_type_name_fr: Option<String>,
    pub unit_type_cover_url: Option<String>,
}
