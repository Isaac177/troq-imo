use std::sync::Arc;

use axum::routing::get;
use axum::Extension;
use domain::pricing::quotes::QuoteChecks;
use axum::Router;
use kernel::kernel::Kernel;

use crate::audience::Audience;
use crate::{amenities, health, quote_checks::DomainQuoteChecks, registry, spec};

fn public_core() -> Router<Kernel> {
    Router::new()
        .route("/health", get(health::health))
        .route("/amenity-groups", get(amenities::groups))
        .route("/amenities", get(amenities::amenities))
}

pub fn router(kernel: Kernel) -> Router {
    let checks: Arc<dyn QuoteChecks> = Arc::new(DomainQuoteChecks::new(kernel.clone()));
    Router::new()
        .nest(Audience::Public.prefix(), public_core().merge(registry::public()))
        .nest(Audience::Guest.prefix(), registry::guest())
        .nest(Audience::Host.prefix(), registry::host())
        .nest(Audience::Admin.prefix(), registry::admin())
        .route("/api/openapi.json", get(spec::serve))
        .layer(Extension(checks))
        .with_state(kernel)
}
