use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::paging::{Page, PageRequest};
use sqlx::{PgPool, Postgres, QueryBuilder};

use super::types::{AdminPropertyItem, AdminPropertyQuery};
use crate::properties::facts::property_facts_sql;
use crate::properties::http::like_pattern;
use crate::properties::messages::PROPERTY_STATUS_UNKNOWN;
use crate::properties::property::types::PROPERTY_STATUSES;

pub async fn list(pool: &PgPool, city: City, query: &AdminPropertyQuery, page: PageRequest) -> Result<Page<AdminPropertyItem>> {
    if let Some(status) = query.status.as_deref() {
        if !PROPERTY_STATUSES.contains(&status) {
            return Err(Error::bad_request(PROPERTY_STATUS_UNKNOWN));
        }
    }
    let search = like_pattern(query.search.as_deref());
    let mut total = QueryBuilder::<Postgres>::new(
        r#"SELECT COUNT(*) FROM stay_properties p JOIN hosts h ON h.id = p."operatorId" WHERE TRUE"#,
    );
    push_filters(&mut total, city, query.status.as_deref(), search.as_deref());
    let total: i64 = total.build_query_scalar().fetch_one(pool).await?;
    let mut items = QueryBuilder::<Postgres>::new(format!(
        r#"SELECT p.id, p."nameEn" AS name_en, p."nameFr" AS name_fr, p."operatorId" AS host_id,
                  h.display_name AS host_name, p.city_id AS city, p.neighborhood, p."addressLine" AS address_line,
                  h.status AS host_status, p."publishedRevision" AS published_revision, p.status, p.version,
                  (SELECT COUNT(*) FROM stay_property_revisions r
                     WHERE r."propertyId" = p.id AND r."reviewStatus" IN ('SUBMITTED', 'CHANGES_REQUESTED')) AS open_revisions,
                  p."updatedAt" AS updated_at, {}
           FROM stay_properties p JOIN hosts h ON h.id = p."operatorId" WHERE TRUE"#,
        property_facts_sql("p")
    ));
    push_filters(&mut items, city, query.status.as_deref(), search.as_deref());
    items
        .push(r#" ORDER BY p."updatedAt" DESC, p.id LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let rows = items.build_query_as::<AdminPropertyItem>().fetch_all(pool).await?;
    Ok(Page::new(rows, total, page))
}

fn push_filters(builder: &mut QueryBuilder<'_, Postgres>, city: City, status: Option<&str>, search: Option<&str>) {
    builder.push(" AND p.city_id = ").push_bind(city.code().to_string());
    if let Some(status) = status {
        builder.push(" AND p.status = ").push_bind(status.to_string());
    }
    if let Some(search) = search {
        builder
            .push(r#" AND (p."nameEn" ILIKE "#)
            .push_bind(search.to_string())
            .push(r#" ESCAPE '\' OR p."nameFr" ILIKE "#)
            .push_bind(search.to_string())
            .push(r#" ESCAPE '\' OR h.display_name ILIKE "#)
            .push_bind(search.to_string())
            .push(r#" ESCAPE '\')"#);
    }
}
