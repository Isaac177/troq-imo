use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use kernel::{City, Error, Result};
use sqlx::PgPool;

use super::booking_mode::{approval_deadline, initial_state};
use super::validators::{clean_special_request, policy_ids, validate_acceptance, validate_guest};
use crate::reservations::codes::{
    CHECK_IN_PASSED, DATES_UNAVAILABLE, QUOTE_EXPIRED, QUOTE_NOT_FOUND, RATE_PLAN_GONE,
};
use crate::reservations::contract::{CommissionSnap, StoredBreakdown};
use crate::reservations::lifecycle::types::HoldRequest;
use crate::reservations::links::{billing, inventory, pricing, settings};
use crate::reservations::support::{nights_between, parse_minor};

pub struct Prepared {
    pub quote: pricing::QuoteRow,
    pub stored: StoredBreakdown,
    pub context: pricing::QuoteContext,
    pub accepted_policies: Vec<String>,
    pub special_request: Option<String>,
    pub nights: Vec<NaiveDate>,
    pub initial: &'static str,
    pub approval: Option<DateTime<Utc>>,
    pub commission: CommissionSnap,
}

fn offset_of(timezone: &str) -> Result<FixedOffset> {
    kernel::timezone::offset_for_timezone(timezone).ok_or_else(|| Error::internal("property timezone has no offset"))
}

pub async fn prepare(pool: &PgPool, city: City, session: &str, input: &HoldRequest) -> Result<Prepared> {
    validate_guest(&input.guest)?;
    let quote = pricing::find_quote(pool, city, &input.quote_id)
        .await?
        .filter(|quote| quote.session_ref == session)
        .ok_or_else(|| Error::not_found(QUOTE_NOT_FOUND))?;
    if quote.expires_at <= Utc::now() {
        return Err(Error::conflict(QUOTE_EXPIRED));
    }
    let stored: StoredBreakdown = serde_json::from_value(quote.breakdown.clone())
        .map_err(|error| Error::internal(format!("quote breakdown unreadable: {error}")))?;
    let accepted_policies = validate_acceptance(&input.accepted_policy_version_ids, &policy_ids(&stored.policies))?;
    let special_request = clean_special_request(input.special_request.as_deref())?;

    let context = pricing::load_context(pool, city, &quote.property_id, &stored.rate_plan_id)
        .await?
        .ok_or_else(|| Error::conflict(RATE_PLAN_GONE))?;
    pricing::ensure_bookable(&context, &stored.unit_type_id, quote.adult_count, quote.child_count)?;
    if context.occupancy_basis != "PER_UNIT" && context.occupancy_basis != "PER_GUEST" {
        return Err(Error::internal("unknown occupancy basis"));
    }
    let offset = offset_of(&context.timezone)?;
    let booking_day = Utc::now().with_timezone(&offset).date_naive();
    if quote.check_in < booking_day {
        return Err(Error::conflict(CHECK_IN_PASSED));
    }

    let nights = nights_between(quote.check_in, quote.check_out);
    let open = pricing::open_nights(pool, &stored.unit_type_id, quote.check_in, quote.check_out).await?;
    if open.len() != nights.len() {
        return Err(Error::conflict(DATES_UNAVAILABLE));
    }
    inventory::enforce(
        pool,
        &quote.property_id,
        &inventory::StayRequest {
            unit_type_id: stored.unit_type_id.clone(),
            check_in: quote.check_in,
            check_out: quote.check_out,
            booking_day,
        },
    )
    .await?;

    let initial = initial_state(&context.booking_mode)?;
    let approval_hours = settings::approval_window_hours(pool, city).await?;
    let approval = approval_deadline(&context.booking_mode, Utc::now(), approval_hours)?;
    let commission = match stored.commission.clone() {
        Some(snapshot) => snapshot,
        None => CommissionSnap::without_commission(&quote.total_amount)?,
    };
    billing::ensure_can_book(pool, &context.host_id, parse_minor(&commission.amount)?).await?;
    Ok(Prepared {
        quote,
        stored,
        context,
        accepted_policies,
        special_request,
        nights,
        initial,
        approval,
        commission,
    })
}
