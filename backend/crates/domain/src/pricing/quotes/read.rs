use chrono::Utc;
use kernel::auth::Actor;
use kernel::{Error, Result};
use sqlx::PgPool;

use super::checks::QuoteChecks;
use super::schedule::pay_at_property;
use super::store::{self, QuoteRow};
use super::types::{QuoteView, StoredBreakdown};
use crate::pricing::messages::QUOTE_NOT_FOUND;

pub async fn get_quote(
    pool: &PgPool,
    checks: &dyn QuoteChecks,
    actor: &Actor,
    quote_id: &str,
) -> Result<QuoteView> {
    let row = store::find(pool, actor.city, quote_id)
        .await?
        .filter(|row| row.session_ref == actor.user_id().as_str())
        .ok_or_else(|| Error::not_found(QUOTE_NOT_FOUND))?;
    let mut view = quote_view(row)?;
    view.unit_type_cover_url = checks.cover_url(actor.city, &view.unit_type_id).await?;
    Ok(view)
}

fn quote_view(row: QuoteRow) -> Result<QuoteView> {
    let stored: StoredBreakdown = serde_json::from_value(row.breakdown)
        .map_err(|error| Error::internal(format!("quote breakdown unreadable: {error}")))?;
    let total = kernel::Money::parse(&row.total_amount)?;
    Ok(QuoteView {
        unit_type_cover_url: None,
        expired: row.expires_at <= Utc::now(),
        id: row.id,
        property_id: row.property_id,
        unit_type_id: stored.unit_type_id,
        rate_plan_id: stored.rate_plan_id,
        check_in: row.check_in,
        check_out: row.check_out,
        adult_count: row.adult_count,
        child_count: row.child_count,
        currency: kernel::money::CURRENCY,
        subtotal: stored.subtotal,
        discount: stored.discount,
        total,
        nights: stored.nights,
        policies: stored.policies,
        cancellation_deadline: stored.cancellation_deadline,
        payment_schedule: pay_at_property(total),
        expires_at: row.expires_at,
    })
}
