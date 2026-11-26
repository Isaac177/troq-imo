use kernel::error::codes::NOT_FOUND;
use kernel::{City, Error, Result};
use sqlx::PgPool;

use super::check::{enforce, StayRequest};

pub async fn enforce_for_city(
    pool: &PgPool,
    city: City,
    property_id: &str,
    request: &StayRequest,
) -> Result<()> {
    let found: bool = sqlx::query_scalar(
        r#"SELECT EXISTS (
            SELECT 1 FROM stay_unit_types u
            JOIN stay_properties p ON p.id = u."propertyId" AND p.city_id = u.city_id
            WHERE p.id = $1 AND u.id = $2 AND u.city_id = $3
        )"#,
    )
    .bind(property_id)
    .bind(&request.unit_type_id)
    .bind(city.code())
    .fetch_one(pool)
    .await?;
    if !found {
        return Err(Error::not_found(NOT_FOUND));
    }
    enforce(pool, property_id, request).await
}
