use chrono::NaiveDate;
use kernel::City;
use kernel::Result;
use sqlx::PgPool;

use crate::reservations::links::pricing::{open_nights, room_available};

pub async fn dates_available(
    pool: &PgPool,
    _city: City,
    unit_type_id: &str,
    reservation_id: &str,
    old_nights: &[NaiveDate],
    new_nights: &[NaiveDate],
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Result<bool> {
    let open = open_nights(pool, unit_type_id, check_in, check_out).await?;
    if new_nights
        .iter()
        .filter(|night| !old_nights.contains(night))
        .any(|night| !open.contains(night))
    {
        return Ok(false);
    }
    room_available(pool, unit_type_id, check_in, check_out, reservation_id).await
}
