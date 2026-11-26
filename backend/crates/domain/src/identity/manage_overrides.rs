use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::ids::new_id;
use kernel::outbox;
use kernel::permissions::names::{self, HOSTS_TEAM};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

use super::overrides::OverrideEffect;

use super::messages::PERMISSION_INVALID;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetHostOverride {
    pub permission: String,
    pub effect: OverrideEffect,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostOverride {
    pub id: String,
    pub user_id: String,
    pub host_id: String,
    pub permission: String,
    pub effect: OverrideEffect,
}

pub(super) fn authorize_manager(actor: &Actor, host_id: &str) -> Result<()> {
    actor.ensure_writable()?;
    if !actor.access.is_member_of(host_id) {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    actor.require_scoped(HOSTS_TEAM, Some(host_id), None)
}

fn authorize(actor: &Actor, host_id: &str, permission: &str, effect: OverrideEffect) -> Result<()> {
    authorize_manager(actor, host_id)?;
    if !names::is_known(permission) {
        return Err(Error::bad_request(PERMISSION_INVALID));
    }
    if effect == OverrideEffect::Allow {
        actor.require_scoped(permission, Some(host_id), None)?;
    }
    Ok(())
}

pub async fn set_host_override(
    pool: &PgPool,
    actor: &Actor,
    host_id: &str,
    user_id: &str,
    input: SetHostOverride,
) -> Result<HostOverride> {
    authorize(actor, host_id, &input.permission, input.effect)?;
    let mut tx = pool.begin().await?;
    let member: Option<(String,)> = sqlx::query_as(
        "SELECT target.id FROM hosts h JOIN host_members target ON target.host_id = h.id JOIN identity_users target_user ON target_user.id = target.user_id JOIN host_members manager ON manager.host_id = h.id JOIN identity_users manager_user ON manager_user.id = manager.user_id WHERE h.id = $1 AND h.city_id = $2 AND h.status = 'ACTIVE' AND target.user_id = $3 AND target.status = 'ACTIVE' AND target_user.status = 'ACTIVE' AND manager.user_id = $4 AND manager.status = 'ACTIVE' AND manager_user.status = 'ACTIVE' FOR SHARE OF h, target, target_user, manager, manager_user",
    )
    .bind(host_id)
    .bind(actor.city.code())
    .bind(user_id)
    .bind(actor.user_id().as_str())
    .fetch_optional(&mut *tx)
    .await?;
    if member.is_none() {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    let id: String = sqlx::query_scalar(
        "INSERT INTO identity_member_overrides (id, user_id, permission, scope_type, scope_id, host_id, effect, created_by) VALUES ($1, $2, $3, 'HOST', $4, $4, $5, $6) ON CONFLICT (user_id, permission, scope_type, scope_id) DO UPDATE SET effect = EXCLUDED.effect, created_by = EXCLUDED.created_by, created_at = now() RETURNING id",
    )
    .bind(new_id("override"))
    .bind(user_id)
    .bind(&input.permission)
    .bind(host_id)
    .bind(input.effect.as_str())
    .bind(actor.user_id().as_str())
    .fetch_one(&mut *tx)
    .await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "identity.override.updated",
        resource_type: "identity_override",
        resource_id: &id,
        reason: None,
        change: json!({ "hostId": host_id, "userId": user_id, "permission": input.permission, "effect": input.effect }),
    }).await?;
    outbox::enqueue(&mut tx, actor.city, "identity_override", &id, "identity.override.updated", json!({ "hostId": host_id, "userId": user_id })).await?;
    tx.commit().await?;
    Ok(HostOverride { id, user_id: user_id.to_owned(), host_id: host_id.to_owned(), permission: input.permission, effect: input.effect })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::auth::{CityAccess, CurrentUser, Role};
    use kernel::city::City;
    use kernel::ids::{HostId, UserId};
    use kernel::permissions::{Access, Grant};

    fn actor(grants: Vec<Grant>, member: bool) -> Actor {
        Actor {
            user: CurrentUser {
                user_id: UserId::new("manager"),
                role: Role::Host,
                issued_at: 0,
                auth_city: Some("GOMA".into()),
                city_access_mode: None,
            },
            city: City::GOMA,
            request_id: "request-1".into(),
            city_access: CityAccess::Full,
            access: Access {
                grants,
                host_ids: if member { vec![HostId::new("host-a")] } else { vec![] },
                auth_city: Some("GOMA".into()),
            },
        }
    }

    #[test]
    fn a_manager_may_only_allow_permissions_they_hold_in_scope() {
        let manager = actor(vec![Grant::host(HOSTS_TEAM, "host-a")], true);
        assert!(authorize(&manager, "host-a", names::PROPERTIES_READ, OverrideEffect::Allow).is_err());
        assert!(authorize(&manager, "host-a", names::PROPERTIES_READ, OverrideEffect::Deny).is_ok());
        let manager = actor(vec![Grant::host(HOSTS_TEAM, "host-a"), Grant::host(names::PROPERTIES_READ, "host-a")], true);
        assert!(authorize(&manager, "host-a", names::PROPERTIES_READ, OverrideEffect::Allow).is_ok());
    }

    #[test]
    fn another_host_and_unknown_permission_are_rejected() {
        let manager = actor(vec![Grant::host(HOSTS_TEAM, "host-a")], true);
        assert!(authorize(&manager, "host-b", names::PROPERTIES_READ, OverrideEffect::Deny).is_err());
        assert_eq!(authorize(&manager, "host-a", "unknown.action", OverrideEffect::Deny).unwrap_err().code().as_str(), "IDENTITY_PERMISSION_INVALID");
    }

    #[test]
    fn clearing_requires_host_membership_and_team_management() {
        let no_membership = actor(vec![Grant::host(HOSTS_TEAM, "host-a")], false);
        assert_eq!(authorize_manager(&no_membership, "host-a").unwrap_err().code().as_str(), "NOT_FOUND");
        let no_team_permission = actor(vec![Grant::host(names::PROPERTIES_READ, "host-a")], true);
        assert_eq!(authorize_manager(&no_team_permission, "host-a").unwrap_err().code().as_str(), "PERMISSION_MISSING");
        let manager = actor(vec![Grant::host(HOSTS_TEAM, "host-a")], true);
        assert!(authorize_manager(&manager, "host-a").is_ok());
    }

    #[test]
    fn view_only_city_cannot_change_overrides() {
        let mut manager = actor(vec![Grant::host(HOSTS_TEAM, "host-a")], true);
        manager.city_access = CityAccess::ViewOnly;
        assert_eq!(authorize(&manager, "host-a", names::PROPERTIES_READ, OverrideEffect::Deny).unwrap_err().code().as_str(), "CITY_VIEW_ONLY");
    }
}
