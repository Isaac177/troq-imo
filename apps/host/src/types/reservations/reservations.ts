import type { components } from '@mungini/api';

export type HostReservation = components['schemas']['OperatorReservationView'];
export type ReservationGuest = NonNullable<HostReservation['guest']>;
export type ReservationNight = HostReservation['nights'][number];
export type ReservationState = 'PENDING_PAYMENT' | 'REQUESTED' | 'CONFIRMED' | 'DECLINED' | 'CANCELLED' | 'EXPIRED' | 'CHECKED_IN' | 'CHECKED_OUT' | 'NO_SHOW';
export type HostReservationPage = { items: HostReservation[]; total: number; page: number; pageSize: number };
export type PropertyOption = { value: string; label: string };
export type RequestResult<T> = { data: T | null; error: string | null };
