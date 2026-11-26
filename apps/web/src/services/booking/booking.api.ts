import type { SessionClient } from '@mungini/session';
import type { components } from '@mungini/api';

export const getAvailability = async (client: SessionClient, propertyId: string, checkIn: string, checkOut: string, adults: number, children: number) => {
  const { data } = await client.api.GET('/api/public/properties/{propertyId}/availability', { params: { path: { propertyId }, query: { checkIn, checkOut, adults: String(adults), children: String(children) } } });
  if (!data) throw new Error('AVAILABILITY_FAILED');
  return data.data;
};

export const createQuote = async (client: SessionClient, body: components['schemas']['QuoteRequest']) => {
  const { data } = await client.api.POST('/api/guest/quotes', { body });
  if (!data) throw new Error('QUOTE_FAILED');
  return data.data;
};

export const createBooking = async (client: SessionClient, body: components['schemas']['HoldRequest']) => {
  const { data } = await client.api.POST('/api/guest/reservations', { body });
  if (!data) throw new Error('BOOKING_FAILED');
  return data.data;
};
