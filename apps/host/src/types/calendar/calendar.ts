import type { components } from '@mungini/api';

export type CalendarData = components['schemas']['InventoryCalendarView'];
export type CalendarReservation = components['schemas']['CalendarReservation'];
export type CalendarBlock = components['schemas']['CalendarBlock'];
export type CalendarRestriction = components['schemas']['RestrictionView'];
export type CalendarUnit = components['schemas']['CalendarUnit'];
export type CalendarDay = components['schemas']['CalendarDay'];
export type PropertyOption = { value: string; label: string };
export type RequestResult<T> = { data: T | null; error: string | null };
