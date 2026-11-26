use kernel::{Error, Result};
use sqlx::PgPool;

use super::{account, state};
use crate::billing::commission::rate::current_rate_bps;
use crate::billing::messages::{CREDIT_INSUFFICIENT, CREDIT_LISTING_UNAVAILABLE};
use crate::billing::settings;

pub async fn ensure_can_book(pool: &PgPool, host_id: &str, commission_minor: i64) -> Result<()> {
    let city = settings::host_city(pool, host_id).await?;
    let rate_bps = current_rate_bps(pool, city, host_id).await?;
    if rate_bps <= 0 {
        return Ok(());
    }
    let balance = account::balance_minor(pool, host_id).await?;
    let min_credit = settings::min_credit_cdf(pool, city).await?;
    if state::is_disabled(rate_bps, balance, min_credit) {
        return Err(Error::conflict(CREDIT_LISTING_UNAVAILABLE));
    }
    if balance < commission_minor {
        return Err(Error::conflict(CREDIT_INSUFFICIENT));
    }
    Ok(())
}
