use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::{PRICING_WRITE, PROPERTIES_READ};
use kernel::{Error, Page, PageRequest, Result};
use serde_json::json;
use sqlx::PgPool;

use super::engine::RateRule;
use super::queries::{self as plan_queries, PlanRow};
use super::rule_queries::{self as queries, NewRule};
use super::types::{weekdays_from_mask, CreateRuleInput, RemovedRule, RuleView};
use super::validation;
use crate::pricing::amounts::money;
use crate::pricing::idempotency::{self as keys, RULE_CREATE, RULE_REMOVE};
use crate::pricing::messages::*;
use crate::pricing::scope;
use crate::pricing::validation as common;

pub async fn list_rules(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    plan_id: &str,
    paging: PageRequest,
    as_admin: bool,
) -> Result<Page<RuleView>> {
    let paging = paging.validated()?;
    let property = scope::resolve(pool, actor, property_id, PROPERTIES_READ, as_admin).await?;
    plan_for(pool, actor, &property.id, plan_id).await?;
    let (items, total) =
        queries::list_rules(pool, actor.city, plan_id, paging.limit(), paging.offset()).await?;
    Ok(Page::new(items, total, paging))
}

pub async fn create_rule(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    plan_id: &str,
    input: CreateRuleInput,
    as_admin: bool,
) -> Result<RuleView> {
    let property = scope::resolve(pool, actor, property_id, PRICING_WRITE, as_admin).await?;
    let city = actor.city;
    let key = input.idempotency_key.as_deref();
    if let Some(view) = keys::replay::<RuleView>(pool, city, RULE_CREATE, key).await? {
        return Ok(view);
    }
    common::period(input.start_date, input.end_date)?;
    let priority = common::priority(input.priority)?;
    let adjustment = validation::adjustment(&input.adjustment_type)?;
    let amount_minor = validation::rule_amount(adjustment, &input.amount)?;
    let weekday_mask = common::weekday_mask(&input.weekdays)?;
    let plan = open_plan(pool, actor, &property.id, plan_id).await?;
    let existing = queries::load_rules(pool, city, &plan.id).await?;
    let candidate = RateRule {
        id: "candidate".into(),
        start: input.start_date,
        end: input.end_date,
        weekday_mask,
        adjustment,
        amount_minor,
        priority,
    };
    if let Some(overlap) = validation::overlapping_base(&existing, &candidate) {
        return Err(Error::conflict(BASE_RATE_OVERLAP).with_params(json!({ "ruleId": overlap.id })));
    }

    let id = new_id("rule");
    let amount = money(amount_minor);
    let mut tx = pool.begin().await?;
    if !keys::claim(&mut tx, city, RULE_CREATE, key, &id).await? {
        tx.rollback().await?;
        return keys::replay_or_busy(pool, city, RULE_CREATE, key).await;
    }
    queries::insert_rule(
        &mut tx,
        &NewRule {
            id: &id,
            city,
            rate_plan_id: &plan.id,
            start: input.start_date,
            end: input.end_date,
            weekday_mask,
            adjustment_type: &input.adjustment_type,
            amount,
            priority,
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "rate_rule.create",
            resource_type: "rate_rule",
            resource_id: &id,
            reason: None,
            change: json!({ "ratePlanId": plan.id, "adjustmentType": input.adjustment_type, "amount": amount, "priority": priority }),
        },
    )
    .await?;
    let view = RuleView {
        id,
        rate_plan_id: plan.id,
        start_date: input.start_date,
        end_date: input.end_date,
        weekdays: weekdays_from_mask(weekday_mask),
        adjustment_type: input.adjustment_type,
        amount,
        priority,
    };
    keys::remember(&mut tx, city, RULE_CREATE, key, &view).await?;
    tx.commit().await?;
    Ok(view)
}

pub async fn remove_rule(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    plan_id: &str,
    rule_id: &str,
    idempotency_key: Option<String>,
    as_admin: bool,
) -> Result<RemovedRule> {
    let property = scope::resolve(pool, actor, property_id, PRICING_WRITE, as_admin).await?;
    let city = actor.city;
    let key = idempotency_key.as_deref();
    if let Some(removed) = keys::replay::<RemovedRule>(pool, city, RULE_REMOVE, key).await? {
        return Ok(removed);
    }
    let plan = open_plan(pool, actor, &property.id, plan_id).await?;
    let mut tx = pool.begin().await?;
    if !keys::claim(&mut tx, city, RULE_REMOVE, key, rule_id).await? {
        tx.rollback().await?;
        return keys::replay_or_busy(pool, city, RULE_REMOVE, key).await;
    }
    if !queries::delete_rule(&mut tx, city, &plan.id, rule_id).await? {
        return Err(Error::not_found(RATE_RULE_NOT_FOUND));
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "rate_rule.remove",
            resource_type: "rate_rule",
            resource_id: rule_id,
            reason: None,
            change: json!({ "ratePlanId": plan.id }),
        },
    )
    .await?;
    let removed = RemovedRule { id: rule_id.to_string() };
    keys::remember(&mut tx, city, RULE_REMOVE, key, &removed).await?;
    tx.commit().await?;
    Ok(removed)
}

async fn plan_for(pool: &PgPool, actor: &Actor, property_id: &str, plan_id: &str) -> Result<PlanRow> {
    plan_queries::find_plan(pool, actor.city, property_id, plan_id)
        .await?
        .ok_or_else(|| Error::not_found(RATE_PLAN_NOT_FOUND))
}

async fn open_plan(pool: &PgPool, actor: &Actor, property_id: &str, plan_id: &str) -> Result<PlanRow> {
    let plan = plan_for(pool, actor, property_id, plan_id).await?;
    if plan.status == "ARCHIVED" {
        return Err(Error::conflict(RATE_PLAN_ARCHIVED));
    }
    Ok(plan)
}
