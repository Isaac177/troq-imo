use kernel::audit::{self, AuditEntry};
use kernel::auth::Actor;
use kernel::error::{Error, Result};
use kernel::outbox;
use serde_json::json;
use sqlx::{PgConnection, PgPool};

use super::events::payload;
use super::guards::{ensure_not_archived, ensure_status, ensure_version};
use super::types::LifecycleInput;
use super::{queries, readiness};
use crate::properties::messages::{LISTING_ACTION_UNKNOWN, LISTING_UPCOMING_STAYS, PROPERTY_NOT_FOUND};
use crate::properties::property::queries as property_queries;
use crate::properties::property::types::PropertyView;
use crate::properties::revisions::content::ListingContent;

fn publishable_states(platform: bool) -> &'static [&'static str] {
    if platform {
        &["DRAFT", "APPROVED", "PAUSED"]
    } else {
        &["APPROVED", "PAUSED"]
    }
}

struct Step {
    status: &'static str,
    event: &'static str,
    audit: &'static str,
}

pub async fn apply(
    pool: &PgPool,
    actor: &Actor,
    property_id: &str,
    input: LifecycleInput,
    platform: bool,
) -> Result<PropertyView> {
    actor.ensure_writable()?;
    let mut tx = pool.begin().await?;
    let property = property_queries::lock(&mut tx, property_id)
        .await?
        .ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))?;
    actor.ensure_city(kernel::City::from_code(&property.city)?)?;
    ensure_version(property.version, input.expected_version)?;
    let step = transition(&mut tx, &property, &input.action, platform).await?;
    queries::set_property_status(&mut tx, &property.id, step.status).await?;
    let data = payload(&mut tx, &property.id, None, json!({ "status": step.status })).await?;
    outbox::enqueue(&mut tx, actor.city, "property", &property.id, step.event, data).await?;
    audit::record(&mut tx, &actor.audit(), AuditEntry {
        action: step.audit,
        resource_type: "property",
        resource_id: &property.id,
        reason: None,
        change: json!({ "from": property.status, "to": step.status }),
    })
    .await?;
    tx.commit().await?;
    property_queries::find(pool, &property.id).await?.ok_or_else(|| Error::not_found(PROPERTY_NOT_FOUND))
}

async fn transition(conn: &mut PgConnection, property: &PropertyView, action: &str, platform: bool) -> Result<Step> {
    match action {
        "PUBLISH" => {
            ensure_status(&property.status, publishable_states(platform), "publish")?;
            let content = ListingContent::from_property(property)?;
            readiness::ensure_ready(conn, &property.operator_id, &property.id, &content).await?;
            Ok(Step { status: "PUBLISHED", event: "property.published", audit: "listing.publish" })
        }
        "PAUSE" => {
            ensure_status(&property.status, &["PUBLISHED"], "pause")?;
            Ok(Step { status: "PAUSED", event: "property.paused", audit: "listing.pause" })
        }
        "ARCHIVE" => {
            ensure_not_archived(property)?;
            let upcoming = queries::upcoming_stay_count(conn, &property.id, &property.timezone).await?;
            if upcoming > 0 {
                return Err(Error::conflict(LISTING_UPCOMING_STAYS).with_params(json!({ "count": upcoming })));
            }
            Ok(Step { status: "ARCHIVED", event: "property.archived", audit: "listing.archive" })
        }
        _ => Err(Error::bad_request(LISTING_ACTION_UNKNOWN).with_params(json!({ "action": action }))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_platform_staff_may_publish_a_draft() {
        assert!(publishable_states(true).contains(&"DRAFT"));
        assert!(!publishable_states(false).contains(&"DRAFT"));
        assert!(publishable_states(false).contains(&"APPROVED"));
        assert!(publishable_states(false).contains(&"PAUSED"));
    }
}
