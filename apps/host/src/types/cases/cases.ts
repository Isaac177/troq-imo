import type { components } from '@mungini/api';
import type { StatusTone } from '@mungini/theme/tones';

export type HostCase = components['schemas']['CaseView'];
export type CaseEvidence = components['schemas']['EvidenceView'];
export type CaseEvidenceContent = components['schemas']['EvidenceContent'];
export type CaseState = 'OPEN' | 'AWAITING_RESPONSE' | 'RESOLVED' | 'CLOSED';
export type CaseType = 'GUEST_COMPLAINT' | 'OPERATOR_COMPLAINT' | 'DAMAGE_CLAIM' | 'REFUND_DISPUTE' | 'NO_SHOW_DISPUTE' | 'GENERAL';
export type HostCaseType = Exclude<CaseType, 'REFUND_DISPUTE'>;
export type HostCasePage = { items: HostCase[]; total: number; page: number; pageSize: number };
export type CaseOption = { value: string; label: string };
export type RequestResult<T> = { data: T | null; error: string | null };
export type CaseResolution = unknown;

export type OpenCasePayload = {
  caseType: HostCaseType;
  hostId: string;
  reservationId: string;
  reason: string;
  claimAmount?: string;
};

export type AddEvidencePayload = {
  fileName: string;
  mimeType: string;
  contentBase64: string;
};

export const CASE_STATES: CaseState[] = ['OPEN', 'AWAITING_RESPONSE', 'RESOLVED', 'CLOSED'];

export const HOST_CASE_TYPES: HostCaseType[] = ['GUEST_COMPLAINT', 'OPERATOR_COMPLAINT', 'DAMAGE_CLAIM', 'NO_SHOW_DISPUTE', 'GENERAL'];

export const OPEN_CASE_STATES = ['OPEN', 'AWAITING_RESPONSE'];

export const CASE_STATE_TONES: Record<string, StatusTone> = {
  OPEN: 'info',
  AWAITING_RESPONSE: 'warning',
  RESOLVED: 'success',
  CLOSED: 'muted',
};

export const EVIDENCE_MIME_TYPES = ['application/pdf', 'image/png', 'image/jpeg'];

export const MAX_EVIDENCE_BYTES = 5 * 1024 * 1024;
