use axum::routing::{delete, get, patch, post};
use axum::Router;
use kernel::KernelState;

use super::assignments::handler as assignments;
use super::availability::handler as availability;
use super::blocks::handler as blocks;
use super::calendar::handler as calendar;
use super::restrictions::handler as restrictions;
use super::unit_types::handler as unit_types;

pub fn public_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new().route("/calendar/feeds/:city/:token", get(calendar::public_feed::<S>))
}

pub fn guest_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
}

pub fn host_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/properties/:property_id/restrictions", get(restrictions::host_list::<S>))
        .route("/restrictions/bulk", post(restrictions::host_bulk::<S>))
        .route("/properties/:property_id/blocks", get(blocks::host_list::<S>))
        .route("/blocks", post(blocks::host_create::<S>))
        .route("/blocks/:block_id", get(blocks::host_get::<S>).delete(blocks::host_remove::<S>))
        .route("/availability", get(availability::host_get::<S>))
        .route("/unit-assignments", post(assignments::host_assign::<S>))
        .route("/unit-assignments/:assignment_id/move", post(assignments::host_move::<S>))
        .route("/reservation-items/:item_id/unit-assignments", get(assignments::host_history::<S>))
        .route("/reservation-items/:item_id/unit-options", get(assignments::host_options::<S>))
        .route("/calendar", get(calendar::host_get::<S>))
        .route("/calendar/inventory-default", post(calendar::host_defaults::<S>))
        .route("/calendar/feed.ics", get(calendar::host_feed_ics::<S>))
        .route(
            "/calendar/feed-tokens",
            get(calendar::host_tokens::<S>).post(calendar::host_token_create::<S>),
        )
        .route("/calendar/feed-tokens/:token_id", delete(calendar::host_token_revoke::<S>))
        .route(
            "/properties/:property_id/unit-types",
            get(unit_types::host_list::<S>).post(unit_types::host_create::<S>),
        )
        .route(
            "/properties/:property_id/unit-types/:unit_type_id",
            patch(unit_types::host_update::<S>).delete(unit_types::host_archive::<S>),
        )
}

pub fn admin_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/calendar", get(calendar::admin_get::<S>))
        .route("/blocks", get(blocks::admin_list::<S>).post(blocks::admin_create::<S>))
        .route("/blocks/:block_id/remove", post(blocks::admin_remove::<S>))
        .route("/inventory-defaults", post(calendar::admin_defaults::<S>))
        .route("/restrictions/bulk", post(restrictions::admin_bulk::<S>))
        .route(
            "/properties/:property_id/unit-types",
            get(unit_types::admin_list::<S>).post(unit_types::admin_create::<S>),
        )
        .route(
            "/properties/:property_id/unit-types/:unit_type_id",
            patch(unit_types::admin_update::<S>).delete(unit_types::admin_archive::<S>),
        )
}
