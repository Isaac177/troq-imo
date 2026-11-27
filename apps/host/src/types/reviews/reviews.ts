import type { components } from '@mungini/api';

export type Review = components['schemas']['ReviewView'] & { guestName?: string | null };
export type RequestResult<T> = { data: T | null; error: string | null };
