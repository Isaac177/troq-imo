use kernel::auth::Actor;
use kernel::permissions::names::PROPERTIES_MODERATE;
use kernel::{Kernel, Page, PageRequest, Result};
use sqlx::{Postgres, QueryBuilder};

use super::reservation_filters;
use super::reservation_queries::{self, FROM, SELECT, SUMMARY};
use super::reservation_types::{ReservationFilter, ReservationRow, ReservationSummary};
use crate::properties::media::cover_for_unit_type;

pub async fn list(kernel: &Kernel, actor: &Actor, input: ReservationFilter) -> Result<Page<ReservationRow>> {
    fetch(kernel, actor, input, true).await
}

pub async fn export(kernel: &Kernel, actor: &Actor, input: ReservationFilter) -> Result<Page<ReservationRow>> {
    fetch(kernel, actor, input, false).await
}

async fn fetch(kernel: &Kernel, actor: &Actor, input: ReservationFilter, include_covers: bool) -> Result<Page<ReservationRow>> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let parsed = reservation_filters::validated(&input)?;
    let default = PageRequest::default();
    let page = PageRequest { page: input.page.unwrap_or(default.page), page_size: input.page_size.unwrap_or(default.page_size) }.validated()?;
    let pool = kernel.db.pool();
    let mut count = QueryBuilder::<Postgres>::new("SELECT COUNT(*)");
    count.push(FROM);
    reservation_queries::filters(&mut count, actor.city.code(), &input, &parsed, true);
    let total = count.build_query_scalar().fetch_one(pool).await?;
    let mut query = QueryBuilder::<Postgres>::new(SELECT);
    query.push(FROM);
    reservation_queries::filters(&mut query, actor.city.code(), &input, &parsed, true);
    query.push(r#" ORDER BY r."createdAt" DESC, r.id LIMIT "#).push_bind(page.limit()).push(" OFFSET ").push_bind(page.offset());
    let mut items: Vec<ReservationRow> = query.build_query_as().fetch_all(pool).await?;
    if include_covers {
        for item in &mut items {
            if let Some(id) = &item.unit_type_id {
                item.unit_type_cover_url = cover_for_unit_type(pool, &kernel.storage, actor.city, id).await?;
            }
        }
    }
    Ok(Page::new(items, total, page))
}

pub async fn summary(kernel: &Kernel, actor: &Actor, input: ReservationFilter) -> Result<ReservationSummary> {
    actor.require_platform(PROPERTIES_MODERATE)?;
    let parsed = reservation_filters::validated(&input)?;
    let mut query = QueryBuilder::<Postgres>::new(SUMMARY);
    query.push(FROM);
    reservation_queries::filters(&mut query, actor.city.code(), &input, &parsed, false);
    Ok(query.build_query_as::<ReservationSummary>().fetch_one(kernel.db.pool()).await?)
}
