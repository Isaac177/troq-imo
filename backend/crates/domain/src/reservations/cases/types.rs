use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenCaseInput {
    pub case_type: String,
    pub reservation_id: Option<String>,
    pub host_id: Option<String>,
    pub reason: String,
    pub claim_amount: Option<String>,
}

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct CaseListQuery {
    pub host_id: String,
    pub state: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, Default, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct MyCasesQuery {
    pub state: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CloseCaseInput {
    pub resolution: String,
    pub expected_version: i32,
}

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CaseView {
    pub id: String,
    pub operator_id: String,
    pub reservation_id: Option<String>,
    pub case_type: String,
    pub reason: String,
    #[schema(value_type = Object)]
    pub parties: Value,
    pub state: String,
    pub deadline_at: DateTime<Utc>,
    #[schema(value_type = Option<Object>)]
    pub resolution: Option<Value>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub reservation_reference: Option<String>,
}
