use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::{PRICING_WRITE, PROPERTIES_READ};
use kernel::{Error, Page, PageRequest, Result};
use serde_json::json;
use sqlx::{PgPool, Postgres, Transaction};

use super::queries::{self, NewPlan, PlanChange};
use super::types::{CreatePlanInput, PlanView, UpdatePlanInput};
use super::validation::{BOOKING_MODES, MEAL_PLANS, OCCUPANCY_BASES, PLAN_STATUSES};
use crate::pricing::idempotency::{self as keys, PLAN_CREATE, PLAN_UPDATE};
use crate::pricing::messages::*;
use crate::pricing::scope;
use crate::pricing::validation::{choice, money_minor};

pub async fn list_plans(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    paging: PageRequest,
    as_admin: bool,
) -> Result<Page<PlanView>> {
    let paging = paging.validated()?;
    let property = scope::resolve(pool, actor, property_id, PROPERTIES_READ, as_admin).await?;
    let (items, total) =
        queries::list_plans(pool, actor.city, &property.id, paging.limit(), paging.offset()).await?;
    Ok(Page::new(items, total, paging))
}

pub async fn create_plan(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    input: CreatePlanInput,
    as_admin: bool,
) -> Result<PlanView> {
    let property = scope::resolve(pool, actor, property_id, PRICING_WRITE, as_admin).await?;
    let city = actor.city;
    let key = input.idempotency_key.as_deref();
    if let Some(view) = keys::replay::<PlanView>(pool, city, PLAN_CREATE, key).await? {
        return Ok(view);
    }
    choice(&input.occupancy_basis, &OCCUPANCY_BASES, "occupancyBasis")?;
    choice(&input.meal_plan, &MEAL_PLANS, "mealPlan")?;
    choice(&input.booking_mode, &BOOKING_MODES, "bookingMode")?;
    let child_minor = money_minor(&input.child_nightly_amount, "childNightlyAmount")?;
    if !queries::unit_type_in_property(pool, city, &input.unit_type_id, &property.id).await? {
        return Err(Error::bad_request(UNIT_TYPE_NOT_IN_PROPERTY));
    }
    require_policy_kind(pool, &input.cancellation_policy_version_id, "CANCELLATION").await?;
    require_policy_kind(pool, &input.payment_policy_version_id, "PAYMENT").await?;

    let id = new_id("rate");
    let mut tx = pool.begin().await?;
    if !keys::claim(&mut tx, city, PLAN_CREATE, key, &id).await? {
        tx.rollback().await?;
        return keys::replay_or_busy(pool, city, PLAN_CREATE, key).await;
    }
    queries::insert_plan(
        &mut tx,
        &NewPlan {
            id: &id,
            city,
            property_id: &property.id,
            unit_type_id: &input.unit_type_id,
            occupancy_basis: &input.occupancy_basis,
            meal_plan: &input.meal_plan,
            booking_mode: &input.booking_mode,
            cancellation_policy_version_id: &input.cancellation_policy_version_id,
            payment_policy_version_id: &input.payment_policy_version_id,
            child_nightly_amount: crate::pricing::amounts::money(child_minor),
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "rate_plan.create",
            resource_type: "rate_plan",
            resource_id: &id,
            reason: None,
            change: json!({ "unitTypeId": input.unit_type_id, "occupancyBasis": input.occupancy_basis }),
        },
    )
    .await?;
    let view = load_plan_in(&mut tx, actor, &property.id, &id).await?;
    keys::remember(&mut tx, city, PLAN_CREATE, key, &view).await?;
    tx.commit().await?;
    Ok(view)
}

pub async fn update_plan(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    plan_id: &str,
    input: UpdatePlanInput,
    as_admin: bool,
) -> Result<PlanView> {
    let property = scope::resolve(pool, actor, property_id, PRICING_WRITE, as_admin).await?;
    let city = actor.city;
    let key = input.idempotency_key.as_deref();
    if let Some(view) = keys::replay::<PlanView>(pool, city, PLAN_UPDATE, key).await? {
        return Ok(view);
    }
    if let Some(status) = input.status.as_deref() {
        choice(status, &PLAN_STATUSES, "status")?;
    }
    if let Some(mode) = input.booking_mode.as_deref() {
        choice(mode, &BOOKING_MODES, "bookingMode")?;
    }
    if let Some(meal) = input.meal_plan.as_deref() {
        choice(meal, &MEAL_PLANS, "mealPlan")?;
    }
    let child_amount = input
        .child_nightly_amount
        .as_deref()
        .map(|amount| money_minor(amount, "childNightlyAmount").map(crate::pricing::amounts::money))
        .transpose()?;
    let current = queries::find_plan(pool, city, &property.id, plan_id)
        .await?
        .ok_or_else(|| Error::not_found(RATE_PLAN_NOT_FOUND))?;
    if current.status == "ARCHIVED" {
        return Err(Error::conflict(RATE_PLAN_ARCHIVED));
    }
    if input.status.as_deref() == Some("ACTIVE")
        && !queries::has_base_rate(pool, city, &current.id).await?
    {
        return Err(Error::conflict(RATE_PLAN_NEEDS_BASE_RATE));
    }

    let mut tx = pool.begin().await?;
    if !keys::claim(&mut tx, city, PLAN_UPDATE, key, &current.id).await? {
        tx.rollback().await?;
        return keys::replay_or_busy(pool, city, PLAN_UPDATE, key).await;
    }
    let updated = queries::update_plan(
        &mut tx,
        city,
        &current.id,
        input.expected_version,
        &PlanChange {
            status: input.status.as_deref(),
            booking_mode: input.booking_mode.as_deref(),
            meal_plan: input.meal_plan.as_deref(),
            child_nightly_amount: child_amount,
        },
    )
    .await?;
    if !updated {
        return Err(Error::conflict(RATE_PLAN_CHANGED));
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "rate_plan.update",
            resource_type: "rate_plan",
            resource_id: &current.id,
            reason: None,
            change: json!({ "status": input.status, "bookingMode": input.booking_mode, "childNightlyAmount": input.child_nightly_amount }),
        },
    )
    .await?;
    let view = load_plan_in(&mut tx, actor, &property.id, &current.id).await?;
    keys::remember(&mut tx, city, PLAN_UPDATE, key, &view).await?;
    tx.commit().await?;
    Ok(view)
}

async fn load_plan_in(
    tx: &mut Transaction<'_, Postgres>,
    actor: &Actor,
    property_id: &str,
    plan_id: &str,
) -> Result<PlanView> {
    queries::find_plan(&mut **tx, actor.city, property_id, plan_id)
        .await?
        .map(queries::plan_view)
        .transpose()?
        .ok_or_else(|| Error::not_found(RATE_PLAN_NOT_FOUND))
}

async fn require_policy_kind(pool: &PgPool, policy_id: &str, expected: &str) -> Result<()> {
    match super::policy_queries::policy_kind(pool, policy_id).await? {
        Some(kind) if kind == expected => Ok(()),
        _ => Err(Error::bad_request(POLICY_NOT_FOUND).with_params(json!({ "kind": expected }))),
    }
}
