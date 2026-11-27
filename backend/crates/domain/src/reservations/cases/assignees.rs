use kernel::auth::Actor;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Kernel, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase", default)]
#[into_params(parameter_in = Query)]
pub struct AssigneeQuery {
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CaseAssignee {
    pub user_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

const ELIGIBLE: &str = "u.status = 'ACTIVE' AND NOT EXISTS (SELECT 1 FROM identity_member_overrides o WHERE o.user_id = u.id AND o.permission = $1 AND o.scope_type = 'PLATFORM' AND o.effect = 'DENY') AND (EXISTS (SELECT 1 FROM identity_grants g WHERE g.user_id = u.id AND g.permission = $1 AND g.scope_type = 'PLATFORM' AND (g.expires_at IS NULL OR g.expires_at > now())) OR EXISTS (SELECT 1 FROM identity_member_overrides o WHERE o.user_id = u.id AND o.permission = $1 AND o.scope_type = 'PLATFORM' AND o.effect = 'ALLOW'))";

pub async fn eligible(pool: &PgPool, user_id: &str) -> Result<bool> {
    let sql = format!("SELECT EXISTS(SELECT 1 FROM identity_users u WHERE {ELIGIBLE} AND u.id = $2)");
    Ok(sqlx::query_scalar(&sql).bind(PROPERTIES_MODERATE).bind(user_id).fetch_one(pool).await?)
}

pub async fn list(kernel: &Kernel, actor: &Actor, query: AssigneeQuery) -> Result<Page<CaseAssignee>> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let page = PageRequest { page: query.page.unwrap_or(1), page_size: query.page_size.unwrap_or(20) }.validated()?;
    let search = query.search.as_deref().map(str::trim).filter(|value| !value.is_empty()).map(|value| format!("%{}%", value.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")));
    let filter = "($2::text IS NULL OR concat_ws(' ', u.first_name, u.last_name) ILIKE $2 ESCAPE '\\' OR u.email ILIKE $2 ESCAPE '\\')";
    let sql = format!("SELECT u.id AS user_id, NULLIF(trim(concat_ws(' ', u.first_name, u.last_name)), '') AS name, u.email FROM identity_users u WHERE {ELIGIBLE} AND {filter} ORDER BY lower(COALESCE(NULLIF(trim(concat_ws(' ', u.first_name, u.last_name)), ''), u.email, '')), u.id LIMIT $3 OFFSET $4");
    let items = sqlx::query_as::<_, CaseAssignee>(&sql).bind(PROPERTIES_MODERATE).bind(&search).bind(page.limit()).bind(page.offset()).fetch_all(kernel.db.pool()).await?;
    let count_sql = format!("SELECT COUNT(*) FROM identity_users u WHERE {ELIGIBLE} AND {filter}");
    let total = sqlx::query_scalar::<_, i64>(&count_sql).bind(PROPERTIES_MODERATE).bind(&search).fetch_one(kernel.db.pool()).await?;
    Ok(Page::new(items, total, page))
}
