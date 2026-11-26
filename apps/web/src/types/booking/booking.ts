import type { components } from '@mungini/api';

export type AvailabilityOption = components['schemas']['AvailabilityOptionView'];
export type Quote = components['schemas']['QuoteView'];
export type Policy = { id: string; kind: 'PAYMENT' | 'CANCELLATION'; content: Record<string, unknown> };
export type StayOption = { id: string; name: string };
