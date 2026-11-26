use chrono::{DateTime, Utc};
use kernel::auth::Actor;
use kernel::{Error, Kernel, Page, PageRequest, Result};
use serde_json::Value;
use sqlx::{FromRow, PgPool};

use super::types::{AmendmentListItem, AmendmentListQuery};
use crate::reservations::codes::RESERVATION_NOT_FOUND;
use crate::reservations::contract::nights_from;
use crate::reservations::lifecycle::queries::reservation_owner;
use crate::reservations::support::paging;

#[derive(Debug, FromRow)]
struct AmendmentRow {
    id: String,
    state: String,
    proposed_contract: Value,
    delta_amount: String,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    responded_at: Option<DateTime<Utc>>,
}

pub async fn list_amendments(kernel: &Kernel, actor: &Actor, reservation_id: &str, query: AmendmentListQuery) -> Result<Page<AmendmentListItem>> {
    let page = paging(query.page, query.page_size)?;
    let pool = kernel.db.pool();
    let owner = reservation_owner(pool, actor.city, reservation_id).await?;
    if owner.as_deref() != Some(actor.user_id().as_str()) {
        return Err(Error::not_found(RESERVATION_NOT_FOUND));
    }
    let (items, total) = list(pool, reservation_id, page).await?;
    Ok(Page::new(items, total, page))
}

pub(crate) async fn list(pool: &PgPool, reservation_id: &str, page: PageRequest) -> Result<(Vec<AmendmentListItem>, i64)> {
    let total: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM stay_amendments WHERE "reservationId" = $1"#)
        .bind(reservation_id)
        .fetch_one(pool)
        .await?;
    let rows = sqlx::query_as::<_, AmendmentRow>(
        r#"SELECT id, state, "proposedContract" AS proposed_contract, "deltaAmount"::text AS delta_amount,
                  "expiresAt" AS expires_at, "createdAt" AS created_at, "respondedAt" AS responded_at
           FROM stay_amendments WHERE "reservationId" = $1
           ORDER BY "createdAt" DESC, id
           LIMIT $2 OFFSET $3"#,
    )
    .bind(reservation_id)
    .bind(page.limit())
    .bind(page.offset())
    .fetch_all(pool)
    .await?;
    let items = rows.into_iter().map(into_item).collect::<Result<Vec<_>>>()?;
    Ok((items, total))
}

fn into_item(row: AmendmentRow) -> Result<AmendmentListItem> {
    let contract = &row.proposed_contract;
    let text = |key: &str| contract.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
    let nights = nights_from(contract.get("nights").cloned().unwrap_or(Value::Null))?;
    Ok(AmendmentListItem {
        id: row.id,
        state: row.state,
        check_in: text("checkIn"),
        check_out: text("checkOut"),
        total: text("total"),
        delta: row.delta_amount,
        nights,
        expires_at: row.expires_at,
        created_at: row.created_at,
        responded_at: row.responded_at,
    })
}
