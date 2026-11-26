use kernel::auth::Actor;
use kernel::{Page, PageRequest, Result};
use sqlx::PgPool;

use super::types::{MineLedgerRow, MineSummary};
use super::{account, entries, state};
use crate::billing::amounts::wire;
use crate::billing::commission::rate::current_rate_bps;
use crate::billing::{scope, settings};

pub async fn summary(pool: &PgPool, actor: &Actor, host_id: &str) -> Result<MineSummary> {
    scope::authorize_host_read(pool, actor, host_id).await?;
    let city = actor.city;
    let rate_bps = current_rate_bps(pool, city, host_id).await?;
    let balance_minor = account::balance_minor(pool, host_id).await?;
    let min_credit = settings::min_credit_cdf(pool, city).await?;
    let low_threshold = settings::low_credit_cdf(pool, city).await?;
    let disabled = state::is_disabled(rate_bps, balance_minor, min_credit);
    Ok(MineSummary {
        balance: wire(balance_minor),
        rate_bps,
        enabled: !disabled,
        low: !disabled && rate_bps > 0 && state::is_low(balance_minor, low_threshold),
        low_threshold,
    })
}

pub async fn ledger(pool: &PgPool, actor: &Actor, host_id: &str, page: PageRequest) -> Result<Page<MineLedgerRow>> {
    scope::authorize_host_read(pool, actor, host_id).await?;
    let page = page.validated()?;
    let (records, total) = entries::ledger_page(pool, host_id, None, page).await?;
    Ok(Page::new(records.into_iter().map(MineLedgerRow::from).collect(), total, page))
}
