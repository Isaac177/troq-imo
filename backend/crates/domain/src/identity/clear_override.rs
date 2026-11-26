use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::outbox;
use serde_json::json;
use sqlx::PgPool;

use super::manage_overrides::authorize_manager;

pub async fn clear_host_override(
    pool: &PgPool,
    actor: &Actor,
    host_id: &str,
    override_id: &str,
) -> Result<()> {
    authorize_manager(actor, host_id)?;
    let mut tx = pool.begin().await?;
    let deleted: Option<(String, String, String)> = sqlx::query_as(
        "DELETE FROM identity_member_overrides o USING hosts h, host_members target, identity_users target_user, host_members manager, identity_users manager_user WHERE o.id = $1 AND o.scope_type = 'HOST' AND o.host_id = $2 AND h.id = o.host_id AND h.city_id = $3 AND h.status = 'ACTIVE' AND target.host_id = h.id AND target.user_id = o.user_id AND target.status = 'ACTIVE' AND target_user.id = target.user_id AND target_user.status = 'ACTIVE' AND manager.host_id = h.id AND manager.user_id = $4 AND manager.status = 'ACTIVE' AND manager_user.id = manager.user_id AND manager_user.status = 'ACTIVE' RETURNING o.user_id, o.permission, o.effect",
    )
    .bind(override_id)
    .bind(host_id)
    .bind(actor.city.code())
    .bind(actor.user_id().as_str())
    .fetch_optional(&mut *tx)
    .await?;
    let (user_id, permission, effect) = deleted.ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    authorize_removal(actor, host_id, &permission, &effect)?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "identity.override.removed",
        resource_type: "identity_override",
        resource_id: override_id,
        reason: None,
        change: json!({ "hostId": host_id, "userId": user_id, "permission": permission, "effect": effect }),
    }).await?;
    outbox::enqueue(&mut tx, actor.city, "identity_override", override_id, "identity.override.removed", json!({ "hostId": host_id, "userId": user_id })).await?;
    tx.commit().await?;
    Ok(())
}

fn authorize_removal(actor: &Actor, host_id: &str, permission: &str, effect: &str) -> Result<()> {
    if effect == "DENY" {
        actor.require_scoped(permission, Some(host_id), None)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::auth::{CityAccess, CurrentUser, Role};
    use kernel::city::City;
    use kernel::ids::{HostId, UserId};
    use kernel::permissions::names::{HOSTS_TEAM, PROPERTIES_WRITE};
    use kernel::permissions::{Access, Grant};

    fn actor(grants: Vec<Grant>) -> Actor {
        Actor {
            user: CurrentUser {
                user_id: UserId::new("manager"),
                role: Role::Host,
                issued_at: 0,
                auth_city: Some("GOMA".into()),
                city_access_mode: None,
            },
            city: City::GOMA,
            request_id: "request".into(),
            city_access: CityAccess::Full,
            access: Access {
                grants,
                host_ids: vec![HostId::new("host-a")],
                auth_city: Some("GOMA".into()),
            },
        }
    }

    #[test]
    fn clearing_deny_requires_the_permission_in_the_same_host() {
        let manager = actor(vec![Grant::host(HOSTS_TEAM, "host-a"), Grant::host(PROPERTIES_WRITE, "host-b")]);
        assert!(authorize_removal(&manager, "host-a", PROPERTIES_WRITE, "DENY").is_err());
        assert!(authorize_removal(&manager, "host-a", PROPERTIES_WRITE, "ALLOW").is_ok());
        let authorized = actor(vec![Grant::host(HOSTS_TEAM, "host-a"), Grant::host(PROPERTIES_WRITE, "host-a")]);
        assert!(authorize_removal(&authorized, "host-a", PROPERTIES_WRITE, "DENY").is_ok());
    }
}
