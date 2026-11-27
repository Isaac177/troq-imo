use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Path, Query, State};
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::paging::Page;
use kernel::KernelState;

use super::extract;
use super::host_list::{self, HostReviewView};
use super::reply::reply_review;
use super::types::{page_request, ReplyReviewInput, ReviewListQuery, ReviewView};

#[utoipa::path(
    post, path = "/api/host/reviews/{reviewId}/reply", operation_id = "reviews_reply", tag = "reviews",
    params(("reviewId" = String, Path), ("x-city" = Option<String>, Header), ("x-request-id" = Option<String>, Header)),
    request_body = ReplyReviewInput,
    responses(
        (status = 200, body = Envelope<ReviewView>, headers(("x-request-id" = String))),
        (status = 400, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
pub async fn reply<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(review_id): Path<String>,
    input: std::result::Result<Json<ReplyReviewInput>, JsonRejection>,
) -> Result<Json<Envelope<ReviewView>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let input = extract::body(input)?;
    let view = reply_review(state.kernel(), &actor, &review_id, input).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/host/properties/{propertyId}/reviews", operation_id = "reviews_host_list", tag = "reviews",
    params(("propertyId" = String, Path), ReviewListQuery, ("x-city" = Option<String>, Header), ("x-request-id" = Option<String>, Header)),
    responses(
        (status = 200, body = Envelope<Page<HostReviewView>>, headers(("x-request-id" = String))),
        (status = 400, body = ErrorBody)
    )
)]
pub async fn list<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    query: std::result::Result<Query<ReviewListQuery>, QueryRejection>,
) -> Result<Json<Envelope<Page<HostReviewView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let query = extract::query(query)?;
    let page = page_request(query.page, query.page_size);
    let data = host_list::list(state.kernel(), &actor, &property_id, query.status.as_deref(), page).await?;
    Ok(Json(Envelope::ok(data, request_id)))
}
