import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { AdminCase, AdminCaseDetail, EvidenceContent, RequestResult } from '@/types/cases/cases';

export type PagedResult<T> = { items: T[]; total: number; page: number; pageSize: number };
export type CaseQuery = { state: string; caseType: string; overdue: boolean; search: string; page: number; pageSize: number };

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listCases = async (client: SessionClient, query: CaseQuery): Promise<RequestResult<PagedResult<AdminCase>>> => {
  const response = await client.api.GET('/api/admin/cases', {
    params: {
      query: {
        page: query.page,
        pageSize: query.pageSize,
        ...(query.state ? { state: query.state } : {}),
        ...(query.caseType ? { caseType: query.caseType } : {}),
        ...(query.overdue ? { overdue: true } : {}),
        ...(query.search.trim() ? { search: query.search.trim() } : {}),
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getCase = async (client: SessionClient, caseId: string): Promise<RequestResult<AdminCaseDetail>> => {
  const response = await client.api.GET('/api/admin/cases/{caseId}', { params: { path: { caseId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const closeCase = async (client: SessionClient, caseId: string, expectedVersion: number, resolution: string): Promise<RequestResult<AdminCase>> => {
  const response = await client.api.POST('/api/admin/cases/{caseId}/close', {
    params: { path: { caseId } },
    body: { expectedVersion, resolution },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const escalateCase = async (client: SessionClient, caseId: string, expectedVersion: number, reason: string): Promise<RequestResult<AdminCase>> => {
  const response = await client.api.POST('/api/admin/cases/{caseId}/escalate', {
    params: { path: { caseId } },
    body: { expectedVersion, reason },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getEvidence = async (client: SessionClient, evidenceId: string): Promise<RequestResult<EvidenceContent>> => {
  const response = await client.api.GET('/api/admin/cases/evidence/{evidenceId}', { params: { path: { evidenceId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');

export const listAssignees = async (client: SessionClient, search: string, page: number): Promise<RequestResult<PagedResult<import('@/types/cases/cases').CaseAssignee>>> => {
  const response = await client.api.GET('/api/admin/cases/assignees', { params: { query: { search, page, pageSize: 20 } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const assignCase = async (client: SessionClient, caseId: string, expectedVersion: number, assigneeUserId: string | null): Promise<RequestResult<AdminCase>> => {
  const response = await client.api.POST('/api/admin/cases/{caseId}/assign', { params: { path: { caseId } }, body: { expectedVersion, assigneeUserId } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};
