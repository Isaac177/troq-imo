use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::ids::new_id;
use kernel::outbox;
use kernel::paging::{Page, PageRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

use super::messages::PERMISSION_INVALID;
use super::presets;
use super::team::ensure_manager;

const INVITATION_DAYS: i64 = 7;
const MAX_EMAIL: usize = 254;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvitation {
    pub email: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvitationView {
    pub id: String,
    pub host_id: String,
    pub email: String,
    pub permissions: Vec<String>,
    pub status: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyInvitationView {
    pub id: String,
    pub host_id: String,
    pub host_name: String,
    pub permissions: Vec<String>,
    pub expires_at: DateTime<Utc>,
}

fn clean_email(value: &str) -> Result<String> {
    let email = value.trim().to_ascii_lowercase();
    let valid = email.len() <= MAX_EMAIL
        && email.is_ascii()
        && !email.chars().any(|c| c.is_whitespace() || c.is_control())
        && email
            .split_once('@')
            .is_some_and(|(local, domain)| {
                !local.is_empty() && !domain.contains('@') && domain.contains('.')
                    && !domain.starts_with('.') && !domain.ends_with('.')
            });
    if valid {
        Ok(email)
    } else {
        Err(Error::bad_request(super::messages::CONTACT_INVALID))
    }
}

fn clean_permissions(values: &[String]) -> Result<Vec<String>> {
    let mut permissions = Vec::new();
    for permission in values {
        if !presets::is_delegable(permission) {
            return Err(Error::bad_request(PERMISSION_INVALID).with_params(json!({ "permission": permission })));
        }
        if !permissions.contains(permission) {
            permissions.push(permission.clone());
        }
    }
    if permissions.is_empty() {
        return Err(Error::bad_request(PERMISSION_INVALID));
    }
    permissions.sort();
    Ok(permissions)
}

async fn active_host(pool: &PgPool, actor: &Actor, host_id: &str) -> Result<()> {
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM hosts WHERE id = $1 AND city_id = $2 AND status = 'ACTIVE')",
    )
    .bind(host_id)
    .bind(actor.city.code())
    .fetch_one(pool)
    .await?;
    if exists {
        Ok(())
    } else {
        Err(Error::not_found(codes::NOT_FOUND))
    }
}

pub async fn create_invitation(
    pool: &PgPool,
    actor: &Actor,
    host_id: &str,
    input: CreateInvitation,
) -> Result<InvitationView> {
    ensure_manager(actor, host_id)?;
    active_host(pool, actor, host_id).await?;
    let email = clean_email(&input.email)?;
    let permissions = clean_permissions(&input.permissions)?;
    for permission in &permissions {
        actor.require_scoped(permission, Some(host_id), None)?;
    }
    let existing_member: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM host_members m JOIN identity_users u ON u.id = m.user_id WHERE m.host_id = $1 AND m.status = 'ACTIVE' AND lower(u.email) = $2)",
    )
    .bind(host_id)
    .bind(&email)
    .fetch_one(pool)
    .await?;
    if existing_member {
        return Err(Error::conflict(codes::NOT_FOUND).with_params(json!({ "member": true })));
    }
    let id = new_id("invite");
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, (DateTime<Utc>, DateTime<Utc>)>(
        "INSERT INTO identity_invitations (id, host_id, email, permissions, invited_by, expires_at) VALUES ($1, $2, $3, $4, $5, now() + make_interval(days => $6::int)) ON CONFLICT (host_id, lower(email)) WHERE status = 'PENDING' DO NOTHING RETURNING expires_at, created_at",
    )
    .bind(&id)
    .bind(host_id)
    .bind(&email)
    .bind(&permissions)
    .bind(actor.user_id().as_str())
    .bind(INVITATION_DAYS)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| Error::conflict(codes::NOT_FOUND).with_params(json!({ "pending": true })))?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "identity.invitation.created",
        resource_type: "identity_invitation",
        resource_id: &id,
        reason: None,
        change: json!({ "hostId": host_id, "email": email, "permissions": permissions }),
    })
    .await?;
    outbox::enqueue(&mut tx, actor.city, "identity_invitation", &id, "identity.invitation.created", json!({ "hostId": host_id, "email": email })).await?;
    tx.commit().await?;
    Ok(InvitationView {
        id,
        host_id: host_id.to_owned(),
        email,
        permissions,
        status: "PENDING".into(),
        expires_at: row.0,
        created_at: row.1,
    })
}

pub async fn list_invitations(
    pool: &PgPool,
    actor: &Actor,
    host_id: &str,
    request: PageRequest,
) -> Result<Page<InvitationView>> {
    ensure_manager(actor, host_id)?;
    active_host(pool, actor, host_id).await?;
    let request = request.validated()?;
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM identity_invitations WHERE host_id = $1 AND status = 'PENDING' AND expires_at > now()",
    )
    .bind(host_id)
    .fetch_one(pool)
    .await?;
    let rows = sqlx::query_as::<_, (String, String, Vec<String>, String, DateTime<Utc>, DateTime<Utc>)>(
        "SELECT id, email, permissions, status, expires_at, created_at FROM identity_invitations WHERE host_id = $1 AND status = 'PENDING' AND expires_at > now() ORDER BY created_at DESC, id LIMIT $2 OFFSET $3",
    )
    .bind(host_id)
    .bind(request.limit())
    .bind(request.offset())
    .fetch_all(pool)
    .await?;
    let items = rows
        .into_iter()
        .map(|(id, email, permissions, status, expires_at, created_at)| InvitationView {
            id,
            host_id: host_id.to_owned(),
            email,
            permissions,
            status,
            expires_at,
            created_at,
        })
        .collect();
    Ok(Page::new(items, total, request))
}

pub async fn revoke_invitation(pool: &PgPool, actor: &Actor, host_id: &str, invitation_id: &str) -> Result<()> {
    ensure_manager(actor, host_id)?;
    active_host(pool, actor, host_id).await?;
    let mut tx = pool.begin().await?;
    let revoked = sqlx::query(
        "UPDATE identity_invitations SET status = 'REVOKED', resolved_at = now() WHERE id = $1 AND host_id = $2 AND status = 'PENDING'",
    )
    .bind(invitation_id)
    .bind(host_id)
    .execute(&mut *tx)
    .await?;
    if revoked.rows_affected() == 0 {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "identity.invitation.revoked",
        resource_type: "identity_invitation",
        resource_id: invitation_id,
        reason: None,
        change: json!({ "hostId": host_id }),
    })
    .await?;
    outbox::enqueue(&mut tx, actor.city, "identity_invitation", invitation_id, "identity.invitation.revoked", json!({ "hostId": host_id })).await?;
    tx.commit().await?;
    Ok(())
}

fn invitation_status(status: &str, expires_at: DateTime<Utc>) -> String {
    if status == "PENDING" && expires_at <= Utc::now() {
        "EXPIRED".into()
    } else {
        status.to_string()
    }
}

pub async fn list_my_invitations(pool: &PgPool, actor: &Actor, request: PageRequest) -> Result<Page<MyInvitationView>> {
    let request = request.validated()?;
    let email: Option<String> = sqlx::query_scalar("SELECT lower(email) FROM identity_users WHERE id = $1 AND status = 'ACTIVE'")
        .bind(actor.user_id().as_str())
        .fetch_optional(pool)
        .await?
        .flatten();
    let Some(email) = email else {
        return Ok(Page::new(Vec::new(), 0, request));
    };
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM identity_invitations WHERE lower(email) = $1 AND status = 'PENDING' AND expires_at > now()",
    )
    .bind(&email)
    .fetch_one(pool)
    .await?;
    let rows = sqlx::query_as::<_, (String, String, String, Vec<String>, DateTime<Utc>)>(
        "SELECT i.id, i.host_id, h.display_name, i.permissions, i.expires_at FROM identity_invitations i JOIN hosts h ON h.id = i.host_id WHERE lower(i.email) = $1 AND i.status = 'PENDING' AND i.expires_at > now() AND h.status = 'ACTIVE' ORDER BY i.created_at DESC, i.id LIMIT $2 OFFSET $3",
    )
    .bind(&email)
    .bind(request.limit())
    .bind(request.offset())
    .fetch_all(pool)
    .await?;
    let items = rows
        .into_iter()
        .map(|(id, host_id, host_name, permissions, expires_at)| MyInvitationView { id, host_id, host_name, permissions, expires_at })
        .collect();
    Ok(Page::new(items, total, request))
}

pub async fn accept_invitation(pool: &PgPool, actor: &Actor, invitation_id: &str) -> Result<InvitationView> {
    actor.ensure_writable()?;
    let email: Option<String> = sqlx::query_scalar("SELECT lower(email) FROM identity_users WHERE id = $1 AND status = 'ACTIVE'")
        .bind(actor.user_id().as_str())
        .fetch_optional(pool)
        .await?
        .flatten();
    let email = email.ok_or_else(|| Error::forbidden(codes::AUTH_REQUIRED))?;
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, (String, String, Vec<String>, String, DateTime<Utc>, DateTime<Utc>)>(
        "SELECT id, host_id, permissions, status, expires_at, created_at FROM identity_invitations WHERE id = $1 AND lower(email) = $2 FOR UPDATE",
    )
    .bind(invitation_id)
    .bind(&email)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    if row.3 != "PENDING" || row.4 <= Utc::now() {
        return Err(Error::conflict(codes::NOT_FOUND).with_params(json!({ "invitation": invitation_status(&row.3, row.4) })));
    }
    let host: Option<(String, String)> = sqlx::query_as(
        "SELECT id, city_id FROM hosts WHERE id = $1 AND status = 'ACTIVE' FOR SHARE",
    )
    .bind(&row.1)
    .fetch_optional(&mut *tx)
    .await?;
    let Some((host_id, city)) = host else {
        return Err(Error::conflict(codes::NOT_FOUND).with_params(json!({ "host": "inactive" })));
    };
    if city != actor.city.code() {
        return Err(Error::forbidden(codes::CITY_MISMATCH));
    }
    sqlx::query(
        "INSERT INTO host_members (id, host_id, user_id, is_owner, status) VALUES ($1, $2, $3, false, 'ACTIVE') ON CONFLICT (host_id, user_id) WHERE status = 'ACTIVE' DO NOTHING",
    )
    .bind(new_id("member"))
    .bind(&host_id)
    .bind(actor.user_id().as_str())
    .execute(&mut *tx)
    .await?;
    for permission in &row.2 {
        sqlx::query(
            "INSERT INTO identity_grants (id, user_id, permission, scope_type, scope_id, host_id) VALUES ($1, $2, $3, 'HOST', $4, $4)",
        )
        .bind(new_id("grant"))
        .bind(actor.user_id().as_str())
        .bind(permission)
        .bind(&host_id)
        .execute(&mut *tx)
        .await?;
    }
    sqlx::query("UPDATE identity_invitations SET status = 'ACCEPTED', accepted_by = $2, resolved_at = now() WHERE id = $1")
        .bind(invitation_id)
        .bind(actor.user_id().as_str())
        .execute(&mut *tx)
        .await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "identity.invitation.accepted",
        resource_type: "identity_invitation",
        resource_id: invitation_id,
        reason: None,
        change: json!({ "hostId": host_id }),
    })
    .await?;
    outbox::enqueue(&mut tx, actor.city, "identity_invitation", invitation_id, "identity.invitation.accepted", json!({ "hostId": host_id, "userId": actor.user_id().as_str() })).await?;
    tx.commit().await?;
    Ok(InvitationView {
        id: row.0,
        host_id,
        email,
        permissions: row.2,
        status: "ACCEPTED".into(),
        expires_at: row.4,
        created_at: row.5,
    })
}

pub async fn expire_invitations(pool: &PgPool, limit: i64) -> Result<u64> {
    if !(1..=1000).contains(&limit) {
        return Err(Error::bad_request(kernel::error::codes::PAGE_SIZE_INVALID));
    }
    let result = sqlx::query(
        "UPDATE identity_invitations SET status = 'EXPIRED', resolved_at = now() WHERE id IN (SELECT id FROM identity_invitations WHERE status = 'PENDING' AND expires_at <= now() ORDER BY expires_at LIMIT $1 FOR UPDATE SKIP LOCKED)",
    )
    .bind(limit)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emails_are_normalized_and_validated() {
        assert_eq!(clean_email(" User@Example.com ").unwrap(), "user@example.com");
        assert!(clean_email("bad@example").is_err());
        assert!(clean_email("two@@example.com").is_err());
    }

    #[test]
    fn invitations_require_a_delegable_permission() {
        assert!(clean_permissions(&[]).is_err());
        assert!(clean_permissions(&[kernel::permissions::names::BILLING_SETTLEMENTS.into()]).is_err());
        assert_eq!(clean_permissions(&[kernel::permissions::names::PROPERTIES_READ.into()]).unwrap().len(), 1);
    }

    #[test]
    fn expired_pending_invitations_report_expired() {
        assert_eq!(invitation_status("PENDING", Utc::now() - chrono::Duration::minutes(1)), "EXPIRED");
        assert_eq!(invitation_status("REVOKED", Utc::now() - chrono::Duration::minutes(1)), "REVOKED");
    }
}
