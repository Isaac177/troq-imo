mod admin_credits;
mod admin_rates;
mod extract;
mod host;

use axum::routing::{get, post};
use axum::Router;
use kernel::KernelState;
use utoipa::OpenApi;

pub fn host_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/credits", get(host::summary::<S>))
        .route("/credits/ledger", get(host::ledger::<S>))
}

pub fn admin_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/commission/hosts/:host_id/rate",
            get(admin_rates::current::<S>).post(admin_rates::set::<S>),
        )
        .route("/commission/hosts/:host_id/rate/history", get(admin_rates::history::<S>))
        .route("/credits/accounts", get(admin_credits::list::<S>))
        .route("/credits/summary", get(admin_credits::summary::<S>))
        .route("/credits/hosts/:host_id", get(admin_credits::detail::<S>))
        .route("/credits/hosts/:host_id/ledger", get(admin_credits::ledger_page::<S>))
        .route("/credits/hosts/:host_id/topup", post(admin_credits::topup::<S>))
        .route("/credits/hosts/:host_id/adjust", post(admin_credits::adjust::<S>))
}

#[derive(OpenApi)]
#[openapi(paths(
    host::summary,
    host::ledger,
    admin_rates::current,
    admin_rates::history,
    admin_rates::set,
    admin_credits::list,
    admin_credits::summary,
    admin_credits::detail,
    admin_credits::ledger_page,
    admin_credits::topup,
    admin_credits::adjust,
))]
struct BillingDoc;

pub fn document() -> utoipa::openapi::OpenApi {
    BillingDoc::openapi()
}
