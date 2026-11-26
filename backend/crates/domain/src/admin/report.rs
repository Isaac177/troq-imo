use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::permissions::names::ANALYTICS_READ;
use kernel::Kernel;

use super::{report_queries, report_validation};
use super::report_types::{AnalyticsQuery, AnalyticsSummary};

fn authorize(actor: &Actor, host: &str, property: Option<&str>) -> Result<()> {
    if !actor.access.is_member_of(host) {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    actor.require_scoped(ANALYTICS_READ, Some(host), property)
}

pub async fn summary(kernel: &Kernel, actor: &Actor, query: AnalyticsQuery) -> Result<AnalyticsSummary> {
    report_validation::validate(&query)?;
    let host = query.host_id.trim();
    let property = query.property_id.as_deref().map(str::trim);
    authorize(actor, host, property)?;
    let found: bool = sqlx::query_scalar(r#"
        SELECT EXISTS (SELECT 1 FROM hosts h WHERE h.id = $1 AND h.city_id = $2 AND h.status = 'ACTIVE'
          AND ($3::text IS NULL OR EXISTS (SELECT 1 FROM stay_properties p
            WHERE p.id = $3 AND p."operatorId" = h.id AND p.city_id = h.city_id)))"#)
        .bind(host).bind(actor.city.code()).bind(property).fetch_one(kernel.db.pool()).await?;
    if !found {
        return Err(Error::not_found(codes::NOT_FOUND));
    }
    let reservations = report_queries::reservations(kernel.db.pool(), actor.city, &query).await?;
    let reviews = report_queries::reviews(kernel.db.pool(), actor.city, &query).await?;
    Ok(AnalyticsSummary {
        currency: "CDF", reservations: reservations.reservations, confirmed: reservations.confirmed,
        checked_in: reservations.checked_in, checked_out: reservations.checked_out,
        cancelled: reservations.cancelled, no_show: reservations.no_show,
        gross_revenue: reservations.gross_revenue, approved_reviews: reviews.approved_reviews,
        average_rating: reviews.average_rating,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::auth::{CityAccess, CurrentUser, Role};
    use kernel::ids::{HostId, UserId};
    use kernel::permissions::{Access, Grant};

    fn actor(grants: Vec<Grant>, hosts: &[&str]) -> Actor {
        Actor {
            user: CurrentUser { user_id: UserId::new("seed-user"), role: Role::Host, issued_at: 0, auth_city: Some("GOMA".into()), city_access_mode: None },
            city: kernel::City::GOMA, request_id: "test".into(), city_access: CityAccess::Full,
            access: Access { grants, host_ids: hosts.iter().map(|host| HostId::new(*host)).collect(), auth_city: Some("GOMA".into()) },
        }
    }

    #[test]
    fn host_and_property_grants_do_not_expand_report_scope() {
        let member = actor(vec![Grant::property(ANALYTICS_READ, "property-a")], &["host-a"]);
        assert!(authorize(&member, "host-a", Some("property-a")).is_ok());
        assert!(authorize(&member, "host-a", Some("property-b")).is_err());
        assert!(authorize(&member, "host-a", None).is_err());
        assert!(authorize(&member, "host-b", Some("property-a")).is_err());
    }
}
