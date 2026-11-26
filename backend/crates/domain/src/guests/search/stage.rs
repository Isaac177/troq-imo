use std::collections::BTreeMap;

use kernel::error::{Error, Result};
use serde_json::Value;
use sqlx::PgConnection;

use super::pricing::Offer;
use super::query::SearchQuery;
use super::rank::{self, SearchHit};

pub async fn create(connection: &mut PgConnection) -> Result<()> {
    sqlx::query("CREATE TEMP TABLE stay_search_offers (property_id text NOT NULL, hit jsonb NOT NULL, total_minor bigint) ON COMMIT DROP")
        .execute(connection)
        .await?;
    Ok(())
}

pub async fn insert(connection: &mut PgConnection, offers: Vec<Offer>, query: &SearchQuery) -> Result<usize> {
    let mut ids = Vec::new();
    let mut hits = Vec::new();
    let mut totals = Vec::new();
    for offer in offers {
        let total = offer.total_minor;
        if let Some(hit) = rank::build_hits(vec![offer]).into_iter().next() {
            ids.push(hit.id.clone());
            hits.push(serde_json::to_value(hit).map_err(|error| Error::internal(error.to_string()))?);
            totals.push(total);
        }
    }
    let inserted = sqlx::query(
        r#"INSERT INTO stay_search_offers (property_id, hit, total_minor)
           SELECT property_id, hit, total_minor FROM unnest($1::text[], $2::jsonb[], $3::bigint[]) AS t(property_id, hit, total_minor)
           WHERE ($4::bigint IS NULL OR total_minor >= $4) AND ($5::bigint IS NULL OR total_minor <= $5)"#,
    )
    .bind(&ids)
    .bind(&hits)
    .bind(&totals)
    .bind(query.price_min_minor)
    .bind(query.price_max_minor)
    .execute(connection)
    .await?
    .rows_affected();
    Ok(inserted as usize)
}

pub async fn group(connection: &mut PgConnection) -> Result<()> {
    sqlx::query(
        r#"CREATE TEMP TABLE stay_search_properties ON COMMIT DROP AS
        SELECT property_id, min(hit->>'name') AS name,
               bool_or(hit->>'verificationStatus' = 'VERIFIED') AS verified,
               bool_or((hit->>'instantBooking')::boolean) AS instant_booking,
               min(total_minor) AS best_minor, min((hit->>'distanceKm')::float8) AS distance_km,
               count(*)::bigint AS offer_count
        FROM stay_search_offers GROUP BY property_id"#,
    )
    .execute(connection)
    .await?;
    Ok(())
}

pub async fn page(connection: &mut PgConnection, query: &SearchQuery) -> Result<Vec<SearchHit>> {
    let order = match query.sort.as_str() {
        "price" => "best_minor ASC NULLS LAST, name COLLATE \"C\", property_id",
        "distance" => "distance_km ASC NULLS LAST, name COLLATE \"C\", property_id",
        "name" => "name COLLATE \"C\", property_id",
        _ => "verified DESC, instant_booking DESC, best_minor ASC NULLS LAST, name COLLATE \"C\", property_id",
    };
    let sql = format!(
        r#"SELECT o.property_id, o.hit, o.total_minor FROM stay_search_offers o
        JOIN (SELECT property_id FROM stay_search_properties ORDER BY {order} LIMIT $1 OFFSET $2) p USING (property_id)
        ORDER BY o.property_id, o.total_minor ASC NULLS LAST, o.hit->'best'->>'ratePlanId'"#
    );
    let rows: Vec<(String, Value, Option<i64>)> = sqlx::query_as(&sql)
        .bind(query.page_size)
        .bind((query.page - 1) * query.page_size)
        .fetch_all(&mut *connection)
        .await?;
    let mut groups: BTreeMap<String, SearchHit> = BTreeMap::new();
    for (id, value, total) in rows {
        let mut hit: SearchHit =
            serde_json::from_value(value).map_err(|error| Error::internal(error.to_string()))?;
        hit.best.total_minor = total;
        hit.best_minor = total;
        hit.offers = vec![hit.best.clone()];
        if let Some(existing) = groups.get_mut(&id) {
            existing.instant_booking |= hit.instant_booking;
            existing.offers.push(hit.best);
        } else {
            groups.insert(id, hit);
        }
    }
    let mut hits: Vec<SearchHit> = groups.into_values().collect();
    rank::sort_hits(&mut hits, &query.sort);
    Ok(hits)
}
