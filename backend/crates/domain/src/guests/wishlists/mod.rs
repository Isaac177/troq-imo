pub(super) mod handlers;
mod item_filter;
mod queries;
mod service;
mod types;
mod validators;

#[cfg(test)]
mod tests;

pub use service::{
    add_item as add_wishlist_item, create_wishlist, list_items, list_wishlists,
    remove_item as remove_wishlist_item, update_wishlist,
};
pub use types::{
    CreateWishlist, ItemsQuery, UpdateWishlist, WishlistItem, WishlistView, MAX_ITEMS_PER_WISHLIST,
    MAX_WISHLISTS_PER_GUEST,
};
