mod amounts;
mod http;
mod messages;
mod scope;
mod settings;
mod validators;

pub mod admin;
pub mod commission;
pub mod credits;

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
    Router::new()
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
