mod collaborators;
#[cfg(test)]
mod db_tests;
mod discovery;
mod extract;
mod idempotent;
mod listing;
mod locale;
mod messages;
mod page;
mod retention;
mod saved_searches;
mod search;
mod wishlists;

use axum::routing::{delete, get, patch, post};
use axum::Router;
use utoipa::OpenApi;

use extract::Shared;

pub use discovery::{
    availability, calendar, media_content, property_detail, property_is_listed, published_count,
    AvailabilityView, CalendarView, DestinationView, PropertyDetailView,
};
pub use messages::*;
pub use retention::{
    anonymize_expired_guests, purge_search_events, retain_until, REDACTED, SEARCH_EVENT_RETENTION_MONTHS,
};
pub use saved_searches::{
    create_saved_search, delete_saved_search, list_saved_searches, CreateSavedSearch, SavedSearchView,
};
pub use search::{
    run_search, search_catalog, RawSearch, SearchHit, SearchQuery, SearchResponse, ENGINE_VERSION,
};
pub use wishlists::{
    add_wishlist_item, create_wishlist, list_items, list_wishlists, remove_wishlist_item,
    update_wishlist, CreateWishlist, UpdateWishlist, WishlistItem, WishlistView,
};

pub fn public_routes<S: Shared>() -> Router<S> {
    use discovery::handlers as public;
    Router::new()
        .route("/destinations", get(public::destinations::<S>))
        .route("/properties", get(public::search::<S>))
        .route("/search/catalog", get(public::catalog::<S>))
        .route("/properties/:property_id", get(public::property::<S>))
        .route("/properties/:property_id/availability", get(public::property_availability::<S>))
        .route("/properties/:property_id/calendar", get(public::property_calendar::<S>))
        .route("/media/:media_id/content", get(public::media_content::<S>))
}

pub fn guest_routes<S: Shared>() -> Router<S> {
    use saved_searches::handlers as saved;
    use wishlists::handlers as lists;
    Router::new()
        .route("/wishlists", get(lists::list::<S>).post(lists::create::<S>))
        .route("/wishlists/:wishlist_id", patch(lists::update::<S>))
        .route("/wishlists/:wishlist_id/items", get(lists::items::<S>))
        .route(
            "/wishlists/:wishlist_id/items/:property_id",
            post(lists::add_item::<S>).delete(lists::remove_item::<S>),
        )
        .route("/saved-searches", get(saved::list::<S>).post(saved::create::<S>))
        .route("/saved-searches/:saved_search_id", delete(saved::remove::<S>))
}

pub fn host_routes<S: Shared>() -> Router<S> {
    Router::new()
}

pub fn admin_routes<S: Shared>() -> Router<S> {
    Router::new()
}

#[derive(OpenApi)]
#[openapi(paths(
    discovery::handlers::destinations,
    discovery::handlers::search,
    discovery::handlers::catalog,
    discovery::handlers::property,
    discovery::handlers::property_availability,
    discovery::handlers::property_calendar,
    discovery::handlers::media_content,
    wishlists::handlers::list,
    wishlists::handlers::create,
    wishlists::handlers::update,
    wishlists::handlers::items,
    wishlists::handlers::add_item,
    wishlists::handlers::remove_item,
    saved_searches::handlers::list,
    saved_searches::handlers::create,
    saved_searches::handlers::remove,
))]
struct Doc;

pub fn openapi() -> utoipa::openapi::OpenApi {
    Doc::openapi()
}
