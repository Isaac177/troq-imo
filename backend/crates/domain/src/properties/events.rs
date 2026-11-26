use kernel::error::Result;
use sqlx::{Postgres, Transaction};

pub async fn owner_user(tx: &mut Transaction<'_, Postgres>, host_id: &str) -> Result<Option<String>> {
    Ok(sqlx::query_scalar(
        "SELECT m.user_id FROM host_members m JOIN identity_users u ON u.id = m.user_id WHERE m.host_id = $1 AND m.is_owner AND m.status = 'ACTIVE' AND u.status = 'ACTIVE' ORDER BY m.created_at, m.id LIMIT 1",
    )
    .bind(host_id)
    .fetch_optional(&mut **tx)
    .await?)
}
