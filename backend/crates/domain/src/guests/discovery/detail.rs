use std::collections::HashMap;

use chrono::Utc;
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::storage::Storage;
use kernel::Money;
use serde_json::Value;
use sqlx::PgPool;

use super::queries;
use super::types::{ImageView, MediaView, PropertyDetailView, RatePlanView, UnitTypeView};
use crate::guests::collaborators::{nightly_amount_minor, rules_for_plans};
use crate::guests::messages::PROPERTY_NOT_FOUND;
use crate::properties::media::image_url;

async fn signed(storage: &Storage, media_id: &str, key: &str) -> Option<String> {
    match image_url(storage, key).await {
        Ok(url) => Some(url),
        Err(error) => {
            tracing::warn!(media_id = %media_id, error = %error, "stay media unavailable");
            None
        }
    }
}

async fn images(
    pool: &PgPool,
    storage: &Storage,
    unit_ids: &[String],
    locale: &str,
) -> Result<HashMap<String, Vec<ImageView>>> {
    let mut grouped: HashMap<String, Vec<ImageView>> = HashMap::new();
    for row in queries::unit_images(pool, unit_ids, locale).await? {
        if let Some(url) = signed(storage, &row.id, &row.storage_key).await {
            grouped.entry(row.unit_type_id).or_default().push(ImageView {
                id: row.id,
                url,
                alt: row.alt,
                is_cover: row.is_cover,
            });
        }
    }
    Ok(grouped)
}

async fn property_media(pool: &PgPool, storage: &Storage, property_id: &str, locale: &str) -> Result<Vec<MediaView>> {
    let mut media = Vec::new();
    for item in queries::ready_media(pool, property_id, locale).await? {
        if let Some(url) = signed(storage, &item.id, &item.storage_key).await {
            media.push(MediaView { content_path: url.clone(), url, id: item.id, alt: item.alt, is_cover: item.is_cover });
        }
    }
    Ok(media)
}

pub async fn property_detail(
    pool: &PgPool,
    storage: &Storage,
    city: City,
    property_id: &str,
    locale: &str,
) -> Result<PropertyDetailView> {
    let property = queries::published_property(pool, city, property_id, locale)
        .await?
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    let units = queries::active_unit_types(pool, &property.id, locale).await?;
    let unit_ids: Vec<String> = units.iter().map(|unit| unit.id.clone()).collect();
    let plans = queries::active_plans(pool, &unit_ids).await?;
    let mut unit_images = images(pool, storage, &unit_ids, locale).await?;
    let mut policy_ids: Vec<String> = plans
        .iter()
        .flat_map(|plan| [plan.cancellation_policy_id.clone(), plan.payment_policy_id.clone()])
        .collect();
    policy_ids.sort();
    policy_ids.dedup();
    let policies: HashMap<String, Value> = if policy_ids.is_empty() {
        HashMap::new()
    } else {
        queries::policy_contents(pool, &policy_ids).await?.into_iter().collect()
    };
    let plan_ids: Vec<String> = plans.iter().map(|plan| plan.id.clone()).collect();
    let mut connection = pool.acquire().await?;
    let rules = rules_for_plans(&mut connection, city, &plan_ids).await?;
    let today = Utc::now().date_naive();
    Ok(PropertyDetailView {
        id: property.id.clone(),
        city: city.code().to_string(),
        timezone: property.timezone,
        name: property.name,
        description: property.description,
        verification_status: property.verification_status,
        host_name: Some(property.host_name),
        amenities: queries::amenity_codes(pool, &property.id).await?,
        media: property_media(pool, storage, &property.id, locale).await?,
        unit_types: units
            .into_iter()
            .map(|unit| {
                let images = unit_images.remove(&unit.id).unwrap_or_default();
                UnitTypeView {
                    cover_url: images.first().map(|image| image.url.clone()),
                    images,
                    id: unit.id,
                    name: unit.name,
                    description: unit.description,
                    max_adults: unit.max_adults,
                    max_children: unit.max_children,
                    bedroom_count: unit.bedroom_count,
                    bed_count: unit.bed_count,
                    bathroom_count: unit.bathroom_count,
                }
            })
            .collect(),
        rate_plans: plans
            .into_iter()
            .map(|plan| RatePlanView {
                nightly: rules
                    .get(&plan.id)
                    .and_then(|plan_rules| nightly_amount_minor(today, plan_rules))
                    .map(|minor| Money::from_minor(minor).to_wire()),
                cancellation: policies.get(&plan.cancellation_policy_id).cloned(),
                payment: policies.get(&plan.payment_policy_id).cloned(),
                id: plan.id,
                unit_type_id: plan.unit_type_id,
                occupancy_basis: plan.occupancy_basis,
                meal_plan: plan.meal_plan,
                booking_mode: plan.booking_mode,
            })
            .collect(),
    })
}
