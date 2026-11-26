use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::kernel::Kernel;
use serde_json::json;

use super::codes::{REVIEW_NOT_CHECKED_OUT, REVIEW_RESERVATION_NOT_FOUND};
use super::events::{self, REVIEW_SUBMITTED};
use super::queries;
use super::types::{ReviewView, SubmitReviewInput, CHECKED_OUT};
use super::validators::{optional_text, rating, COMMENT_LIMIT};

pub async fn submit_review(
    kernel: &Kernel,
    actor: &Actor,
    reservation_id: &str,
    input: SubmitReviewInput,
) -> Result<ReviewView> {
    let stars = rating(input.rating)?;
    let comment = optional_text(input.comment, COMMENT_LIMIT)?;
    let pool = kernel.db.pool();
    let party = queries::party_of(pool, actor.city, reservation_id).await?;
    if actor.user_id().as_str() != party.guest_user_id {
        return Err(Error::not_found(REVIEW_RESERVATION_NOT_FOUND));
    }
    if party.state != CHECKED_OUT {
        return Err(Error::conflict(REVIEW_NOT_CHECKED_OUT));
    }
    let mut tx = pool.begin().await?;
    let row = queries::insert(
        &mut tx,
        actor.city,
        &new_id("review"),
        reservation_id,
        &party,
        stars,
        comment.as_deref(),
    )
    .await?;
    events::emit(
        &mut tx,
        actor.city,
        reservation_id,
        REVIEW_SUBMITTED,
        json!({ "reviewId": row.id, "rating": row.rating, "comment": row.comment }),
    )
    .await?;
    tx.commit().await?;
    Ok(row.into_view())
}
