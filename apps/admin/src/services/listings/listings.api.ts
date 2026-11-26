import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { AdminRevision, ListingDecisionBody, QueueItem, RequestResult, Revision } from '@/types/listings/listings';

export type PagedResult<T> = { items: T[]; total: number; page: number; pageSize: number };

const fail = <T>(error: unknown): RequestResult<T> => {
  const params = (error as { error?: { params?: { blockers?: string[] } } })?.error?.params;
  return { data: null, error: apiErrorCode(error), blockers: params?.blockers ?? [] };
};

export const listQueue = async (client: SessionClient, status: string, page: number, pageSize: number): Promise<RequestResult<PagedResult<QueueItem>>> => {
  const response = await client.api.GET('/api/admin/listings/queue', {
    params: { query: { page, pageSize, ...(status ? { status } : {}) } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const getRevision = async (client: SessionClient, revisionId: string): Promise<RequestResult<AdminRevision>> => {
  const response = await client.api.GET('/api/admin/listings/{revisionId}', { params: { path: { revisionId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const decideListing = async (client: SessionClient, revisionId: string, body: ListingDecisionBody): Promise<RequestResult<Revision>> => {
  const response = await client.api.POST('/api/admin/listings/{revisionId}/decision', {
    params: { path: { revisionId } },
    body: body as unknown as components['schemas']['DecisionInput'],
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null, blockers: [] };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
