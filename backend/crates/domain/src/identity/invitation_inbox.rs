use chrono::{DateTime, Utc};
use kernel::auth::Actor;
use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use sqlx::PgPool;

use super::invitations::MyInvitationView;

pub async fn list_my_invitations(pool: &PgPool, actor: &Actor, request: PageRequest) -> Result<Page<MyInvitationView>> {
    let request = request.validated()?;
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM identity_invitations i JOIN hosts h ON h.id = i.host_id JOIN identity_users u ON lower(i.email) = lower(u.email) AND h.city_id = u.home_city_id WHERE u.id = $1 AND u.status = 'ACTIVE' AND i.status = 'PENDING' AND i.expires_at > now() AND h.status = 'ACTIVE'",
    ).bind(actor.user_id().as_str()).fetch_one(pool).await?;
    let rows = sqlx::query_as::<_, (String, String, String, Vec<String>, DateTime<Utc>)>(
        "SELECT i.id, i.host_id, h.display_name, i.permissions, i.expires_at FROM identity_invitations i JOIN hosts h ON h.id = i.host_id JOIN identity_users u ON lower(i.email) = lower(u.email) AND h.city_id = u.home_city_id WHERE u.id = $1 AND u.status = 'ACTIVE' AND i.status = 'PENDING' AND i.expires_at > now() AND h.status = 'ACTIVE' ORDER BY i.created_at DESC, i.id LIMIT $2 OFFSET $3",
    ).bind(actor.user_id().as_str()).bind(request.limit()).bind(request.offset()).fetch_all(pool).await?;
    let items = rows.into_iter().map(|(id, host_id, host_name, permissions, expires_at)| MyInvitationView { id, host_id, host_name, permissions, expires_at }).collect();
    Ok(Page::new(items, total, request))
}
