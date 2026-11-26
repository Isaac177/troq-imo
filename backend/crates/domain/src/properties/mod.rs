pub mod media;
pub mod amenities;

mod catalog_docs;
mod catalog_routes;
mod categories;
mod listings;
mod template_media;
mod templates;
mod docs;
mod events;
mod facts;
mod http;
mod messages;
mod property;
mod revisions;
mod routes;
mod scope;
mod validation;

pub use categories::{active_codes as active_category_codes, ensure_selectable as ensure_category_selectable};
pub use property::queries::{find as find_property, lock as lock_property};
pub use property::types::{PropertyView, PROPERTY_STATUSES};
pub use routes::{admin_routes, guest_routes, host_routes, public_routes};
pub use scope::{admin_property, ensure_moderator, property_for, property_for_actor};
pub use template_media::read::covers_for_templates;

pub fn openapi() -> utoipa::openapi::OpenApi {
    docs::document()
}
