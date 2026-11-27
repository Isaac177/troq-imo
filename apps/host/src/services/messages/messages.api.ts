import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { RequestResult } from '@/types/reservations/reservations';
import type { ConversationPage } from '@/types/messages/messages';


export const listConversations = async (client: SessionClient, hostId: string, page: number, search: string): Promise<RequestResult<ConversationPage>> => {
  const response = await client.api.GET('/api/host/message-conversations', { params: { query: { hostId, page, pageSize: 20, ...(search.trim() ? { search: search.trim() } : {}) } } });
  if (response.error || !response.data) return { data: null, error: apiErrorCode(response.error) };
  return { data: response.data.data, error: null };
};

export const markMessagesRead = async (client: SessionClient, reservationId: string, messageId: string): Promise<boolean> => {
  const result = await client.api.POST('/api/host/reservations/{reservationId}/messages/read', { params: { path: { reservationId } }, body: { messageId } });
  return Boolean(result.data) && !result.error;
};
