import type { components } from '@mungini/api';

export type Trip = components['schemas']['MyReservationView'];
export type TripPage = components['schemas']['Envelope_Page_MyReservationView']['data'];
export type City = components['schemas']['CityView'];
export type TripDetail = components['schemas']['ReservationView'];
export type CancellationPreview = components['schemas']['Envelope_CancellationView']['data'];
export type CancelledStay = components['schemas']['Envelope_CancelledStayView']['data'];
