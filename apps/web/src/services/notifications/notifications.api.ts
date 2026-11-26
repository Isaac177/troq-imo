import type { SessionClient } from '@mungini/session';

export const listNotifications = async (client: SessionClient, page: number, unreadOnly: boolean, locale: string, pageSize = 10) => {
  const result = await client.api.GET('/api/guest/notifications', {
    params: { query: { page, pageSize, locale, unreadOnly } },
  });
  if (!result.data) throw new Error('NOTIFICATIONS_LOAD_FAILED');
  return result.data.data;
};

export const getUnreadCount = async (client: SessionClient) => {
  const result = await client.api.GET('/api/guest/notifications/unread-count');
  if (!result.data) throw new Error('NOTIFICATIONS_UNREAD_FAILED');
  return result.data.data.unread;
};

export const markNotificationRead = async (client: SessionClient, id: string) => {
  const result = await client.api.POST('/api/guest/notifications/{id}/read', { params: { path: { id } } });
  if (!result.data) throw new Error('NOTIFICATION_READ_FAILED');
  return result.data.data.updated;
};

export const markAllNotificationsRead = async (client: SessionClient) => {
  const result = await client.api.POST('/api/guest/notifications/read-all');
  if (!result.data) throw new Error('NOTIFICATIONS_READ_ALL_FAILED');
  return result.data.data.updated;
};

export const listNotificationCities = async (client: SessionClient) => {
  const result = await client.api.GET('/api/public/cities');
  if (!result.data) throw new Error('NOTIFICATION_CITIES_FAILED');
  return result.data.data;
};
