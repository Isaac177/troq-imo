import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { CreditLedgerPage, CreditSummary, RequestResult } from '@/types/credits/credits';

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const getCreditSummary = async (client: SessionClient, hostId: string): Promise<RequestResult<CreditSummary>> => {
  const response = await client.api.GET('/api/host/credits', { params: { query: { hostId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getCreditLedger = async (client: SessionClient, hostId: string, page: number, pageSize: number): Promise<RequestResult<CreditLedgerPage>> => {
  const response = await client.api.GET('/api/host/credits/ledger', { params: { query: { hostId, page, pageSize } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
