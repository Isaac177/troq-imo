use axum::routing::{delete, get, post};
use axum::Router;
use kernel::KernelState;
use utoipa::OpenApi;

use super::onboarding::handlers;
use super::onboarding::types::{
    AddDocumentInput, AdminApplicationView, AdminHostView, ApplicationFilter, ApplicationInput,
    ApplicationView, HostDecisionInput, HostDocumentView, HostStatusInput, HostStatusView, OperationNotice,
};
use super::onboarding::DocumentReview;

pub fn public_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
}

pub fn guest_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
}

pub fn host_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
        .route("/applications", get(handlers::list_mine::<S>).post(handlers::save::<S>))
        .route("/applications/:application_id/submit", post(handlers::submit::<S>))
        .route("/applications/:application_id/withdraw", post(handlers::withdraw::<S>))
        .route("/applications/:application_id/documents", post(handlers::add_document::<S>))
        .route("/applications/:application_id/documents/:document_id", delete(handlers::remove_document::<S>))
}

pub fn admin_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
        .route("/host-applications", get(handlers::admin_list::<S>))
        .route("/host-applications/:application_id", get(handlers::admin_detail::<S>))
        .route("/host-applications/:application_id/decision", post(handlers::admin_decide::<S>))
        .route("/host-applications/:application_id/documents/:document_id/review", post(handlers::admin_review_document::<S>))
        .route("/hosts", get(handlers::admin_hosts::<S>))
        .route("/hosts/:host_id/status", post(handlers::admin_host_status::<S>))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::list_mine, handlers::save, handlers::submit, handlers::withdraw,
        handlers::add_document, handlers::remove_document, handlers::admin_list, handlers::admin_detail,
        handlers::admin_decide, handlers::admin_review_document, handlers::admin_hosts, handlers::admin_host_status
    ),
    components(schemas(
        ApplicationInput, ApplicationView, ApplicationFilter, AddDocumentInput, HostDocumentView,
        AdminApplicationView, AdminHostView, HostDecisionInput, DocumentReview, HostStatusInput,
        HostStatusView, OperationNotice
    ))
)]
struct Doc;

pub fn openapi() -> utoipa::openapi::OpenApi {
    Doc::openapi()
}
