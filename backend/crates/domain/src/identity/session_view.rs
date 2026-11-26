use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::permissions::Scope;
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SessionUser {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub role: String,
    pub city: String,
    pub preferred_language: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PermissionGrantView {
    pub permission: String,
    pub scope_type: String,
    pub scope_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SessionHost {
    pub host_id: String,
    pub display_name: String,
    pub is_owner: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SessionView {
    pub user: SessionUser,
    pub permissions: Vec<PermissionGrantView>,
    pub hosts: Vec<SessionHost>,
}

type AccountRow = (String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, String);

fn scope_view(scope: &Scope) -> (String, Option<String>) {
    match scope {
        Scope::Platform => ("PLATFORM".into(), None),
        Scope::Host(id) => ("HOST".into(), Some(id.as_str().to_string())),
        Scope::Property(id) => ("PROPERTY".into(), Some(id.as_str().to_string())),
    }
}

pub async fn session_view(pool: &PgPool, actor: &Actor) -> Result<SessionView> {
    let row = sqlx::query_as::<_, AccountRow>(
        "SELECT id, first_name, last_name, email, phone, role, preferred_language FROM identity_users WHERE id = $1 AND status = 'ACTIVE'",
    )
    .bind(actor.user_id().as_str())
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| Error::unauthorized(codes::AUTH_REQUIRED))?;
    let (id, first_name, last_name, email, phone, role, preferred_language) = row;
    let hosts = sqlx::query_as::<_, (String, String, bool)>(
        "SELECT h.id, h.display_name, m.is_owner FROM host_members m JOIN hosts h ON h.id = m.host_id WHERE m.user_id = $1 AND m.status = 'ACTIVE' AND h.status = 'ACTIVE' ORDER BY h.display_name, h.id",
    )
    .bind(&id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(host_id, display_name, is_owner)| SessionHost { host_id, display_name, is_owner })
    .collect();
    let mut permissions: Vec<PermissionGrantView> = actor
        .access
        .grants
        .iter()
        .map(|grant| {
            let (scope_type, scope_id) = scope_view(&grant.scope);
            PermissionGrantView { permission: grant.permission.clone(), scope_type, scope_id }
        })
        .collect();
    permissions.sort_by(|left, right| {
        left.permission
            .cmp(&right.permission)
            .then_with(|| left.scope_id.cmp(&right.scope_id))
    });
    permissions.dedup_by(|left, right| left.permission == right.permission && left.scope_id == right.scope_id);
    Ok(SessionView {
        user: SessionUser {
            id,
            first_name,
            last_name,
            email,
            phone,
            role: role.unwrap_or_else(|| "guest".into()),
            city: actor.city.code().to_string(),
            preferred_language,
        },
        permissions,
        hosts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::ids::{HostId, PropertyId};

    #[test]
    fn scopes_render_for_the_client() {
        assert_eq!(scope_view(&Scope::Platform), ("PLATFORM".into(), None));
        assert_eq!(
            scope_view(&Scope::Host(HostId::new("h1"))),
            ("HOST".into(), Some("h1".into()))
        );
        assert_eq!(
            scope_view(&Scope::Property(PropertyId::new("p1"))),
            ("PROPERTY".into(), Some("p1".into()))
        );
    }
}
