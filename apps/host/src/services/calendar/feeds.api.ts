import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';

export const listFeeds = async (client: SessionClient, propertyId: string, page: number) => {
  const response = await client.api.GET('/api/host/calendar/feed-tokens', { params: { query: { propertyId, page, pageSize: 10 } } });
  return response.error || !response.data ? { data: null, error: apiErrorCode(response.error) } : { data: response.data.data, error: null };
};

export const createFeed = async (client: SessionClient, propertyId: string) => {
  const response = await client.api.POST('/api/host/calendar/feed-tokens', { body: { propertyId } });
  return response.error || !response.data ? { data: null, error: apiErrorCode(response.error) } : { data: response.data.data, error: null };
};

export const revokeFeed = async (client: SessionClient, tokenId: string) => {
  const response = await client.api.DELETE('/api/host/calendar/feed-tokens/{tokenId}', { params: { path: { tokenId } } });
  return response.error || !response.data ? { data: null, error: apiErrorCode(response.error) } : { data: response.data.data, error: null };
};
