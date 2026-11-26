use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::kernel::Kernel;
use kernel::permissions::names::REVIEWS_WRITE;
use serde_json::json;

use super::codes::{REVIEW_NOT_APPROVED, REVIEW_NOT_FOUND};
use super::events::{self, REVIEW_REPLIED};
use super::queries;
use super::types::{ReplyReviewInput, ReviewView, APPROVED};
use super::validators::{required_text, REPLY_LIMIT};

pub async fn reply_review(
    kernel: &Kernel,
    actor: &Actor,
    review_id: &str,
    input: ReplyReviewInput,
) -> Result<ReviewView> {
    let reply = required_text(input.reply, REPLY_LIMIT)?;
    let pool = kernel.db.pool();
    let review = queries::find(pool, actor.city, review_id)
        .await?
        .ok_or_else(|| Error::not_found(REVIEW_NOT_FOUND))?;
    actor.ensure_host_scope(REVIEWS_WRITE, &review.operator_id, &review.property_id)?;
    if review.status != APPROVED {
        return Err(Error::conflict(REVIEW_NOT_APPROVED));
    }
    let mut tx = pool.begin().await?;
    let row = queries::set_reply(&mut tx, actor.city, review_id, &reply)
        .await?
        .ok_or_else(|| Error::not_found(REVIEW_NOT_FOUND))?;
    events::emit(
        &mut tx,
        actor.city,
        &row.reservation_id,
        REVIEW_REPLIED,
        json!({ "reviewId": row.id, "reply": reply }),
    )
    .await?;
    tx.commit().await?;
    Ok(row.into_view())
}
