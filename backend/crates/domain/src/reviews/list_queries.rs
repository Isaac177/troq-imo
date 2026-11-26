use kernel::city::City;
use kernel::error::Result;
use kernel::paging::PageRequest;
use sqlx::{PgPool, Postgres, QueryBuilder};

use super::queries::ReviewRow;
use super::sql::REVIEW_COLUMNS;

pub async fn list_for_property(
    pool: &PgPool,
    city: City,
    property_id: &str,
    status: Option<&str>,
    page: PageRequest,
) -> Result<(Vec<ReviewRow>, i64)> {
    let mut count = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM stay_reviews WHERE city_id = ");
    count.push_bind(city.code()).push(r#" AND "propertyId" = "#).push_bind(property_id.to_string());
    push_status(&mut count, status);
    let total: i64 = count.build_query_scalar().fetch_one(pool).await?;

    let mut items = QueryBuilder::<Postgres>::new(format!(
        "SELECT {REVIEW_COLUMNS} FROM stay_reviews WHERE city_id = "
    ));
    items.push_bind(city.code()).push(r#" AND "propertyId" = "#).push_bind(property_id.to_string());
    push_status(&mut items, status);
    items
        .push(r#" ORDER BY "createdAt" DESC, id DESC LIMIT "#)
        .push_bind(page.limit())
        .push(" OFFSET ")
        .push_bind(page.offset());
    let rows = items.build_query_as::<ReviewRow>().fetch_all(pool).await?;
    Ok((rows, total))
}

pub async fn list_queue(
    pool: &PgPool,
    city: City,
    status: &str,
    page: PageRequest,
) -> Result<(Vec<ReviewRow>, i64)> {
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM stay_reviews WHERE city_id = $1 AND status = $2",
    )
    .bind(city.code())
    .bind(status)
    .fetch_one(pool)
    .await?;
    let sql = format!(
        r#"SELECT {REVIEW_COLUMNS} FROM stay_reviews
           WHERE city_id = $1 AND status = $2
           ORDER BY "createdAt" ASC, id ASC
           LIMIT $3 OFFSET $4"#
    );
    let rows = sqlx::query_as::<_, ReviewRow>(&sql)
        .bind(city.code())
        .bind(status)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(pool)
        .await?;
    Ok((rows, total))
}

fn push_status(builder: &mut QueryBuilder<'_, Postgres>, status: Option<&str>) {
    if let Some(status) = status {
        builder.push(" AND status = ").push_bind(status.to_string());
    }
}
