use utoipa::OpenApi;

use super::amenities::handler as amenities;
use super::categories::handler as categories;
use super::listings::admin_handler as listings;
use super::media::handler as media;
use super::template_media::handler as template_media;
use super::templates::handler as templates;

#[derive(OpenApi)]
#[openapi(paths(
    categories::public_catalog,
    categories::admin_list,
    categories::admin_create,
    categories::admin_update,
    amenities::admin_groups,
    amenities::admin_group_create,
    amenities::admin_group_update,
    amenities::admin_amenities,
    amenities::admin_amenity_create,
    amenities::admin_amenity_update,
    media::host_list,
    media::host_add,
    media::host_update,
    media::host_remove,
    media::host_content,
    media::admin_list,
    media::admin_add,
    media::admin_update,
    media::admin_remove,
    media::admin_content,
    templates::host_list,
    templates::host_create,
    templates::host_update,
    templates::admin_list,
    templates::admin_create,
    templates::admin_update,
    template_media::host_list,
    template_media::host_add,
    template_media::host_update,
    template_media::host_remove,
    template_media::host_content,
    template_media::admin_list,
    template_media::admin_add,
    template_media::admin_update,
    template_media::admin_remove,
    template_media::admin_content,
    listings::queue,
    listings::revision,
    listings::decide,
    listings::list,
    listings::get,
    listings::revisions,
    listings::create,
    listings::update,
    listings::suspend,
    listings::reinstate,
    listings::lifecycle,
))]
struct CatalogDoc;

pub fn document() -> utoipa::openapi::OpenApi {
    CatalogDoc::openapi()
}
