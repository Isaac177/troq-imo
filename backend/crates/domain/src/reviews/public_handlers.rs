use axum::extract::rejection::QueryRejection;
use axum::extract::{Path, Query, State};
use axum::Json;
use kernel::auth::{RequestCity, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::paging::Page;
use kernel::KernelState;

use super::extract;
use super::property_reviews::list_property_reviews;
use super::types::{page_request, PublicReviewQuery, ReviewView};

#[utoipa::path(
    get, path = "/api/public/properties/{propertyId}/reviews", operation_id = "reviews_public_list", tag = "reviews",
    params(
        ("propertyId" = String, Path),
        PublicReviewQuery,
        ("x-city" = Option<String>, Header),
        ("x-request-id" = Option<String>, Header)
    ),
    responses(
        (status = 200, body = Envelope<Page<ReviewView>>, headers(("x-request-id" = String))),
        (status = 400, body = ErrorBody)
    )
)]
pub async fn list<S>(
    State(state): State<S>,
    RequestId(request_id): RequestId,
    RequestCity(city): RequestCity,
    Path(property_id): Path<String>,
    query: std::result::Result<Query<PublicReviewQuery>, QueryRejection>,
) -> Result<Json<Envelope<Page<ReviewView>>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let query = extract::query(query)?;
    let page = page_request(query.page, query.page_size);
    let data = list_property_reviews(state.kernel(), city, None, &property_id, None, page).await?;
    Ok(Json(Envelope::ok(data, request_id)))
}
