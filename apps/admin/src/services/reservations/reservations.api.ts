import type { SessionClient } from '@mungini/session';

export const listReservations = (client: SessionClient, search: string, state: string, page: number, pageSize: number) =>
  client.api.GET('/api/admin/reservations', {
    params: { query: { page, pageSize, ...(search ? { search } : {}), ...(state ? { state } : {}) } },
  });

export const getReservation = (client: SessionClient, reservationId: string, page: number) =>
  client.api.GET('/api/admin/reservations/{reservationId}', {
    params: { path: { reservationId }, query: { page, pageSize: 10 } },
  });

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
