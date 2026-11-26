import type { components } from '@mungini/api';

export type AdminProperty = components['schemas']['AdminPropertyItem'];
export type AdminPropertyDetail = components['schemas']['AdminPropertyDetail'];
export type RequestResult<T> = { data: T | null; error: string | null };
