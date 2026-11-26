mod amounts;
mod http;
mod idempotency;
mod messages;
mod scope;
mod validation;

pub mod promotions;
pub mod quotes;
pub mod rates;
pub mod settings;

use axum::Router;
use kernel::KernelState;

pub use messages::*;

pub fn public_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
}

pub fn guest_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    http::guest_routes::<S>()
}

pub fn host_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    http::host_routes::<S>()
}

pub fn admin_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    http::admin_routes::<S>()
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    http::document()
}
