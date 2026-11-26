use chrono::{DateTime, NaiveDate, Utc};
use kernel::{City, PageRequest, Result};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

use crate::reservations::support::escape_like;

#[derive(Debug, FromRow)]
pub struct OperatorReservationRow {
    pub id: String,
    pub reference: String,
    pub state: String,
    pub operator_id: String,
    pub property_id: String,
    pub unit_type_id: String,
    pub item_id: String,
    pub assigned_unit_code: Option<String>,
    pub assignment_id: Option<String>,
    pub assignment_start: Option<NaiveDate>,
    pub property_name_en: String,
    pub property_name_fr: String,
    pub unit_type_name_en: String,
    pub unit_type_name_fr: String,
    pub timezone: String,
    pub approval_deadline_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub special_request: Option<String>,
    pub special_request_state: String,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adult_count: i32,
    pub child_count: i32,
    pub total: String,
    pub commission_rate_bps: i32,
    pub commission_amount: String,
    pub net_amount: String,
    pub currency: String,
    pub full_name: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub version: i32,
    pub nightly_charges: Value,
}

const ROW_BASE: &str = r#"SELECT r.id, r.reference, r.state, r."operatorId" AS operator_id, r."propertyId" AS property_id,
    i."unitTypeId" AS unit_type_id, i."checkIn" AS check_in, i."checkOut" AS check_out,
    i."adultCount" AS adult_count, i."childCount" AS child_count,
    r."totalAmount"::text AS total, r."commissionRateBps" AS commission_rate_bps,
    r."commissionAmount"::text AS commission_amount, r."operatorNetAmount"::text AS net_amount, r.currency, r.version, g."fullName" AS full_name,
    g."contactPhone" AS contact_phone, g."contactEmail" AS contact_email,
    i."nightlyCharges" AS nightly_charges,
    p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr, p.timezone,
    ut."nameEn" AS unit_type_name_en, ut."nameFr" AS unit_type_name_fr,
    i.id AS item_id,
    (SELECT pu."localCode" FROM stay_unit_assignments ua JOIN stay_physical_units pu ON pu.id = ua."physicalUnitId"
      WHERE ua."reservationItemId" = i.id AND ua."releasedAt" IS NULL ORDER BY ua."startDate" LIMIT 1) AS assigned_unit_code,
    (SELECT ua.id FROM stay_unit_assignments ua WHERE ua."reservationItemId" = i.id AND ua."releasedAt" IS NULL ORDER BY ua."startDate" LIMIT 1) AS assignment_id,
    (SELECT ua."startDate" FROM stay_unit_assignments ua WHERE ua."reservationItemId" = i.id AND ua."releasedAt" IS NULL ORDER BY ua."startDate" LIMIT 1) AS assignment_start,
    r."approvalDeadlineAt" AS approval_deadline_at, r."createdAt" AS created_at,
    r."specialRequest" AS special_request, r."specialRequestState" AS special_request_state
    FROM stay_reservations r
    JOIN stay_reservation_items i ON i."reservationId" = r.id
    LEFT JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
    JOIN stay_properties p ON p.id = r."propertyId"
    JOIN stay_unit_types ut ON ut.id = i."unitTypeId""#;

const ORDER_BY: &str = r#" ORDER BY
    CASE r.state WHEN 'REQUESTED' THEN 0 WHEN 'PENDING_PAYMENT' THEN 1 WHEN 'CHECKED_IN' THEN 2 WHEN 'CONFIRMED' THEN 3 ELSE 4 END,
    CASE WHEN r.state IN ('REQUESTED', 'PENDING_PAYMENT', 'CHECKED_IN', 'CONFIRMED') THEN i."checkIn" END ASC,
    i."checkIn" DESC, r.id"#;

pub struct ListFilter<'a> {
    pub city: City,
    pub host_id: &'a str,
    pub property_id: Option<&'a str>,
    pub state: Option<&'a str>,
    pub search: Option<&'a str>,
}

fn push_filters(builder: &mut QueryBuilder<'_, Postgres>, filter: &ListFilter<'_>) {
    builder.push(" AND r.city_id = ").push_bind(filter.city.code());
    if let Some(property_id) = filter.property_id {
        builder.push(r#" AND r."propertyId" = "#).push_bind(property_id.to_string());
    }
    if let Some(state) = filter.state {
        builder.push(" AND r.state = ").push_bind(state.to_string());
    }
    if let Some(search) = filter.search {
        let pattern = format!("%{}%", escape_like(search));
        builder
            .push(" AND (r.reference ILIKE ")
            .push_bind(pattern.clone())
            .push(r#" OR g."fullName" ILIKE "#)
            .push_bind(pattern)
            .push(")");
    }
}

pub async fn list(pool: &PgPool, filter: &ListFilter<'_>, page: PageRequest) -> Result<(Vec<OperatorReservationRow>, i64)> {
    let mut count = QueryBuilder::<Postgres>::new(
        r#"SELECT COUNT(*) FROM stay_reservations r
           JOIN stay_reservation_items i ON i."reservationId" = r.id
           LEFT JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
           WHERE r."operatorId" = "#,
    );
    count.push_bind(filter.host_id.to_string());
    push_filters(&mut count, filter);
    let total: i64 = count.build_query_scalar().fetch_one(pool).await?;

    let mut items = QueryBuilder::<Postgres>::new(format!(r#"{ROW_BASE} WHERE r."operatorId" = "#));
    items.push_bind(filter.host_id.to_string());
    push_filters(&mut items, filter);
    items
        .push(ORDER_BY)
        .push(" LIMIT ")
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let rows = items.build_query_as::<OperatorReservationRow>().fetch_all(pool).await?;
    Ok((rows, total))
}

pub async fn find(pool: &PgPool, city: City, reservation_id: &str) -> Result<Option<OperatorReservationRow>> {
    let sql = format!("{ROW_BASE} WHERE r.id = $1 AND r.city_id = $2");
    let row = sqlx::query_as::<_, OperatorReservationRow>(&sql)
        .bind(reservation_id)
        .bind(city.code())
        .fetch_optional(pool)
        .await?;
    Ok(row)
}
