import type { components } from '@mungini/api';

export type AdminCase = components['schemas']['AdminCaseItem'];
export type AdminCaseDetail = components['schemas']['AdminCaseDetail'];
export type CaseEvidence = components['schemas']['EvidenceView'];
export type EvidenceContent = components['schemas']['AdminEvidenceContent'];
export type RequestResult<T> = { data: T | null; error: string | null };
