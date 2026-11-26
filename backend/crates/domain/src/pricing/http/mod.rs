mod admin_rates;
mod extract;
mod host_rates;
mod quotes;
mod settings;

use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use kernel::KernelState;
use utoipa::OpenApi;

pub fn host_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/price-preview", post(host_rates::price_preview::<S>))
        .route(
            "/properties/:property_id/rate-plans",
            get(host_rates::list_plans::<S>).post(host_rates::create_plan::<S>),
        )
        .route("/properties/:property_id/rate-plans/:plan_id", patch(host_rates::update_plan::<S>))
        .route(
            "/properties/:property_id/rate-plans/:plan_id/rules",
            get(host_rates::list_rules::<S>).post(host_rates::create_rule::<S>),
        )
        .route(
            "/properties/:property_id/rate-plans/:plan_id/rules/:rule_id",
            delete(host_rates::remove_rule::<S>),
        )
        .route(
            "/properties/:property_id/promotions",
            get(host_rates::list_promotions::<S>).post(host_rates::create_promotion::<S>),
        )
        .route(
            "/properties/:property_id/promotions/:promotion_id",
            patch(host_rates::update_promotion::<S>),
        )
        .route("/rate-policies", get(host_rates::list_policies::<S>))
        .route("/quotes", post(quotes::create::<S>))
        .route("/quotes/:quote_id", get(quotes::get::<S>))
}

pub fn guest_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/quotes", post(quotes::create::<S>))
        .route("/quotes/:quote_id", get(quotes::get::<S>))
}

pub fn admin_routes<S>() -> Router<S>
where
    S: KernelState + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/properties/:property_id/rate-plans",
            get(admin_rates::list_plans::<S>).post(admin_rates::create_plan::<S>),
        )
        .route("/properties/:property_id/rate-plans/:plan_id", patch(admin_rates::update_plan::<S>))
        .route(
            "/properties/:property_id/rate-plans/:plan_id/rules",
            get(admin_rates::list_rules::<S>).post(admin_rates::create_rule::<S>),
        )
        .route(
            "/properties/:property_id/rate-plans/:plan_id/rules/:rule_id",
            delete(admin_rates::remove_rule::<S>),
        )
        .route("/properties/:property_id/promotions", get(admin_rates::list_promotions::<S>))
        .route(
            "/properties/:property_id/promotions/:promotion_id",
            patch(admin_rates::update_promotion::<S>),
        )
        .route(
            "/properties/:property_id/promotions/:promotion_id/funding",
            put(admin_rates::set_funding::<S>),
        )
        .route("/properties/:property_id/price-preview", post(admin_rates::price_preview::<S>))
        .route("/rate-policies", get(admin_rates::list_policies::<S>))
        .route("/settings", get(settings::list::<S>))
        .route("/settings/:key", patch(settings::update::<S>))
}

#[derive(OpenApi)]
#[openapi(paths(
    host_rates::list_plans,
    host_rates::create_plan,
    host_rates::update_plan,
    host_rates::list_rules,
    host_rates::create_rule,
    host_rates::remove_rule,
    host_rates::list_promotions,
    host_rates::create_promotion,
    host_rates::update_promotion,
    host_rates::list_policies,
    host_rates::price_preview,
    admin_rates::list_plans,
    admin_rates::create_plan,
    admin_rates::update_plan,
    admin_rates::list_rules,
    admin_rates::create_rule,
    admin_rates::remove_rule,
    admin_rates::list_promotions,
    admin_rates::update_promotion,
    admin_rates::set_funding,
    admin_rates::list_policies,
    admin_rates::price_preview,
    quotes::create,
    quotes::get,
    settings::list,
    settings::update,
))]
struct PricingDoc;

pub fn document() -> utoipa::openapi::OpenApi {
    PricingDoc::openapi()
}
