import { browserTokenStore, createSessionClient } from '@mungini/session';
import { apiBaseUrl } from '@/env';

export const sessionClient = createSessionClient({
  baseUrl: apiBaseUrl,
  store: browserTokenStore('mungini-host'),
});
