import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { CalendarData, RequestResult } from '@/types/calendar/calendar';

type BlockView = components['schemas']['BlockView'];

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const getCalendar = async (client: SessionClient, propertyId: string, from: string, to: string, page: number): Promise<RequestResult<CalendarData>> => {
  const response = await client.api.GET('/api/host/calendar', {
    params: { query: { propertyId, from, to, page, pageSize: 10 } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const createBlock = async (client: SessionClient, input: { propertyId: string; unitTypeId: string; startDate: string; endDate: string; reason: string }): Promise<RequestResult<BlockView>> => {
  const response = await client.api.POST('/api/host/blocks', {
    body: { ...input, idempotencyKey: crypto.randomUUID() },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const removeBlock = async (client: SessionClient, blockId: string, expectedVersion: number): Promise<RequestResult<BlockView>> => {
  const response = await client.api.DELETE('/api/host/blocks/{blockId}', {
    params: { path: { blockId } },
    body: { expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};
