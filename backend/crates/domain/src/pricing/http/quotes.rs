use std::sync::Arc;

use axum::extract::rejection::JsonRejection;
use axum::extract::{Json, Path, State};
use axum::Extension;
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::KernelState;

use super::extract::{body, reply, Reply};
use crate::pricing::quotes::create::create_quote;
use crate::pricing::quotes::read::get_quote;
use crate::pricing::quotes::{QuoteChecks, QuoteRequest, QuoteView};

#[utoipa::path(
    post, path = "/api/guest/quotes", operation_id = "guest_quotes_create", request_body = QuoteRequest,
    responses((status = 200, body = Envelope<QuoteView>), (status = 409, body = ErrorBody))
)]
pub async fn create<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    Extension(checks): Extension<Arc<dyn QuoteChecks>>,
    actor: Actor,
    input: std::result::Result<Json<QuoteRequest>, JsonRejection>,
) -> Reply<QuoteView> {
    let input = body(input)?;
    let view = create_quote(state.kernel().db.pool(), checks.as_ref(), &actor, input).await?;
    reply(view, &actor)
}

#[utoipa::path(
    get, path = "/api/guest/quotes/{quoteId}", operation_id = "guest_quotes_get",
    params(("quoteId" = String, Path)),
    responses((status = 200, body = Envelope<QuoteView>), (status = 404, body = ErrorBody))
)]
pub async fn get<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    Extension(checks): Extension<Arc<dyn QuoteChecks>>,
    actor: Actor,
    Path(quote_id): Path<String>,
) -> Reply<QuoteView> {
    let view = get_quote(state.kernel().db.pool(), checks.as_ref(), &actor, &quote_id).await?;
    reply(view, &actor)
}
