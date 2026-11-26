use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use sqlx::PgPool;

use super::query::{SearchQuery, ENGINE_VERSION};

pub struct EventInput<'a> {
    pub city: City,
    pub fingerprint: &'a str,
    pub query: &'a SearchQuery,
    pub result_count: i64,
    pub candidate_plans: i32,
    pub priced_plans: i32,
    pub duration_ms: i32,
}

pub fn anonymised(query: &SearchQuery) -> SearchQuery {
    let mut stored = query.clone();
    stored.child_ages.clear();
    stored.search = None;
    stored
}

pub async fn record(pool: &PgPool, event: EventInput<'_>) -> Result<()> {
    let query = serde_json::to_value(anonymised(event.query)).map_err(|error| Error::internal(error.to_string()))?;
    sqlx::query(
        r#"INSERT INTO stay_search_events (id, city_id, fingerprint, engine_version, query, result_count,
               candidate_plans, priced_plans, duration_ms)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
    )
    .bind(new_id("sev"))
    .bind(event.city.code())
    .bind(event.fingerprint)
    .bind(ENGINE_VERSION as i32)
    .bind(query)
    .bind(event.result_count as i32)
    .bind(event.candidate_plans)
    .bind(event.priced_plans)
    .bind(event.duration_ms)
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guests::search::raw::RawSearch;

    #[test]
    fn stored_events_carry_no_child_ages_and_no_search_text() {
        let query = SearchQuery::from_raw(RawSearch {
            children: Some(1),
            child_ages: vec![5],
            search: Some("Lake".into()),
            ..RawSearch::default()
        })
        .unwrap();
        let stored = anonymised(&query);
        assert!(stored.child_ages.is_empty());
        assert!(stored.search.is_none());
        assert_eq!(stored.children, 1);
        assert_eq!(query.child_ages, vec![5]);
    }
}
