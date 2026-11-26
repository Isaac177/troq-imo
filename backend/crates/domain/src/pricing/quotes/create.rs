use chrono::{Duration, Utc};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::timezone::offset_for_timezone;
use kernel::{Error, Result};
use serde_json::{json, Value};
use sqlx::PgPool;

use super::availability::{open_nights, room_available};
use super::cancellation::{free_cancellation_deadline, CancellationTerms};
use super::charges::{price_stay, StayTerms};
use super::checks::{QuoteChecks, StayWindow};
use super::context::{ensure_bookable, guest_factor, load_context, load_policies, PolicyRow};
use super::schedule::pay_at_property;
use super::store::{self, NewQuote};
use super::types::{DiscountView, NightLine, QuoteRequest, QuoteView};
use crate::pricing::amounts::{money, parse_stored};
use crate::pricing::messages::*;
use crate::pricing::promotions::load_active;
use crate::pricing::rates::load_rules;
use crate::pricing::settings::{get_i64, SettingKey};

pub async fn create_quote(
    pool: &PgPool,
    checks: &dyn QuoteChecks,
    actor: &Actor,
    request: QuoteRequest,
) -> Result<QuoteView> {
    let city = actor.city;
    let input = request.validated()?;
    let context = load_context(pool, city, &input.property_id, &input.rate_plan_id)
        .await?
        .ok_or_else(|| Error::not_found(RATE_PLAN_NOT_FOUND))?;
    ensure_bookable(&context, &input.unit_type_id, input.adult_count, input.child_count)?;

    let offset = offset_for_timezone(&context.timezone)
        .ok_or_else(|| Error::internal(TIMEZONE_UNSUPPORTED.as_str()))?;
    let today = Utc::now().with_timezone(&offset).date_naive();
    if input.check_in < today {
        return Err(Error::bad_request(CHECK_IN_IN_PAST));
    }
    checks
        .enforce_restrictions(
            city,
            &input.property_id,
            &StayWindow {
                unit_type_id: context.unit_type_id.clone(),
                check_in: input.check_in,
                check_out: input.check_out,
                booking_day: today,
            },
        )
        .await?;

    let nights = input.nights();
    let open = open_nights(pool, &context.unit_type_id, input.check_in, input.check_out).await?;
    if open.len() != nights.len()
        || !room_available(pool, &context.unit_type_id, input.check_in, input.check_out, "").await?
    {
        return Err(Error::conflict(NOT_AVAILABLE));
    }

    let rules = load_rules(pool, city, &context.rate_plan_id).await?;
    let promotions = load_active(
        pool,
        city,
        &input.property_id,
        &context.rate_plan_id,
        input.check_in,
        input.check_out,
    )
    .await?;
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

    let policy_ids = vec![context.cancellation_policy_id.clone(), context.payment_policy_id.clone()];
    let policy_rows = load_policy_rows(pool, &policy_ids).await?;
    let deadline = free_cancellation_deadline(input.check_in, offset, cancellation_terms(&policy_rows)?)?;
    let policies = policies_value(&policy_rows);

    let quote_id = new_id("quote");
    let ttl = get_i64(pool, city, SettingKey::QuoteTtlMinutes).await?;
    let expires_at = Utc::now() + Duration::minutes(ttl);
    let lines: Vec<NightLine> = priced
        .nights
        .iter()
        .map(|night| NightLine { date: night.date, amount: money(night.amount_minor) })
        .collect();
    let subtotal = money(priced.subtotal_minor);
    let discount = priced.discount.as_ref().map(|applied| DiscountView {
        promotion_id: applied.promotion_id.clone(),
        amount: money(applied.amount_minor),
    });
    let total = money(priced.total_minor);
    let commission = checks.commission(city, &context.host_id, total).await?;
    checks.ensure_can_book(city, &context.host_id, commission.amount).await?;
    let breakdown = json!({
        "unitTypeId": context.unit_type_id,
        "ratePlanId": context.rate_plan_id,
        "nights": lines,
        "policies": policies,
        "subtotal": subtotal,
        "discount": discount,
        "cancellationDeadline": deadline,
        "commission": commission,
    });
    store::insert(
        pool,
        &NewQuote {
            id: &quote_id,
            city,
            session_ref: actor.user_id().as_str(),
            property_id: &input.property_id,
            check_in: input.check_in,
            check_out: input.check_out,
            adult_count: input.adult_count,
            child_count: input.child_count,
            total: &total.to_wire(),
            breakdown: &breakdown,
            policy_ids: &policy_ids,
            expires_at,
        },
    )
    .await?;

    Ok(QuoteView {
        unit_type_cover_url: checks.cover_url(city, &context.unit_type_id).await?,
        id: quote_id,
        property_id: input.property_id,
        unit_type_id: context.unit_type_id,
        rate_plan_id: context.rate_plan_id,
        check_in: input.check_in,
        check_out: input.check_out,
        adult_count: input.adult_count,
        child_count: input.child_count,
        currency: kernel::money::CURRENCY,
        subtotal: Some(subtotal),
        discount,
        total,
        nights: lines,
        policies,
        cancellation_deadline: Some(deadline),
        payment_schedule: pay_at_property(total),
        expires_at,
        expired: false,
    })
}

async fn load_policy_rows(pool: &PgPool, ids: &[String]) -> Result<Vec<PolicyRow>> {
    let mut rows = load_policies(pool, ids).await?;
    if rows.len() != ids.len() {
        return Err(Error::conflict(POLICIES_NOT_ACTIVE));
    }
    rows.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(rows)
}

fn policies_value(rows: &[PolicyRow]) -> Value {
    Value::Array(
        rows.iter()
            .map(|row| json!({ "id": row.id, "kind": row.kind, "content": row.content }))
            .collect(),
    )
}

fn cancellation_terms(rows: &[PolicyRow]) -> Result<CancellationTerms> {
    let row = rows
        .iter()
        .find(|row| row.kind == "CANCELLATION")
        .ok_or_else(|| Error::conflict(CANCELLATION_POLICY_MISSING))?;
    CancellationTerms::from_content(&row.content)
}
