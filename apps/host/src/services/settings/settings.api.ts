import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { Preference, PreferenceList, RequestResult } from '@/types/settings/settings';

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const getPreferences = async (client: SessionClient): Promise<RequestResult<PreferenceList>> => {
  const response = await client.api.GET('/api/host/notification-preferences');
  if (!response.data) return { data: null, error: 'INTERNAL_ERROR' };
  return { data: response.data.data, error: null };
};

export const setPreference = async (client: SessionClient, preference: Preference): Promise<RequestResult<Preference>> => {
  const response = await client.api.PUT('/api/host/notification-preferences', { body: preference });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');

export const updateProfile = async (client: SessionClient, input: { firstName: string; lastName: string; preferredLanguage: string }): Promise<RequestResult<{ message: string }>> => {
  const response = await client.api.PATCH('/api/guest/profile', { body: input });
  if (!response.data) return { data: null, error: 'INTERNAL_ERROR' };
  return { data: response.data.data, error: null };
};
