use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{ErrorBody, Result};
use kernel::kernel::KernelState;
use kernel::paging::Page;
use kernel::permissions::names::PROPERTIES_CONFIGURE;
use serde::Deserialize;
use utoipa::IntoParams;

use super::types::{Amenity, AmenityFilters, AmenityGroup, AmenityWrite, GroupFilters, GroupWrite, SavedCatalogItem};
use super::{create_amenity, create_group, list_amenities, list_groups, update_amenity, update_group};
use crate::properties::http::{paging, JsonBody, Params};

type Reply<T> = Result<Json<Envelope<T>>>;

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct AdminCatalogQuery {
    pub locale: Option<String>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub group_code: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

fn locale(query: &AdminCatalogQuery) -> &str {
    query.locale.as_deref().map(str::trim).filter(|value| !value.is_empty()).unwrap_or("en")
}

fn active(query: &AdminCatalogQuery) -> Option<bool> {
    match query.status.as_deref() {
        Some("ACTIVE") => Some(true),
        Some("RETIRED") => Some(false),
        _ => None,
    }
}

#[utoipa::path(
    get, path = "/api/admin/amenity-groups", operation_id = "admin_amenity_groups_list", params(AdminCatalogQuery),
    responses((status = 200, body = Envelope<Page<AmenityGroup>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn admin_groups<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Params(query): Params<AdminCatalogQuery>) -> Reply<Page<AmenityGroup>>
where S: KernelState + Clone + Send + Sync + 'static {
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    let filters = GroupFilters { active: active(&query), search: query.search.clone() };
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(list_groups(state.kernel().db.pool(), locale(&query), &filters, page).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/amenity-groups", operation_id = "admin_amenity_groups_create", request_body = GroupWrite,
    responses((status = 200, body = Envelope<SavedCatalogItem>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_group_create<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, JsonBody(input): JsonBody<GroupWrite>) -> Reply<SavedCatalogItem>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(create_group(state.kernel().db.pool(), &actor, input).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/admin/amenity-groups/{code}", operation_id = "admin_amenity_groups_update",
    params(("code" = String, Path)), request_body = GroupWrite,
    responses((status = 200, body = Envelope<SavedCatalogItem>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_group_update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(code): Path<String>, JsonBody(input): JsonBody<GroupWrite>) -> Reply<SavedCatalogItem>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(update_group(state.kernel().db.pool(), &actor, &code, input).await?, rid)))
}

#[utoipa::path(
    get, path = "/api/admin/amenities", operation_id = "admin_amenities_list", params(AdminCatalogQuery),
    responses((status = 200, body = Envelope<Page<Amenity>>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn admin_amenities<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Params(query): Params<AdminCatalogQuery>) -> Reply<Page<Amenity>>
where S: KernelState + Clone + Send + Sync + 'static {
    actor.require_platform(PROPERTIES_CONFIGURE)?;
    let filters = AmenityFilters { active: active(&query), group_code: query.group_code.clone(), search: query.search.clone() };
    let page = paging(query.page, query.page_size)?;
    Ok(Json(Envelope::ok(list_amenities(state.kernel().db.pool(), locale(&query), &filters, page).await?, rid)))
}

#[utoipa::path(
    post, path = "/api/admin/amenities", operation_id = "admin_amenities_create", request_body = AmenityWrite,
    responses((status = 200, body = Envelope<SavedCatalogItem>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 409, body = ErrorBody))
)]
pub async fn admin_amenity_create<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, JsonBody(input): JsonBody<AmenityWrite>) -> Reply<SavedCatalogItem>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(create_amenity(state.kernel().db.pool(), &actor, input).await?, rid)))
}

#[utoipa::path(
    patch, path = "/api/admin/amenities/{code}", operation_id = "admin_amenities_update",
    params(("code" = String, Path)), request_body = AmenityWrite,
    responses((status = 200, body = Envelope<SavedCatalogItem>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn admin_amenity_update<S>(State(state): State<S>, actor: Actor, RequestId(rid): RequestId, Path(code): Path<String>, JsonBody(input): JsonBody<AmenityWrite>) -> Reply<SavedCatalogItem>
where S: KernelState + Clone + Send + Sync + 'static {
    Ok(Json(Envelope::ok(update_amenity(state.kernel().db.pool(), &actor, &code, input).await?, rid)))
}
