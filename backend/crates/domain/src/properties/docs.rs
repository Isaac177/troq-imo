use utoipa::OpenApi;

use super::property::handler;
use super::revisions::handler as revisions;

#[derive(OpenApi)]
#[openapi(paths(handler::list, handler::get, handler::create, revisions::list, revisions::get, revisions::create, revisions::update, revisions::withdraw, revisions::readiness, revisions::submit, revisions::lifecycle))]
struct PropertiesDoc;

pub fn document() -> utoipa::openapi::OpenApi {
    let mut document = PropertiesDoc::openapi();
    document.merge(super::catalog_docs::document());
    document
}
