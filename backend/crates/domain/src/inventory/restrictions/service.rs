use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use sqlx::PgPool;

use super::queries;
use super::types::RestrictionView;

pub async fn list_page(
    pool: &PgPool,
    property_id: &str,
    page: PageRequest,
) -> Result<Page<RestrictionView>> {
    let (items, total) = queries::list(pool, property_id, page.limit(), page.offset()).await?;
    Ok(Page::new(items, total, page))
}
