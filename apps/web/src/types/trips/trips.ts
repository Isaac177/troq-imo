import type { components } from '@mungini/api';

export type Trip = components['schemas']['MyReservationView'];
export type TripPage = components['schemas']['Envelope_Page_MyReservationView']['data'];
export type City = { code: string; name: string };
export type TripDetail = components['schemas']['ReservationView'];
