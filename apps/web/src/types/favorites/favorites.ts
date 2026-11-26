import type { components } from '@mungini/api';

export type Wishlist = components['schemas']['WishlistView'];
export type WishlistItem = components['schemas']['WishlistItem'];
export type WishlistPage = components['schemas']['Envelope_Page_WishlistView']['data'];
export type WishlistItemPage = components['schemas']['Envelope_Page_WishlistItem']['data'];
