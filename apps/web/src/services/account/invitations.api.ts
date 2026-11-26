import type { SessionClient } from '@mungini/session';
import type { InvitationPage } from '@/types/account/invitations';

export const listInvitations = async (client: SessionClient, page: number): Promise<InvitationPage> => {
  const result = await client.api.GET('/api/guest/invitations', { params: { query: { page, pageSize: 10 } } });
  if (!result.data) throw new Error('INVITATIONS_LOAD_FAILED');
  return result.data.data;
};

export const acceptInvitation = async (client: SessionClient, invitationId: string): Promise<void> => {
  const result = await client.api.POST('/api/guest/invitations/{invitationId}/accept', { params: { path: { invitationId } } });
  if (!result.data) throw new Error(result.error?.error.code ?? 'INVITATION_ACCEPT_FAILED');
};
