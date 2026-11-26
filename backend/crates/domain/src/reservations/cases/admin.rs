use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{City, Error, Kernel, Page, PageRequest, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use utoipa::{IntoParams, ToSchema};

use super::evidence::{evidence_for_case, EvidenceView};
use super::queries;
use super::read::ensure_case_state;
use super::rules::{ensure_closable, validate_reason, CASE_TYPES};
use super::types::CloseCaseInput;
use crate::reservations::codes::{CASE_NOT_FOUND, CASE_TYPE_UNKNOWN, CASE_VERSION_CONFLICT};
use crate::reservations::events::{self, types::CASE_CLOSED};
use crate::reservations::support::{escape_like, paging};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct AdminCaseQuery {
    pub state: Option<String>,
    pub case_type: Option<String>,
    pub host_id: Option<String>,
    pub overdue: Option<bool>,
    pub escalated: Option<bool>,
    pub assignee: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminCaseItem {
    pub id: String,
    pub operator_id: String,
    pub operator_name: String,
    pub reservation_id: Option<String>,
    pub reservation_reference: Option<String>,
    pub case_type: String,
    pub reason: String,
    #[schema(value_type = Object)]
    pub parties: Value,
    pub state: String,
    pub deadline_at: DateTime<Utc>,
    pub is_overdue: bool,
    #[schema(value_type = Option<Object>)]
    pub resolution: Option<Value>,
    pub claim_amount: Option<String>,
    pub assigned_to_user_id: Option<String>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub escalated_at: Option<DateTime<Utc>>,
    pub escalation_reason: Option<String>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminCaseDetail {
    pub item: AdminCaseItem,
    pub evidence: Vec<EvidenceView>,
}

const SELECT_SQL: &str = r#"SELECT c.id, c."operatorId" AS operator_id, o.display_name AS operator_name,
       c."reservationId" AS reservation_id, r.reference AS reservation_reference,
       c."caseType" AS case_type, c.reason, c.parties, c.state, c."deadlineAt" AS deadline_at,
       (c.state IN ('OPEN', 'AWAITING_RESPONSE') AND c."deadlineAt" < now()) AS is_overdue,
       c.resolution, c."claimAmount"::text AS claim_amount,
       c."assignedToUserId" AS assigned_to_user_id, c."assignedAt" AS assigned_at,
       c."escalatedAt" AS escalated_at, c."escalationReason" AS escalation_reason, c.version,
       c."createdAt" AS created_at, c."closedAt" AS closed_at"#;

const FROM_SQL: &str = r#" FROM stay_cases c
       JOIN hosts o ON o.id = c."operatorId"
       LEFT JOIN stay_reservations r ON r.id = c."reservationId"
       WHERE TRUE"#;

pub async fn list_cases(kernel: &Kernel, actor: &Actor, query: AdminCaseQuery) -> Result<Page<AdminCaseItem>> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    if let Some(state) = query.state.as_deref() {
        ensure_case_state(state)?;
    }
    if let Some(case_type) = query.case_type.as_deref() {
        if !CASE_TYPES.contains(&case_type) {
            return Err(Error::bad_request(CASE_TYPE_UNKNOWN).with_params(json!({ "caseType": case_type })));
        }
    }
    let page = paging(query.page, query.page_size)?;
    let (items, total) = fetch(kernel.db.pool(), actor.city, &query, page).await?;
    Ok(Page::new(items, total, page))
}

async fn fetch(pool: &PgPool, city: City, query: &AdminCaseQuery, page: PageRequest) -> Result<(Vec<AdminCaseItem>, i64)> {
    let mut total_builder = QueryBuilder::<Postgres>::new(format!("SELECT COUNT(*){FROM_SQL}"));
    push_filters(&mut total_builder, city, query);
    let total: i64 = total_builder.build_query_scalar().fetch_one(pool).await?;
    let mut items_builder = QueryBuilder::<Postgres>::new(format!("{SELECT_SQL}{FROM_SQL}"));
    push_filters(&mut items_builder, city, query);
    items_builder
        .push(r#" ORDER BY (c.state IN ('OPEN', 'AWAITING_RESPONSE')) DESC, (c."escalatedAt" IS NOT NULL) DESC, c."deadlineAt" ASC, c.id LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let items = items_builder.build_query_as::<AdminCaseItem>().fetch_all(pool).await?;
    Ok((items, total))
}

fn push_filters(builder: &mut QueryBuilder<'_, Postgres>, city: City, query: &AdminCaseQuery) {
    builder.push(" AND c.city_id = ").push_bind(city.code());
    if let Some(state) = query.state.as_deref() {
        builder.push(" AND c.state = ").push_bind(state.to_string());
    }
    if let Some(case_type) = query.case_type.as_deref() {
        builder.push(r#" AND c."caseType" = "#).push_bind(case_type.to_string());
    }
    if let Some(host_id) = query.host_id.as_deref().filter(|value| !value.trim().is_empty()) {
        builder.push(r#" AND c."operatorId" = "#).push_bind(host_id.to_string());
    }
    if query.overdue == Some(true) {
        builder.push(r#" AND c.state IN ('OPEN', 'AWAITING_RESPONSE') AND c."deadlineAt" < now()"#);
    }
    if query.escalated == Some(true) {
        builder.push(r#" AND c."escalatedAt" IS NOT NULL"#);
    }
    if let Some(assignee) = query.assignee.as_deref().filter(|value| !value.trim().is_empty()) {
        if assignee == "none" {
            builder.push(r#" AND c."assignedToUserId" IS NULL"#);
        } else {
            builder.push(r#" AND c."assignedToUserId" = "#).push_bind(assignee.to_string());
        }
    }
    if let Some(search) = query.search.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        let pattern = format!("%{}%", escape_like(search));
        builder
            .push(" AND (c.reason ILIKE ")
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR o.display_name ILIKE "#)
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR r.reference ILIKE "#)
            .push_bind(pattern)
            .push(r#" ESCAPE '\')"#);
    }
}

pub async fn find_item(pool: &PgPool, city: City, case_id: &str) -> Result<AdminCaseItem> {
    sqlx::query_as::<_, AdminCaseItem>(&format!("{SELECT_SQL}{FROM_SQL} AND c.id = $1 AND c.city_id = $2"))
        .bind(case_id)
        .bind(city.code())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found(CASE_NOT_FOUND))
}

pub async fn get_case(kernel: &Kernel, actor: &Actor, case_id: &str) -> Result<AdminCaseDetail> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let pool = kernel.db.pool();
    let item = find_item(pool, actor.city, case_id).await?;
    let evidence = evidence_for_case(pool, &item.id).await?;
    Ok(AdminCaseDetail { item, evidence })
}

pub async fn close_case(kernel: &Kernel, actor: &Actor, case_id: &str, input: CloseCaseInput) -> Result<AdminCaseItem> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let pool = kernel.db.pool();
    let resolution = validate_reason(&input.resolution)?;
    let case = find_item(pool, actor.city, case_id).await?;
    ensure_closable(&case.state)?;
    let mut tx = pool.begin().await?;
    if !queries::close_if_version(&mut tx, &case.id, input.expected_version, &resolution).await? {
        return Err(Error::conflict(CASE_VERSION_CONFLICT));
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "case.close",
            resource_type: "case",
            resource_id: &case.id,
            reason: Some(&resolution),
            change: json!({ "from": case.state, "to": "CLOSED", "byPlatform": true }),
        },
    )
    .await?;
    events::emit(&mut tx, actor.city, "case", &case.id, CASE_CLOSED, json!({ "caseId": case.id, "operatorId": case.operator_id })).await?;
    tx.commit().await?;
    find_item(pool, actor.city, &case.id).await
}
