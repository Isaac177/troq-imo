use kernel::{City, PageRequest, Result};
use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};

use super::types::TaskView;

const TASK_COLUMNS: &str = r#"id, "propertyId" AS property_id, "physicalUnitId" AS physical_unit_id,
    "reservationItemId" AS reservation_item_id, "taskType" AS task_type, "assigneeUserId" AS assignee_user_id,
    state, priority, checklist, version, "createdAt" AS created_at, "updatedAt" AS updated_at,
    "completedAt" AS completed_at"#;

pub async fn property_host(pool: &PgPool, city: City, property_id: &str) -> Result<Option<String>> {
    Ok(sqlx::query_scalar::<_, String>(r#"SELECT "operatorId" FROM stay_properties WHERE id = $1 AND city_id = $2"#)
        .bind(property_id)
        .bind(city.code())
        .fetch_optional(pool)
        .await?)
}

pub async fn unit_in_property(pool: &PgPool, unit_id: &str, property_id: &str) -> Result<bool> {
    let found = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM stay_physical_units WHERE id = $1 AND "propertyId" = $2"#)
        .bind(unit_id)
        .bind(property_id)
        .fetch_one(pool)
        .await?;
    Ok(found == 1)
}

pub async fn item_in_property(pool: &PgPool, item_id: &str, property_id: &str) -> Result<bool> {
    let found = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM stay_reservation_items WHERE id = $1 AND "propertyId" = $2"#)
        .bind(item_id)
        .bind(property_id)
        .fetch_one(pool)
        .await?;
    Ok(found == 1)
}

pub struct NewTask<'a> {
    pub id: &'a str,
    pub city: City,
    pub property_id: &'a str,
    pub physical_unit_id: Option<&'a str>,
    pub reservation_item_id: Option<&'a str>,
    pub task_type: &'a str,
    pub priority: &'a str,
    pub checklist: &'a Value,
}

pub async fn insert_task(tx: &mut Transaction<'_, Postgres>, new: &NewTask<'_>) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_operations_tasks (id, city_id, "propertyId", "physicalUnitId", "reservationItemId",
               "taskType", state, priority, checklist)
           VALUES ($1, $2, $3, $4, $5, $6, 'OPEN', $7, $8)"#,
    )
    .bind(new.id)
    .bind(new.city.code())
    .bind(new.property_id)
    .bind(new.physical_unit_id)
    .bind(new.reservation_item_id)
    .bind(new.task_type)
    .bind(new.priority)
    .bind(new.checklist)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn find_task(pool: &PgPool, city: City, task_id: &str) -> Result<Option<TaskView>> {
    let sql = format!("SELECT {TASK_COLUMNS} FROM stay_operations_tasks WHERE id = $1 AND city_id = $2");
    Ok(sqlx::query_as::<_, TaskView>(&sql).bind(task_id).bind(city.code()).fetch_optional(pool).await?)
}

pub struct TaskFilter<'a> {
    pub city: City,
    pub property_id: &'a str,
    pub allowed_types: &'a [&'a str],
    pub state: Option<&'a str>,
    pub task_type: Option<&'a str>,
}

fn push_filters(builder: &mut QueryBuilder<'_, Postgres>, filter: &TaskFilter<'_>) {
    let types: Vec<String> = filter.allowed_types.iter().map(|value| value.to_string()).collect();
    builder
        .push(" AND city_id = ")
        .push_bind(filter.city.code())
        .push(r#" AND "propertyId" = "#)
        .push_bind(filter.property_id.to_string())
        .push(r#" AND "taskType" = ANY("#)
        .push_bind(types)
        .push(")");
    if let Some(state) = filter.state {
        builder.push(" AND state = ").push_bind(state.to_string());
    }
    if let Some(task_type) = filter.task_type {
        builder.push(r#" AND "taskType" = "#).push_bind(task_type.to_string());
    }
}

pub async fn list_tasks(pool: &PgPool, filter: &TaskFilter<'_>, page: PageRequest) -> Result<(Vec<TaskView>, i64)> {
    let mut count = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM stay_operations_tasks WHERE TRUE");
    push_filters(&mut count, filter);
    let total: i64 = count.build_query_scalar().fetch_one(pool).await?;
    let mut items = QueryBuilder::<Postgres>::new(format!("SELECT {TASK_COLUMNS} FROM stay_operations_tasks WHERE TRUE"));
    push_filters(&mut items, filter);
    items
        .push(r#" ORDER BY CASE priority WHEN 'URGENT' THEN 0 WHEN 'HIGH' THEN 1 WHEN 'NORMAL' THEN 2 ELSE 3 END, "createdAt", id LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let rows = items.build_query_as::<TaskView>().fetch_all(pool).await?;
    Ok((rows, total))
}

pub async fn update_if_version(
    tx: &mut Transaction<'_, Postgres>,
    task_id: &str,
    expected_version: i32,
    state: &str,
    assignee: Option<&str>,
    completed: bool,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_operations_tasks
           SET state = $3, "assigneeUserId" = $4, version = version + 1, "updatedAt" = now(),
               "completedAt" = CASE WHEN $5 THEN now() ELSE NULL END
           WHERE id = $1 AND version = $2"#,
    )
    .bind(task_id)
    .bind(expected_version)
    .bind(state)
    .bind(assignee)
    .bind(completed)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}
