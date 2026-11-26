use kernel::{City, Result};
use sqlx::PgPool;
use super::report_types::{AnalyticsQuery, ReservationTotals, ReviewTotals};

pub async fn reservations(pool: &PgPool, city: City, query: &AnalyticsQuery) -> Result<ReservationTotals> {
    Ok(sqlx::query_as::<_, ReservationTotals>(r#"
        SELECT COUNT(*) AS reservations,
          COUNT(*) FILTER (WHERE r.state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')) AS confirmed,
          COUNT(*) FILTER (WHERE r.state = 'CHECKED_IN') AS checked_in,
          COUNT(*) FILTER (WHERE r.state = 'CHECKED_OUT') AS checked_out,
          COUNT(*) FILTER (WHERE r.state = 'CANCELLED') AS cancelled,
          COUNT(*) FILTER (WHERE r.state = 'NO_SHOW') AS no_show,
          COALESCE(SUM(r."totalAmount") FILTER (WHERE r.state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')), 0)::text AS gross_revenue
        FROM stay_reservations r WHERE r."operatorId" = $1 AND r.city_id = $2
          AND ($3::text IS NULL OR r."propertyId" = $3)
          AND (($4::date IS NULL AND $5::date IS NULL) OR EXISTS (
            SELECT 1 FROM stay_reservation_items i WHERE i."reservationId" = r.id
              AND ($4::date IS NULL OR i."checkIn" >= $4) AND ($5::date IS NULL OR i."checkIn" <= $5)
          ))"#)
        .bind(query.host_id.trim()).bind(city.code()).bind(query.property_id.as_deref().map(str::trim))
        .bind(query.from).bind(query.to).fetch_one(pool).await?)
}

pub async fn reviews(pool: &PgPool, city: City, query: &AnalyticsQuery) -> Result<ReviewTotals> {
    Ok(sqlx::query_as::<_, ReviewTotals>(r#"
        SELECT COUNT(*) AS approved_reviews, AVG(v.rating)::float8 AS average_rating
        FROM stay_reviews v WHERE v.status = 'APPROVED' AND v."operatorId" = $1 AND v.city_id = $2
          AND ($3::text IS NULL OR v."propertyId" = $3)
          AND (($4::date IS NULL AND $5::date IS NULL) OR EXISTS (
            SELECT 1 FROM stay_reservation_items i WHERE i."reservationId" = v."reservationId"
              AND ($4::date IS NULL OR i."checkIn" >= $4) AND ($5::date IS NULL OR i."checkIn" <= $5)
          ))"#)
        .bind(query.host_id.trim()).bind(city.code()).bind(query.property_id.as_deref().map(str::trim))
        .bind(query.from).bind(query.to).fetch_one(pool).await?)
}
