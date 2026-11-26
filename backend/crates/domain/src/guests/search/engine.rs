use std::time::Instant;

use chrono::{NaiveDate, Utc};
use kernel::city::City;
use kernel::error::Result;
use kernel::storage::Storage;
use sqlx::PgPool;

use super::events::{self, EventInput};
use super::pricing::{self, Stay};
use super::query::{SearchQuery, ENGINE_VERSION};
use super::rank::{CoverView, SearchHit};
use super::response::{Meta, SearchResponse, Timings};
use super::{aggregates, candidates, stage};
use crate::properties::media::image_url;

const CANDIDATE_BATCH_SIZE: i64 = 250;

pub async fn run(
    pool: &PgPool,
    storage: &Storage,
    city: City,
    locale: &str,
    query: SearchQuery,
    analytics_consent: bool,
) -> Result<SearchResponse> {
    let started = Instant::now();
    let window = query.window();
    let nights: Vec<NaiveDate> = window
        .map(|window| window.check_in.iter_days().take_while(|day| *day < window.check_out).collect())
        .unwrap_or_default();
    let stay = window.map(|window| Stay { window, nights: &nights });
    let mut transaction = pool.begin().await?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(&mut *transaction)
        .await?;
    stage::create(&mut transaction).await?;
    let mut offset = 0;
    let mut candidate_plans = 0usize;
    let mut priced_plans = 0usize;
    let mut candidates_ms = 0.0;
    loop {
        let batch_started = Instant::now();
        let rows = candidates::candidates(&mut transaction, city, locale, &query, window.as_ref(), CANDIDATE_BATCH_SIZE, offset)
            .await?;
        candidates_ms += batch_started.elapsed().as_secs_f64() * 1000.0;
        let count = rows.len();
        if count == 0 {
            break;
        }
        candidate_plans += count;
        let offers = pricing::price(&mut transaction, city, rows, &query, stay.as_ref()).await?;
        let inserted = stage::insert(&mut transaction, offers, &query).await?;
        if stay.is_some() {
            priced_plans += inserted;
        }
        offset += count as i64;
        if count < CANDIDATE_BATCH_SIZE as usize {
            break;
        }
    }
    let pricing_ms = started.elapsed().as_secs_f64() * 1000.0;
    stage::group(&mut transaction).await?;
    let facets = aggregates::facets(&mut transaction).await?;
    let stats = aggregates::stats(&mut transaction).await?;
    let total = stats.result_count;
    let mut items = stage::page(&mut transaction, &query).await?;
    transaction.commit().await?;
    attach_covers(pool, storage, locale, &mut items).await?;
    let fingerprint = query.fingerprint();
    let total_ms = started.elapsed().as_secs_f64() * 1000.0;
    let analytics_recorded = analytics_consent
        && record_event(pool, city, &fingerprint, &query, total, candidate_plans, priced_plans, total_ms).await;
    Ok(SearchResponse {
        engine_version: ENGINE_VERSION,
        fingerprint,
        page: query.page as u32,
        page_size: query.page_size as u32,
        query,
        data_as_of: Utc::now(),
        items,
        total,
        facets,
        stats,
        meta: Meta {
            candidate_plans,
            priced_plans,
            timings_ms: Timings {
                candidates: round1(candidates_ms),
                pricing: round1(pricing_ms - candidates_ms),
                total: round1(total_ms),
            },
            analytics_recorded,
        },
    })
}

async fn attach_covers(pool: &PgPool, storage: &Storage, locale: &str, items: &mut [SearchHit]) -> Result<()> {
    let ids: Vec<String> = items.iter().map(|hit| hit.id.clone()).collect();
    let covers = candidates::cover_keys(pool, &ids, locale).await?;
    for hit in items {
        let Some(cover) = covers.get(&hit.id) else {
            continue;
        };
        match image_url(storage, &cover.storage_key).await {
            Ok(url) => {
                hit.cover = Some(CoverView {
                    media_id: cover.media_id.clone(),
                    alt: cover.alt.clone(),
                    content_path: url.clone(),
                    url,
                })
            }
            Err(error) => tracing::warn!(media_id = %cover.media_id, error = %error, "stay cover unavailable"),
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn record_event(
    pool: &PgPool,
    city: City,
    fingerprint: &str,
    query: &SearchQuery,
    result_count: i64,
    candidate_plans: usize,
    priced_plans: usize,
    total_ms: f64,
) -> bool {
    let event = EventInput {
        city,
        fingerprint,
        query,
        result_count,
        candidate_plans: candidate_plans.min(i32::MAX as usize) as i32,
        priced_plans: priced_plans.min(i32::MAX as usize) as i32,
        duration_ms: total_ms.min(i32::MAX as f64) as i32,
    };
    match events::record(pool, event).await {
        Ok(()) => true,
        Err(error) => {
            tracing::warn!(error = %error, "search event was not recorded");
            false
        }
    }
}

fn round1(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}
