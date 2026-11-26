import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { AdminProperty, AdminPropertyDetail, RequestResult } from '@/types/properties/properties';

export type PagedResult<T> = { items: T[]; total: number; page: number; pageSize: number };

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listProperties = async (client: SessionClient, status: string, search: string, page: number, pageSize: number): Promise<RequestResult<PagedResult<AdminProperty>>> => {
  const response = await client.api.GET('/api/admin/properties', {
    params: { query: { page, pageSize, ...(status ? { status } : {}), ...(search.trim() ? { search: search.trim() } : {}) } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getProperty = async (client: SessionClient, propertyId: string): Promise<RequestResult<AdminPropertyDetail>> => {
  const response = await client.api.GET('/api/admin/properties/{propertyId}', { params: { path: { propertyId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const suspendProperty = async (client: SessionClient, propertyId: string, expectedVersion: number, reasonCode: string): Promise<RequestResult<AdminPropertyDetail>> => {
  const response = await client.api.POST('/api/admin/properties/{propertyId}/suspend', { params: { path: { propertyId } }, body: { expectedVersion, reasonCode } });
  if (response.error || !response.data) return fail(response.error);
  const detail = await getProperty(client, propertyId);
  return { data: detail.data, error: detail.error };
};

export const reinstateProperty = async (client: SessionClient, propertyId: string, expectedVersion: number): Promise<RequestResult<AdminPropertyDetail>> => {
  const response = await client.api.POST('/api/admin/properties/{propertyId}/reinstate', { params: { path: { propertyId } }, body: { expectedVersion } });
  if (response.error || !response.data) return fail(response.error);
  const detail = await getProperty(client, propertyId);
  return { data: detail.data, error: detail.error };
};

export const listAmenities = (client: SessionClient) => client.api.GET('/api/public/amenities', { params: { query: { page: 1, pageSize: 100 } } });
