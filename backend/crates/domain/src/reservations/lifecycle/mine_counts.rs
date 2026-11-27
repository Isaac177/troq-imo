use kernel::{City, Page, Result};
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;

use super::mine::tab_predicate;
use super::types::MyReservationView;

#[derive(Debug, FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyReservationCounts {
    pub all: i64,
    pub pending: i64,
    pub upcoming: i64,
    pub active: i64,
    pub past: i64,
    pub cancelled: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyReservationsPage {
    pub items: Vec<MyReservationView>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub counts: MyReservationCounts,
}

impl MyReservationsPage {
    pub fn new(page: Page<MyReservationView>, counts: MyReservationCounts) -> Self {
        Self { items: page.items, total: page.total, page: page.page, page_size: page.page_size, counts }
    }
}

pub async fn counts(pool: &PgPool, city: City, guest: &str) -> Result<MyReservationCounts> {
    let sql = format!(
        r#"SELECT COUNT(*) AS "all",
                  COUNT(*) FILTER (WHERE {}) AS pending,
                  COUNT(*) FILTER (WHERE {}) AS upcoming,
                  COUNT(*) FILTER (WHERE {}) AS active,
                  COUNT(*) FILTER (WHERE {}) AS past,
                  COUNT(*) FILTER (WHERE {}) AS cancelled
           FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           JOIN stay_properties p ON p.id = r."propertyId"
           WHERE r."guestUserId" = $1 AND r.city_id = $2"#,
        tab_predicate(Some("pending"))?, tab_predicate(Some("upcoming"))?,
        tab_predicate(Some("active"))?, tab_predicate(Some("past"))?,
        tab_predicate(Some("cancelled"))?,
    );
    Ok(sqlx::query_as::<_, MyReservationCounts>(&sql)
        .bind(guest)
        .bind(city.code())
        .fetch_one(pool)
        .await?)
}
