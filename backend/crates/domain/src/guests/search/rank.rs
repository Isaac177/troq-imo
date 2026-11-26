use std::collections::BTreeMap;

use kernel::Money;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::candidates::CandidateRow;
use super::normalize::cancellation_class;
use super::pricing::Offer;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CancellationView {
    pub class: String,
    pub free_until_hours: Option<i64>,
    pub penalty_percent: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OfferView {
    pub unit_type_id: String,
    pub category_code: Option<String>,
    pub unit_name: String,
    pub rate_plan_id: String,
    pub property_type: String,
    pub booking_mode: String,
    pub meal_plan: String,
    pub occupancy_basis: String,
    pub payment_collection: String,
    pub cancellation: CancellationView,
    pub accessibility: Vec<String>,
    pub bedroom_count: i32,
    pub bed_count: i32,
    pub bathroom_count: i32,
    pub remaining_rooms: Option<i64>,
    pub total: Option<String>,
    #[serde(default)]
    pub nightly: Option<String>,
    #[serde(skip)]
    #[schema(ignore)]
    pub total_minor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CoverView {
    pub media_id: String,
    pub alt: String,
    pub content_path: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchHit {
    pub id: String,
    pub name: String,
    pub verification_status: String,
    pub cover: Option<CoverView>,
    pub review_count: i64,
    pub verified_rating: Option<f64>,
    pub distance_km: Option<f64>,
    pub amenities: Vec<String>,
    pub instant_booking: bool,
    pub best: OfferView,
    pub offers: Vec<OfferView>,
    #[serde(skip)]
    #[schema(ignore)]
    pub best_minor: Option<i64>,
}

pub fn property_type(inventory_model: &str) -> &'static str {
    if inventory_model == "WHOLE_UNIT" {
        "ENTIRE_PLACE"
    } else {
        "ROOM"
    }
}

fn offer_view(offer: &Offer) -> OfferView {
    let row: &CandidateRow = &offer.row;
    let free_hours = row.cancellation_content.get("freeUntilHours").and_then(serde_json::Value::as_i64);
    let penalty = row.cancellation_content.get("penaltyPercent").and_then(serde_json::Value::as_i64);
    OfferView {
        unit_type_id: row.unit_type_id.clone(),
        category_code: row.category_code.clone(),
        unit_name: row.unit_name.clone(),
        rate_plan_id: row.plan_id.clone(),
        property_type: property_type(&row.inventory_model).to_string(),
        booking_mode: row.booking_mode.clone(),
        meal_plan: row.meal_plan.clone(),
        occupancy_basis: row.occupancy_basis.clone(),
        payment_collection: row
            .payment_content
            .get("collection")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("UNKNOWN")
            .to_string(),
        cancellation: CancellationView {
            class: cancellation_class(free_hours, penalty).to_string(),
            free_until_hours: free_hours,
            penalty_percent: penalty,
        },
        accessibility: string_list(&row.accessibility),
        bedroom_count: row.bedroom_count,
        bed_count: row.bed_count,
        bathroom_count: row.bathroom_count,
        remaining_rooms: row.remaining_rooms,
        total: offer.total_minor.map(|minor| Money::from_minor(minor).to_wire()),
        nightly: offer.nightly_minor.map(|minor| Money::from_minor(minor).to_wire()),
        total_minor: offer.total_minor,
    }
}

pub fn string_list(value: &serde_json::Value) -> Vec<String> {
    let mut items: Vec<String> = value
        .as_array()
        .map(|items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
        .unwrap_or_default();
    items.sort();
    items.dedup();
    items
}

pub fn build_hits(offers: Vec<Offer>) -> Vec<SearchHit> {
    let mut grouped: BTreeMap<String, Vec<Offer>> = BTreeMap::new();
    for offer in offers {
        grouped.entry(offer.row.property_id.clone()).or_default().push(offer);
    }
    grouped
        .into_values()
        .filter_map(|group| {
            let first = group.first()?.row.clone();
            let mut views: Vec<OfferView> = group.iter().map(offer_view).collect();
            views.sort_by(|left, right| {
                left.total_minor
                    .unwrap_or(i64::MAX)
                    .cmp(&right.total_minor.unwrap_or(i64::MAX))
                    .then(left.rate_plan_id.cmp(&right.rate_plan_id))
            });
            let best = views.first()?.clone();
            let best_minor = best.total_minor;
            let instant_booking = views.iter().any(|view| view.booking_mode == "INSTANT");
            Some(SearchHit {
                id: first.property_id,
                name: first.name,
                verification_status: first.verification_status,
                cover: None,
                review_count: 0,
                verified_rating: None,
                distance_km: first.distance_km.map(|km| (km * 10.0).round() / 10.0),
                amenities: string_list(&first.amenities),
                instant_booking,
                best,
                offers: views,
                best_minor,
            })
        })
        .collect()
}

pub fn sort_hits(hits: &mut [SearchHit], sort: &str) {
    hits.sort_by(|left, right| match sort {
        "price" => left
            .best_minor
            .unwrap_or(i64::MAX)
            .cmp(&right.best_minor.unwrap_or(i64::MAX))
            .then(left.name.cmp(&right.name))
            .then(left.id.cmp(&right.id)),
        "distance" => left
            .distance_km
            .unwrap_or(f64::MAX)
            .total_cmp(&right.distance_km.unwrap_or(f64::MAX))
            .then(left.name.cmp(&right.name))
            .then(left.id.cmp(&right.id)),
        "name" => left.name.cmp(&right.name).then(left.id.cmp(&right.id)),
        _ => (left.verification_status != "VERIFIED")
            .cmp(&(right.verification_status != "VERIFIED"))
            .then((!left.instant_booking).cmp(&!right.instant_booking))
            .then(left.best_minor.unwrap_or(i64::MAX).cmp(&right.best_minor.unwrap_or(i64::MAX)))
            .then(left.name.cmp(&right.name))
            .then(left.id.cmp(&right.id)),
    });
}
