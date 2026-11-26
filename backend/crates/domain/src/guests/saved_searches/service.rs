use chrono::Utc;
use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::paging::{Page, PageRequest};
use sqlx::PgPool;

use super::queries;
use super::types::{CreateSavedSearch, SavedSearchRemoved, SavedSearchView, MAX_SAVED_SEARCHES_PER_GUEST};
use super::validators::check_parameters;
use crate::guests::idempotent::{self, SAVED_SEARCH_CREATE};
use crate::guests::messages::{SAVED_SEARCH_LIMIT_REACHED, SAVED_SEARCH_NOT_FOUND};

pub async fn create_saved_search(
    pool: &PgPool,
    city: City,
    guest: &str,
    input: CreateSavedSearch,
) -> Result<SavedSearchView> {
    let key = idempotent::clean(input.idempotency_key.as_deref())?;
    if let Some(stored) = idempotent::replay::<SavedSearchView>(pool, city, SAVED_SEARCH_CREATE, key).await? {
        return Ok(stored);
    }
    check_parameters(&input.parameters)?;
    if queries::count(pool, city, guest).await? >= MAX_SAVED_SEARCHES_PER_GUEST {
        return Err(Error::conflict(SAVED_SEARCH_LIMIT_REACHED)
            .with_params(serde_json::json!({ "max": MAX_SAVED_SEARCHES_PER_GUEST })));
    }
    let id = new_id("saved");
    let mut tx = pool.begin().await?;
    if !idempotent::claim(&mut tx, city, SAVED_SEARCH_CREATE, key, &id).await? {
        tx.rollback().await?;
        return idempotent::replay_or_busy(pool, city, SAVED_SEARCH_CREATE, key).await;
    }
    let created_at = queries::insert(&mut tx, city, &id, guest, &input.parameters, input.alert_consent).await?;
    let created = SavedSearchView {
        id,
        parameters: input.parameters,
        alert_consent: input.alert_consent,
        created_at,
    };
    idempotent::remember(&mut tx, city, SAVED_SEARCH_CREATE, key, &created).await?;
    tx.commit().await?;
    Ok(created)
}

pub async fn list_saved_searches(
    pool: &PgPool,
    city: City,
    guest: &str,
    request: PageRequest,
) -> Result<Page<SavedSearchView>> {
    let request = request.validated()?;
    let total = queries::count(pool, city, guest).await?;
    let rows = queries::list(pool, city, guest, request.limit(), request.offset()).await?;
    Ok(Page::new(rows, total, request))
}

pub async fn delete_saved_search(
    pool: &PgPool,
    city: City,
    guest: &str,
    saved_search_id: &str,
) -> Result<SavedSearchRemoved> {
    if !queries::delete(pool, city, guest, saved_search_id).await? {
        return Err(Error::not_found(SAVED_SEARCH_NOT_FOUND));
    }
    Ok(SavedSearchRemoved {
        saved_search_id: saved_search_id.to_string(),
        removed_at: Utc::now(),
    })
}
