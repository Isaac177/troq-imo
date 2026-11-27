import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';


export const getLanguages = async (client: SessionClient): Promise<string[]> => {
  const result = await client.api.GET('/api/public/languages');
  if (result.error || !result.data) throw new Error('LANGUAGES_LOAD_FAILED');
  return result.data.data;
};

export const updateProfile = async (client: SessionClient, body: components['schemas']['ProfileInput']) => {
  const result = await client.api.PATCH('/api/guest/profile', { body });
  if (result.error || !result.data) throw new Error('PROFILE_UPDATE_FAILED');
};
