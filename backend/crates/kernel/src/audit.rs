use serde_json::Value;
use sqlx::{Postgres, Transaction};

use crate::city::City;
use crate::error::Result;
use crate::ids::{new_id, UserId};

#[derive(Debug, Clone)]
pub struct AuditActor {
    pub user_id: UserId,
    pub city: City,
    pub permissions: Vec<String>,
    pub request_id: String,
}

pub struct AuditEntry<'a> {
    pub action: &'a str,
    pub resource_type: &'a str,
    pub resource_id: &'a str,
    pub reason: Option<&'a str>,
    pub change: Value,
}

pub async fn record(
    tx: &mut Transaction<'_, Postgres>,
    actor: &AuditActor,
    entry: AuditEntry<'_>,
) -> Result<()> {
    let actor_scope = serde_json::json!({
        "city": actor.city,
        "permissions": actor.permissions,
    });
    sqlx::query(
        r#"INSERT INTO kernel_audit_records
           (id, city_id, actor_user_id, actor_scope, action, resource_type, resource_id, reason, correlation_id, change_record)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
    )
    .bind(new_id("audit"))
    .bind(actor.city.code())
    .bind(actor.user_id.as_str())
    .bind(actor_scope)
    .bind(entry.action)
    .bind(entry.resource_type)
    .bind(entry.resource_id)
    .bind(entry.reason)
    .bind(&actor.request_id)
    .bind(entry.change)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
