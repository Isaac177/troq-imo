use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::kernel::Kernel;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::REVIEWS_MODERATE;
use serde_json::json;

use super::codes::REVIEW_NOT_FOUND;
use super::covers;
use super::list_queries;
use super::queries;
use super::types::{DecideReviewInput, ReviewView, PENDING};
use super::validators::{hide_reason, moderation_status, review_status};

const ACTION: &str = "review.moderate";
const RESOURCE: &str = "review";

pub async fn decide_review(
    kernel: &Kernel,
    actor: &Actor,
    review_id: &str,
    input: DecideReviewInput,
) -> Result<ReviewView> {
    actor.require_platform(REVIEWS_MODERATE)?;
    let status = moderation_status(&input.status)?;
    let reason = hide_reason(status, input.reason_code.as_deref())?;
    let mut tx = kernel.db.pool().begin().await?;
    let previous = queries::lock_status(&mut tx, actor.city, review_id)
        .await?
        .ok_or_else(|| Error::not_found(REVIEW_NOT_FOUND))?;
    let row = queries::set_status(&mut tx, actor.city, review_id, status, actor.user_id().as_str())
        .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: ACTION,
            resource_type: RESOURCE,
            resource_id: review_id,
            reason,
            change: json!({ "from": previous, "to": status }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(row.into_view())
}

pub async fn moderation_queue(
    kernel: &Kernel,
    actor: &Actor,
    status: Option<&str>,
    page: PageRequest,
) -> Result<Page<ReviewView>> {
    actor.require_platform(REVIEWS_MODERATE)?;
    let status = review_status(status.unwrap_or(PENDING))?;
    let page = page.validated()?;
    let (rows, total) = list_queries::list_queue(kernel.db.pool(), actor.city, status, page).await?;
    let views = rows.into_iter().map(|row| row.into_view()).collect();
    Ok(Page::new(covers::attach(kernel, views).await?, total, page))
}
