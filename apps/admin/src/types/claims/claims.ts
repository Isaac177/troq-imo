import type { components } from '@mungini/api';

export type ClaimRow = components['schemas']['ClaimRow'];
export type ClaimSummary = components['schemas']['ClaimSummary'];
export type ClaimDetail = components['schemas']['ClaimDetail'];
export type ClaimDecision = components['schemas']['DecisionRow'];
export type ClaimEvidence = components['schemas']['EvidenceRow'];
export type PayoutResult = components['schemas']['PayoutResult'];
export type RequestResult<T> = { data: T | null; error: string | null };
