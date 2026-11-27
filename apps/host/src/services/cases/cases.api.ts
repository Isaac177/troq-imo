import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type {
  AddEvidencePayload,
  CaseEvidence,
  CaseEvidenceContent,
  CaseOption,
  HostCase,
  HostCasePage,
  OpenCasePayload,
  RequestResult,
} from '@/types/cases/cases';

export type CaseQuery = {
  hostId: string;
  state: string;
  page: number;
  pageSize: number;
};

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listCases = async (client: SessionClient, query: CaseQuery): Promise<RequestResult<HostCasePage>> => {
  const response = await client.api.GET('/api/host/cases', {
    params: {
      query: {
        hostId: query.hostId,
        page: query.page,
        pageSize: query.pageSize,
        ...(query.state ? { state: query.state } : {}),
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getCase = async (client: SessionClient, caseId: string): Promise<RequestResult<HostCase>> => {
  const response = await client.api.GET('/api/host/cases/{caseId}', { params: { path: { caseId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const openCase = async (client: SessionClient, input: OpenCasePayload): Promise<RequestResult<HostCase>> => {
  const response = await client.api.POST('/api/host/cases', { body: input });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const closeCase = async (client: SessionClient, caseId: string, expectedVersion: number, resolution: string): Promise<RequestResult<HostCase>> => {
  const response = await client.api.POST('/api/host/cases/{caseId}/close', {
    params: { path: { caseId } },
    body: { expectedVersion, resolution },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listEvidence = async (client: SessionClient, caseId: string): Promise<RequestResult<CaseEvidence[]>> => {
  const response = await client.api.GET('/api/host/cases/{caseId}/evidence', { params: { path: { caseId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const addEvidence = async (client: SessionClient, caseId: string, input: AddEvidencePayload): Promise<RequestResult<CaseEvidence>> => {
  const response = await client.api.POST('/api/host/cases/{caseId}/evidence', {
    params: { path: { caseId } },
    body: input,
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getEvidence = async (client: SessionClient, evidenceId: string): Promise<RequestResult<CaseEvidenceContent>> => {
  const response = await client.api.GET('/api/host/cases/evidence/{evidenceId}', { params: { path: { evidenceId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listReservationChoices = async (client: SessionClient, hostId: string, locale: string): Promise<RequestResult<CaseOption[]>> => {
  const response = await client.api.GET('/api/host/reservations', {
    params: { query: { hostId, page: 1, pageSize: 100, locale } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data.items.map((item) => ({ value: item.id, label: item.reference })), error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
