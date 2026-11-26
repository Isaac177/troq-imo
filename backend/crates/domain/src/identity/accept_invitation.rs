use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::ids::new_id;
use kernel::outbox;
use serde_json::json;
use sqlx::PgPool;

use super::invitations::{invitation_status, InvitationView};

pub async fn accept_invitation(pool: &PgPool, actor: &Actor, invitation_id: &str) -> Result<InvitationView> {
    actor.ensure_writable()?;
    let user: Option<(Option<String>, String)> = sqlx::query_as("SELECT lower(email), home_city_id FROM identity_users WHERE id = $1 AND status = 'ACTIVE'")
        .bind(actor.user_id().as_str())
        .fetch_optional(pool)
        .await?;
    let (email, home_city) = user.ok_or_else(|| Error::forbidden(codes::AUTH_REQUIRED))?;
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
    if city != actor.city.code() || city != home_city {
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

