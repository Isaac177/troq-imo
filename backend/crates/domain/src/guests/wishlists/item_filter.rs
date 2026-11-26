use kernel::error::Result;
use sqlx::PgPool;

use super::types::WishlistItem;
use crate::guests::listing::{listed, property_name};

fn name_column() -> String {
    property_name("p", "$LOCALE")
}

pub async fn list_items(
    pool: &PgPool,
    wishlist_id: &str,
    property_ids: Option<&[String]>,
    locale: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<WishlistItem>, i64)> {
    let listed = listed("p", "h");
    let filter = r#"i."wishlistId" = $1 AND ($2::text[] IS NULL OR i."propertyId" = ANY($2))"#;
    let from = format!(
        r#"FROM stay_wishlist_items i
           JOIN stay_properties p ON p.id = i."propertyId"
           JOIN hosts h ON h.id = p."operatorId"
           WHERE {filter} AND {listed}"#
    );
    let total = sqlx::query_scalar::<_, i64>(&format!("SELECT COUNT(*) {from}"))
        .bind(wishlist_id)
        .bind(property_ids)
        .fetch_one(pool)
        .await?;
    let name = name_column().replace("$LOCALE", "$3");
    let rows = sqlx::query_as::<_, WishlistItem>(&format!(
        r#"SELECT i."propertyId" AS property_id, {name} AS name, i."addedAt" AS added_at
           {from} ORDER BY i."addedAt", i."propertyId" LIMIT $4 OFFSET $5"#
    ))
    .bind(wishlist_id)
    .bind(property_ids)
    .bind(locale)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok((rows, total))
}
