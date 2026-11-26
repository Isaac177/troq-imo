use chrono::{DateTime, Utc};
use kernel::city::City;
use kernel::error::Result;
use sqlx::{PgPool, Postgres, Transaction};

use super::types::WishlistView;
use crate::guests::listing::listed;

fn wishlist_select() -> String {
    format!(
        r#"SELECT w.id, w.name, w.visibility, w."createdAt" AS created_at,
    (SELECT COUNT(*) FROM stay_wishlist_items i
       JOIN stay_properties p ON p.id = i."propertyId"
       JOIN hosts h ON h.id = p."operatorId"
      WHERE i."wishlistId" = w.id AND {listed}) AS item_count
    FROM stay_wishlists w"#,
        listed = listed("p", "h")
    )
}

pub async fn count_wishlists(pool: &PgPool, city: City, guest: &str) -> Result<i64> {
    Ok(sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM stay_wishlists WHERE city_id = $1 AND "guestUserId" = $2"#,
    )
    .bind(city.code())
    .bind(guest)
    .fetch_one(pool)
    .await?)
}

pub async fn list_wishlists(
    pool: &PgPool,
    city: City,
    guest: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<WishlistView>> {
    Ok(sqlx::query_as::<_, WishlistView>(&format!(
        r#"{select} WHERE w.city_id = $1 AND w."guestUserId" = $2
           ORDER BY w."createdAt", w.id LIMIT $3 OFFSET $4"#,
        select = wishlist_select()
    ))
    .bind(city.code())
    .bind(guest)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?)
}

pub async fn find_wishlist(
    pool: &PgPool,
    city: City,
    guest: &str,
    wishlist_id: &str,
) -> Result<Option<WishlistView>> {
    Ok(sqlx::query_as::<_, WishlistView>(&format!(
        r#"{select} WHERE w.city_id = $1 AND w."guestUserId" = $2 AND w.id = $3"#,
        select = wishlist_select()
    ))
    .bind(city.code())
    .bind(guest)
    .bind(wishlist_id)
    .fetch_optional(pool)
    .await?)
}

pub async fn insert_wishlist(
    tx: &mut Transaction<'_, Postgres>,
    city: City,
    id: &str,
    guest: &str,
    name: &str,
    visibility: &str,
) -> Result<DateTime<Utc>> {
    Ok(sqlx::query_scalar(
        r#"INSERT INTO stay_wishlists (id, city_id, "guestUserId", name, visibility)
           VALUES ($1, $2, $3, $4, $5) RETURNING "createdAt""#,
    )
    .bind(id)
    .bind(city.code())
    .bind(guest)
    .bind(name)
    .bind(visibility)
    .fetch_one(&mut **tx)
    .await?)
}

pub async fn update_wishlist(
    pool: &PgPool,
    city: City,
    guest: &str,
    wishlist_id: &str,
    name: Option<&str>,
    visibility: Option<&str>,
) -> Result<bool> {
    let result = sqlx::query(
        r#"UPDATE stay_wishlists
           SET name = COALESCE($4, name), visibility = COALESCE($5, visibility), "updatedAt" = now()
           WHERE id = $3 AND city_id = $1 AND "guestUserId" = $2"#,
    )
    .bind(city.code())
    .bind(guest)
    .bind(wishlist_id)
    .bind(name)
    .bind(visibility)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn item_count(pool: &PgPool, wishlist_id: &str) -> Result<i64> {
    Ok(sqlx::query_scalar(r#"SELECT COUNT(*) FROM stay_wishlist_items WHERE "wishlistId" = $1"#)
        .bind(wishlist_id)
        .fetch_one(pool)
        .await?)
}

pub async fn add_item(pool: &PgPool, wishlist_id: &str, property_id: &str) -> Result<()> {
    sqlx::query(
        r#"INSERT INTO stay_wishlist_items ("wishlistId", "propertyId") VALUES ($1, $2)
           ON CONFLICT ("wishlistId", "propertyId") DO NOTHING"#,
    )
    .bind(wishlist_id)
    .bind(property_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_item(pool: &PgPool, wishlist_id: &str, property_id: &str) -> Result<bool> {
    let result = sqlx::query(
        r#"DELETE FROM stay_wishlist_items WHERE "wishlistId" = $1 AND "propertyId" = $2"#,
    )
    .bind(wishlist_id)
    .bind(property_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}
