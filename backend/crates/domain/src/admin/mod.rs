mod report;
mod report_handler;
mod report_queries;
mod report_types;
mod report_validation;
mod reservation_detail;
mod reservation_detail_handler;
mod reservation_detail_media;
mod reservation_detail_queries;
mod reservation_detail_types;
mod reservation_filters;
mod reservation_handlers;
mod reservation_list;
mod reservation_queries;
mod reservation_types;
mod dashboard;
mod dashboard_handler;
mod dashboard_hosts;
mod dashboard_queries;
mod dashboard_types;
mod guest_bookings;
mod guest_handlers;
mod guests;
mod handlers;

use axum::routing::get;
use axum::Router;
use kernel::KernelState;
use utoipa::OpenApi;

pub use guest_bookings::{GuestBooking, GuestBookingsQuery};
pub use guests::{GuestFilter, GuestItem};
pub use reservation_types::{ReservationFilter, ReservationRow, ReservationSummary};
pub use dashboard_types::{CityFigures, DashboardDay, DashboardFigures, DashboardQuery, DashboardSummary};

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
        .route("/analytics/summary", get(report_handler::summary::<S>))
        .route("/dashboard", get(dashboard_handler::host_summary::<S>))
}

pub fn admin_routes<S>() -> Router<S>
where S: KernelState + Clone + Send + Sync + 'static {
    Router::new()
        .route("/guests", get(guest_handlers::list::<S>))
        .route("/guests/:user_id", get(guest_handlers::detail::<S>))
        .route("/guests/:userId/bookings", get(handlers::guest_bookings::<S>))
        .route("/dashboard", get(dashboard_handler::summary::<S>))
        .route("/dashboard/hosts", get(dashboard_handler::hosts::<S>))
        .route("/reservations", get(reservation_handlers::list::<S>))
        .route("/reservations/summary", get(reservation_handlers::summary::<S>))
        .route("/reservations/export", get(reservation_handlers::export::<S>))
        .route("/reservations/:reservation_id", get(reservation_detail_handler::get::<S>))
}

#[derive(OpenApi)]
#[openapi(paths(handlers::guest_bookings, guest_handlers::list, guest_handlers::detail, dashboard_handler::summary, dashboard_handler::hosts, dashboard_handler::host_summary, reservation_handlers::list, reservation_handlers::summary, reservation_handlers::export, reservation_detail_handler::get, report_handler::summary), components(schemas(GuestBooking, GuestItem, CityFigures, DashboardDay, DashboardFigures, DashboardSummary, dashboard_hosts::HostFigures, ReservationRow, ReservationSummary, report_types::AnalyticsSummary)))]
struct Doc;

pub fn openapi() -> utoipa::openapi::OpenApi {
    Doc::openapi()
}
