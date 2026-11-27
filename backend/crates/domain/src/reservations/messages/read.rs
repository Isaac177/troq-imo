use kernel::auth::Actor;
use kernel::{Error, Kernel, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::reservations::codes::RESERVATION_NOT_FOUND;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReadMessagesInput {
    pub message_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReadMessagesResult {
    pub marked: bool,
}

pub async fn mark(kernel: &Kernel, actor: &Actor, reservation_id: &str, input: ReadMessagesInput) -> Result<ReadMessagesResult> {
    let pool = kernel.db.pool();
    super::service::ensure_host_access(pool, actor, reservation_id).await?;
    let exists = sqlx::query_scalar::<_, bool>(r#"SELECT EXISTS(SELECT 1 FROM stay_reservation_messages WHERE id = $1 AND "reservationId" = $2)"#)
        .bind(&input.message_id).bind(reservation_id).fetch_one(pool).await?;
    if !exists { return Err(Error::not_found(RESERVATION_NOT_FOUND)); }
    sqlx::query(r#"INSERT INTO stay_message_host_reads (reservation_id, user_id, message_id, message_created_at)
        SELECT "reservationId", $3, id, "createdAt" FROM stay_reservation_messages WHERE id = $1 AND "reservationId" = $2
        ON CONFLICT (reservation_id, user_id) DO UPDATE SET message_id = EXCLUDED.message_id, message_created_at = EXCLUDED.message_created_at, updated_at = now()
        WHERE (stay_message_host_reads.message_created_at, stay_message_host_reads.message_id) < (EXCLUDED.message_created_at, EXCLUDED.message_id)"#)
        .bind(&input.message_id).bind(reservation_id).bind(actor.user_id().as_str()).execute(pool).await?;
    Ok(ReadMessagesResult { marked: true })
}
