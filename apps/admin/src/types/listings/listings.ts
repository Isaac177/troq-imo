import type { components } from '@mungini/api';

export type QueueItem = components['schemas']['QueueItem'];
export type AdminRevision = components['schemas']['AdminRevisionView'];
export type ListingContent = components['schemas']['ListingContent'];
export type Revision = components['schemas']['RevisionView'];
export type ListingDecisionBody = { decision: string; reasonCodes: string[]; guidance: string | null; expectedVersion: number };
export type RequestResult<T> = { data: T | null; error: string | null; blockers: string[] };
