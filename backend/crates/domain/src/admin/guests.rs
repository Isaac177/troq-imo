use chrono::{DateTime, Utc};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::PROPERTIES_MODERATE;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct GuestFilter {
    pub search: Option<String>,
    pub flag: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GuestItem {
    pub user_id: String,
    pub full_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub bookings: i64,
    pub active_bookings: i64,
    pub cancelled_bookings: i64,
    pub total_spent: String,
    pub open_cases: i64,
    pub last_booking_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub registered_at: Option<DateTime<Utc>>,
}

const FLAGS: [&str; 2] = ["cases", "cancellations"];

struct Window<'a> {
    city: &'a str,
    search: Option<String>,
    flag: Option<&'a str>,
    user_id: Option<&'a str>,
}

fn prepare<'a>(actor: &Actor, filter: &'a GuestFilter) -> Result<(Window<'a>, PageRequest)> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let flag = filter.flag.as_deref().map(str::trim).filter(|value| !value.is_empty());
    if let Some(flag) = flag {
        if !FLAGS.contains(&flag) {
            return Err(Error::bad_request(codes::MALFORMED_REQUEST));
        }
    }
    let search = filter
        .search
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("%{}%", value.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")));
    let page = PageRequest { page: filter.page.unwrap_or(1), page_size: filter.page_size.unwrap_or(20) }.validated()?;
    Ok((Window { city: actor.city.code(), search, flag, user_id: None }, page))
}

fn sources(builder: &mut QueryBuilder<'_, Postgres>, city: &str) {
    builder.push(
        r#"WITH g AS (
            SELECT r."guestUserId" AS user_id, COUNT(*) AS bookings,
              COUNT(*) FILTER (WHERE r.state IN ('REQUESTED','PENDING_PAYMENT','CONFIRMED','CHECKED_IN')) AS active_bookings,
              COUNT(*) FILTER (WHERE r.state IN ('CANCELLED','NO_SHOW','DECLINED','EXPIRED')) AS cancelled_bookings,
              COALESCE(SUM(r."totalAmount") FILTER (WHERE r.state IN ('CONFIRMED','CHECKED_IN','CHECKED_OUT')), 0)::text AS total_spent,
              MAX(r."createdAt") AS last_booking_at
            FROM stay_reservations r WHERE r.city_id = "#,
    );
    builder.push_bind(city.to_string());
    builder.push(r#" AND r."guestUserId" IS NOT NULL GROUP BY r."guestUserId"
        ), open_cases AS (
            SELECT c.parties->>'guestUserId' AS user_id, COUNT(*) AS open_cases
            FROM stay_cases c WHERE c.city_id = "#);
    builder.push_bind(city.to_string());
    builder.push(
        r#" AND c.state IN ('OPEN','AWAITING_RESPONSE') GROUP BY 1
        ), primary_guest AS (
            SELECT DISTINCT ON (r."guestUserId") r."guestUserId" AS user_id, g."fullName", g."contactPhone", g."contactEmail"
            FROM stay_reservations r
            JOIN stay_reservation_items i ON i."reservationId" = r.id
            JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
            WHERE r.city_id = "#,
    );
    builder.push_bind(city.to_string());
    builder.push(r#" ORDER BY r."guestUserId", r."createdAt" DESC)"#);
}

fn filters(builder: &mut QueryBuilder<'_, Postgres>, window: &Window<'_>) {
    if let Some(search) = &window.search {
        builder
            .push(" AND (COALESCE(u.first_name || ' ' || u.last_name, pg.\"fullName\", '') ILIKE ")
            .push_bind(search.clone())
            .push(r#" ESCAPE '\' OR COALESCE(u.email, pg."contactEmail", '') ILIKE "#)
            .push_bind(search.clone())
            .push(r#" ESCAPE '\' OR COALESCE(u.phone, pg."contactPhone", '') ILIKE "#)
            .push_bind(search.clone())
            .push(r#" ESCAPE '\')"#);
    }
    match window.flag {
        Some("cases") => { builder.push(" AND COALESCE(c.open_cases, 0) > 0"); }
        Some("cancellations") => { builder.push(" AND g.cancelled_bookings > 0"); }
        _ => {}
    }
    if let Some(user_id) = window.user_id {
        builder.push(" AND g.user_id = ").push_bind(user_id.to_string());
    }
}

async fn count(pool: &PgPool, window: &Window<'_>) -> Result<i64> {
    let mut query = QueryBuilder::<Postgres>::new("");
    sources(&mut query, window.city);
    query.push(
        r#" SELECT COUNT(*)::bigint FROM g
           LEFT JOIN primary_guest pg ON pg.user_id = g.user_id
           LEFT JOIN identity_users u ON u.id = g.user_id
           LEFT JOIN open_cases c ON c.user_id = g.user_id
           WHERE TRUE"#,
    );
    filters(&mut query, window);
    Ok(query.build_query_scalar().fetch_one(pool).await?)
}

async fn fetch(pool: &PgPool, window: &Window<'_>, page: &PageRequest) -> Result<Vec<GuestItem>> {
    let mut builder = QueryBuilder::<Postgres>::new("");
    sources(&mut builder, window.city);
    builder.push(
        r#" SELECT g.user_id,
             COALESCE(NULLIF(TRIM(COALESCE(u.first_name, '') || ' ' || COALESCE(u.last_name, '')), ''), pg."fullName") AS full_name,
             COALESCE(u.phone, pg."contactPhone") AS contact_phone, COALESCE(u.email, pg."contactEmail") AS contact_email,
             g.bookings, g.active_bookings, g.cancelled_bookings, g.total_spent, COALESCE(c.open_cases, 0) AS open_cases,
             g.last_booking_at, u.status, u.created_at AS registered_at
           FROM g
           LEFT JOIN primary_guest pg ON pg.user_id = g.user_id
           LEFT JOIN identity_users u ON u.id = g.user_id
           LEFT JOIN open_cases c ON c.user_id = g.user_id
           WHERE TRUE"#,
    );
    filters(&mut builder, window);
    builder.push(" ORDER BY g.last_booking_at DESC NULLS LAST, g.user_id LIMIT ").push_bind(page.limit());
    builder.push(" OFFSET ").push_bind(page.offset());
    Ok(builder.build_query_as::<GuestItem>().fetch_all(pool).await?)
}

pub async fn list(pool: &PgPool, actor: &Actor, filter: GuestFilter) -> Result<Page<GuestItem>> {
    let (window, page) = prepare(actor, &filter)?;
    let total = count(pool, &window).await?;
    let items = fetch(pool, &window, &page).await?;
    Ok(Page::new(items, total, page))
}

pub async fn detail(pool: &PgPool, actor: &Actor, user_id: &str) -> Result<GuestItem> {
    let filter = GuestFilter::default();
    let (mut window, page) = prepare(actor, &filter)?;
    window.user_id = Some(user_id);
    fetch(pool, &window, &page).await?
        .into_iter()
        .next()
        .ok_or_else(|| Error::not_found(codes::NOT_FOUND))
}
