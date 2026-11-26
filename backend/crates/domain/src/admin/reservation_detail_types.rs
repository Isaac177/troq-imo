use chrono::{DateTime, NaiveDate, Utc};
use kernel::Page;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct DetailPage { pub page: Option<u32>, pub page_size: Option<u32> }

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Core {
    pub id: String, pub reference: String, pub state: String, pub booking_mode: String,
    pub source: String, pub city: String, pub version: i32, pub operator_id: String,
    pub operator_name: String, pub property_id: String, pub property_name_en: String,
    pub property_name_fr: String, pub total_amount: String, pub commission_rate_bps: i32,
    pub commission_amount: String, pub operator_net_amount: String, pub commission_held: String,
    pub commission_taken: String, pub commission_refunded: String, pub payment_ref: Option<String>,
    pub approval_deadline_at: Option<DateTime<Utc>>, pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>, pub cancellation_charge_amount: Option<String>,
    pub special_request: Option<String>, pub created_at: DateTime<Utc>, pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub contract: Value,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String, pub unit_type_id: String, pub unit_type_name_en: String, pub unit_type_name_fr: String,
    #[sqlx(skip)] pub unit_type_cover_url: Option<String>, pub unit_code: Option<String>,
    pub check_in: NaiveDate, pub check_out: NaiveDate, pub quantity: i32, pub adult_count: i32,
    pub child_count: i32, pub subtotal_amount: String,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Guest { pub item_id: String, pub full_name: String, pub contact_phone: Option<String>, pub contact_email: Option<String>, pub role: String }

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Timeline { pub id: String, pub action: String, pub reason: Option<String>, pub created_at: DateTime<Utc> }

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Amendment { pub id: String, pub state: String, pub delta_amount: String, pub expires_at: Option<DateTime<Utc>>, pub created_at: DateTime<Utc>, pub responded_at: Option<DateTime<Utc>> }

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Case { pub id: String, pub case_type: String, pub state: String, pub deadline_at: DateTime<Utc> }

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String, pub alt_en: String, pub alt_fr: String, pub is_cover: bool,
    #[serde(skip)] pub storage_key: String,
    #[sqlx(skip)] pub url: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UnitType { pub id: String, pub name_en: String, pub name_fr: String, pub images: Page<Image> }

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    #[serde(flatten)] pub core: Core,
    pub payment_state: String, pub cancellation_deadline: Option<DateTime<Utc>>,
    pub items: Page<Item>, pub unit_type: Option<UnitType>, pub guests: Page<Guest>,
    pub timeline: Page<Timeline>, pub amendments: Page<Amendment>, pub cases: Page<Case>,
}
