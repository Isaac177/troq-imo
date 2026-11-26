pub mod assignments;
pub mod availability;
pub mod blocks;
pub mod calendar;
mod docs;
mod http;
pub mod messages;
pub mod nights;
pub mod pools;
mod query;
pub mod restrictions;
mod routes;
pub mod scope;
pub mod unit_types;

#[cfg(test)]
mod db_tests;
#[cfg(test)]
mod test_support;

pub use routes::{admin_routes, guest_routes, host_routes, public_routes};

pub fn openapi() -> utoipa::openapi::OpenApi {
    docs::document()
}
