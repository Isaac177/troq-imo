use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use super::facets::{Facets, Stats};
use super::query::SearchQuery;
use super::rank::SearchHit;

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Timings {
    pub candidates: f64,
    pub pricing: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub candidate_plans: usize,
    pub priced_plans: usize,
    pub timings_ms: Timings,
    pub analytics_recorded: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub engine_version: u32,
    pub fingerprint: String,
    pub query: SearchQuery,
    pub data_as_of: DateTime<Utc>,
    pub items: Vec<SearchHit>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub facets: Facets,
    pub stats: Stats,
    pub meta: Meta,
}
