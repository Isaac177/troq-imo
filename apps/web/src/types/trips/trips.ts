import type { components } from '@mungini/api';

export type Trip = components['schemas']['MyReservationView'];
export type TripPage = { items: Trip[]; total: number; page: number; pageSize: number };
export type TripCounts = Record<'all' | 'pending' | 'upcoming' | 'active' | 'past' | 'cancelled', number>;
export type TripPageWithCounts = TripPage & { counts: TripCounts };
export type City = components['schemas']['CityView'];
export type TripImage = { id: string; url: string; altEn: string; altFr: string };
export type TripDetail = components['schemas']['ReservationView'];
export type CancellationPreview = components['schemas']['Envelope_CancellationView']['data'];
export type CancelledStay = components['schemas']['Envelope_CancelledStayView']['data'];
