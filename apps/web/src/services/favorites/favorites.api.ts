import type { SessionClient } from '@mungini/session';

export const listWishlists = async (client: SessionClient, page: number, pageSize = 10) => {
  const result = await client.api.GET('/api/guest/wishlists', { params: { query: { page, pageSize } } });
  if (!result.data) throw new Error('WISHLISTS_LOAD_FAILED');
  return result.data.data;
};

export const createWishlist = async (client: SessionClient, name: string) => {
  const result = await client.api.POST('/api/guest/wishlists', { body: { name, visibility: 'PRIVATE' } });
  if (!result.data) throw new Error('WISHLIST_CREATE_FAILED');
  return result.data.data;
};

export const listItems = async (client: SessionClient, wishlistId: string, page: number, locale: string, propertyIds?: string) => {
  const result = await client.api.GET('/api/guest/wishlists/{wishlistId}/items', { params: { path: { wishlistId }, query: { page, pageSize: 10, locale, propertyIds } } });
  if (!result.data) throw new Error('WISHLIST_ITEMS_LOAD_FAILED');
  return result.data.data;
};

export const addItem = async (client: SessionClient, wishlistId: string, propertyId: string) => {
  const result = await client.api.POST('/api/guest/wishlists/{wishlistId}/items/{propertyId}', { params: { path: { wishlistId, propertyId } } });
  if (!result.data) throw new Error('WISHLIST_ADD_FAILED');
};

export const removeItem = async (client: SessionClient, wishlistId: string, propertyId: string) => {
  const result = await client.api.DELETE('/api/guest/wishlists/{wishlistId}/items/{propertyId}', { params: { path: { wishlistId, propertyId } } });
  if (!result.data) throw new Error('WISHLIST_REMOVE_FAILED');
};
