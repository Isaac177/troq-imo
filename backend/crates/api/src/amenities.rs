use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::Json;
use domain::properties::amenities::{self, Amenity, AmenityGroup};
use kernel::auth::RequestId;
use kernel::envelope::Envelope;
use kernel::error::{codes::MALFORMED_REQUEST, Error, Result};
use kernel::kernel::Kernel;
use kernel::paging::{Page, PageRequest};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct GroupsQuery {
    locale: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct AmenitiesQuery {
    locale: Option<String>,
    group_code: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

fn locale(value: &Option<String>) -> &str {
    value.as_deref().map(str::trim).filter(|value| !value.is_empty()).unwrap_or("en")
}

fn paging(page: Option<u32>, page_size: Option<u32>) -> PageRequest {
    let default = PageRequest::default();
    PageRequest { page: page.unwrap_or(default.page), page_size: page_size.unwrap_or(default.page_size) }
}

#[utoipa::path(
    get, path = "/api/public/amenity-groups", operation_id = "amenity_groups_list",
    params(GroupsQuery, ("x-request-id" = Option<String>, Header)),
    responses(
        (status = 200, body = Envelope<Page<AmenityGroup>>, headers(("x-request-id" = String))),
        (status = 400, body = kernel::error::ErrorBody, headers(("x-request-id" = String))),
        (status = 500, body = kernel::error::ErrorBody, headers(("x-request-id" = String)))
    )
)]
pub async fn groups(
    State(kernel): State<Kernel>,
    RequestId(request_id): RequestId,
    query: std::result::Result<Query<GroupsQuery>, QueryRejection>,
) -> Result<Json<Envelope<Page<AmenityGroup>>>> {
    let Query(query) = query.map_err(|_| Error::bad_request(MALFORMED_REQUEST))?;
    let page = amenities::list_active_groups(kernel.db.pool(), locale(&query.locale), paging(query.page, query.page_size)).await?;
    Ok(Json(Envelope::ok(page, request_id)))
}

#[utoipa::path(
    get, path = "/api/public/amenities", operation_id = "amenities_list",
    params(AmenitiesQuery, ("x-request-id" = Option<String>, Header)),
    responses(
        (status = 200, body = Envelope<Page<Amenity>>, headers(("x-request-id" = String))),
        (status = 400, body = kernel::error::ErrorBody, headers(("x-request-id" = String))),
        (status = 500, body = kernel::error::ErrorBody, headers(("x-request-id" = String)))
    )
)]
pub async fn amenities(
    State(kernel): State<Kernel>,
    RequestId(request_id): RequestId,
    query: std::result::Result<Query<AmenitiesQuery>, QueryRejection>,
) -> Result<Json<Envelope<Page<Amenity>>>> {
    let Query(query) = query.map_err(|_| Error::bad_request(MALFORMED_REQUEST))?;
    let page = amenities::list_active_amenities(
        kernel.db.pool(), locale(&query.locale), query.group_code.as_deref(), paging(query.page, query.page_size),
    ).await?;
    Ok(Json(Envelope::ok(page, request_id)))
}

#[cfg(test)]
mod tests {
    use super::{locale, paging};

    #[test]
    fn uses_requested_locale_or_english() {
        assert_eq!(locale(&Some(" fr ".to_owned())), "fr");
        assert_eq!(locale(&Some(" ".to_owned())), "en");
        assert_eq!(locale(&None), "en");
    }

    #[test]
    fn pagination_defaults_and_validation_are_preserved() {
        let page = paging(None, None).validated().unwrap();
        assert_eq!((page.page, page.page_size), (1, 20));
        assert!(paging(Some(0), None).validated().is_err());
        assert!(paging(None, Some(101)).validated().is_err());
    }
}
