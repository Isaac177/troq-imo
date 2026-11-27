use chrono::{DateTime, NaiveDate, Utc};
use kernel::auth::Actor;
use kernel::{City, Error, Kernel, Page, PageRequest, Result};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool};

use super::types::{MineCover, MyReservationView, MyReservationsQuery, PendingAmendmentView};
use super::mine_counts::{self, MyReservationsPage};
use crate::reservations::codes::RESERVATION_TAB_UNKNOWN;
use crate::reservations::contract::{discount_from, nights_from};
use crate::reservations::links::properties;
use crate::reservations::support::paging;

#[derive(Debug, FromRow)]
struct MineRow {
    id: String,
    reference: String,
    state: String,
    property_id: String,
    unit_type_id: String,
    check_in: NaiveDate,
    check_out: NaiveDate,
    adult_count: i32,
    child_count: i32,
    total: String,
    currency: String,
    version: i32,
    amendment_id: Option<String>,
    amendment_check_in: Option<String>,
    amendment_check_out: Option<String>,
    amendment_total: Option<String>,
    amendment_delta: Option<String>,
    amendment_expires_at: Option<DateTime<Utc>>,
    amendment_nights: Option<Value>,
    amendment_discount: Option<Value>,
    discount: Option<Value>,
    nightly_charges: Value,
    property_name_en: String,
    property_name_fr: String,
    unit_name_en: String,
    unit_name_fr: String,
    approval_deadline_at: Option<DateTime<Utc>>,
}

pub fn tab_predicate(tab: Option<&str>) -> Result<&'static str> {
    match tab {
        None => Ok("TRUE"),
        Some("pending") => Ok("r.state IN ('REQUESTED', 'PENDING_PAYMENT')"),
        Some("upcoming") => Ok("r.state = 'CONFIRMED' AND i.\"checkOut\" > (now() AT TIME ZONE p.timezone)::date"),
        Some("active") => Ok("r.state = 'CHECKED_IN'"),
        Some("past") => Ok("r.state IN ('CHECKED_OUT', 'NO_SHOW', 'COMPLETED') OR (r.state = 'CONFIRMED' AND i.\"checkOut\" <= (now() AT TIME ZONE p.timezone)::date)"),
        Some("cancelled") => Ok("r.state IN ('CANCELLED', 'EXPIRED', 'DECLINED')"),
        Some(other) => Err(Error::bad_request(RESERVATION_TAB_UNKNOWN).with_params(json!({ "tab": other }))),
    }
}

pub async fn my_reservations(kernel: &Kernel, actor: &Actor, query: MyReservationsQuery) -> Result<MyReservationsPage> {
    let page = paging(query.page, query.page_size)?;
    let predicate = tab_predicate(query.tab.as_deref())?;
    let (mut items, total) = list(kernel.db.pool(), actor.city, actor.user_id().as_str(), page, predicate).await?;
    attach_covers(kernel, &mut items).await?;
    let counts = mine_counts::counts(kernel.db.pool(), actor.city, actor.user_id().as_str()).await?;
    Ok(MyReservationsPage::new(Page::new(items, total, page), counts))
}

async fn attach_covers(kernel: &Kernel, items: &mut [MyReservationView]) -> Result<()> {
    let mut ids: Vec<String> = items.iter().map(|item| item.property_id.clone()).collect();
    ids.sort();
    ids.dedup();
    let covers = properties::property_covers(kernel, &ids).await?;
    let mut unit_type_ids: Vec<String> = items.iter().map(|item| item.unit_type_id.clone()).collect();
    unit_type_ids.sort();
    unit_type_ids.dedup();
    let unit_covers = properties::covers_for_unit_types(kernel, &unit_type_ids).await?;
    for item in items.iter_mut() {
        item.unit_type_cover_url = unit_covers.get(&item.unit_type_id).cloned();
        let Some(cover) = covers.get(&item.property_id) else {
            continue;
        };
        item.cover = Some(MineCover { url: cover.url.clone(), alt_en: cover.alt_en.clone(), alt_fr: cover.alt_fr.clone() });
    }
    Ok(())
}

pub async fn list(
    pool: &PgPool,
    city: City,
    guest: &str,
    page: PageRequest,
    predicate: &str,
) -> Result<(Vec<MyReservationView>, i64)> {
    let count_sql = format!(
        r#"SELECT COUNT(*) FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           JOIN stay_properties p ON p.id = r."propertyId"
           WHERE r."guestUserId" = $1 AND r.city_id = $2 AND ({predicate})"#
    );
    let total: i64 = sqlx::query_scalar(&count_sql).bind(guest).bind(city.code()).fetch_one(pool).await?;
    let list_sql = format!(
        r#"SELECT r.id, r.reference, r.state, r."propertyId" AS property_id, i."unitTypeId" AS unit_type_id,
                  i."checkIn" AS check_in, i."checkOut" AS check_out, i."adultCount" AS adult_count,
                  i."childCount" AS child_count, r."totalAmount"::text AS total, r.currency, r.version,
                  am.id AS amendment_id, am.check_in AS amendment_check_in, am.check_out AS amendment_check_out,
                  am.total AS amendment_total, am.delta AS amendment_delta, am.expires_at AS amendment_expires_at,
                  am.nights AS amendment_nights, am.discount AS amendment_discount,
                  r."contractSnapshot"->'discount' AS discount, i."nightlyCharges" AS nightly_charges,
                  p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr,
                  ut."nameEn" AS unit_name_en, ut."nameFr" AS unit_name_fr,
                  r."approvalDeadlineAt" AS approval_deadline_at
           FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           JOIN stay_properties p ON p.id = r."propertyId"
           JOIN stay_unit_types ut ON ut.id = i."unitTypeId"
           LEFT JOIN LATERAL (
               SELECT a.id, a."proposedContract"->>'checkIn' AS check_in, a."proposedContract"->>'checkOut' AS check_out,
                      a."proposedContract"->>'total' AS total, a."deltaAmount"::text AS delta, a."expiresAt" AS expires_at,
                      a."proposedContract"->'nights' AS nights, a."proposedContract"->'discount' AS discount
               FROM stay_amendments a
               WHERE a."reservationId" = r.id AND a.state = 'PROPOSED' AND a."expiresAt" > now()
               LIMIT 1
           ) am ON true
           WHERE r."guestUserId" = $1 AND r.city_id = $4 AND ({predicate})
           ORDER BY i."checkIn" DESC, r.id
           LIMIT $2 OFFSET $3"#
    );
    let rows = sqlx::query_as::<_, MineRow>(&list_sql)
        .bind(guest)
        .bind(page.limit())
        .bind(page.offset())
        .bind(city.code())
        .fetch_all(pool)
        .await?;
    let items = rows.into_iter().map(into_view).collect::<Result<Vec<_>>>()?;
    Ok((items, total))
}

fn into_view(row: MineRow) -> Result<MyReservationView> {
    let pending_amendment = match row.amendment_id {
        Some(id) => Some(PendingAmendmentView {
            id,
            check_in: row.amendment_check_in.unwrap_or_default(),
            check_out: row.amendment_check_out.unwrap_or_default(),
            total: row.amendment_total.unwrap_or_default(),
            delta: row.amendment_delta.unwrap_or_default(),
            expires_at: row.amendment_expires_at.unwrap_or_else(Utc::now),
            nights: nights_from(row.amendment_nights.unwrap_or(Value::Null))?,
            discount: discount_from(row.amendment_discount)?,
        }),
        None => None,
    };
    Ok(MyReservationView {
        nights: nights_from(row.nightly_charges)?,
        discount: discount_from(row.discount)?,
        pending_amendment,
        id: row.id,
        reference: row.reference,
        state: row.state,
        property_id: row.property_id,
        unit_type_id: row.unit_type_id,
        check_in: row.check_in,
        check_out: row.check_out,
        adult_count: row.adult_count,
        child_count: row.child_count,
        currency: row.currency,
        total: row.total,
        version: row.version,
        property_name_en: row.property_name_en,
        property_name_fr: row.property_name_fr,
        unit_name_en: row.unit_name_en,
        unit_name_fr: row.unit_name_fr,
        approval_deadline_at: row.approval_deadline_at,
        cover: None,
        unit_type_cover_url: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_map_to_state_predicates_and_unknown_tabs_fail() {
        assert_eq!(tab_predicate(None).unwrap(), "TRUE");
        assert!(tab_predicate(Some("pending")).unwrap().contains("REQUESTED"));
        assert!(tab_predicate(Some("cancelled")).unwrap().contains("DECLINED"));
        assert!(tab_predicate(Some("other")).is_err());
    }
}
