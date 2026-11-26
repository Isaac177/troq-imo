use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::KernelState;
use super::report;
use super::report_types::{AnalyticsQuery, AnalyticsSummary};

#[utoipa::path(
    get, path = "/api/host/analytics/summary", operation_id = "host_analytics_summary",
    params(AnalyticsQuery),
    responses((status = 200, body = Envelope<AnalyticsSummary>), (status = 400, body = ErrorBody), (status = 403, body = ErrorBody), (status = 404, body = ErrorBody))
)]
pub async fn summary<S>(
    State(state): State<S>, actor: Actor, RequestId(request_id): RequestId,
    query: std::result::Result<Query<AnalyticsQuery>, QueryRejection>,
) -> Result<Json<Envelope<AnalyticsSummary>>>
where S: KernelState + Clone + Send + Sync + 'static {
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    Ok(Json(Envelope::ok(report::summary(state.kernel(), &actor, query).await?, request_id)))
}
