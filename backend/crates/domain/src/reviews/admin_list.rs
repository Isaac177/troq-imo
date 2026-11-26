use chrono::{DateTime, Utc};
use kernel::auth::Actor;
use kernel::city::City;
use kernel::error::Result;
use kernel::kernel::Kernel;
use kernel::paging::{Page, PageRequest};
use kernel::permissions::names::REVIEWS_MODERATE;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, QueryBuilder};
use utoipa::{IntoParams, ToSchema};

use super::covers;
use super::validators::{clean_search, escape_like, optional_review_status, rating};

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct AdminReviewQuery {
    pub status: Option<String>,
    pub rating: Option<i16>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminReviewItem {
    pub id: String,
    pub property_id: String,
    pub property_name_en: String,
    pub property_name_fr: String,
    pub operator_name: String,
    pub city: String,
    pub reservation_reference: String,
    pub guest_name: Option<String>,
    pub rating: i16,
    pub comment: Option<String>,
    pub status: String,
    pub operator_reply: Option<String>,
    pub operator_replied_at: Option<DateTime<Utc>>,
    pub moderated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub unit_type_id: Option<String>,
    pub unit_type_name_en: Option<String>,
    pub unit_type_name_fr: Option<String>,
    #[sqlx(skip)]
    pub unit_type_cover_url: Option<String>,
}

const SELECT_SQL: &str = r#"SELECT v.id, v."propertyId" AS property_id, p."nameEn" AS property_name_en,
    p."nameFr" AS property_name_fr, h.display_name AS operator_name, v.city_id AS city,
    r.reference AS reservation_reference,
    (SELECT g."fullName" FROM stay_reservation_items i
       JOIN stay_reservation_guests g ON g."reservationItemId" = i.id AND g.role = 'PRIMARY'
      WHERE i."reservationId" = v."reservationId" LIMIT 1) AS guest_name,
    v.rating, v.comment, v.status, v."operatorReply" AS operator_reply,
    v."operatorRepliedAt" AS operator_replied_at, v."moderatedAt" AS moderated_at,
    v."createdAt" AS created_at,
    (SELECT i."unitTypeId" FROM stay_reservation_items i WHERE i."reservationId" = v."reservationId" LIMIT 1) AS unit_type_id,
    (SELECT t."nameEn" FROM stay_reservation_items i JOIN stay_unit_types t ON t.id = i."unitTypeId"
      WHERE i."reservationId" = v."reservationId" LIMIT 1) AS unit_type_name_en,
    (SELECT t."nameFr" FROM stay_reservation_items i JOIN stay_unit_types t ON t.id = i."unitTypeId"
      WHERE i."reservationId" = v."reservationId" LIMIT 1) AS unit_type_name_fr"#;

const FROM_SQL: &str = r#" FROM stay_reviews v
    JOIN stay_properties p ON p.id = v."propertyId"
    JOIN hosts h ON h.id = v."operatorId"
    JOIN stay_reservations r ON r.id = v."reservationId"
    WHERE v.city_id = "#;

pub async fn list_reviews(
    kernel: &Kernel,
    actor: &Actor,
    query: AdminReviewQuery,
    page: PageRequest,
) -> Result<Page<AdminReviewItem>> {
    actor.require_platform(REVIEWS_MODERATE)?;
    optional_review_status(query.status.as_deref())?;
    if let Some(value) = query.rating {
        rating(value)?;
    }
    let page = page.validated()?;
    let (mut items, total) = fetch(kernel.db.pool(), actor.city, &query, page).await?;
    let ids = covers::unit_type_ids(items.iter().map(|item| item.unit_type_id.as_ref()));
    let urls = covers::cover_urls(kernel, &ids).await?;
    for item in &mut items {
        item.unit_type_cover_url = item.unit_type_id.as_ref().and_then(|id| urls.get(id).cloned());
    }
    Ok(Page::new(items, total, page))
}

async fn fetch(
    pool: &PgPool,
    city: City,
    query: &AdminReviewQuery,
    page: PageRequest,
) -> Result<(Vec<AdminReviewItem>, i64)> {
    let search = clean_search(query.search.as_deref());
    let mut count = QueryBuilder::<Postgres>::new(format!("SELECT COUNT(*){FROM_SQL}"));
    count.push_bind(city.code());
    push_filters(&mut count, query, search);
    let total: i64 = count.build_query_scalar().fetch_one(pool).await?;

    let mut items = QueryBuilder::<Postgres>::new(format!("{SELECT_SQL}{FROM_SQL}"));
    items.push_bind(city.code());
    push_filters(&mut items, query, search);
    items
        .push(r#" ORDER BY v."createdAt" DESC, v.id LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let rows = items.build_query_as::<AdminReviewItem>().fetch_all(pool).await?;
    Ok((rows, total))
}

fn push_filters(builder: &mut QueryBuilder<'_, Postgres>, query: &AdminReviewQuery, search: Option<&str>) {
    if let Some(status) = query.status.as_deref() {
        builder.push(" AND v.status = ").push_bind(status.to_string());
    }
    if let Some(value) = query.rating {
        builder.push(" AND v.rating = ").push_bind(value);
    }
    if let Some(search) = search {
        let pattern = format!("%{}%", escape_like(search));
        builder
            .push(r#" AND (p."nameEn" ILIKE "#)
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR p."nameFr" ILIKE "#)
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR h.display_name ILIKE "#)
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR r.reference ILIKE "#)
            .push_bind(pattern.clone())
            .push(r#" ESCAPE '\' OR v.comment ILIKE "#)
            .push_bind(pattern)
            .push(r#" ESCAPE '\')"#);
    }
}
