import type { SessionClient } from '@mungini/session';

export const listTrips = async (client: SessionClient, page: number, tab: string) => {
  const result = await client.api.GET('/api/guest/reservations/mine', {
    params: { query: { page, pageSize: 10, ...(tab ? { tab } : {}) } },
  });
  if (!result.data) throw new Error('TRIPS_LOAD_FAILED');
  return result.data.data;
};

export const listCities = async (client: SessionClient) => {
  const result = await client.api.GET('/api/public/cities');
  if (!result.data) throw new Error('CITIES_LOAD_FAILED');
  return result.data.data;
};

export const getTrip = async (client: SessionClient, reservationId: string) => {
  const result = await client.api.GET('/api/guest/reservations/{reservationId}', { params: { path: { reservationId } } });
  if (!result.data) throw new Error('TRIP_LOAD_FAILED');
  return result.data.data;
};

export const getCancellationPreview = async (client: SessionClient, reservationId: string) => {
  const result = await client.api.GET('/api/guest/reservations/{reservationId}/cancellation', { params: { path: { reservationId } } });
  if (!result.data) throw new Error(result.error?.error.code ?? 'CANCELLATION_PREVIEW_FAILED');
  return result.data.data;
};

export const cancelTrip = async (client: SessionClient, reservationId: string, expectedVersion: number, idempotencyKey: string) => {
  const result = await client.api.POST('/api/guest/reservations/{reservationId}/cancel', {
    params: { path: { reservationId } },
    body: { expectedVersion, idempotencyKey },
  });
  if (!result.data) throw new Error(result.error?.error.code ?? 'TRIP_CANCEL_FAILED');
  return result.data.data;
};

export const withdrawTrip = async (client: SessionClient, reservationId: string, expectedVersion: number) => {
  const result = await client.api.POST('/api/guest/reservations/{reservationId}/withdraw', {
    params: { path: { reservationId } },
    body: { expectedVersion },
  });
  if (!result.data) throw new Error(result.error?.error.code ?? 'TRIP_WITHDRAW_FAILED');
  return result.data.data;
};
