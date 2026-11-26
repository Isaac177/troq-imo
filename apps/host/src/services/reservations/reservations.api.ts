import type { components } from '@mungini/api';
import type { SessionClient } from '@mungini/session';
import { apiErrorCode } from '@mungini/session';
import type { HostReservation, HostReservationPage, RequestResult } from '@/types/reservations/reservations';

type ReservationView = components['schemas']['ReservationView'];
type AmendmentProposal = components['schemas']['AmendmentProposal'];
type MessageView = components['schemas']['MessageView'];
type MessagePage = components['schemas']['Envelope_Page_MessageView']['data'];
type UnitOption = components['schemas']['UnitOptionView'];
type Assignment = components['schemas']['AssignmentView'];

export type ReservationQuery = {
  hostId: string;
  propertyId: string;
  state: string;
  search: string;
  page: number;
  pageSize: number;
  locale: string;
};

const fail = <T>(error: unknown): RequestResult<T> => ({ data: null, error: apiErrorCode(error) });

const reservationQuery = (query: ReservationQuery) => ({
  hostId: query.hostId,
  page: query.page,
  pageSize: query.pageSize,
  locale: query.locale,
  ...(query.propertyId ? { propertyId: query.propertyId } : {}),
  ...(query.state ? { state: query.state } : {}),
  ...(query.search.trim() ? { search: query.search.trim() } : {}),
});

export const listReservations = async (client: SessionClient, query: ReservationQuery): Promise<RequestResult<HostReservationPage>> => {
  const response = await client.api.GET('/api/host/reservations', { params: { query: reservationQuery(query) } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const exportReservations = async (client: SessionClient, query: ReservationQuery): Promise<RequestResult<HostReservationPage>> => {
  const response = await client.api.GET('/api/host/reservations/export', { params: { query: reservationQuery(query) } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const getReservation = async (client: SessionClient, reservationId: string, locale: string): Promise<RequestResult<HostReservation>> => {
  const response = await client.api.GET('/api/host/reservations/{reservationId}', {
    params: { path: { reservationId }, query: { locale } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const decideReservation = async (client: SessionClient, reservationId: string, action: 'ACCEPT' | 'DECLINE', expectedVersion: number): Promise<RequestResult<ReservationView>> => {
  const response = await client.api.POST('/api/host/reservations/{reservationId}/decision', {
    params: { path: { reservationId } },
    body: { action, expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const decideSpecialRequest = async (client: SessionClient, reservationId: string, decision: 'ACCEPT' | 'DECLINE', expectedVersion: number): Promise<RequestResult<ReservationView>> => {
  const response = await client.api.POST('/api/host/reservations/{reservationId}/special-request', {
    params: { path: { reservationId } },
    body: { decision, expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const checkInReservation = async (client: SessionClient, reservationId: string, expectedVersion: number): Promise<RequestResult<ReservationView>> => {
  const response = await client.api.POST('/api/host/reservations/{reservationId}/check-in', {
    params: { path: { reservationId } },
    body: { expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const checkOutReservation = async (client: SessionClient, reservationId: string, expectedVersion: number): Promise<RequestResult<ReservationView>> => {
  const response = await client.api.POST('/api/host/reservations/{reservationId}/check-out', {
    params: { path: { reservationId } },
    body: { expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const noShowReservation = async (client: SessionClient, reservationId: string, expectedVersion: number): Promise<RequestResult<ReservationView>> => {
  const response = await client.api.POST('/api/host/reservations/{reservationId}/no-show', {
    params: { path: { reservationId } },
    body: { expectedVersion },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const cancelReservation = async (client: SessionClient, reservationId: string, expectedVersion: number, reason: string): Promise<RequestResult<ReservationView>> => {
  const response = await client.api.POST('/api/host/reservations/{reservationId}/cancel', {
    params: { path: { reservationId } },
    body: { expectedVersion, reason },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const proposeAmendment = async (client: SessionClient, reservationId: string, checkIn: string, checkOut: string): Promise<RequestResult<AmendmentProposal>> => {
  const response = await client.api.POST('/api/host/reservations/{reservationId}/amendments', {
    params: { path: { reservationId } },
    body: { checkIn, checkOut },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listMessages = async (client: SessionClient, reservationId: string): Promise<RequestResult<MessagePage>> => {
  const response = await client.api.GET('/api/host/reservations/{reservationId}/messages', {
    params: { path: { reservationId }, query: { page: 1, pageSize: 50 } },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const sendMessage = async (client: SessionClient, reservationId: string, body: string): Promise<RequestResult<MessageView>> => {
  const response = await client.api.POST('/api/host/reservations/{reservationId}/messages', {
    params: { path: { reservationId } },
    body: { body },
  });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const listCities = (client: SessionClient) => client.api.GET('/api/public/cities');

export const listUnitOptions = async (client: SessionClient, itemId: string): Promise<RequestResult<UnitOption[]>> => {
  const response = await client.api.GET('/api/host/reservation-items/{itemId}/unit-options', { params: { path: { itemId } } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const assignUnit = async (client: SessionClient, itemId: string, physicalUnitId: string): Promise<RequestResult<Assignment>> => {
  const response = await client.api.POST('/api/host/unit-assignments', { body: { reservationItemId: itemId, physicalUnitId } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};

export const moveAssignment = async (client: SessionClient, assignmentId: string, physicalUnitId: string, fromDate: string): Promise<RequestResult<Assignment>> => {
  const response = await client.api.POST('/api/host/unit-assignments/{assignmentId}/move', { params: { path: { assignmentId } }, body: { physicalUnitId, fromDate } });
  if (response.error || !response.data) return fail(response.error);
  return { data: response.data.data, error: null };
};
