mod admin_handlers;
mod admin_list;
mod codes;
mod covers;
mod events;
mod extract;
mod guest_handlers;
mod host_handlers;
mod list_queries;
mod moderation;
mod property_reviews;
mod public_handlers;
mod queries;
mod reply;
mod sql;
mod submit;
mod types;
mod validators;

use axum::routing::{get, post};
use axum::Router;
use kernel::KernelState;
use utoipa::OpenApi;

pub use admin_list::{AdminReviewItem, AdminReviewQuery};
pub use codes::*;
pub use events::{REVIEW_REPLIED, REVIEW_SUBMITTED};
pub use moderation::{decide_review, moderation_queue};
pub use property_reviews::list_property_reviews;
pub use reply::reply_review;
pub use submit::submit_review;
pub use types::{
    DecideReviewInput, ReplyReviewInput, ReviewView, SubmitReviewInput, APPROVED, HIDDEN, PENDING,
};

pub fn public_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new().route("/properties/:propertyId/reviews", get(public_handlers::list::<S>))
}

pub fn guest_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new().route("/reservations/:reservationId/review", post(guest_handlers::submit::<S>))
}

pub fn host_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/reviews/:reviewId/reply", post(host_handlers::reply::<S>))
        .route("/properties/:propertyId/reviews", get(host_handlers::list::<S>))
}

pub fn admin_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/reviews", get(admin_handlers::list::<S>))
        .route("/reviews/moderation-queue", get(admin_handlers::queue::<S>))
        .route("/reviews/:reviewId/moderation", post(admin_handlers::decide::<S>))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        public_handlers::list,
        guest_handlers::submit,
        host_handlers::reply,
        host_handlers::list,
        admin_handlers::list,
        admin_handlers::queue,
        admin_handlers::decide
    ),
    components(schemas(ReviewView, AdminReviewItem, SubmitReviewInput, ReplyReviewInput, DecideReviewInput))
)]
struct Doc;

pub fn openapi() -> utoipa::openapi::OpenApi {
    Doc::openapi()
}
