use kernel::Result;
use sqlx::PgConnection;

use super::notice::Target;

pub async fn resolve(conn: &mut PgConnection, target: &Target) -> Result<Vec<String>> {
    match target {
        Target::User(user_id) if user_id.is_empty() => Ok(Vec::new()),
        Target::User(user_id) => Ok(vec![user_id.clone()]),
        Target::PlatformTeam => platform_team(conn).await,
    }
}

async fn platform_team(conn: &mut PgConnection) -> Result<Vec<String>> {
    let ids = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT g.user_id FROM identity_grants g JOIN identity_users u ON u.id = g.user_id WHERE g.scope_type = 'PLATFORM' AND u.status = 'ACTIVE' AND (g.expires_at IS NULL OR g.expires_at > now()) ORDER BY g.user_id",
    )
    .fetch_all(conn)
    .await?;
    Ok(ids)
}
