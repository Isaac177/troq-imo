use kernel::error::Result;
use sqlx::PgPool;

use super::check::{active_rules, with_turnover, StayRequest};

pub async fn enforce_amendment(
    pool: &PgPool,
    property_id: &str,
    request: &StayRequest,
    reservation_id: &str,
) -> Result<()> {
    let rules = active_rules(pool, property_id, &request.unit_type_id).await?;
    match with_turnover(pool, &rules, request, Some(reservation_id))
        .await?
        .into_iter()
        .next()
    {
        Some(violation) => Err(violation.into_error()),
        None => Ok(()),
    }
}
