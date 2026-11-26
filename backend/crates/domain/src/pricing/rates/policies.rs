use kernel::auth::Actor;
use kernel::permissions::names::{PROPERTIES_MODERATE, PROPERTIES_READ};
use kernel::Result;
use sqlx::PgPool;

use super::policy_queries;
use super::types::PolicyView;

pub async fn list_policies(
    pool: &PgPool,
    actor: &Actor,
    kind: Option<&str>,
    as_admin: bool,
) -> Result<Vec<PolicyView>> {
    if as_admin {
        actor.require_platform(PROPERTIES_MODERATE)?;
    } else {
        actor.require(PROPERTIES_READ)?;
    }
    policy_queries::list_policies(pool, kind).await
}
