use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::{PRICING_CAMPAIGNS, PROPERTIES_READ};
use kernel::{Error, Page, PageRequest, Result};
use serde_json::json;
use sqlx::{PgPool, Postgres, Transaction};

use super::queries::{self, NewPromotion};
use super::types::{CreatePromotionInput, PromotionView, UpdatePromotionInput};
use crate::pricing::amounts::money;
use crate::pricing::idempotency::{self as keys, PROMOTION_CREATE, PROMOTION_UPDATE};
use crate::pricing::messages::*;
use crate::pricing::rates::queries as plan_queries;
use crate::pricing::scope;
use crate::pricing::validation as common;

const PROMOTION_STATUSES: [&str; 2] = ["ACTIVE", "PAUSED"];

pub async fn list_promotions(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    paging: PageRequest,
) -> Result<Page<PromotionView>> {
    let paging = paging.validated()?;
    let property = scope::for_host(pool, actor, property_id, PROPERTIES_READ).await?;
    let (items, total) =
        queries::list(pool, actor.city, &property.id, paging.limit(), paging.offset()).await?;
    Ok(Page::new(items, total, paging))
}

pub async fn create_promotion(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    input: CreatePromotionInput,
) -> Result<PromotionView> {
    let property = scope::for_host(pool, actor, property_id, PRICING_CAMPAIGNS).await?;
    let city = actor.city;
    let key = input.idempotency_key.as_deref();
    if let Some(view) = keys::replay::<PromotionView>(pool, city, PROMOTION_CREATE, key).await? {
        return Ok(view);
    }
    common::period(input.start_date, input.end_date)?;
    let min_nights = common::min_nights(input.min_nights)?;
    let percent_minor = common::promotion_percent(&input.discount_percent)?;
    let cap_minor = input
        .cap_amount
        .as_deref()
        .map(|cap| common::money_minor(cap, "capAmount"))
        .transpose()?;
    let priority = common::priority(input.priority)?;
    let redemption_limit = common::redemption_limit(input.redemption_limit)?;
    if let Some(plan_id) = input.rate_plan_id.as_deref() {
        plan_queries::find_plan(pool, city, &property.id, plan_id)
            .await?
            .ok_or_else(|| Error::bad_request(RATE_PLAN_NOT_IN_PROPERTY))?;
    }

    let id = new_id("promo");
    let discount_percent = money(percent_minor);
    let cap_amount = cap_minor.map(money);
    let mut tx = pool.begin().await?;
    if !keys::claim(&mut tx, city, PROMOTION_CREATE, key, &id).await? {
        tx.rollback().await?;
        return keys::replay_or_busy(pool, city, PROMOTION_CREATE, key).await;
    }
    queries::insert(
        &mut tx,
        &NewPromotion {
            id: &id,
            city,
            property_id: &property.id,
            rate_plan_id: input.rate_plan_id.as_deref(),
            min_nights: i32::try_from(min_nights)
                .map_err(|_| Error::bad_request(MIN_NIGHTS_OUT_OF_RANGE))?,
            discount_percent,
            cap_amount,
            start: input.start_date,
            end: input.end_date,
            priority,
            redemption_limit,
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "promotion.create",
            resource_type: "promotion",
            resource_id: &id,
            reason: None,
            change: json!({ "minNights": min_nights, "discountPercent": discount_percent, "capAmount": cap_amount }),
        },
    )
    .await?;
    let view = load_in(&mut tx, actor, &property.id, &id).await?;
    keys::remember(&mut tx, city, PROMOTION_CREATE, key, &view).await?;
    tx.commit().await?;
    Ok(view)
}

pub async fn update_promotion(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    promotion_id: &str,
    input: UpdatePromotionInput,
    as_admin: bool,
) -> Result<PromotionView> {
    let property = scope::resolve(pool, actor, property_id, PRICING_CAMPAIGNS, as_admin).await?;
    let city = actor.city;
    let key = input.idempotency_key.as_deref();
    if let Some(view) = keys::replay::<PromotionView>(pool, city, PROMOTION_UPDATE, key).await? {
        return Ok(view);
    }
    if let Some(status) = input.status.as_deref() {
        common::choice(status, &PROMOTION_STATUSES, "status")?;
    }
    let redemption_limit = common::redemption_limit(input.redemption_limit)?;
    let current = queries::find(pool, city, &property.id, promotion_id)
        .await?
        .ok_or_else(|| Error::not_found(PROMOTION_NOT_FOUND))?;
    if let Some(limit) = redemption_limit {
        if i64::from(limit) < current.redeemed {
            return Err(Error::conflict(PROMOTION_LIMIT_BELOW_REDEEMED)
                .with_params(json!({ "redeemed": current.redeemed })));
        }
    }

    let mut tx = pool.begin().await?;
    if !keys::claim(&mut tx, city, PROMOTION_UPDATE, key, &current.id).await? {
        tx.rollback().await?;
        return keys::replay_or_busy(pool, city, PROMOTION_UPDATE, key).await;
    }
    queries::set_status_and_limit(&mut tx, city, &current.id, input.status.as_deref(), redemption_limit).await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "promotion.update",
            resource_type: "promotion",
            resource_id: &current.id,
            reason: None,
            change: json!({ "status": input.status, "redemptionLimit": redemption_limit }),
        },
    )
    .await?;
    let view = load_in(&mut tx, actor, &property.id, &current.id).await?;
    keys::remember(&mut tx, city, PROMOTION_UPDATE, key, &view).await?;
    tx.commit().await?;
    Ok(view)
}

async fn load_in(
    tx: &mut Transaction<'_, Postgres>,
    actor: &Actor,
    property_id: &str,
    promotion_id: &str,
) -> Result<PromotionView> {
    queries::find(&mut **tx, actor.city, property_id, promotion_id)
        .await?
        .map(queries::promotion_view)
        .transpose()?
        .ok_or_else(|| Error::not_found(PROMOTION_NOT_FOUND))
}
