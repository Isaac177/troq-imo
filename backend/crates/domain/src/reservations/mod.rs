mod codes;
mod contract;
mod events;
mod http;
mod support;

#[cfg(test)]
mod test_support;

pub mod amendments;
pub mod cases;
pub mod claims;
pub mod holds;
pub mod lifecycle;
pub mod links;
pub mod messages;
pub mod operations;

use axum::Router;
use kernel::KernelState;

pub use codes::*;
pub use contract::{DiscountView, NightLine};
pub use lifecycle::state::{can_move, RESERVATION_STATES};
pub use lifecycle::assign_physical_unit;
pub use lifecycle::types::{ReservationView, OperatorReservationView, HoldRequest, GuestInput};
pub use lifecycle::decision::apply_decision;
pub use lifecycle::platform_cancel::{platform_cancel, preview as platform_cancel_preview};
pub use lifecycle::view::view as reservation_view;
pub use events::types as event_types;
pub use holds::expiry::{expire_due_holds, sweep as sweep_expired_holds};
pub use holds::hold::create_hold;

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
