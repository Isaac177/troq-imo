use std::collections::HashMap;

use kernel::auth::Actor;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::REVIEWS_WRITE;
use kernel::{Error, Kernel, Result};
use serde::Serialize;
use utoipa::ToSchema;

use super::codes::REVIEW_NOT_FOUND;
use super::property_reviews::list_property_reviews;
use super::queries;
use super::types::ReviewView;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HostReviewView {
    #[serde(flatten)]
    pub review: ReviewView,
    pub guest_name: Option<String>,
}

fn authorize(actor: &Actor, host_id: &str, property_id: &str) -> Result<()> {
    actor.ensure_host_scope(REVIEWS_WRITE, host_id, property_id)
}

pub async fn list(kernel: &Kernel, actor: &Actor, property_id: &str, status: Option<&str>, page: PageRequest) -> Result<Page<HostReviewView>> {
    let pool = kernel.db.pool();
    let host_id = queries::property_host(pool, actor.city, property_id).await?
        .ok_or_else(|| Error::not_found(REVIEW_NOT_FOUND))?;
    authorize(actor, &host_id, property_id)?;
    let page = list_property_reviews(kernel, actor.city, Some(actor), property_id, status, page).await?;
    let ids: Vec<String> = page.items.iter().map(|review| review.id.clone()).collect();
    let names: Vec<(String, Option<String>)> = sqlx::query_as(
        r#"SELECT v.id, (SELECT NULLIF(trim(g."fullName"), '') FROM stay_reservation_items i
           JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
           WHERE i."reservationId" = v."reservationId" AND g."anonymizedAt" IS NULL
           ORDER BY i.id, g.id LIMIT 1) AS guest_name
           FROM stay_reviews v WHERE v.id = ANY($1) AND v.city_id = $2 AND v."propertyId" = $3"#,
    ).bind(&ids).bind(actor.city.code()).bind(property_id).fetch_all(pool).await?;
    let mut names: HashMap<String, Option<String>> = names.into_iter().collect();
    let items = page.items.into_iter().map(|review| {
        let guest_name = names.remove(&review.id).flatten();
        HostReviewView { review, guest_name }
    }).collect();
    Ok(Page::new(items, page.total, PageRequest { page: page.page, page_size: page.page_size }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::auth::{CityAccess, CurrentUser, Role};
    use kernel::ids::{HostId, UserId};
    use kernel::permissions::{Access, Grant};

    #[test]
    fn requires_host_membership_and_property_permission() {
        let actor = Actor {
            user: CurrentUser { user_id: UserId::new("seed-reviewer"), role: Role::Host, issued_at: 0, auth_city: Some("GOMA".into()), city_access_mode: None },
            city: kernel::City::GOMA,
            request_id: "test".into(),
            city_access: CityAccess::Full,
            access: Access { grants: vec![Grant::property(REVIEWS_WRITE, "property-a")], host_ids: vec![HostId::new("host-a")], auth_city: Some("GOMA".into()) },
        };
        assert!(authorize(&actor, "host-a", "property-a").is_ok());
        assert!(authorize(&actor, "host-a", "property-b").is_err());
        assert!(authorize(&actor, "host-b", "property-a").is_err());
    }
}
