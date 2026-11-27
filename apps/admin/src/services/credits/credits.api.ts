import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type {
  AdjustInput,
  CommissionRate,
  CommissionRatePage,
  CreditAccountPage,
  CreditBalance,
  CreditDetail,
  CreditLedgerPage,
  CreditSummary,
  RateInput,
  RequestResult,
  TopupInput,
} from '@/types/credits/credits';

export type CreditAccountsQuery = { status: string; search: string; page: number; pageSize: number };

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');

export const getCreditSummary = async (client: SessionClient, hostId?: string): Promise<RequestResult<CreditSummary>> => {
  const response = await client.api.GET('/api/admin/credits/summary', {
    params: { query: hostId ? { hostId } : {} },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCreditAccounts = async (client: SessionClient, query: CreditAccountsQuery): Promise<RequestResult<CreditAccountPage>> => {
  const response = await client.api.GET('/api/admin/credits/accounts', {
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

export const getCreditDetail = async (client: SessionClient, hostId: string): Promise<RequestResult<CreditDetail>> => {
  const response = await client.api.GET('/api/admin/credits/hosts/{hostId}', { params: { path: { hostId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getCreditLedger = async (client: SessionClient, hostId: string, page: number, pageSize: number): Promise<RequestResult<CreditLedgerPage>> => {
  const response = await client.api.GET('/api/admin/credits/hosts/{hostId}/ledger', {
    params: { path: { hostId }, query: { page, pageSize } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const topUpCredit = async (client: SessionClient, hostId: string, input: TopupInput): Promise<RequestResult<CreditBalance>> => {
  const response = await client.api.POST('/api/admin/credits/hosts/{hostId}/topup', { params: { path: { hostId } }, body: input });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const adjustCredit = async (client: SessionClient, hostId: string, input: AdjustInput): Promise<RequestResult<CreditBalance>> => {
  const response = await client.api.POST('/api/admin/credits/hosts/{hostId}/adjust', { params: { path: { hostId } }, body: input });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getCommissionRate = async (client: SessionClient, hostId: string): Promise<RequestResult<CommissionRate>> => {
  const response = await client.api.GET('/api/admin/commission/hosts/{hostId}/rate', { params: { path: { hostId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getCommissionRateHistory = async (client: SessionClient, hostId: string, page: number, pageSize: number): Promise<RequestResult<CommissionRatePage>> => {
  const response = await client.api.GET('/api/admin/commission/hosts/{hostId}/rate/history', {
    params: { path: { hostId }, query: { page, pageSize } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const setCommissionRate = async (client: SessionClient, hostId: string, input: RateInput): Promise<RequestResult<{ hostId: string; id: string; rateBps: number }>> => {
  const response = await client.api.POST('/api/admin/commission/hosts/{hostId}/rate', { params: { path: { hostId } }, body: input });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};
