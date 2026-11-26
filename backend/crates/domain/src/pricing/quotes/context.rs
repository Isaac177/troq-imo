use kernel::{City, Error, Result};
use sqlx::{FromRow, PgPool};

use crate::pricing::messages::*;

#[derive(Debug, FromRow)]
pub struct QuoteContext {
    pub host_id: String,
    pub timezone: String,
    pub property_status: String,
    pub host_status: String,
    pub unit_type_id: String,
    pub unit_status: String,
    pub max_adults: i32,
    pub max_children: i32,
    pub rate_plan_id: String,
    pub rate_unit_type_id: String,
    pub rate_status: String,
    pub occupancy_basis: String,
    pub booking_mode: String,
    pub cancellation_policy_id: String,
    pub payment_policy_id: String,
    pub child_nightly_amount: String,
}

#[derive(Debug, FromRow)]
pub struct PolicyRow {
    pub id: String,
    pub kind: String,
    pub content: serde_json::Value,
}

pub async fn load_context(
    pool: &PgPool,
    city: City,
    property_id: &str,
    rate_plan_id: &str,
) -> Result<Option<QuoteContext>> {
    let context = sqlx::query_as::<_, QuoteContext>(
        r#"SELECT o.id AS host_id, p.timezone, p.status AS property_status, o.status AS host_status,
                  u.id AS unit_type_id, u.status AS unit_status, u."maxAdults" AS max_adults,
                  u."maxChildren" AS max_children,
                  r.id AS rate_plan_id, r."unitTypeId" AS rate_unit_type_id, r.status AS rate_status,
                  r."occupancyBasis" AS occupancy_basis, r."bookingMode" AS booking_mode,
                  r."cancellationPolicyVersionId" AS cancellation_policy_id,
                  r."paymentPolicyVersionId" AS payment_policy_id,
                  r."childNightlyAmount"::text AS child_nightly_amount
           FROM stay_rate_plans r
           JOIN stay_properties p ON p.id = r."propertyId"
           JOIN hosts o ON o.id = p."operatorId"
           JOIN stay_unit_types u ON u.id = r."unitTypeId"
           WHERE r.id = $1 AND p.id = $2 AND p.city_id = $3"#,
    )
    .bind(rate_plan_id)
    .bind(property_id)
    .bind(city.code())
    .fetch_optional(pool)
    .await?;
    Ok(context)
}

pub async fn load_policies(pool: &PgPool, ids: &[String]) -> Result<Vec<PolicyRow>> {
    let rows = sqlx::query_as::<_, PolicyRow>(
        r#"SELECT id, kind, content FROM stay_policy_versions
           WHERE id = ANY($1) AND "tombstonedAt" IS NULL"#,
    )
    .bind(ids.to_vec())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub fn ensure_bookable(
    context: &QuoteContext,
    unit_type_id: &str,
    adults: i32,
    children: i32,
) -> Result<()> {
    if context.property_status != "PUBLISHED" {
        return Err(Error::conflict(PROPERTY_NOT_PUBLISHED));
    }
    if context.host_status != "ACTIVE" {
        return Err(Error::conflict(HOST_NOT_ACTIVE));
    }
    if context.unit_status != "ACTIVE" {
        return Err(Error::conflict(UNIT_TYPE_NOT_ACTIVE));
    }
    if context.rate_status != "ACTIVE" {
        return Err(Error::conflict(RATE_PLAN_NOT_ACTIVE));
    }
    if context.unit_type_id != unit_type_id || context.rate_unit_type_id != unit_type_id {
        return Err(Error::bad_request(RATE_PLAN_UNIT_TYPE_MISMATCH));
    }
    if adults > context.max_adults || children > context.max_children {
        return Err(Error::bad_request(GUEST_CAPACITY_EXCEEDED));
    }
    Ok(())
}

pub fn guest_factor(occupancy_basis: &str, adults: i32) -> i64 {
    if occupancy_basis == "PER_GUEST" {
        i64::from(adults)
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> QuoteContext {
        QuoteContext {
            host_id: "h".into(),
            timezone: "Africa/Kinshasa".into(),
            property_status: "PUBLISHED".into(),
            host_status: "ACTIVE".into(),
            unit_type_id: "u".into(),
            unit_status: "ACTIVE".into(),
            max_adults: 2,
            max_children: 1,
            rate_plan_id: "r".into(),
            rate_unit_type_id: "u".into(),
            rate_status: "ACTIVE".into(),
            occupancy_basis: "PER_UNIT".into(),
            booking_mode: "INSTANT".into(),
            cancellation_policy_id: "c".into(),
            payment_policy_id: "p".into(),
            child_nightly_amount: "0.00".into(),
        }
    }

    #[test]
    fn active_context_within_capacity_is_bookable() {
        assert!(ensure_bookable(&context(), "u", 2, 1).is_ok());
    }

    #[test]
    fn capacity_and_status_failures_carry_their_code() {
        let error = ensure_bookable(&context(), "u", 3, 0).unwrap_err();
        assert_eq!(error.code().as_str(), "GUEST_CAPACITY_EXCEEDED");
        let mut draft = context();
        draft.rate_status = "DRAFT".into();
        assert_eq!(ensure_bookable(&draft, "u", 1, 0).unwrap_err().code().as_str(), "RATE_PLAN_NOT_ACTIVE");
        assert_eq!(ensure_bookable(&context(), "other", 1, 0).unwrap_err().code().as_str(), "RATE_PLAN_UNIT_TYPE_MISMATCH");
    }

    #[test]
    fn per_guest_basis_multiplies_by_adults() {
        assert_eq!(guest_factor("PER_GUEST", 3), 3);
        assert_eq!(guest_factor("PER_UNIT", 3), 1);
    }
}
