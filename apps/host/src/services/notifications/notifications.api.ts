import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { MarkedCount, NotificationFilter, NotificationPage, RequestResult, UnreadCount } from '@/types/notifications/notifications';

type ListNotificationsInput = {
  locale: string;
  filter: NotificationFilter;
  page: number;
  pageSize: number;
};

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

export const listNotifications = async (client: SessionClient, input: ListNotificationsInput): Promise<RequestResult<NotificationPage>> => {
  const response = await client.api.GET('/api/host/notifications', {
    params: {
      query: {
        locale: input.locale,
        unreadOnly: input.filter === 'unread',
        page: input.page,
        pageSize: input.pageSize,
      },
    },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getUnreadCount = async (client: SessionClient): Promise<RequestResult<UnreadCount>> => {
  const response = await client.api.GET('/api/host/notifications/unread-count');
  if (!response.data) return fail(null);
  return { data: response.data.data, error: null };
};

export const markNotificationRead = async (client: SessionClient, id: string): Promise<RequestResult<MarkedCount>> => {
  const response = await client.api.POST('/api/host/notifications/{id}/read', {
    params: { path: { id } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const markAllNotificationsRead = async (client: SessionClient): Promise<RequestResult<MarkedCount>> => {
  const response = await client.api.POST('/api/host/notifications/read-all');
  if (!response.data) return fail(null);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
