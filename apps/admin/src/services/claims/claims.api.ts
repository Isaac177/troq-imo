import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { ClaimDetail, ClaimRow, ClaimSummary, PayoutResult, RequestResult } from '@/types/claims/claims';

export type PagedResult<T> = { items: T[]; total: number; page: number; pageSize: number };
export type ClaimQuery = { status: string; search: string; page: number; pageSize: number };

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listClaims = async (client: SessionClient, query: ClaimQuery): Promise<RequestResult<PagedResult<ClaimRow>>> => {
  const response = await client.api.GET('/api/admin/claims', {
    params: {
      query: {
        page: query.page,
        pageSize: query.pageSize,
        ...(query.status ? { status: query.status } : {}),
        ...(query.search.trim() ? { search: query.search.trim() } : {}),
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getClaimSummary = async (client: SessionClient, query: ClaimQuery): Promise<RequestResult<ClaimSummary>> => {
  const response = await client.api.GET('/api/admin/claims/summary', {
    params: {
      query: {
        ...(query.status ? { status: query.status } : {}),
        ...(query.search.trim() ? { search: query.search.trim() } : {}),
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getClaim = async (client: SessionClient, caseId: string): Promise<RequestResult<ClaimDetail>> => {
  const response = await client.api.GET('/api/admin/claims/{caseId}', { params: { path: { caseId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const decideClaim = async (client: SessionClient, caseId: string, input: { decision: string; reasonCode: string; approvedAmount: string | null; expectedVersion: number }): Promise<RequestResult<ClaimDetail>> => {
  const response = await client.api.POST('/api/admin/claims/{caseId}/decision', { params: { path: { caseId } }, body: input });
  if (response.error || !response.data) return fail(response.error);
  const detail = await getClaim(client, caseId);
  return { data: detail.data, error: detail.error };
};

export const recordPayout = async (client: SessionClient, caseId: string, input: { payoutState: string; payoutRef: string | null }): Promise<RequestResult<PayoutResult>> => {
  const response = await client.api.POST('/api/admin/claims/{caseId}/payout', { params: { path: { caseId } }, body: input });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
