use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::Json;
use chrono::NaiveDate;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{KernelState, City};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::IntoParams;

use super::report_types::{AnalyticsSummary, ReservationTotals, ReviewTotals};

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct ReportQuery {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

async fn totals(pool: &PgPool, city: City, query: &ReportQuery) -> Result<AnalyticsSummary> {
    let reservations = sqlx::query_as::<_, ReservationTotals>(r#"
        SELECT COUNT(*) AS reservations,
          COUNT(*) FILTER (WHERE r.state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')) AS confirmed,
          COUNT(*) FILTER (WHERE r.state = 'CHECKED_IN') AS checked_in,
          COUNT(*) FILTER (WHERE r.state = 'CHECKED_OUT') AS checked_out,
          COUNT(*) FILTER (WHERE r.state = 'CANCELLED') AS cancelled,
          COUNT(*) FILTER (WHERE r.state = 'NO_SHOW') AS no_show,
          COALESCE(SUM(r."totalAmount") FILTER (WHERE r.state IN ('CONFIRMED', 'CHECKED_IN', 'CHECKED_OUT')), 0)::text AS gross_revenue
        FROM stay_reservations r WHERE r.city_id = $1
          AND (($2::date IS NULL AND $3::date IS NULL) OR EXISTS (
            SELECT 1 FROM stay_reservation_items i WHERE i."reservationId" = r.id
              AND ($2::date IS NULL OR i."checkIn" >= $2) AND ($3::date IS NULL OR i."checkIn" <= $3)))"#)
        .bind(city.code()).bind(query.from).bind(query.to).fetch_one(pool).await?;
    let reviews = sqlx::query_as::<_, ReviewTotals>(r#"
        SELECT COUNT(*) AS approved_reviews, AVG(v.rating)::float8 AS average_rating
        FROM stay_reviews v WHERE v.status = 'APPROVED' AND v.city_id = $1
          AND (($2::date IS NULL AND $3::date IS NULL) OR EXISTS (
            SELECT 1 FROM stay_reservation_items i WHERE i."reservationId" = v."reservationId"
              AND ($2::date IS NULL OR i."checkIn" >= $2) AND ($3::date IS NULL OR i."checkIn" <= $3)))"#)
        .bind(city.code()).bind(query.from).bind(query.to).fetch_one(pool).await?;
    Ok(AnalyticsSummary {
        currency: "CDF", reservations: reservations.reservations, confirmed: reservations.confirmed,
        checked_in: reservations.checked_in, checked_out: reservations.checked_out,
        cancelled: reservations.cancelled, no_show: reservations.no_show,
        gross_revenue: reservations.gross_revenue, approved_reviews: reviews.approved_reviews,
        average_rating: reviews.average_rating,
    })
}

#[utoipa::path(
    get, path = "/api/admin/reports/summary", operation_id = "admin_reports_summary",
    params(ReportQuery),
    responses((status = 200, body = Envelope<AnalyticsSummary>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody))
)]
pub async fn summary<S>(State(state): State<S>, actor: Actor, RequestId(request_id): RequestId, query: std::result::Result<Query<ReportQuery>, QueryRejection>) -> Result<Json<Envelope<AnalyticsSummary>>>
where S: KernelState + Clone + Send + Sync + 'static {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    if query.from.zip(query.to).is_some_and(|(from, to)| from > to) {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    Ok(Json(Envelope::ok(totals(state.kernel().db.pool(), actor.city, &query).await?, request_id)))
}
