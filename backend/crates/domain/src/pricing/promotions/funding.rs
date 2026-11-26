use std::collections::HashMap;

use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::{Error, Page, PageRequest, Result};
use serde_json::json;
use sqlx::PgPool;

use super::queries;
use super::types::{AdminPromotionView, FundingInput};
use crate::pricing::messages::*;
use crate::pricing::scope;
use crate::pricing::validation::choice;

const FUNDING: [&str; 2] = ["OPERATOR", "PLATFORM"];
const DEFAULT_FUNDING: &str = "OPERATOR";

pub async fn list_for_admin(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    paging: PageRequest,
) -> Result<Page<AdminPromotionView>> {
    let paging = paging.validated()?;
    let property = scope::for_admin(pool, actor, property_id).await?;
    let (items, total) =
        queries::list(pool, actor.city, &property.id, paging.limit(), paging.offset()).await?;
    let ids: Vec<String> = items.iter().map(|item| item.id.clone()).collect();
    let funding: HashMap<String, String> = sqlx::query_as::<_, (String, String)>(
        r#"SELECT "promotionId", "fundedBy" FROM stay_promotion_funding
           WHERE city_id = $1 AND "promotionId" = ANY($2)"#,
    )
    .bind(actor.city.code())
    .bind(&ids)
    .fetch_all(pool)
    .await?
    .into_iter()
    .collect();
    let items = items
        .into_iter()
        .map(|promotion| {
            let funded_by = funding.get(&promotion.id).cloned().unwrap_or_else(|| DEFAULT_FUNDING.to_string());
            AdminPromotionView { promotion, funded_by }
        })
        .collect();
    Ok(Page::new(items, total, paging))
}

pub async fn set_funding(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    promotion_id: &str,
    input: FundingInput,
) -> Result<AdminPromotionView> {
    choice(&input.funded_by, &FUNDING, "fundedBy").map_err(|_| Error::bad_request(FUNDING_UNKNOWN))?;
    let property = scope::for_admin(pool, actor, property_id).await?;
    let city = actor.city;
    let current = queries::find(pool, city, &property.id, promotion_id)
        .await?
        .ok_or_else(|| Error::not_found(PROMOTION_NOT_FOUND))?;
    let previous: Option<String> = sqlx::query_scalar(
        r#"SELECT "fundedBy" FROM stay_promotion_funding WHERE city_id = $1 AND "promotionId" = $2"#,
    )
    .bind(city.code())
    .bind(&current.id)
    .fetch_optional(pool)
    .await?;
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO stay_promotion_funding (city_id, "promotionId", "fundedBy", "updatedByUserId")
           VALUES ($1, $2, $3, $4)
           ON CONFLICT ("promotionId") DO UPDATE SET "fundedBy" = EXCLUDED."fundedBy",
               "updatedByUserId" = EXCLUDED."updatedByUserId", "updatedAt" = now()"#,
    )
    .bind(city.code())
    .bind(&current.id)
    .bind(&input.funded_by)
    .bind(actor.user_id().as_str())
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "promotion.funding_changed",
            resource_type: "promotion",
            resource_id: &current.id,
            reason: Some(input.funded_by.as_str()),
            change: json!({ "from": previous.unwrap_or_else(|| DEFAULT_FUNDING.to_string()), "to": input.funded_by }),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(AdminPromotionView {
        promotion: queries::promotion_view(current)?,
        funded_by: input.funded_by,
    })
}
