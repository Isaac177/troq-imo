use utoipa::OpenApi;

use super::assignments::handler as assignments;
use super::availability::handler as availability;
use super::blocks::handler as blocks;
use super::calendar::handler as calendar;
use super::restrictions::handler as restrictions;
use super::unit_types::handler as unit_types;

#[derive(OpenApi)]
#[openapi(paths(
    restrictions::host_list,
    restrictions::host_bulk,
    restrictions::admin_bulk,
    blocks::host_create,
    blocks::host_list,
    blocks::host_get,
    blocks::host_remove,
    blocks::admin_list,
    blocks::admin_create,
    blocks::admin_remove,
    availability::host_get,
    assignments::host_assign,
    assignments::host_move,
    assignments::host_history,
    assignments::host_options,
    calendar::host_get,
    calendar::admin_get,
    calendar::host_defaults,
    calendar::admin_defaults,
    calendar::host_feed_ics,
    calendar::host_tokens,
    calendar::host_token_create,
    calendar::host_token_revoke,
    calendar::public_feed,
    unit_types::host_list,
    unit_types::host_create,
    unit_types::host_update,
    unit_types::host_archive,
    unit_types::admin_list,
    unit_types::admin_create,
    unit_types::admin_update,
    unit_types::admin_archive,
))]
struct InventoryDoc;

pub fn document() -> utoipa::openapi::OpenApi {
    InventoryDoc::openapi()
}
