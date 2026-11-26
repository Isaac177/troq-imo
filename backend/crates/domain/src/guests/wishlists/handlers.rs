use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::Result;
use crate::guests::extract::Shared;
use kernel::paging::Page;

use super::service;
use super::types::{CreateWishlist, ItemsQuery, UpdateWishlist, WishlistItem, WishlistView};
use crate::guests::extract::{ApiJson, ApiQuery};
use crate::guests::page::PageQuery;

#[utoipa::path(
    get, path = "/api/guest/wishlists", operation_id = "guests_wishlists_list", tag = "guests",
    params(PageQuery),
    responses((status = 200, body = Envelope<Page<WishlistView>>))
)]
pub async fn list<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    ApiQuery(query): ApiQuery<PageQuery>,
) -> Result<Json<Envelope<Page<WishlistView>>>> {
    let pool = state.kernel().db.pool();
    let page = service::list_wishlists(pool, actor.city, actor.user_id().as_str(), query.request()).await?;
    Ok(Json(Envelope::ok(page, actor.request_id)))
}

#[utoipa::path(
    post, path = "/api/guest/wishlists", operation_id = "guests_wishlists_create", tag = "guests",
    request_body = CreateWishlist,
    responses((status = 200, body = Envelope<WishlistView>), (status = 409, body = kernel::error::ErrorBody, description = "List limit reached"))
)]
pub async fn create<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    ApiJson(input): ApiJson<CreateWishlist>,
) -> Result<Json<Envelope<WishlistView>>> {
    let pool = state.kernel().db.pool();
    let created = service::create_wishlist(pool, actor.city, actor.user_id().as_str(), input).await?;
    Ok(Json(Envelope::ok(created, actor.request_id)))
}

#[utoipa::path(
    patch, path = "/api/guest/wishlists/{wishlistId}", operation_id = "guests_wishlists_update", tag = "guests",
    params(("wishlistId" = String, Path)),
    request_body = UpdateWishlist,
    responses((status = 200, body = Envelope<WishlistView>), (status = 404, body = kernel::error::ErrorBody, description = "List not found"))
)]
pub async fn update<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    Path(wishlist_id): Path<String>,
    ApiJson(input): ApiJson<UpdateWishlist>,
) -> Result<Json<Envelope<WishlistView>>> {
    let pool = state.kernel().db.pool();
    let updated = service::update_wishlist(pool, actor.city, actor.user_id().as_str(), &wishlist_id, input).await?;
    Ok(Json(Envelope::ok(updated, actor.request_id)))
}

#[utoipa::path(
    get, path = "/api/guest/wishlists/{wishlistId}/items", operation_id = "guests_wishlist_items_list", tag = "guests",
    params(("wishlistId" = String, Path), ItemsQuery),
    responses((status = 200, body = Envelope<Page<WishlistItem>>), (status = 404, body = kernel::error::ErrorBody, description = "List not found"))
)]
pub async fn items<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    Path(wishlist_id): Path<String>,
    ApiQuery(query): ApiQuery<ItemsQuery>,
) -> Result<Json<Envelope<Page<WishlistItem>>>> {
    let pool = state.kernel().db.pool();
    let page = service::list_items(pool, actor.city, actor.user_id().as_str(), &wishlist_id, &query).await?;
    Ok(Json(Envelope::ok(page, actor.request_id)))
}

#[utoipa::path(
    post, path = "/api/guest/wishlists/{wishlistId}/items/{propertyId}", operation_id = "guests_wishlist_items_add", tag = "guests",
    params(("wishlistId" = String, Path), ("propertyId" = String, Path)),
    responses((status = 200, body = Envelope<WishlistView>), (status = 404, body = kernel::error::ErrorBody, description = "List or property not found"), (status = 409, body = kernel::error::ErrorBody, description = "List is full"))
)]
pub async fn add_item<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    Path((wishlist_id, property_id)): Path<(String, String)>,
) -> Result<Json<Envelope<WishlistView>>> {
    let pool = state.kernel().db.pool();
    let view = service::add_item(pool, actor.city, actor.user_id().as_str(), &wishlist_id, &property_id).await?;
    Ok(Json(Envelope::ok(view, actor.request_id)))
}

#[utoipa::path(
    delete, path = "/api/guest/wishlists/{wishlistId}/items/{propertyId}", operation_id = "guests_wishlist_items_remove", tag = "guests",
    params(("wishlistId" = String, Path), ("propertyId" = String, Path)),
    responses((status = 200, body = Envelope<WishlistView>), (status = 404, body = kernel::error::ErrorBody, description = "List not found"))
)]
pub async fn remove_item<S: Shared>(
    State(state): State<S>,
    actor: Actor,
    Path((wishlist_id, property_id)): Path<(String, String)>,
) -> Result<Json<Envelope<WishlistView>>> {
    let pool = state.kernel().db.pool();
    let view = service::remove_item(pool, actor.city, actor.user_id().as_str(), &wishlist_id, &property_id).await?;
    Ok(Json(Envelope::ok(view, actor.request_id)))
}
