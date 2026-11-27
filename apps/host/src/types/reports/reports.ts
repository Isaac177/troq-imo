import type { components } from '@mungini/api';

export type ReportSummary = components['schemas']['AnalyticsSummary'];
export type ReportRange = { start: string | null; end: string | null };
export type ReportProperty = { id: string; nameEn: string; nameFr: string };
