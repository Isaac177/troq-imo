use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::MESSAGES_WRITE;
use kernel::{Error, Kernel, Page, Result};
use serde_json::json;
use sqlx::{FromRow, PgPool};

use super::types::{MessageView, MessagesQuery, SendMessageInput};
use crate::reservations::codes::{MESSAGE_BODY_INVALID, RESERVATION_NOT_FOUND};
use crate::reservations::lifecycle::access::hide_membership;
use crate::reservations::events::{self, types::MESSAGE_SENT};
use crate::reservations::support::paging;

const BODY_LIMIT: usize = 2000;

#[derive(Debug, FromRow)]
struct PartyRow {
    guest_user_id: String,
    operator_id: String,
    property_id: String,
}

async fn party_of(pool: &PgPool, actor: &Actor, reservation_id: &str) -> Result<PartyRow> {
    sqlx::query_as::<_, PartyRow>(
        r#"SELECT "guestUserId" AS guest_user_id, "operatorId" AS operator_id, "propertyId" AS property_id
           FROM stay_reservations WHERE id = $1 AND city_id = $2"#,
    )
    .bind(reservation_id)
    .bind(actor.city.code())
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| Error::not_found(RESERVATION_NOT_FOUND))
}

fn role_of(actor: &Actor, party: &PartyRow) -> Result<&'static str> {
    if actor.user_id().as_str() == party.guest_user_id {
        return Ok("GUEST");
    }
    actor
        .ensure_host_scope(MESSAGES_WRITE, &party.operator_id, &party.property_id)
        .map_err(hide_membership)?;
    Ok("OPERATOR")
}

pub async fn list(kernel: &Kernel, actor: &Actor, reservation_id: &str, query: MessagesQuery) -> Result<Page<MessageView>> {
    let pool = kernel.db.pool();
    let page = paging(query.page, query.page_size)?;
    let party = party_of(pool, actor, reservation_id).await?;
    role_of(actor, &party)?;
    let total: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM stay_reservation_messages WHERE "reservationId" = $1"#)
        .bind(reservation_id)
        .fetch_one(pool)
        .await?;
    let items = sqlx::query_as::<_, MessageView>(
        r#"SELECT id, "senderRole" AS sender_role, body, "createdAt" AS created_at
           FROM stay_reservation_messages
           WHERE "reservationId" = $1
           ORDER BY "createdAt" ASC, id ASC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(reservation_id)
    .bind(page.limit())
    .bind(page.offset())
    .fetch_all(pool)
    .await?;
    Ok(Page::new(items, total, page))
}

pub fn clean_body(body: &str) -> Result<String> {
    let body = body.trim().to_string();
    if body.is_empty() || body.chars().count() > BODY_LIMIT {
        return Err(Error::bad_request(MESSAGE_BODY_INVALID).with_params(json!({ "max": BODY_LIMIT })));
    }
    Ok(body)
}

pub async fn send(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: SendMessageInput) -> Result<MessageView> {
    let pool = kernel.db.pool();
    let body = clean_body(&input.body)?;
    let party = party_of(pool, actor, reservation_id).await?;
    let role = role_of(actor, &party)?;
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, MessageView>(
        r#"INSERT INTO stay_reservation_messages (id, city_id, "reservationId", "senderRole", "senderUserId", body)
           VALUES ($1, $2, $3, $4, $5, $6)
           RETURNING id, "senderRole" AS sender_role, body, "createdAt" AS created_at"#,
    )
    .bind(new_id("msg"))
    .bind(actor.city.code())
    .bind(reservation_id)
    .bind(role)
    .bind(actor.user_id().as_str())
    .bind(body)
    .fetch_one(&mut *tx)
    .await?;
    events::emit(
        &mut tx,
        actor.city,
        "reservation",
        reservation_id,
        MESSAGE_SENT,
        json!({ "reservationId": reservation_id, "senderRole": row.sender_role, "body": row.body }),
    )
    .await?;
    tx.commit().await?;
    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_bodies_are_trimmed_and_bounded() {
        assert_eq!(clean_body("  hello ").unwrap(), "hello");
        assert!(clean_body("   ").is_err());
        assert!(clean_body(&"x".repeat(BODY_LIMIT + 1)).is_err());
        assert!(clean_body(&"x".repeat(BODY_LIMIT)).is_ok());
    }
}
