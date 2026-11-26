use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::ids::new_id;
use kernel::outbox;
use kernel::paging::PageRequest;
use kernel::permissions::names::HOSTS_REVIEW_APPLICATIONS;
use kernel::Page;
use serde_json::json;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::hosts::messages;
use super::types::{
    reason, AdminApplicationView, AdminHostView, ApplicationFilter, HostDecisionInput, HostDocumentView,
    HostStatusInput, APPROVE_REASONS, DOCUMENT_REJECT_REASONS, REINSTATE_REASONS, REJECT_REASONS,
    SUSPEND_REASONS,
};
use super::{applications, documents};

type AdminRow = (
    String,
    String,
    Option<String>,
    Option<String>,
    String,
    String,
    String,
    String,
    String,
    Option<String>,
    Option<DateTime<Utc>>,
    Option<DateTime<Utc>>,
    Option<String>,
    DateTime<Utc>,
);

fn request(filter: &ApplicationFilter) -> Result<PageRequest> {
    PageRequest {
        page: filter.page.unwrap_or(1),
        page_size: filter.page_size.unwrap_or(20),
    }
    .validated()
}

pub async fn list_applications(
    pool: &PgPool,
    actor: &Actor,
    filter: ApplicationFilter,
) -> Result<Page<AdminApplicationView>> {
    actor.require_platform(HOSTS_REVIEW_APPLICATIONS)?;
    let request = request(&filter)?;
    let status = filter.status.as_deref().map(str::trim).filter(|value| !value.is_empty());
    let search = filter.search.as_deref().map(|value| format!("%{}%", value.trim())).filter(|value| value != "%%");
    let mut count = QueryBuilder::<Postgres>::new(
        "SELECT COUNT(*) FROM host_applications a JOIN identity_users u ON u.id = a.applicant_user_id WHERE a.city_id = ",
    );
    count.push_bind(actor.city.code());
    push_filters(&mut count, status, search.as_deref());
    let total: i64 = count.build_query_scalar().fetch_one(pool).await?;
    let mut query = QueryBuilder::<Postgres>::new(
        r#"SELECT a.id, a.applicant_user_id, u.first_name, u.last_name, COALESCE(u.email, u.phone, ''), a.city_id, a.display_name, a.kind, a.status, a.reason_code, a.submitted_at, a.decided_at, a.host_id, a.created_at FROM host_applications a JOIN identity_users u ON u.id = a.applicant_user_id WHERE a.city_id = "#,
    );
    query.push_bind(actor.city.code());
    push_filters(&mut query, status, search.as_deref());
    query.push(" ORDER BY a.created_at DESC, a.id LIMIT ").push_bind(request.limit());
    query.push(" OFFSET ").push_bind(request.offset());
    let rows = query.build_query_as::<AdminRow>().fetch_all(pool).await?;
    let items = rows
        .into_iter()
        .map(|row| {
            let name = [row.2.clone().unwrap_or_default(), row.3.clone().unwrap_or_default()]
                .into_iter()
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            AdminApplicationView {
                id: row.0,
                applicant_user_id: row.1,
                applicant_name: (!name.is_empty()).then_some(name),
                applicant_email: (!row.4.is_empty()).then_some(row.4),
                city_id: row.5,
                display_name: row.6,
                kind: row.7,
                status: row.8,
                reason_code: row.9,
                submitted_at: row.10,
                decided_at: row.11,
                host_id: row.12,
                created_at: row.13,
            }
        })
        .collect();
    Ok(Page::new(items, total, request))
}

fn push_filters(builder: &mut QueryBuilder<'_, Postgres>, status: Option<&str>, search: Option<&str>) {
    if let Some(status) = status {
        builder.push(" AND a.status = ").push_bind(status.to_string());
    }
    if let Some(search) = search {
        builder
            .push(" AND (a.display_name ILIKE ")
            .push_bind(search.to_string())
            .push(" OR COALESCE(u.email, '') ILIKE ")
            .push_bind(search.to_string())
            .push(" OR COALESCE(u.phone, '') ILIKE ")
            .push_bind(search.to_string())
            .push(")");
    }
}

pub async fn application_detail(pool: &PgPool, actor: &Actor, id: &str) -> Result<super::types::ApplicationView> {
    actor.require_platform(HOSTS_REVIEW_APPLICATIONS)?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM host_applications WHERE id = $1 AND city_id = $2)",
    )
    .bind(id)
    .bind(actor.city.code())
    .fetch_one(pool)
    .await?;
    if !exists {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    applications::review_view(pool, id).await
}

pub async fn decide(pool: &PgPool, actor: &Actor, id: &str, input: HostDecisionInput) -> Result<super::types::ApplicationView> {
    actor.require_platform(HOSTS_REVIEW_APPLICATIONS)?;
    let approve = match input.decision.trim().to_ascii_uppercase().as_str() {
        "APPROVE" => true,
        "REJECT" => false,
        _ => return Err(Error::bad_request(messages::APPLICATION_STATE)),
    };
    let allowed = if approve { APPROVE_REASONS } else { REJECT_REASONS };
    let reason_code = reason(&input.reason_code, allowed).ok_or_else(|| Error::bad_request(messages::REASON_INVALID))?;
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, (String, String, String, String, Option<String>)>(
        "SELECT applicant_user_id, display_name, kind, status, host_id FROM host_applications WHERE id = $1 AND city_id = $2 FOR UPDATE",
    )
    .bind(id)
    .bind(actor.city.code())
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| Error::not_found(codes::NOT_FOUND))?;
    if row.3 != "PENDING_REVIEW" {
        return Err(Error::conflict(messages::APPLICATION_STATE).with_params(json!({ "status": row.3 })));
    }
    let host_id = if approve {
        pending_documents(&mut tx, id).await?;
        let host_id = new_id("host");
        sqlx::query("INSERT INTO hosts (id, city_id, display_name, kind, status) VALUES ($1, $2, $3, $4, 'ACTIVE')")
            .bind(&host_id)
            .bind(actor.city.code())
            .bind(&row.1)
            .bind(&row.2)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO host_members (id, host_id, user_id, is_owner, status) VALUES ($1, $2, $3, true, 'ACTIVE')")
            .bind(new_id("member"))
            .bind(&host_id)
            .bind(&row.0)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE identity_users SET role = 'host' WHERE id = $1 AND role = 'guest'")
            .bind(&row.0)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE host_applications SET status = 'APPROVED', reason_code = $2, decided_by = $3, decided_at = now(), host_id = $4, updated_at = now() WHERE id = $1")
            .bind(id)
            .bind(reason_code)
            .bind(actor.user_id().as_str())
            .bind(&host_id)
            .execute(&mut *tx)
            .await?;
        outbox::enqueue(&mut tx, actor.city, "host", &host_id, "host.created", json!({ "hostId": host_id, "applicationId": id })).await?;
        Some(host_id)
    } else {
        sqlx::query("UPDATE host_applications SET status = 'REJECTED', reason_code = $2, decided_by = $3, decided_at = now(), updated_at = now() WHERE id = $1")
            .bind(id)
            .bind(reason_code)
            .bind(actor.user_id().as_str())
            .execute(&mut *tx)
            .await?;
        None
    };
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: if approve { "host.application.approved" } else { "host.application.rejected" },
        resource_type: "host_application",
        resource_id: id,
        reason: Some(reason_code),
        change: json!({ "decision": input.decision, "hostId": host_id }),
    })
    .await?;
    outbox::enqueue(&mut tx, actor.city, "host_application", id, if approve { "host.application.approved" } else { "host.application.rejected" }, json!({ "applicationId": id })).await?;
    tx.commit().await?;
    applications::review_view(pool, id).await
}

async fn pending_documents(tx: &mut sqlx::Transaction<'_, Postgres>, id: &str) -> Result<()> {
    let open: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM host_application_documents WHERE application_id = $1 AND status <> 'VERIFIED'",
    )
    .bind(id)
    .fetch_one(&mut **tx)
    .await?;
    if open > 0 {
        return Err(Error::conflict(messages::DOCUMENT_REQUIRED).with_params(json!({ "pending": open })));
    }
    Ok(())
}

pub async fn review_document(
    pool: &PgPool,
    actor: &Actor,
    application_id: &str,
    document_id: &str,
    input: DocumentReview,
) -> Result<HostDocumentView> {
    actor.require_platform(HOSTS_REVIEW_APPLICATIONS)?;
    let verified = match input.status.trim().to_ascii_uppercase().as_str() {
        "VERIFIED" => true,
        "REJECTED" => false,
        _ => return Err(Error::bad_request(messages::REASON_INVALID)),
    };
    let reason_code = if verified {
        None
    } else {
        Some(reason(&input.reason_code.unwrap_or_default(), DOCUMENT_REJECT_REASONS).ok_or_else(|| Error::bad_request(messages::REASON_INVALID))?)
    };
    let mut tx = pool.begin().await?;
    let updated = sqlx::query(
        "UPDATE host_application_documents d SET status = $3, reason_code = $4, reviewed_by = $5, reviewed_at = now() FROM host_applications a WHERE d.id = $1 AND d.application_id = $2 AND a.id = d.application_id AND a.city_id = $6 AND a.status = 'PENDING_REVIEW'",
    )
    .bind(document_id)
    .bind(application_id)
    .bind(if verified { "VERIFIED" } else { "REJECTED" })
    .bind(reason_code)
    .bind(actor.user_id().as_str())
    .bind(actor.city.code())
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() == 0 {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "host.application.document_reviewed",
        resource_type: "host_application_document",
        resource_id: document_id,
        reason: reason_code,
        change: json!({ "applicationId": application_id }),
    })
    .await?;
    tx.commit().await?;
    let rows = documents::list_for(pool, application_id).await?;
    rows.into_iter().find(|row| row.id == document_id).ok_or_else(|| Error::not_found(codes::NOT_FOUND))
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReview {
    pub status: String,
    pub reason_code: Option<String>,
}

pub async fn list_hosts(pool: &PgPool, actor: &Actor, filter: ApplicationFilter) -> Result<Page<AdminHostView>> {
    actor.require_platform(HOSTS_REVIEW_APPLICATIONS)?;
    let request = request(&filter)?;
    let status = filter.status.as_deref().map(str::trim).filter(|value| !value.is_empty());
    let search = filter.search.as_deref().map(|value| format!("%{}%", value.trim()));
    let mut count = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM hosts h WHERE h.city_id = ");
    count.push_bind(actor.city.code());
    push_host_filters(&mut count, status, search.as_deref());
    let total: i64 = count.build_query_scalar().fetch_one(pool).await?;
    let mut query = QueryBuilder::<Postgres>::new(
        r#"SELECT h.id, h.city_id, h.display_name, h.kind, h.status, h.created_at, h.guest_contact_phone, h.guest_contact_email,
           (SELECT COUNT(*) FROM host_members m WHERE m.host_id = h.id AND m.status = 'ACTIVE'),
           (SELECT COUNT(*) FROM stay_properties p WHERE p."operatorId" = h.id AND p.city_id = h.city_id)
           FROM hosts h WHERE h.city_id = "#,
    );
    query.push_bind(actor.city.code());
    push_host_filters(&mut query, status, search.as_deref());
    query.push(" ORDER BY h.display_name, h.id LIMIT ").push_bind(request.limit());
    query.push(" OFFSET ").push_bind(request.offset());
    let rows = query
        .build_query_as::<(String, String, String, String, String, DateTime<Utc>, String, String, i64, i64)>()
        .fetch_all(pool)
        .await?;
    let items = rows
        .into_iter()
        .map(|row| AdminHostView {
            id: row.0,
            city_id: row.1,
            display_name: row.2,
            kind: row.3,
            status: row.4,
            created_at: row.5,
            guest_contact_phone: row.6,
            guest_contact_email: row.7,
            member_count: row.8,
            property_count: row.9,
        })
        .collect();
    Ok(Page::new(items, total, request))
}

fn push_host_filters(builder: &mut QueryBuilder<'_, Postgres>, status: Option<&str>, search: Option<&str>) {
    if let Some(status) = status {
        builder.push(" AND h.status = ").push_bind(status.to_string());
    }
    if let Some(search) = search {
        builder
            .push(" AND (h.display_name ILIKE ")
            .push_bind(search.to_string())
            .push(" OR h.guest_contact_email ILIKE ")
            .push_bind(search.to_string())
            .push(")");
    }
}

pub async fn change_host_status(pool: &PgPool, actor: &Actor, host_id: &str, input: HostStatusInput) -> Result<String> {
    actor.require_platform(HOSTS_REVIEW_APPLICATIONS)?;
    let suspend = match input.action.trim().to_ascii_uppercase().as_str() {
        "SUSPEND" => true,
        "REINSTATE" => false,
        _ => return Err(Error::bad_request(messages::HOST_STATUS_INVALID)),
    };
    let reason_code = reason(&input.reason_code, if suspend { SUSPEND_REASONS } else { REINSTATE_REASONS })
        .ok_or_else(|| Error::bad_request(messages::REASON_INVALID))?;
    let mut tx = pool.begin().await?;
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM hosts WHERE id = $1 AND city_id = $2 FOR UPDATE",
    )
    .bind(host_id)
    .bind(actor.city.code())
    .fetch_optional(&mut *tx)
    .await?;
    let expected = if suspend { "ACTIVE" } else { "SUSPENDED" };
    if status.as_deref() != Some(expected) {
        return Err(Error::conflict(messages::HOST_STATUS_INVALID).with_params(json!({ "status": status })));
    }
    let next = if suspend { "SUSPENDED" } else { "ACTIVE" };
    sqlx::query("UPDATE hosts SET status = $2 WHERE id = $1").bind(host_id).bind(next).execute(&mut *tx).await?;
    sqlx::query("INSERT INTO host_status_history (id, host_id, status, reason_code, actor_user_id) VALUES ($1, $2, $3, $4, $5)")
        .bind(new_id("hoststatus"))
        .bind(host_id)
        .bind(next)
        .bind(reason_code)
        .bind(actor.user_id().as_str())
        .execute(&mut *tx)
        .await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: "host.status.changed",
        resource_type: "host",
        resource_id: host_id,
        reason: Some(reason_code),
        change: json!({ "status": next }),
    })
    .await?;
    outbox::enqueue(&mut tx, actor.city, "host", host_id, "host.status.changed", json!({ "hostId": host_id, "status": next })).await?;
    tx.commit().await?;
    Ok(next.to_string())
}
