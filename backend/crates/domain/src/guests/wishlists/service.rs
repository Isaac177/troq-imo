use kernel::city::City;
use kernel::error::{Error, Result};
use kernel::ids::new_id;
use kernel::paging::{Page, PageRequest};
use sqlx::PgPool;

use super::item_filter;
use super::queries;
use super::types::{
    CreateWishlist, ItemsQuery, UpdateWishlist, WishlistItem, WishlistView, MAX_ITEMS_PER_WISHLIST,
    MAX_WISHLISTS_PER_GUEST,
};
use super::validators::{clean_name, clean_visibility, parse_property_ids};
use crate::guests::discovery::property_is_listed;
use crate::guests::idempotent::{self, WISHLIST_CREATE};
use crate::guests::locale::clean_locale;
use crate::guests::messages::{PROPERTY_NOT_FOUND, WISHLIST_FULL, WISHLIST_LIMIT_REACHED, WISHLIST_NOT_FOUND};

pub(super) async fn owned(pool: &PgPool, city: City, guest: &str, wishlist_id: &str) -> Result<WishlistView> {
    queries::find_wishlist(pool, city, guest, wishlist_id)
        .await?
        .ok_or_else(|| Error::not_found(WISHLIST_NOT_FOUND))
}

pub async fn list_wishlists(
    pool: &PgPool,
    city: City,
    guest: &str,
    request: PageRequest,
) -> Result<Page<WishlistView>> {
    let request = request.validated()?;
    let total = queries::count_wishlists(pool, city, guest).await?;
    let rows = queries::list_wishlists(pool, city, guest, request.limit(), request.offset()).await?;
    Ok(Page::new(rows, total, request))
}

pub async fn create_wishlist(
    pool: &PgPool,
    city: City,
    guest: &str,
    input: CreateWishlist,
) -> Result<WishlistView> {
    let key = idempotent::clean(input.idempotency_key.as_deref())?;
    if let Some(stored) = idempotent::replay::<WishlistView>(pool, city, WISHLIST_CREATE, key).await? {
        return Ok(stored);
    }
    let name = clean_name(&input.name)?;
    let visibility = clean_visibility(input.visibility.as_deref())?;
    if queries::count_wishlists(pool, city, guest).await? >= MAX_WISHLISTS_PER_GUEST {
        return Err(Error::conflict(WISHLIST_LIMIT_REACHED)
            .with_params(serde_json::json!({ "max": MAX_WISHLISTS_PER_GUEST })));
    }
    let id = new_id("wish");
    let mut tx = pool.begin().await?;
    if !idempotent::claim(&mut tx, city, WISHLIST_CREATE, key, &id).await? {
        tx.rollback().await?;
        return idempotent::replay_or_busy(pool, city, WISHLIST_CREATE, key).await;
    }
    let created_at = queries::insert_wishlist(&mut tx, city, &id, guest, &name, &visibility).await?;
    let created = WishlistView { id, name, visibility, item_count: 0, created_at };
    idempotent::remember(&mut tx, city, WISHLIST_CREATE, key, &created).await?;
    tx.commit().await?;
    Ok(created)
}

pub async fn update_wishlist(
    pool: &PgPool,
    city: City,
    guest: &str,
    wishlist_id: &str,
    input: UpdateWishlist,
) -> Result<WishlistView> {
    owned(pool, city, guest, wishlist_id).await?;
    let name = input.name.as_deref().map(clean_name).transpose()?;
    let visibility = match input.visibility.as_deref() {
        Some(value) => Some(clean_visibility(Some(value))?),
        None => None,
    };
    queries::update_wishlist(pool, city, guest, wishlist_id, name.as_deref(), visibility.as_deref()).await?;
    owned(pool, city, guest, wishlist_id).await
}

pub async fn add_item(
    pool: &PgPool,
    city: City,
    guest: &str,
    wishlist_id: &str,
    property_id: &str,
) -> Result<WishlistView> {
    let list = owned(pool, city, guest, wishlist_id).await?;
    if !property_is_listed(pool, city, property_id).await? {
        return Err(Error::not_found(PROPERTY_NOT_FOUND));
    }
    if queries::item_count(pool, &list.id).await? >= MAX_ITEMS_PER_WISHLIST {
        return Err(Error::conflict(WISHLIST_FULL).with_params(serde_json::json!({ "max": MAX_ITEMS_PER_WISHLIST })));
    }
    queries::add_item(pool, &list.id, property_id).await?;
    owned(pool, city, guest, &list.id).await
}

pub async fn remove_item(
    pool: &PgPool,
    city: City,
    guest: &str,
    wishlist_id: &str,
    property_id: &str,
) -> Result<WishlistView> {
    let list = owned(pool, city, guest, wishlist_id).await?;
    queries::remove_item(pool, &list.id, property_id).await?;
    owned(pool, city, guest, &list.id).await
}

pub async fn list_items(
    pool: &PgPool,
    city: City,
    guest: &str,
    wishlist_id: &str,
    query: &ItemsQuery,
) -> Result<Page<WishlistItem>> {
    let default = PageRequest::default();
    let request = PageRequest {
        page: query.page.unwrap_or(default.page),
        page_size: query.page_size.unwrap_or(default.page_size),
    }
    .validated()?;
    let list = owned(pool, city, guest, wishlist_id).await?;
    let ids = query.property_ids.as_deref().map(parse_property_ids).transpose()?;
    let locale = clean_locale(query.locale.as_deref());
    let (rows, total) =
        item_filter::list_items(pool, &list.id, ids.as_deref(), &locale, request.limit(), request.offset()).await?;
    Ok(Page::new(rows, total, request))
}
