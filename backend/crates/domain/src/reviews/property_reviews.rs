use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::Result;
use kernel::kernel::Kernel;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::{REVIEWS_MODERATE, REVIEWS_WRITE};

use super::covers;
use super::list_queries;
use super::queries;
use super::types::{ReviewView, APPROVED};
use super::validators::optional_review_status;

async fn can_see_all(
    kernel: &Kernel,
    city: City,
    actor: Option<&Actor>,
    property_id: &str,
) -> Result<bool> {
    let Some(actor) = actor else {
        return Ok(false);
    };
    if actor.require_scoped(REVIEWS_MODERATE, None, None).is_ok() {
        return Ok(true);
    }
    Ok(match queries::property_host(kernel.db.pool(), city, property_id).await? {
        Some(host_id) => actor.allows(REVIEWS_WRITE, &host_id, property_id),
        None => false,
    })
}

pub async fn list_property_reviews(
    kernel: &Kernel,
    city: City,
    actor: Option<&Actor>,
    property_id: &str,
    status: Option<&str>,
    page: PageRequest,
) -> Result<Page<ReviewView>> {
    let status = optional_review_status(status)?;
    let page = page.validated()?;
    let status = if can_see_all(kernel, city, actor, property_id).await? {
        status
    } else {
        Some(APPROVED)
    };
    let (rows, total) =
        list_queries::list_for_property(kernel.db.pool(), city, property_id, status, page).await?;
    let views = rows.into_iter().map(|row| row.into_view()).collect();
    Ok(Page::new(covers::attach(kernel, views).await?, total, page))
}
