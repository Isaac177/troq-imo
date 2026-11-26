use std::collections::HashMap;

use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::outbox;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::HOSTS_TEAM;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

use super::messages::PERMISSION_INVALID;
use super::presets;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MemberOverride {
    pub id: String,
    pub permission: String,
    pub effect: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MemberView {
    pub user_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_owner: bool,
    pub permissions: Vec<String>,
    pub overrides: Vec<MemberOverride>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetMemberPermissions {
    pub permissions: Vec<String>,
}

type MemberRow = (String, Option<String>, Option<String>, Option<String>, Option<String>, bool);

pub fn ensure_manager(actor: &Actor, host_id: &str) -> Result<()> {
    actor.ensure_writable()?;
    if !actor.access.is_member_of(host_id) {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    actor.require_scoped(HOSTS_TEAM, Some(host_id), None)
}

async fn host_in_city(pool: &PgPool, actor: &Actor, host_id: &str) -> Result<bool> {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM hosts WHERE id = $1 AND city_id = $2 AND status = 'ACTIVE')",
    )
    .bind(host_id)
    .bind(actor.city.code())
    .fetch_one(pool)
    .await?;
    Ok(exists)
}

pub async fn list_members(
    pool: &PgPool,
    actor: &Actor,
    host_id: &str,
    request: PageRequest,
) -> Result<Page<MemberView>> {
    ensure_manager(actor, host_id)?;
    if !host_in_city(pool, actor, host_id).await? {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    let request = request.validated()?;
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM host_members m JOIN identity_users u ON u.id = m.user_id WHERE m.host_id = $1 AND m.status = 'ACTIVE' AND u.status = 'ACTIVE'",
    )
    .bind(host_id)
    .fetch_one(pool)
    .await?;
    let rows = sqlx::query_as::<_, MemberRow>(
        "SELECT m.user_id, u.first_name, u.last_name, u.email, u.phone, m.is_owner FROM host_members m JOIN identity_users u ON u.id = m.user_id WHERE m.host_id = $1 AND m.status = 'ACTIVE' AND u.status = 'ACTIVE' ORDER BY m.is_owner DESC, u.first_name NULLS LAST, u.email NULLS LAST, m.user_id LIMIT $2 OFFSET $3",
    )
    .bind(host_id)
    .bind(request.limit())
    .bind(request.offset())
    .fetch_all(pool)
    .await?;
    let user_ids: Vec<String> = rows.iter().map(|row| row.0.clone()).collect();
    let grants = sqlx::query_as::<_, (String, String)>(
        "SELECT user_id, permission FROM identity_grants WHERE host_id = $1 AND scope_type = 'HOST' AND user_id = ANY($2) AND (expires_at IS NULL OR expires_at > now()) ORDER BY permission",
    )
    .bind(host_id)
    .bind(&user_ids)
    .fetch_all(pool)
    .await?;
    let overrides = sqlx::query_as::<_, (String, String, String, String)>(
        "SELECT id, user_id, permission, effect FROM identity_member_overrides WHERE host_id = $1 AND scope_type = 'HOST' AND user_id = ANY($2) ORDER BY permission",
    )
    .bind(host_id)
    .bind(&user_ids)
    .fetch_all(pool)
    .await?;
    let mut grants_by_user: HashMap<String, Vec<String>> = HashMap::new();
    for (user_id, permission) in grants {
        grants_by_user.entry(user_id).or_default().push(permission);
    }
    let mut overrides_by_user: HashMap<String, Vec<MemberOverride>> = HashMap::new();
    for (id, user_id, permission, effect) in overrides {
        overrides_by_user.entry(user_id).or_default().push(MemberOverride { id, permission, effect });
    }
    let items = rows
        .into_iter()
        .map(|(user_id, first_name, last_name, email, phone, is_owner)| {
            let permissions = if is_owner {
                presets::DELEGABLE.iter().map(|value| value.to_string()).collect()
            } else {
                let mut values = grants_by_user.remove(&user_id).unwrap_or_default();
                values.sort();
                values.dedup();
                values
            };
            let overrides = overrides_by_user.remove(&user_id).unwrap_or_default();
            MemberView { user_id, first_name, last_name, email, phone, is_owner, permissions, overrides }
        })
        .collect();
    Ok(Page::new(items, total, request))
}

fn validate_permissions(permissions: &[String]) -> Result<Vec<String>> {
    let mut values: Vec<String> = Vec::new();
    for permission in permissions {
        if !presets::is_delegable(permission) {
            return Err(Error::bad_request(PERMISSION_INVALID).with_params(json!({ "permission": permission })));
        }
        if !values.contains(permission) {
            values.push(permission.clone());
        }
    }
    values.sort();
    Ok(values)
}

pub async fn set_member_permissions(
    pool: &PgPool,
    actor: &Actor,
    host_id: &str,
    user_id: &str,
    input: SetMemberPermissions,
) -> Result<MemberView> {
    ensure_manager(actor, host_id)?;
    if !host_in_city(pool, actor, host_id).await? {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    let permissions = validate_permissions(&input.permissions)?;
    for permission in &permissions {
        actor.require_scoped(permission, Some(host_id), None)?;
    }
    let mut tx = pool.begin().await?;
    let locked = sqlx::query_scalar::<_, bool>(
        "SELECT m.is_owner FROM host_members m WHERE m.host_id = $1 AND m.user_id = $2 AND m.status = 'ACTIVE' FOR UPDATE",
    )
    .bind(host_id)
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?;
    if locked.is_none() {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    if locked == Some(true) {
        return Err(Error::conflict(super::messages::OWNER_PROTECTED));
    }
    sqlx::query("DELETE FROM identity_grants WHERE user_id = $1 AND host_id = $2 AND scope_type = 'HOST'")
        .bind(user_id)
        .bind(host_id)
        .execute(&mut *tx)
        .await?;
    for permission in &permissions {
        sqlx::query(
            "INSERT INTO identity_grants (id, user_id, permission, scope_type, scope_id, host_id) VALUES ($1, $2, $3, 'HOST', $4, $4)",
        )
        .bind(kernel::ids::new_id("grant"))
        .bind(user_id)
        .bind(permission)
        .bind(host_id)
        .execute(&mut *tx)
        .await?;
    }
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "identity.member.permissions_updated",
        resource_type: "host_member",
        resource_id: user_id,
        reason: None,
        change: json!({ "hostId": host_id, "permissions": permissions }),
    })
    .await?;
    outbox::enqueue(&mut tx, actor.city, "host_member", user_id, "identity.member.permissions_updated", json!({ "hostId": host_id, "userId": user_id })).await?;
    tx.commit().await?;
    load_member(pool, host_id, user_id).await
}

async fn load_member(pool: &PgPool, host_id: &str, user_id: &str) -> Result<MemberView> {
    let row = sqlx::query_as::<_, MemberRow>(
        "SELECT m.user_id, u.first_name, u.last_name, u.email, u.phone, m.is_owner FROM host_members m JOIN identity_users u ON u.id = m.user_id WHERE m.host_id = $1 AND m.user_id = $2 AND m.status = 'ACTIVE' AND u.status = 'ACTIVE'",
    )
    .bind(host_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    let mut permissions = sqlx::query_scalar::<_, String>(
        "SELECT permission FROM identity_grants WHERE host_id = $1 AND user_id = $2 AND scope_type = 'HOST' AND (expires_at IS NULL OR expires_at > now()) ORDER BY permission",
    )
    .bind(host_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    if row.5 {
        permissions = presets::DELEGABLE.iter().map(|value| value.to_string()).collect();
    }
    let overrides = sqlx::query_as::<_, (String, String, String)>(
        "SELECT id, permission, effect FROM identity_member_overrides WHERE host_id = $1 AND user_id = $2 AND scope_type = 'HOST' ORDER BY permission",
    )
    .bind(host_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, permission, effect)| MemberOverride { id, permission, effect })
    .collect();
    Ok(MemberView {
        user_id: row.0,
        first_name: row.1,
        last_name: row.2,
        email: row.3,
        phone: row.4,
        is_owner: row.5,
        permissions,
        overrides,
    })
}

pub async fn remove_member(pool: &PgPool, actor: &Actor, host_id: &str, user_id: &str) -> Result<()> {
    let self_removal = actor.user_id().as_str() == user_id;
    if !self_removal {
        ensure_manager(actor, host_id)?;
    } else {
        actor.ensure_writable()?;
        if !actor.access.is_member_of(host_id) {
            return Err(Error::not_found(codes::NOT_FOUND));
        }
    }
    if !host_in_city(pool, actor, host_id).await? {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    let mut tx = pool.begin().await?;
    let owner = sqlx::query_scalar::<_, bool>(
        "SELECT m.is_owner FROM host_members m JOIN identity_users u ON u.id = m.user_id WHERE m.host_id = $1 AND m.user_id = $2 AND m.status = 'ACTIVE' AND u.status = 'ACTIVE' FOR UPDATE OF m",
    )
    .bind(host_id)
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    if owner {
        return Err(Error::conflict(super::messages::OWNER_PROTECTED));
    }
    sqlx::query("UPDATE host_members SET status = 'REMOVED', removed_at = now() WHERE host_id = $1 AND user_id = $2 AND status = 'ACTIVE'")
        .bind(host_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM identity_grants WHERE user_id = $1 AND host_id = $2 AND scope_type = 'HOST'")
        .bind(user_id)
        .bind(host_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM identity_member_overrides WHERE user_id = $1 AND host_id = $2 AND scope_type = 'HOST'")
        .bind(user_id)
        .bind(host_id)
        .execute(&mut *tx)
        .await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "identity.member.removed",
        resource_type: "host_member",
        resource_id: user_id,
        reason: None,
        change: json!({ "hostId": host_id, "userId": user_id }),
    })
    .await?;
    outbox::enqueue(&mut tx, actor.city, "host_member", user_id, "identity.member.removed", json!({ "hostId": host_id, "userId": user_id })).await?;
    tx.commit().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::permissions::names::{BILLING_SETTLEMENTS, PROPERTIES_READ};

    #[test]
    fn only_delegable_unique_permissions_are_accepted() {
        let values = validate_permissions(&[PROPERTIES_READ.into(), PROPERTIES_READ.into()]).unwrap();
        assert_eq!(values, vec![PROPERTIES_READ]);
        assert_eq!(
            validate_permissions(&[BILLING_SETTLEMENTS.into()]).unwrap_err().code().as_str(),
            "IDENTITY_PERMISSION_INVALID"
        );
    }
}
