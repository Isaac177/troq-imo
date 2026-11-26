use futures::future::BoxFuture;
use kernel::auth::AccessDirectory;
use kernel::error::{codes, Error, Result};
use kernel::ids::{HostId, UserId};
use kernel::permissions::{Access, Grant, GrantRow};
use sqlx::PgPool;

use super::overrides::{apply_overrides, OverrideEffect, PermissionOverride};

#[derive(Clone)]
pub struct IdentityDirectory {
    pool: PgPool,
}

impl IdentityDirectory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AccessDirectory for IdentityDirectory {
    fn access_for<'a>(&'a self, user_id: &'a UserId) -> BoxFuture<'a, Result<Access>> {
        Box::pin(async move {
            let home_city: Option<String> = sqlx::query_scalar(
                "SELECT home_city_id FROM identity_users WHERE id = $1 AND status = 'ACTIVE'",
            )
            .bind(user_id.as_str())
            .fetch_optional(&self.pool)
            .await?;
            let auth_city = home_city.ok_or_else(|| Error::unauthorized(codes::AUTH_REQUIRED))?;
            let memberships = sqlx::query_as::<_, (String, bool)>(
                "SELECT m.host_id, m.is_owner FROM host_members m JOIN hosts h ON h.id = m.host_id WHERE m.user_id = $1 AND m.status = 'ACTIVE' AND h.status = 'ACTIVE' ORDER BY m.host_id",
            )
            .bind(user_id.as_str())
            .fetch_all(&self.pool)
            .await?;
            let host_ids: Vec<HostId> = memberships.iter().map(|(host_id, _)| HostId::new(host_id)).collect();
            let rows = sqlx::query_as::<_, (String, String, Option<String>)>(
                "SELECT g.permission, g.scope_type, g.scope_id FROM identity_grants g WHERE g.user_id = $1 AND g.scope_type <> 'PROPERTY' AND (g.expires_at IS NULL OR g.expires_at > now()) AND (g.scope_type = 'PLATFORM' OR EXISTS (SELECT 1 FROM host_members m JOIN hosts h ON h.id = m.host_id WHERE m.user_id = g.user_id AND m.host_id = g.host_id AND m.status = 'ACTIVE' AND h.status = 'ACTIVE')) ORDER BY g.id",
            )
            .bind(user_id.as_str())
            .fetch_all(&self.pool)
            .await?;
            let mut grants: Vec<Grant> = rows
                .into_iter()
                .filter_map(|(permission, scope_type, scope_id)| {
                    if !kernel::permissions::names::is_known(&permission) {
                        return None;
                    }
                    Grant::from_row(GrantRow { permission, scope_type, scope_id })
                })
                .collect();
            for (host_id, is_owner) in &memberships {
                if *is_owner {
                    for permission in super::presets::DELEGABLE {
                        let grant = Grant::host(permission, host_id);
                        if !grants.contains(&grant) {
                            grants.push(grant);
                        }
                    }
                }
            }
            let rows = sqlx::query_as::<_, (String, String, Option<String>, String)>(
                "SELECT o.permission, o.scope_type, o.scope_id, o.effect FROM identity_member_overrides o WHERE o.user_id = $1 AND o.scope_type <> 'PROPERTY' AND (o.scope_type = 'PLATFORM' OR EXISTS (SELECT 1 FROM host_members m JOIN hosts h ON h.id = m.host_id WHERE m.user_id = o.user_id AND m.host_id = o.host_id AND m.status = 'ACTIVE' AND h.status = 'ACTIVE')) ORDER BY o.created_at, o.id",
            )
            .bind(user_id.as_str())
            .fetch_all(&self.pool)
            .await?;
            let overrides: Vec<PermissionOverride> = rows.into_iter().filter_map(|(permission, scope_type, scope_id, effect)| {
                if !kernel::permissions::names::is_known(&permission) {
                    return None;
                }
                let grant = Grant::from_row(GrantRow { permission, scope_type, scope_id })?;
                let effect = match effect.as_str() {
                    "ALLOW" => OverrideEffect::Allow,
                    "DENY" => OverrideEffect::Deny,
                    _ => return None,
                };
                Some(PermissionOverride { grant, effect })
            }).collect();
            apply_overrides(&mut grants, &overrides);
            Ok(Access { grants, host_ids, auth_city: Some(auth_city) })
        })
    }

    fn revoked_before<'a>(&'a self, user_id: &'a UserId) -> BoxFuture<'a, Result<Option<i64>>> {
        Box::pin(async move {
            let cutoff: Option<(Option<i64>,)> = sqlx::query_as(
                "SELECT revoked_before FROM identity_users WHERE id = $1 AND status = 'ACTIVE'",
            )
            .bind(user_id.as_str())
            .fetch_optional(&self.pool)
            .await?;
            cutoff.map(|row| row.0).ok_or_else(|| Error::unauthorized(codes::AUTH_REQUIRED))
        })
    }
}
