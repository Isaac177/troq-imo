import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { ReservationRow } from '@/types/reservations/reservations';

export type ReservationExportPage = { items: ReservationRow[]; total: number };
export type ReservationExportResult = { data: ReservationExportPage | null; error: string | null };

export const listReservations = (client: SessionClient, search: string, state: string, page: number, pageSize: number) =>
  client.api.GET('/api/admin/reservations', {
    params: { query: { page, pageSize, ...(search ? { search } : {}), ...(state ? { state } : {}) } },
  });

export const listReservationsExport = async (
  client: SessionClient,
  search: string,
  state: string,
  page: number,
  pageSize: number,
): Promise<ReservationExportResult> => {
  const response = await client.api.GET('/api/admin/reservations/export', {
    params: { query: { page, pageSize, ...(search ? { search } : {}), ...(state ? { state } : {}) } },
  });
  if (response.error || !response.data) return { data: null, error: apiErrorCode(response.error) };
  return { data: response.data.data, error: null };
};

export const getReservation = (client: SessionClient, reservationId: string, page: number) =>
  client.api.GET('/api/admin/reservations/{reservationId}', {
    params: { path: { reservationId }, query: { page, pageSize: 10 } },
  });

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');
