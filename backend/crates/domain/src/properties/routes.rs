use axum::routing::get;
use axum::Router;
use kernel::KernelState;

use super::property::handler;
use super::revisions::handler as revisions;

pub fn public_routes<S>() -> Router<S> where S: KernelState + Clone + Send + Sync + 'static {
    super::catalog_routes::public()
}

pub fn guest_routes<S>() -> Router<S> where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
}

pub fn host_routes<S>() -> Router<S> where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
        .route("/properties", get(handler::list::<S>).post(handler::create::<S>))
        .route("/properties/:property_id", get(handler::get::<S>))
        .route("/properties/:property_id/lifecycle", axum::routing::post(revisions::lifecycle::<S>))
        .route("/properties/:property_id/revisions", get(revisions::list::<S>).post(revisions::create::<S>))
        .route("/properties/:property_id/revisions/:revision_id", get(revisions::get::<S>).patch(revisions::update::<S>))
        .route("/properties/:property_id/revisions/:revision_id/readiness", get(revisions::readiness::<S>))
        .route("/properties/:property_id/revisions/:revision_id/submit", axum::routing::post(revisions::submit::<S>))
        .route("/properties/:property_id/revisions/:revision_id/withdraw", axum::routing::post(revisions::withdraw::<S>))
        .merge(super::catalog_routes::host())
}

pub fn admin_routes<S>() -> Router<S> where S: KernelState + Clone + Send + Sync + 'static {
    super::catalog_routes::admin()
}
