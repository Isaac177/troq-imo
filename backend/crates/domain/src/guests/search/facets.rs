use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FacetValue {
    pub value: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Facets {
    pub property_type: Vec<FacetValue>,
    pub category: Vec<FacetValue>,
    pub meal_plan: Vec<FacetValue>,
    pub payment_collection: Vec<FacetValue>,
    pub cancellation_class: Vec<FacetValue>,
    pub booking_mode: Vec<FacetValue>,
    pub accessibility: Vec<FacetValue>,
    pub amenities: Vec<FacetValue>,
    pub verification_status: Vec<FacetValue>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub currency: String,
    pub min: String,
    pub median: String,
    pub max: String,
    pub average: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DistanceStats {
    pub min_km: f64,
    pub max_km: f64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub result_count: i64,
    pub priced_count: i64,
    pub instant_booking_count: i64,
    pub offer_count: i64,
    pub price: Option<PriceStats>,
    pub distance: Option<DistanceStats>,
}
