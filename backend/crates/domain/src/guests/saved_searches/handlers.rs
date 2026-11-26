use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::Result;
use kernel::paging::Page;

use super::service;
use super::types::{CreateSavedSearch, SavedSearchRemoved, SavedSearchView};
use crate::guests::extract::{ApiJson, ApiQuery, Shared};
use crate::guests::page::PageQuery;

#[utoipa::path(
    get, path = "/api/guest/saved-searches", operation_id = "guests_saved_searches_list", tag = "guests",
    params(PageQuery),
    responses((status = 200, body = Envelope<Page<SavedSearchView>>))
)]
pub async fn list<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    ApiQuery(query): ApiQuery<PageQuery>,
) -> Result<Json<Envelope<Page<SavedSearchView>>>> {
    let pool = state.kernel().db.pool();
    let page = service::list_saved_searches(pool, actor.city, actor.user_id().as_str(), query.request()).await?;
    Ok(Json(Envelope::ok(page, actor.request_id)))
}

#[utoipa::path(
    post, path = "/api/guest/saved-searches", operation_id = "guests_saved_searches_create", tag = "guests",
    request_body = CreateSavedSearch,
    responses((status = 200, body = Envelope<SavedSearchView>), (status = 409, body = kernel::error::ErrorBody, description = "Saved search limit reached"))
)]
pub async fn create<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    ApiJson(input): ApiJson<CreateSavedSearch>,
) -> Result<Json<Envelope<SavedSearchView>>> {
    let pool = state.kernel().db.pool();
    let created = service::create_saved_search(pool, actor.city, actor.user_id().as_str(), input).await?;
    Ok(Json(Envelope::ok(created, actor.request_id)))
}

#[utoipa::path(
    delete, path = "/api/guest/saved-searches/{savedSearchId}", operation_id = "guests_saved_searches_delete", tag = "guests",
    params(("savedSearchId" = String, Path)),
    responses((status = 200, body = Envelope<SavedSearchRemoved>), (status = 404, body = kernel::error::ErrorBody, description = "Saved search not found"))
)]
pub async fn remove<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    Path(saved_search_id): Path<String>,
) -> Result<Json<Envelope<SavedSearchRemoved>>> {
    let pool = state.kernel().db.pool();
    let removed = service::delete_saved_search(pool, actor.city, actor.user_id().as_str(), &saved_search_id).await?;
    Ok(Json(Envelope::ok(removed, actor.request_id)))
}
