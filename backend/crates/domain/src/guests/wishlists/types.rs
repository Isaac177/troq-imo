use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

pub const MAX_WISHLISTS_PER_GUEST: i64 = 20;
pub const MAX_ITEMS_PER_WISHLIST: i64 = 200;
pub const MAX_NAME_CHARS: usize = 80;
pub const MAX_PROPERTY_FILTER: usize = 100;
pub const MAX_ID_CHARS: usize = 128;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateWishlist {
    pub name: String,
    pub visibility: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWishlist {
    pub name: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct ItemsQuery {
    pub property_ids: Option<String>,
    pub locale: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WishlistView {
    pub id: String,
    pub name: String,
    pub visibility: String,
    pub item_count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WishlistItem {
    pub property_id: String,
    pub name: String,
    pub added_at: DateTime<Utc>,
}
