use chrono::{DateTime, NaiveDate, Utc};
use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::{codes, Error, Result};
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Kernel, Page, PageRequest};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct GuestBookingsQuery {
    pub city: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GuestBooking {
    pub id: String,
    pub reference: String,
    pub state: String,
    pub city: String,
    pub operator_name: String,
    pub property_name_en: String,
    pub property_name_fr: String,
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub nights: Option<i32>,
    pub total_amount: String,
    pub created_at: DateTime<Utc>,
}

pub async fn list(
    kernel: &Kernel,
    actor: &Actor,
    user_id: &str,
    query: GuestBookingsQuery,
) -> Result<Page<GuestBooking>> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let user_id = user_id.trim();
    if user_id.is_empty() {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    let city = match query.city.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => City::from_code(value)?,
        None => actor.city,
    };
    if !kernel.config.serves(city) {
        return Err(Error::bad_request(codes::CITY_UNSUPPORTED));
    }
    let default = PageRequest::default();
    let page = PageRequest {
        page: query.page.unwrap_or(default.page),
        page_size: query.page_size.unwrap_or(default.page_size),
    }.validated()?;
    let pool = kernel.db.pool();
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM stay_reservations r JOIN hosts h ON h.id = r.\"operatorId\" AND h.city_id = r.city_id JOIN stay_properties p ON p.id = r.\"propertyId\" AND p.city_id = r.city_id WHERE r.city_id = $1 AND r.\"guestUserId\" = $2",
    ).bind(city.code()).bind(user_id).fetch_one(pool).await?;
    let items = sqlx::query_as::<_, GuestBooking>(r#"
        SELECT r.id, r.reference, r.state, r.city_id AS city, h.display_name AS operator_name,
               p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr,
               dates.check_in, dates.check_out,
               (dates.check_out - dates.check_in)::int AS nights,
               r."totalAmount"::text AS total_amount, r."createdAt" AS created_at
        FROM stay_reservations r
        JOIN hosts h ON h.id = r."operatorId" AND h.city_id = r.city_id
        JOIN stay_properties p ON p.id = r."propertyId" AND p.city_id = r.city_id
        LEFT JOIN LATERAL (
            SELECT MIN(i."checkIn") AS check_in, MAX(i."checkOut") AS check_out
            FROM stay_reservation_items i WHERE i."reservationId" = r.id
        ) dates ON true
        WHERE r.city_id = $1 AND r."guestUserId" = $2
        ORDER BY r."createdAt" DESC, r.id LIMIT $3 OFFSET $4
    "#).bind(city.code()).bind(user_id).bind(page.limit()).bind(page.offset()).fetch_all(pool).await?;
    Ok(Page::new(items, total, page))
}
