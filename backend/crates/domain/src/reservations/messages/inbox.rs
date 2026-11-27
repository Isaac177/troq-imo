use chrono::{DateTime, Utc};
use kernel::auth::Actor;
use kernel::permissions::names::MESSAGES_WRITE;
use kernel::{Error, Kernel, Page, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::reservations::codes::{HOST_NOT_FOUND, HOST_REQUIRED};
use crate::reservations::support::{escape_like, paging};

#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct InboxQuery {
    pub host_id: String,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversationView {
    pub reservation_id: String,
    pub reference: String,
    pub property_name_en: String,
    pub property_name_fr: String,
    pub timezone: String,
    pub last_body: String,
    pub last_at: DateTime<Utc>,
    pub unread_count: i64,
}

pub async fn list(kernel: &Kernel, actor: &Actor, query: InboxQuery) -> Result<Page<ConversationView>> {
    actor.require(MESSAGES_WRITE)?;
    if query.host_id.trim().is_empty() { return Err(Error::bad_request(HOST_REQUIRED)); }
    if !actor.access.is_member_of(&query.host_id) { return Err(Error::not_found(HOST_NOT_FOUND)); }
    actor.require_scoped(MESSAGES_WRITE, Some(&query.host_id), None)?;
    let page = paging(query.page, query.page_size)?;
    let search = query.search.as_deref().map(str::trim).filter(|value| !value.is_empty()).map(|value| format!("%{}%", escape_like(value)));
    let filter = r#"r.city_id = $1 AND r."operatorId" = $2 AND ($3::text IS NULL OR r.reference ILIKE $3 ESCAPE '\' OR p."nameEn" ILIKE $3 ESCAPE '\' OR p."nameFr" ILIKE $3 ESCAPE '\')"#;
    let base = r#" FROM stay_reservations r JOIN stay_properties p ON p.id = r."propertyId" JOIN LATERAL (SELECT body, "createdAt" FROM stay_reservation_messages m WHERE m."reservationId" = r.id ORDER BY "createdAt" DESC, id DESC LIMIT 1) last ON true WHERE "#;
    let pool = kernel.db.pool();
    let total_sql = format!("SELECT COUNT(*){base}{filter}");
    let total = sqlx::query_scalar::<_, i64>(&total_sql).bind(actor.city.code()).bind(&query.host_id).bind(&search).fetch_one(pool).await?;
    let items_sql = format!(r#"SELECT r.id AS reservation_id, r.reference, p."nameEn" AS property_name_en, p."nameFr" AS property_name_fr, p.timezone, last.body AS last_body, last."createdAt" AS last_at, (SELECT COUNT(*) FROM stay_reservation_messages unread LEFT JOIN stay_message_host_reads marker ON marker.reservation_id = r.id AND marker.user_id = $6 WHERE unread."reservationId" = r.id AND unread."senderRole" = 'GUEST' AND (marker.message_id IS NULL OR (unread."createdAt", unread.id) > (marker.message_created_at, marker.message_id))) AS unread_count{base}{filter} ORDER BY last."createdAt" DESC, r.id LIMIT $4 OFFSET $5"#);
    let items = sqlx::query_as::<_, ConversationView>(&items_sql).bind(actor.city.code()).bind(&query.host_id).bind(&search).bind(page.limit()).bind(page.offset()).bind(actor.user_id().as_str()).fetch_all(pool).await?;
    Ok(Page::new(items, total, page))
}
