import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';

export const updateProfile = async (client: SessionClient, body: components['schemas']['ProfileInput']) => {
  const result = await client.api.PATCH('/api/guest/profile', { body });
  if (result.error || !result.data) throw new Error('PROFILE_UPDATE_FAILED');
};
