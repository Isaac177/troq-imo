use chrono::{DateTime, Utc};
use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{City, Error, Kernel, Page, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use utoipa::{IntoParams, ToSchema};

use super::queries::{self, NewTask};
use super::rules::{checklist, ensure_choice, next_state, validate_new, Action, PRIORITIES, TASK_STATES, TASK_TYPES};
use super::types::{CreateTaskInput, TransitionInput};
use crate::reservations::codes::{
    PLATFORM_CANNOT_TAKE_TASK, PROPERTY_NOT_FOUND, TASK_NOT_FOUND, TASK_PRIORITY_UNKNOWN, TASK_STATE_UNKNOWN,
    TASK_TYPE_UNKNOWN, TASK_VERSION_CONFLICT,
};
use crate::reservations::events::{self, types::{TASK_CREATED, TASK_UPDATED}};
use crate::reservations::support::{escape_like, paging};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct AdminTaskQuery {
    pub state: Option<String>,
    pub task_type: Option<String>,
    pub priority: Option<String>,
    pub host_id: Option<String>,
    pub property_id: Option<String>,
    pub assignee: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminTaskItem {
    pub id: String,
    pub property_id: String,
    pub property_name_en: String,
    pub property_name_fr: String,
    pub operator_id: String,
    pub operator_name: String,
    pub unit_code: Option<String>,
    pub reservation_reference: Option<String>,
    pub task_type: String,
    pub assignee_user_id: Option<String>,
    pub state: String,
    pub priority: String,
    #[schema(value_type = Vec<String>)]
    pub checklist: Value,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

const SELECT_SQL: &str = r#"SELECT t.id, t."propertyId" AS property_id, p."nameEn" AS property_name_en,
       p."nameFr" AS property_name_fr, p."operatorId" AS operator_id, o.display_name AS operator_name,
       u."localCode" AS unit_code, r.reference AS reservation_reference, t."taskType" AS task_type,
       t."assigneeUserId" AS assignee_user_id, t.state, t.priority, t.checklist, t.version,
       t."createdAt" AS created_at, t."completedAt" AS completed_at"#;

const FROM_SQL: &str = r#" FROM stay_operations_tasks t
       JOIN stay_properties p ON p.id = t."propertyId"
       JOIN hosts o ON o.id = p."operatorId"
       LEFT JOIN stay_physical_units u ON u.id = t."physicalUnitId"
       LEFT JOIN stay_reservation_items i ON i.id = t."reservationItemId"
       LEFT JOIN stay_reservations r ON r.id = i."reservationId"
       WHERE TRUE"#;

pub async fn list_tasks(kernel: &Kernel, actor: &Actor, query: AdminTaskQuery) -> Result<Page<AdminTaskItem>> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    ensure_choice(query.state.as_deref(), &TASK_STATES, TASK_STATE_UNKNOWN)?;
    ensure_choice(query.task_type.as_deref(), &TASK_TYPES, TASK_TYPE_UNKNOWN)?;
    ensure_choice(query.priority.as_deref(), &PRIORITIES, TASK_PRIORITY_UNKNOWN)?;
    let page = paging(query.page, query.page_size)?;
    let pool = kernel.db.pool();
    let mut total_builder = QueryBuilder::<Postgres>::new(format!("SELECT COUNT(*){FROM_SQL}"));
    push_filters(&mut total_builder, actor.city, &query);
    let total: i64 = total_builder.build_query_scalar().fetch_one(pool).await?;
    let mut items_builder = QueryBuilder::<Postgres>::new(format!("{SELECT_SQL}{FROM_SQL}"));
    push_filters(&mut items_builder, actor.city, &query);
    items_builder
        .push(
            r#" ORDER BY (t.state IN ('OPEN', 'IN_PROGRESS')) DESC,
                CASE t.priority WHEN 'URGENT' THEN 0 WHEN 'HIGH' THEN 1 WHEN 'NORMAL' THEN 2 ELSE 3 END,
                t."createdAt" ASC, t.id LIMIT "#,
        )
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let items = items_builder.build_query_as::<AdminTaskItem>().fetch_all(pool).await?;
    Ok(Page::new(items, total, page))
}

fn push_filters(builder: &mut QueryBuilder<'_, Postgres>, city: City, query: &AdminTaskQuery) {
    builder.push(" AND t.city_id = ").push_bind(city.code());
    if let Some(state) = query.state.as_deref() {
        builder.push(" AND t.state = ").push_bind(state.to_string());
    }
    if let Some(task_type) = query.task_type.as_deref() {
        builder.push(r#" AND t."taskType" = "#).push_bind(task_type.to_string());
    }
    if let Some(priority) = query.priority.as_deref() {
        builder.push(" AND t.priority = ").push_bind(priority.to_string());
    }
    if let Some(host_id) = query.host_id.as_deref().filter(|value| !value.trim().is_empty()) {
        builder.push(r#" AND p."operatorId" = "#).push_bind(host_id.to_string());
    }
    if let Some(property_id) = query.property_id.as_deref().filter(|value| !value.trim().is_empty()) {
        builder.push(r#" AND t."propertyId" = "#).push_bind(property_id.to_string());
    }
    if let Some(assignee) = query.assignee.as_deref().filter(|value| !value.trim().is_empty()) {
        if assignee == "none" {
            builder.push(r#" AND t."assigneeUserId" IS NULL"#);
        } else {
            builder.push(r#" AND t."assigneeUserId" = "#).push_bind(assignee.to_string());
        }
    }
    if let Some(search) = query.search.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
        let pattern = format!("%{}%", escape_like(search));
        builder
            .push(r#" AND (p."nameEn" ILIKE "#)
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR p."nameFr" ILIKE "#)
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR o.display_name ILIKE "#)
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR u."localCode" ILIKE "#)
            .push_bind(pattern)
            .push(r#" ESCAPE '\')"#);
    }
}

async fn find_item(pool: &PgPool, city: City, task_id: &str) -> Result<AdminTaskItem> {
    sqlx::query_as::<_, AdminTaskItem>(&format!("{SELECT_SQL}{FROM_SQL} AND t.id = $1 AND t.city_id = $2"))
        .bind(task_id)
        .bind(city.code())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found(TASK_NOT_FOUND))
}

pub async fn get_task(kernel: &Kernel, actor: &Actor, task_id: &str) -> Result<AdminTaskItem> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    find_item(kernel.db.pool(), actor.city, task_id).await
}

pub async fn create_task(kernel: &Kernel, actor: &Actor, input: CreateTaskInput) -> Result<AdminTaskItem> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let priority = validate_new(&input.task_type, input.priority.as_deref())?;
    let checklist = checklist(input.checklist)?;
    let pool = kernel.db.pool();
    let city = actor.city;
    if queries::property_host(pool, city, &input.property_id).await?.is_none() {
        return Err(Error::not_found(PROPERTY_NOT_FOUND));
    }
    let task_id = new_id("task");
    let mut tx = pool.begin().await?;
    queries::insert_task(
        &mut tx,
        &NewTask {
            id: &task_id,
            city,
            property_id: &input.property_id,
            physical_unit_id: None,
            reservation_item_id: None,
            task_type: &input.task_type,
            priority: &priority,
            checklist: &checklist,
        },
    )
    .await?;
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "operations.task.create",
            resource_type: "operations_task",
            resource_id: &task_id,
            reason: None,
            change: json!({ "taskType": input.task_type, "propertyId": input.property_id, "byPlatform": true }),
        },
    )
    .await?;
    events::emit(
        &mut tx,
        city,
        "operations_task",
        &task_id,
        TASK_CREATED,
        json!({ "taskId": task_id, "propertyId": input.property_id, "taskType": input.task_type }),
    )
    .await?;
    tx.commit().await?;
    find_item(pool, city, &task_id).await
}

pub async fn transition_task(kernel: &Kernel, actor: &Actor, task_id: &str, input: TransitionInput) -> Result<AdminTaskItem> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let action = Action::parse(&input.action)?;
    if action == Action::AssignMe {
        return Err(Error::bad_request(PLATFORM_CANNOT_TAKE_TASK));
    }
    let pool = kernel.db.pool();
    let city = actor.city;
    let task = find_item(pool, city, task_id).await?;
    let state = next_state(&task.state, action)?;
    let assignee = if action == Action::Unassign { None } else { task.assignee_user_id.clone() };
    let completed = state == "DONE" || state == "CANCELLED";
    let mut tx = pool.begin().await?;
    if !queries::update_if_version(&mut tx, &task.id, input.expected_version, state, assignee.as_deref(), completed).await? {
        return Err(Error::conflict(TASK_VERSION_CONFLICT));
    }
    audit::record(
        &mut tx,
        &actor.audit(),
        AuditEntry {
            action: "operations.task.transition",
            resource_type: "operations_task",
            resource_id: &task.id,
            reason: None,
            change: json!({ "action": input.action, "from": task.state, "to": state, "byPlatform": true }),
        },
    )
    .await?;
    events::emit(&mut tx, city, "operations_task", &task.id, TASK_UPDATED, json!({ "taskId": task.id, "state": state })).await?;
    tx.commit().await?;
    find_item(pool, city, &task.id).await
}
