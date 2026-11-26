use axum::routing::{get, patch, post};
use axum::Router;
use kernel::KernelState;

use super::amenities::handler as amenities;
use super::categories::handler as categories;
use super::listings::admin_handler as listings;
use super::media::handler as media;
use super::template_media::handler as template_media;
use super::templates::handler as templates;

pub fn public<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new().route("/accommodation-categories", get(categories::public_catalog::<S>))
}

pub fn host<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/properties/:property_id/media", get(media::host_list::<S>).post(media::host_add::<S>))
        .route(
            "/properties/:property_id/media/:media_id",
            patch(media::host_update::<S>).delete(media::host_remove::<S>),
        )
        .route("/media/:media_id/content", get(media::host_content::<S>))
        .route(
            "/accommodation-templates",
            get(templates::host_list::<S>).post(templates::host_create::<S>),
        )
        .route("/accommodation-templates/:template_id", patch(templates::host_update::<S>))
        .route(
            "/accommodation-templates/:template_id/media",
            get(template_media::host_list::<S>).post(template_media::host_add::<S>),
        )
        .route(
            "/accommodation-templates/:template_id/media/:media_id",
            patch(template_media::host_update::<S>).delete(template_media::host_remove::<S>),
        )
        .route(
            "/accommodation-templates/:template_id/media/:media_id/content",
            get(template_media::host_content::<S>),
        )
}

pub fn admin<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/listings/queue", get(listings::queue::<S>))
        .route("/listings/:revision_id", get(listings::revision::<S>))
        .route("/listings/:revision_id/decision", post(listings::decide::<S>))
        .route("/properties", get(listings::list::<S>).post(listings::create::<S>))
        .route("/properties/:property_id", get(listings::get::<S>).patch(listings::update::<S>))
        .route("/properties/:property_id/revisions", get(listings::revisions::<S>))
        .route("/properties/:property_id/suspend", post(listings::suspend::<S>))
        .route("/properties/:property_id/reinstate", post(listings::reinstate::<S>))
        .route("/properties/:property_id/lifecycle", post(listings::lifecycle::<S>))
        .route("/properties/:property_id/media", get(media::admin_list::<S>).post(media::admin_add::<S>))
        .route(
            "/properties/:property_id/media/:media_id",
            patch(media::admin_update::<S>).delete(media::admin_remove::<S>),
        )
        .route("/media/:media_id/content", get(media::admin_content::<S>))
        .route(
            "/accommodation-categories",
            get(categories::admin_list::<S>).post(categories::admin_create::<S>),
        )
        .route("/accommodation-categories/:code", patch(categories::admin_update::<S>))
        .route(
            "/accommodation-templates",
            get(templates::admin_list::<S>).post(templates::admin_create::<S>),
        )
        .route("/accommodation-templates/:template_id", patch(templates::admin_update::<S>))
        .route(
            "/accommodation-templates/:template_id/media",
            get(template_media::admin_list::<S>).post(template_media::admin_add::<S>),
        )
        .route(
            "/accommodation-templates/:template_id/media/:media_id",
            patch(template_media::admin_update::<S>).delete(template_media::admin_remove::<S>),
        )
        .route(
            "/accommodation-templates/:template_id/media/:media_id/content",
            get(template_media::admin_content::<S>),
        )
        .route("/amenity-groups", get(amenities::admin_groups::<S>).post(amenities::admin_group_create::<S>))
        .route("/amenity-groups/:code", patch(amenities::admin_group_update::<S>))
        .route("/amenities", get(amenities::admin_amenities::<S>).post(amenities::admin_amenity_create::<S>))
        .route("/amenities/:code", patch(amenities::admin_amenity_update::<S>))
}
