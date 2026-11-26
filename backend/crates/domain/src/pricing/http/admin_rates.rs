use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::{Json, Path, Query, State};
use kernel::auth::Actor;
use kernel::envelope::Envelope;
use kernel::error::ErrorBody;
use kernel::{KernelState, Page};

use super::extract::{body, query, reply, Reply};
use crate::pricing::promotions::funding;
use crate::pricing::promotions::service as promotions;
use crate::pricing::promotions::types::{AdminPromotionView, FundingInput, PromotionView, UpdatePromotionInput};
use crate::pricing::quotes::preview::{preview_price, PricePreviewView};
use crate::pricing::quotes::QuoteRequest;
use crate::pricing::rates::types::*;
use crate::pricing::rates::{plans, policies, rules};

#[utoipa::path(
    get, path = "/api/admin/properties/{propertyId}/rate-plans", operation_id = "admin_rate_plans_list",
    params(("propertyId" = String, Path), PagingQuery),
    responses((status = 200, body = Envelope<Page<PlanView>>), (status = 403, body = ErrorBody))
)]
pub async fn list_plans<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(property_id): Path<String>,
    paging: std::result::Result<Query<PagingQuery>, QueryRejection>,
) -> Reply<Page<PlanView>> {
    let paging = query(paging)?.request();
    let page = plans::list_plans(state.kernel().db.pool(), &actor, &property_id, paging, true).await?;
    reply(page, &actor)
}

#[utoipa::path(
    post, path = "/api/admin/properties/{propertyId}/rate-plans", operation_id = "admin_rate_plans_create",
    params(("propertyId" = String, Path)), request_body = CreatePlanInput,
    responses((status = 200, body = Envelope<PlanView>), (status = 400, body = ErrorBody))
)]
pub async fn create_plan<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(property_id): Path<String>,
    input: std::result::Result<Json<CreatePlanInput>, JsonRejection>,
) -> Reply<PlanView> {
    let input = body(input)?;
    let view = plans::create_plan(state.kernel().db.pool(), &actor, &property_id, input, true).await?;
    reply(view, &actor)
}

#[utoipa::path(
    patch, path = "/api/admin/properties/{propertyId}/rate-plans/{ratePlanId}", operation_id = "admin_rate_plans_update",
    params(("propertyId" = String, Path), ("ratePlanId" = String, Path)), request_body = UpdatePlanInput,
    responses((status = 200, body = Envelope<PlanView>), (status = 409, body = ErrorBody))
)]
pub async fn update_plan<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path((property_id, plan_id)): Path<(String, String)>,
    input: std::result::Result<Json<UpdatePlanInput>, JsonRejection>,
) -> Reply<PlanView> {
    let input = body(input)?;
    let view = plans::update_plan(state.kernel().db.pool(), &actor, &property_id, &plan_id, input, true).await?;
    reply(view, &actor)
}

#[utoipa::path(
    get, path = "/api/admin/properties/{propertyId}/rate-plans/{ratePlanId}/rules", operation_id = "admin_rate_rules_list",
    params(("propertyId" = String, Path), ("ratePlanId" = String, Path), PagingQuery),
    responses((status = 200, body = Envelope<Page<RuleView>>))
)]
pub async fn list_rules<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path((property_id, plan_id)): Path<(String, String)>,
    paging: std::result::Result<Query<PagingQuery>, QueryRejection>,
) -> Reply<Page<RuleView>> {
    let paging = query(paging)?.request();
    let page = rules::list_rules(state.kernel().db.pool(), &actor, &property_id, &plan_id, paging, true).await?;
    reply(page, &actor)
}

#[utoipa::path(
    post, path = "/api/admin/properties/{propertyId}/rate-plans/{ratePlanId}/rules", operation_id = "admin_rate_rules_create",
    params(("propertyId" = String, Path), ("ratePlanId" = String, Path)), request_body = CreateRuleInput,
    responses((status = 200, body = Envelope<RuleView>), (status = 409, body = ErrorBody))
)]
pub async fn create_rule<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path((property_id, plan_id)): Path<(String, String)>,
    input: std::result::Result<Json<CreateRuleInput>, JsonRejection>,
) -> Reply<RuleView> {
    let input = body(input)?;
    let view = rules::create_rule(state.kernel().db.pool(), &actor, &property_id, &plan_id, input, true).await?;
    reply(view, &actor)
}

#[utoipa::path(
    delete, path = "/api/admin/properties/{propertyId}/rate-plans/{ratePlanId}/rules/{ruleId}", operation_id = "admin_rate_rules_remove",
    params(("propertyId" = String, Path), ("ratePlanId" = String, Path), ("ruleId" = String, Path), RemoveRuleQuery),
    responses((status = 200, body = Envelope<RemovedRule>), (status = 404, body = ErrorBody))
)]
pub async fn remove_rule<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path((property_id, plan_id, rule_id)): Path<(String, String, String)>,
    params: std::result::Result<Query<RemoveRuleQuery>, QueryRejection>,
) -> Reply<RemovedRule> {
    let key = query(params)?.idempotency_key;
    let removed =
        rules::remove_rule(state.kernel().db.pool(), &actor, &property_id, &plan_id, &rule_id, key, true).await?;
    reply(removed, &actor)
}

#[utoipa::path(
    get, path = "/api/admin/properties/{propertyId}/promotions", operation_id = "admin_promotions_list",
    params(("propertyId" = String, Path), PagingQuery),
    responses((status = 200, body = Envelope<Page<AdminPromotionView>>))
)]
pub async fn list_promotions<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(property_id): Path<String>,
    paging: std::result::Result<Query<PagingQuery>, QueryRejection>,
) -> Reply<Page<AdminPromotionView>> {
    let paging = query(paging)?.request();
    let page = funding::list_for_admin(state.kernel().db.pool(), &actor, &property_id, paging).await?;
    reply(page, &actor)
}

#[utoipa::path(
    patch, path = "/api/admin/properties/{propertyId}/promotions/{promotionId}", operation_id = "admin_promotions_update",
    params(("propertyId" = String, Path), ("promotionId" = String, Path)), request_body = UpdatePromotionInput,
    responses((status = 200, body = Envelope<PromotionView>), (status = 409, body = ErrorBody))
)]
pub async fn update_promotion<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path((property_id, promotion_id)): Path<(String, String)>,
    input: std::result::Result<Json<UpdatePromotionInput>, JsonRejection>,
) -> Reply<PromotionView> {
    let input = body(input)?;
    let view =
        promotions::update_promotion(state.kernel().db.pool(), &actor, &property_id, &promotion_id, input, true)
            .await?;
    reply(view, &actor)
}

#[utoipa::path(
    put, path = "/api/admin/properties/{propertyId}/promotions/{promotionId}/funding", operation_id = "admin_promotions_funding_set",
    params(("propertyId" = String, Path), ("promotionId" = String, Path)), request_body = FundingInput,
    responses((status = 200, body = Envelope<AdminPromotionView>), (status = 400, body = ErrorBody))
)]
pub async fn set_funding<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path((property_id, promotion_id)): Path<(String, String)>,
    input: std::result::Result<Json<FundingInput>, JsonRejection>,
) -> Reply<AdminPromotionView> {
    let input = body(input)?;
    let view = funding::set_funding(state.kernel().db.pool(), &actor, &property_id, &promotion_id, input).await?;
    reply(view, &actor)
}

#[utoipa::path(
    get, path = "/api/admin/rate-policies", operation_id = "admin_rate_policies_list", params(PolicyQuery),
    responses((status = 200, body = Envelope<Vec<PolicyView>>))
)]
pub async fn list_policies<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    params: std::result::Result<Query<PolicyQuery>, QueryRejection>,
) -> Reply<Vec<PolicyView>> {
    let kind = query(params)?.kind.filter(|kind| !kind.trim().is_empty());
    let items = policies::list_policies(state.kernel().db.pool(), &actor, kind.as_deref(), true).await?;
    reply(items, &actor)
}

#[utoipa::path(
    post, path = "/api/admin/properties/{propertyId}/price-preview", operation_id = "admin_price_preview",
    params(("propertyId" = String, Path)), request_body = QuoteRequest,
    responses((status = 200, body = Envelope<PricePreviewView>), (status = 409, body = ErrorBody))
)]
pub async fn price_preview<S: KernelState + Clone + Send + Sync + 'static>(
    State(state): State<S>,
    actor: Actor,
    Path(property_id): Path<String>,
    input: std::result::Result<Json<QuoteRequest>, JsonRejection>,
) -> Reply<PricePreviewView> {
    let mut input = body(input)?;
    input.property_id = property_id.clone();
    let view = preview_price(state.kernel().db.pool(), &actor, &property_id, input, true).await?;
    reply(view, &actor)
}
