use axum::extract::rejection::QueryRejection;
use axum::extract::{Query, State};
use axum::Json;
use kernel::auth::{Actor, RequestId};
use kernel::envelope::Envelope;
use kernel::error::{codes, Error, ErrorBody, Result};
use kernel::KernelState;

use super::dashboard;
use super::dashboard_types::{DashboardQuery, DashboardSummary};

#[utoipa::path(
    get,
    path = "/api/admin/dashboard",
    operation_id = "admin_dashboard_summary",
    params(DashboardQuery),
    responses(
        (status = 200, body = Envelope<DashboardSummary>),
        (status = 400, body = ErrorBody),
        (status = 403, body = ErrorBody)
    )
)]
pub async fn summary<S>(
    State(state): State<S>,
    actor: Actor,
    RequestId(request_id): RequestId,
    query: std::result::Result<Query<DashboardQuery>, QueryRejection>,
) -> Result<Json<Envelope<DashboardSummary>>>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    let Query(query) = query.map_err(|_| Error::bad_request(codes::MALFORMED_REQUEST))?;
    let result = dashboard::summary(state.kernel(), &actor, query).await?;
    Ok(Json(Envelope::ok(result, request_id)))
}
