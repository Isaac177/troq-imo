use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Path, Query, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::paging::Page;
use kernel::KernelState;

use super::admin_list::{list_reviews, AdminReviewItem, AdminReviewQuery};
use super::extract;
use super::moderation::{decide_review, moderation_queue};
use super::types::{page_request, DecideReviewInput, ReviewListQuery, ReviewView};

#[utoipa::path(
    get, path = "/api/admin/reviews", operation_id = "reviews_admin_list", tag = "reviews",
    params(AdminReviewQuery, ("x-city" = Option<String>, Header), ("x-request-id" = Option<String>, Header)),
    responses(
        (status = 200, body = Envelope<Page<AdminReviewItem>>, headers(("x-request-id" = String))),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody)
    )
)]
pub async fn list<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    query: std::result::Result<Query<AdminReviewQuery>, QueryRejection>,
) -> Result<Json<Envelope<Page<AdminReviewItem>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let query = extract::query(query)?;
    let page = page_request(query.page, query.page_size);
    let data = list_reviews(state.kernel(), &actor, query, page).await?;
    Ok(Json(Envelope::ok(data, request_id)))
}

#[utoipa::path(
    get, path = "/api/admin/reviews/moderation-queue", operation_id = "reviews_admin_queue", tag = "reviews",
    params(ReviewListQuery, ("x-city" = Option<String>, Header), ("x-request-id" = Option<String>, Header)),
    responses(
        (status = 200, body = Envelope<Page<ReviewView>>, headers(("x-request-id" = String))),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody)
    )
)]
pub async fn queue<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    query: std::result::Result<Query<ReviewListQuery>, QueryRejection>,
) -> Result<Json<Envelope<Page<ReviewView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let query = extract::query(query)?;
    let page = page_request(query.page, query.page_size);
    let data = moderation_queue(state.kernel(), &actor, query.status.as_deref(), page).await?;
    Ok(Json(Envelope::ok(data, request_id)))
}

#[utoipa::path(
    post, path = "/api/admin/reviews/{reviewId}/moderation", operation_id = "reviews_admin_decide", tag = "reviews",
    params(("reviewId" = String, Path), ("x-city" = Option<String>, Header), ("x-request-id" = Option<String>, Header)),
    request_body = DecideReviewInput,
    responses(
        (status = 200, body = Envelope<ReviewView>, headers(("x-request-id" = String))),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody)
    )
)]
pub async fn decide<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(review_id): Path<String>,
    input: std::result::Result<Json<DecideReviewInput>, JsonRejection>,
) -> Result<Json<Envelope<ReviewView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let input = extract::body(input)?;
    let view = decide_review(state.kernel(), &actor, &review_id, input).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}
