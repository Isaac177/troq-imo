use kernel::error::Result;
use kernel::Money;
use sqlx::PgConnection;

use super::facets::{DistanceStats, FacetValue, Facets, PriceStats, Stats};

async fn facet(connection: &mut PgConnection, expression: &str, array: bool) -> Result<Vec<FacetValue>> {
    let values = if array {
        format!("CROSS JOIN LATERAL jsonb_array_elements_text({expression}) AS value")
    } else {
        format!("CROSS JOIN LATERAL (SELECT {expression} AS value) v")
    };
    let query = format!(
        "SELECT value, count(DISTINCT property_id)::bigint FROM stay_search_offers {values} WHERE value IS NOT NULL GROUP BY value ORDER BY value"
    );
    let rows: Vec<(String, i64)> = sqlx::query_as(&query).fetch_all(connection).await?;
    Ok(rows.into_iter().map(|(value, count)| FacetValue { value, count }).collect())
}

pub async fn facets(connection: &mut PgConnection) -> Result<Facets> {
    Ok(Facets {
        category: facet(&mut *connection, "hit->'best'->>'categoryCode'", false).await?,
        property_type: facet(&mut *connection, "hit->'best'->>'propertyType'", false).await?,
        meal_plan: facet(&mut *connection, "hit->'best'->>'mealPlan'", false).await?,
        payment_collection: facet(&mut *connection, "hit->'best'->>'paymentCollection'", false).await?,
        cancellation_class: facet(&mut *connection, "hit->'best'->'cancellation'->>'class'", false).await?,
        booking_mode: facet(&mut *connection, "hit->'best'->>'bookingMode'", false).await?,
        accessibility: facet(&mut *connection, "hit->'best'->'accessibility'", true).await?,
        amenities: facet(&mut *connection, "hit->'amenities'", true).await?,
        verification_status: facet(&mut *connection, "hit->>'verificationStatus'", false).await?,
    })
}

type StatsRow = (i64, i64, i64, i64, Option<i64>, Option<i64>, Option<i64>, Option<i64>, Option<f64>, Option<f64>);

pub async fn stats(connection: &mut PgConnection) -> Result<Stats> {
    let row: StatsRow = sqlx::query_as(
        r#"WITH ranked AS (
             SELECT best_minor, row_number() OVER (ORDER BY best_minor) AS position,
                    count(*) OVER () AS priced_total
             FROM stay_search_properties WHERE best_minor IS NOT NULL
           ), median AS (
             SELECT trunc(avg(best_minor))::bigint AS amount FROM ranked
             WHERE position IN ((priced_total + 1) / 2, (priced_total + 2) / 2)
           )
           SELECT count(*)::bigint, count(best_minor)::bigint,
           count(*) FILTER (WHERE instant_booking)::bigint, COALESCE(sum(offer_count), 0)::bigint,
           min(best_minor), (SELECT amount FROM median),
           max(best_minor), trunc(avg(best_minor))::bigint, min(distance_km), max(distance_km)
           FROM stay_search_properties"#,
    )
    .fetch_one(connection)
    .await?;
    let wire = |minor: i64| Money::from_minor(minor).to_wire();
    let price = match (row.4, row.5, row.6, row.7) {
        (Some(min), Some(median), Some(max), Some(average)) => Some(PriceStats {
            currency: kernel::money::CURRENCY.to_string(),
            min: wire(min),
            median: wire(median),
            max: wire(max),
            average: wire(average),
        }),
        _ => None,
    };
    let distance = match (row.8, row.9) {
        (Some(min_km), Some(max_km)) => Some(DistanceStats { min_km, max_km }),
        _ => None,
    };
    Ok(Stats {
        result_count: row.0,
        priced_count: row.1,
        instant_booking_count: row.2,
        offer_count: row.3,
        price,
        distance,
    })
}
