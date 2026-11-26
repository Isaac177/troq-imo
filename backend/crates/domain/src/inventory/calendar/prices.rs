use chrono::NaiveDate;
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::money::Money;
use sqlx::PgPool;

use crate::pricing::rates::{load_rules, nightly_minor};

pub async fn nightly_prices(
    pool: &PgPool,
    city: City,
    unit_type_id: &str,
    days: &[NaiveDate],
) -> Result<Vec<Option<String>>> {
    let plans = sqlx::query_scalar::<_, String>(
        r#"SELECT id FROM stay_rate_plans
           WHERE city_id = $1 AND "unitTypeId" = $2 AND status = 'ACTIVE' ORDER BY id"#,
    )
    .bind(city.code())
    .bind(unit_type_id)
    .fetch_all(pool)
    .await?;
    let mut prices: Vec<Option<i64>> = vec![None; days.len()];
    for plan in plans {
        let rules = load_rules(pool, city, &plan).await?;
        for (index, day) in days.iter().enumerate() {
            match nightly_minor(*day, &rules) {
                Ok(minor) => {
                    prices[index] = Some(prices[index].map_or(minor, |current| current.min(minor)))
                }
                Err(Error::Conflict(_)) => (),
                Err(error) => return Err(error),
            }
        }
    }
    Ok(prices
        .into_iter()
        .map(|price| price.map(|minor| Money::from_minor(minor).to_wire()))
        .collect())
}
