use kernel::auth::CurrentUser;
use kernel::city::City;
use kernel::error::Result;
use kernel::paging::{Page, PageRequest};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostMembership {
    pub host_id: String,
    pub display_name: String,
    pub is_owner: bool,
}

pub async fn list_my_hosts(
    pool: &PgPool,
    user: &CurrentUser,
    city: City,
    request: PageRequest,
) -> Result<Page<HostMembership>> {
    let request = request.validated()?;
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM host_members m JOIN hosts h ON h.id = m.host_id JOIN identity_users u ON u.id = m.user_id WHERE m.user_id = $1 AND u.status = 'ACTIVE' AND m.status = 'ACTIVE' AND h.status = 'ACTIVE' AND h.city_id = $2",
    )
    .bind(user.user_id.as_str())
    .bind(city.code())
    .fetch_one(pool)
    .await?;
    let rows = sqlx::query_as::<_, (String, String, bool)>(
        "SELECT h.id, h.display_name, m.is_owner FROM host_members m JOIN hosts h ON h.id = m.host_id JOIN identity_users u ON u.id = m.user_id WHERE m.user_id = $1 AND u.status = 'ACTIVE' AND m.status = 'ACTIVE' AND h.status = 'ACTIVE' AND h.city_id = $2 ORDER BY h.display_name, h.id LIMIT $3 OFFSET $4",
    )
    .bind(user.user_id.as_str())
    .bind(city.code())
    .bind(request.limit())
    .bind(request.offset())
    .fetch_all(pool)
    .await?;
    let items = rows
        .into_iter()
        .map(|(host_id, display_name, is_owner)| HostMembership { host_id, display_name, is_owner })
        .collect();
    Ok(Page::new(items, total, request))
}
