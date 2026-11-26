use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueOtpInput {
    pub channel: String,
    pub contact: String,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VerifyOtpInput {
    pub channel: String,
    pub contact: String,
    pub code: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub channel: String,
    pub contact: String,
    pub proof: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub city_code: String,
    pub preferred_language: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProofResponse {
    pub proof: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OtpChannelResponse {
    pub channel: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CityView {
    pub code: String,
    pub name: String,
    pub timezone: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionView {
    pub name: String,
    pub domain: String,
    pub delegable: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PresetView {
    pub code: String,
    pub audience: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CatalogView {
    pub permissions: Vec<PermissionView>,
    pub presets: Vec<PresetView>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostReferenceView {
    pub kinds: Vec<String>,
    pub document_kinds: Vec<String>,
    pub approve_reasons: Vec<String>,
    pub reject_reasons: Vec<String>,
    pub suspend_reasons: Vec<String>,
    pub reinstate_reasons: Vec<String>,
    pub document_reject_reasons: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub message: String,
}
