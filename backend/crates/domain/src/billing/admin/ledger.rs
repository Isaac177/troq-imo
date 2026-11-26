use kernel::auth::Actor;
use kernel::{Page, PageRequest, Result};
use sqlx::PgPool;

use super::credit_types::AdminLedgerRow;
use crate::billing::credits::entries::ledger_page;
use crate::billing::scope;

pub async fn ledger(
    pool: &PgPool,
    actor: &Actor,
    host_id: &str,
    entry_type: Option<&str>,
    page: PageRequest,
) -> Result<Page<AdminLedgerRow>> {
    scope::require_staff(actor)?;
    scope::ensure_host(pool, host_id, actor.city).await?;
    let page = page.validated()?;
    let (records, total) = ledger_page(pool, host_id, entry_type, page).await?;
    Ok(Page::new(records.into_iter().map(AdminLedgerRow::from).collect(), total, page))
}
