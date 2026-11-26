use chrono::{Duration, NaiveDate, Utc};
use kernel::auth::Actor;
use kernel::error::{codes, Error, Result};
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::Kernel;

use super::dashboard_queries;
use super::dashboard_types::{CityFigures, DashboardQuery, DashboardSummary};

fn window(query: &DashboardQuery, today: NaiveDate) -> Result<(NaiveDate, NaiveDate)> {
    let to = query.to.unwrap_or(today);
    let from = query.from.unwrap_or(to - Duration::days(29));
    if to < from || (to - from).num_days() >= 366 {
        return Err(Error::bad_request(codes::MALFORMED_REQUEST));
    }
    Ok((from, to))
}

pub async fn summary(kernel: &Kernel, actor: &Actor, query: DashboardQuery) -> Result<DashboardSummary> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let (from, to) = window(&query, Utc::now().date_naive())?;
    let host = query.host_id.as_deref().map(str::trim).filter(|value| !value.is_empty());
    let city_codes: Vec<&str> = kernel.config.cities.iter().map(|city| city.code()).collect();
    let pool = kernel.db.pool();
    let totals = dashboard_queries::figures(pool, from, to, host, &city_codes, None).await?;
    let mut cities = Vec::with_capacity(city_codes.len());
    for city in &city_codes {
        let figures = dashboard_queries::figures(pool, from, to, host, &city_codes, Some(city)).await?;
        cities.push(CityFigures { city: (*city).to_owned(), figures });
    }
    let series = dashboard_queries::series(pool, from, to, host, &city_codes).await?;
    Ok(DashboardSummary { from, to, totals, cities, series })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_thirty_days_and_rejects_large_or_inverted_ranges() {
        let today = NaiveDate::from_ymd_opt(2026, 11, 26).unwrap();
        let (from, to) = window(&DashboardQuery::default(), today).unwrap();
        assert_eq!((to - from).num_days(), 29);
        assert!(window(&DashboardQuery { from: Some(today), to: Some(today - Duration::days(1)), host_id: None }, today).is_err());
        assert!(window(&DashboardQuery { from: Some(today - Duration::days(366)), to: Some(today), host_id: None }, today).is_err());
    }
}
