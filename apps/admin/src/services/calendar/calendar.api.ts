import type { SessionClient } from '@mungini/session';

export const listProperties = (client: SessionClient, search: string, page: number) =>
  client.api.GET('/api/admin/properties', { params: { query: { search, page, pageSize: 20 } } });

export const getCalendar = (client: SessionClient, propertyId: string, from: string, to: string, page: number) =>
  client.api.GET('/api/admin/calendar', { params: { query: { propertyId, from, to, page, pageSize: 10 } } });
