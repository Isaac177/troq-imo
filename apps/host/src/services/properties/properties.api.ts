import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';

export type PropertyView = components['schemas']['PropertyView'];
export type RequestResult<T> = { data: T | null; error: string | null; blockers: string[] };

const fail = <T>(error: unknown): RequestResult<T> => {
  const params = (error as { error?: { params?: { blockers?: string[] } } })?.error?.params;
  return { data: null, error: apiErrorCode(error), blockers: params?.blockers ?? [] };
};

export const createProperty = async (client: SessionClient, input: { hostId: string; nameEn: string; nameFr: string; timezone: string; addressLine: string; descriptionEn: string; descriptionFr: string }): Promise<RequestResult<PropertyView>> => {
  const response = await client.api.POST('/api/host/properties', { body: { ...input, idempotencyKey: crypto.randomUUID() } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const propertyLifecycle = async (client: SessionClient, propertyId: string, action: 'PUBLISH' | 'PAUSE' | 'ARCHIVE', expectedVersion: number): Promise<RequestResult<PropertyView>> => {
  const response = await client.api.POST('/api/host/properties/{propertyId}/lifecycle', {
    params: { path: { propertyId } },
    body: { action, expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
