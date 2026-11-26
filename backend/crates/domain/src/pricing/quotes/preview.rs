use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_READ;
use kernel::{Error, Money, Result};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

use super::charges::{price_stay, StayTerms};
use super::context::{guest_factor, load_context};
use super::types::{DiscountView, QuoteRequest};
use crate::pricing::amounts::{money, parse_stored};
use crate::pricing::messages::*;
use crate::pricing::promotions::load_active;
use crate::pricing::rates::explain::{explain_night, NightExplanation};
use crate::pricing::rates::load_rules;
use crate::pricing::scope;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PricePreviewView {
    pub property_id: String,
    pub unit_type_id: String,
    pub rate_plan_id: String,
    pub currency: &'static str,
    #[schema(value_type = String)]
    pub subtotal: Money,
    pub discount: Option<DiscountView>,
    #[schema(value_type = String)]
    pub total: Money,
    pub nights: Vec<PreviewNight>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PreviewNight {
    #[schema(value_type = String)]
    pub amount: Money,
    pub explanation: NightExplanation,
}

pub async fn preview_price(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    request: QuoteRequest,
    as_admin: bool,
) -> Result<PricePreviewView> {
    let city = actor.city;
    let input = request.validated()?;
    let property = scope::resolve(pool, actor, property_id, PROPERTIES_READ, as_admin).await?;
    let context = load_context(pool, city, &property.id, &input.rate_plan_id)
        .await?
        .ok_or_else(|| Error::not_found(RATE_PLAN_NOT_FOUND))?;
    if context.unit_type_id != input.unit_type_id || context.rate_unit_type_id != input.unit_type_id {
        return Err(Error::bad_request(RATE_PLAN_UNIT_TYPE_MISMATCH));
    }
    let rules = load_rules(pool, city, &context.rate_plan_id).await?;
    let promotions =
        load_active(pool, city, &property.id, &context.rate_plan_id, input.check_in, input.check_out).await?;
    let nights = input.nights();
    let priced = price_stay(
        &nights,
        &StayTerms {
            rules: &rules,
            guest_factor: guest_factor(&context.occupancy_basis, input.adult_count),
            child_nightly_minor: parse_stored(&context.child_nightly_amount)?,
            child_count: i64::from(input.child_count),
            promotions: &promotions,
        },
    )?;
    let mut preview_nights = Vec::with_capacity(nights.len());
    for (night, charge) in nights.iter().zip(priced.nights.iter()) {
        preview_nights.push(PreviewNight {
            amount: money(charge.amount_minor),
            explanation: explain_night(*night, &rules)?,
        });
    }
    Ok(PricePreviewView {
        property_id: property.id,
        unit_type_id: context.unit_type_id,
        rate_plan_id: context.rate_plan_id,
        currency: kernel::money::CURRENCY,
        subtotal: money(priced.subtotal_minor),
        discount: priced.discount.map(|applied| DiscountView {
            promotion_id: applied.promotion_id,
            amount: money(applied.amount_minor),
        }),
        total: money(priced.total_minor),
        nights: preview_nights,
    })
}
