use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::ids::new_id;
use kernel::permissions::names::{OPERATIONS_CHECKIN, OPERATIONS_HOUSEKEEPING, OPERATIONS_MAINTENANCE};
use kernel::{Error, Kernel, Page, Result};
use serde_json::json;
use sqlx::PgPool;

use super::queries::{self, NewTask, TaskFilter};
use super::rules::{
    assignee_after, checklist, ensure_state, ensure_task_type_allowed, granted, next_state, task_types_for, validate_new,
    Action, TASK_TYPES,
};
use super::types::{CreateTaskInput, TaskListQuery, TaskView, TransitionInput};
use crate::reservations::codes::{
    ITEM_NOT_IN_PROPERTY, PROPERTY_NOT_FOUND, TASK_NOT_FOUND, TASK_PROPERTY_REQUIRED, TASK_TYPE_FORBIDDEN, TASK_TYPE_UNKNOWN,
    TASK_VERSION_CONFLICT, UNIT_NOT_IN_PROPERTY,
};
use crate::reservations::events::{self, types::{TASK_CREATED, TASK_UPDATED}};
use crate::reservations::support::paging;

const OPERATIONS_PERMISSIONS: [&str; 3] = [OPERATIONS_CHECKIN, OPERATIONS_HOUSEKEEPING, OPERATIONS_MAINTENANCE];

async fn require_property_access(pool: &PgPool, actor: &Actor, property_id: &str) -> Result<()> {
    let host = queries::property_host(pool, actor.city, property_id)
        .await?
        .filter(|id| actor.access.is_member_of(id))
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.require_scoped_any(&OPERATIONS_PERMISSIONS, Some(host.as_str()), Some(property_id))
}

pub async fn create_task(kernel: &Kernel, actor: &Actor, input: CreateTaskInput) -> Result<TaskView> {
    let pool = kernel.db.pool();
    let city = actor.city;
    let permissions = granted(actor);
    let priority = validate_new(&input.task_type, input.priority.as_deref())?;
    ensure_task_type_allowed(&input.task_type, &permissions)?;
    let checklist = checklist(input.checklist)?;
    let host = queries::property_host(pool, city, &input.property_id).await?;
    if !host.is_some_and(|id| actor.access.is_member_of(&id)) {
        return Err(Error::not_found(PROPERTY_NOT_FOUND));
    }
    if let Some(unit) = input.physical_unit_id.as_deref() {
        if !queries::unit_in_property(pool, unit, &input.property_id).await? {
            return Err(Error::bad_request(UNIT_NOT_IN_PROPERTY));
        }
    }
    if let Some(item) = input.reservation_item_id.as_deref() {
        if !queries::item_in_property(pool, item, &input.property_id).await? {
            return Err(Error::bad_request(ITEM_NOT_IN_PROPERTY));
        }
    }
    let task_id = new_id("task");
    let mut tx = pool.begin().await?;
    queries::insert_task(
        &mut tx,
        &NewTask {
            id: &task_id,
            city,
            property_id: &input.property_id,
            physical_unit_id: input.physical_unit_id.as_deref(),
            reservation_item_id: input.reservation_item_id.as_deref(),
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
            change: json!({ "taskType": input.task_type, "propertyId": input.property_id }),
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
    queries::find_task(pool, city, &task_id).await?.ok_or_else(|| Error::internal("created task not readable"))
}

pub async fn list_tasks(kernel: &Kernel, actor: &Actor, query: TaskListQuery) -> Result<Page<TaskView>> {
    let permissions = granted(actor);
    let allowed = task_types_for(&permissions);
    if allowed.is_empty() {
        return Err(Error::forbidden(TASK_TYPE_FORBIDDEN));
    }
    if query.property_id.trim().is_empty() {
        return Err(Error::bad_request(TASK_PROPERTY_REQUIRED));
    }
    ensure_state(query.state.as_deref())?;
    if let Some(task_type) = query.task_type.as_deref() {
        if !TASK_TYPES.contains(&task_type) {
            return Err(Error::bad_request(TASK_TYPE_UNKNOWN).with_params(json!({ "value": task_type })));
        }
        ensure_task_type_allowed(task_type, &permissions)?;
    }
    let pool = kernel.db.pool();
    require_property_access(pool, actor, &query.property_id).await?;
    let page = paging(query.page, query.page_size)?;
    let filter = TaskFilter {
        city: actor.city,
        property_id: &query.property_id,
        allowed_types: &allowed,
        state: query.state.as_deref(),
        task_type: query.task_type.as_deref(),
    };
    let (items, total) = queries::list_tasks(pool, &filter, page).await?;
    Ok(Page::new(items, total, page))
}

async fn visible_task(pool: &PgPool, actor: &Actor, task_id: &str) -> Result<TaskView> {
    let task = queries::find_task(pool, actor.city, task_id).await?.ok_or_else(|| Error::not_found(TASK_NOT_FOUND))?;
    let permissions = granted(actor);
    if !task_types_for(&permissions).contains(&task.task_type.as_str()) {
        return Err(Error::not_found(TASK_NOT_FOUND));
    }
    require_property_access(pool, actor, &task.property_id)
        .await
        .map_err(|_| Error::not_found(TASK_NOT_FOUND))?;
    Ok(task)
}

pub async fn get_task(kernel: &Kernel, actor: &Actor, task_id: &str) -> Result<TaskView> {
    visible_task(kernel.db.pool(), actor, task_id).await
}

pub async fn transition_task(kernel: &Kernel, actor: &Actor, task_id: &str, input: TransitionInput) -> Result<TaskView> {
    let pool = kernel.db.pool();
    let task = visible_task(pool, actor, task_id).await?;
    let action = Action::parse(&input.action)?;
    let state = next_state(&task.state, action)?;
    let assignee = assignee_after(action, actor.user_id().as_str(), task.assignee_user_id.as_deref());
    apply_transition(pool, actor, &task, &input, state, assignee.as_deref()).await
}

async fn apply_transition(
    pool: &PgPool,
    actor: &Actor,
    task: &TaskView,
    input: &TransitionInput,
    state: &str,
    assignee: Option<&str>,
) -> Result<TaskView> {
    let completed = state == "DONE" || state == "CANCELLED";
    let mut tx = pool.begin().await?;
    if !queries::update_if_version(&mut tx, &task.id, input.expected_version, state, assignee, completed).await? {
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
            change: json!({ "action": input.action, "from": task.state, "to": state }),
        },
    )
    .await?;
    events::emit(&mut tx, actor.city, "operations_task", &task.id, TASK_UPDATED, json!({ "taskId": task.id, "state": state })).await?;
    tx.commit().await?;
    queries::find_task(pool, actor.city, &task.id).await?.ok_or_else(|| Error::internal("updated task not readable"))
}
