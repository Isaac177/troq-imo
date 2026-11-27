import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { RequestResult } from '@/types/settings/settings';

type Contact = components['schemas']['HostContact'];

export const getHostContact = async (client: SessionClient, hostId: string): Promise<RequestResult<Contact>> => {
  const response = await client.api.GET('/api/host/hosts/{hostId}/contact', { params: { path: { hostId } } });
  if (response.error || !response.data) return { data: null, error: apiErrorCode(response.error) };
  return { data: response.data.data, error: null };
};

export const updateHostContact = async (client: SessionClient, hostId: string, input: { guestContactEmail: string; guestContactPhone: string }): Promise<RequestResult<Contact>> => {
  const response = await client.api.PATCH('/api/host/hosts/{hostId}/contact', { params: { path: { hostId } }, body: input });
  if (response.error || !response.data) return { data: null, error: apiErrorCode(response.error) };
  return { data: response.data.data, error: null };
};
