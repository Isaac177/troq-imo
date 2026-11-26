use axum::extract::{Path, State};
use axum::Json;
use kernel::auth::{RequestCity, RequestId};
use kernel::envelope::Envelope;
use kernel::error::Result;
use serde_json::Value;

use super::types::{
    AvailabilityQuery, AvailabilityView, CalendarQuery, CalendarView, DestinationView, LocaleQuery,
    MediaContentView, PropertyDetailView,
};
use super::{availability, calendar, detail, media, queries};
use crate::guests::extract::{ApiQuery, Shared};
use crate::guests::locale::clean_locale;
use crate::guests::search::{
    amenity_codes, category_codes, run_search, search_catalog, SearchParams, SearchQuery,
    SearchResponse,
};

#[utoipa::path(
    get, path = "/api/public/destinations", operation_id = "guests_destinations_list", tag = "guests",
    responses((status = 200, body = Envelope<Vec<DestinationView>>))
)]
pub async fn destinations<S: Shared>(
    State(state): State<S>,
    RequestCity(city): RequestCity,
    RequestId(request_id): RequestId,
) -> Result<Json<Envelope<Vec<DestinationView>>>> {
    let count = queries::published_count(state.kernel().db.pool(), city).await?;
    let view = DestinationView { city: city.code().to_string(), property_count: count };
    Ok(Json(Envelope::ok(vec![view], request_id)))
}

#[utoipa::path(
    get, path = "/api/public/properties", operation_id = "guests_properties_search", tag = "guests",
    params(SearchParams),
    responses((status = 200, body = Envelope<SearchResponse>))
)]
pub async fn search<S: Shared>(
    State(state): State<S>,
    RequestCity(city): RequestCity,
    RequestId(request_id): RequestId,
    ApiQuery(params): ApiQuery<SearchParams>,
) -> Result<Json<Envelope<SearchResponse>>> {
    let kernel = state.kernel();
    let locale = clean_locale(params.locale.as_deref());
    let raw = params.into_raw();
    let consent = raw.analytics_consent;
    let query = SearchQuery::from_raw(raw)?;
    let response = run_search(kernel.db.pool(), &kernel.storage, city, &locale, query, consent).await?;
    Ok(Json(Envelope::ok(response, request_id)))
}

#[utoipa::path(
    get, path = "/api/public/search/catalog", operation_id = "guests_search_catalog", tag = "guests",
    responses((status = 200, body = Envelope<serde_json::Value>))
)]
pub async fn catalog<S: Shared>(
    State(state): State<S>,
    RequestId(request_id): RequestId,
) -> Result<Json<Envelope<Value>>> {
    let pool = state.kernel().db.pool();
    let value = search_catalog(&amenity_codes(pool).await?, &category_codes(pool).await?);
    Ok(Json(Envelope::ok(value, request_id)))
}

#[utoipa::path(
    get, path = "/api/public/properties/{propertyId}", operation_id = "guests_property_get", tag = "guests",
    params(("propertyId" = String, Path), LocaleQuery),
    responses((status = 200, body = Envelope<PropertyDetailView>), (status = 404, body = kernel::error::ErrorBody, description = "Property not found"))
)]
pub async fn property<S: Shared>(
    State(state): State<S>,
    RequestCity(city): RequestCity,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    ApiQuery(query): ApiQuery<LocaleQuery>,
) -> Result<Json<Envelope<PropertyDetailView>>> {
    let kernel = state.kernel();
    let locale = clean_locale(query.locale.as_deref());
    let view = detail::property_detail(kernel.db.pool(), &kernel.storage, city, &property_id, &locale).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/public/properties/{propertyId}/availability", operation_id = "guests_property_availability", tag = "guests",
    params(("propertyId" = String, Path), AvailabilityQuery),
    responses((status = 200, body = Envelope<AvailabilityView>), (status = 404, body = kernel::error::ErrorBody, description = "Property not found"))
)]
pub async fn property_availability<S: Shared>(
    State(state): State<S>,
    RequestCity(city): RequestCity,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    ApiQuery(query): ApiQuery<AvailabilityQuery>,
) -> Result<Json<Envelope<AvailabilityView>>> {
    let kernel = state.kernel();
    let view = availability::availability(kernel.db.pool(), &kernel.storage, city, &property_id, &query).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/public/properties/{propertyId}/calendar", operation_id = "guests_property_calendar", tag = "guests",
    params(("propertyId" = String, Path), CalendarQuery),
    responses((status = 200, body = Envelope<CalendarView>), (status = 404, body = kernel::error::ErrorBody, description = "Property or unit type not found"))
)]
pub async fn property_calendar<S: Shared>(
    State(state): State<S>,
    RequestCity(city): RequestCity,
    RequestId(request_id): RequestId,
    Path(property_id): Path<String>,
    ApiQuery(query): ApiQuery<CalendarQuery>,
) -> Result<Json<Envelope<CalendarView>>> {
    let view = calendar::calendar(state.kernel().db.pool(), city, &property_id, &query).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}

#[utoipa::path(
    get, path = "/api/public/media/{mediaId}/content", operation_id = "guests_media_content", tag = "guests",
    params(("mediaId" = String, Path), LocaleQuery),
    responses((status = 200, body = Envelope<MediaContentView>), (status = 404, body = kernel::error::ErrorBody, description = "Media not found"))
)]
pub async fn media_content<S: Shared>(
    State(state): State<S>,
    RequestCity(city): RequestCity,
    RequestId(request_id): RequestId,
    Path(media_id): Path<String>,
    ApiQuery(query): ApiQuery<LocaleQuery>,
) -> Result<Json<Envelope<MediaContentView>>> {
    let kernel = state.kernel();
    let locale = clean_locale(query.locale.as_deref());
    let view = media::media_content(kernel.db.pool(), &kernel.storage, city, &media_id, &locale).await?;
    Ok(Json(Envelope::ok(view, request_id)))
}
