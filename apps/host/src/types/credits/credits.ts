import type { components } from '@mungini/api';

export type CreditSummary = components['schemas']['MineSummary'];
export type CreditLedgerRow = components['schemas']['MineLedgerRow'];
export type CreditLedgerPage = components['schemas']['Envelope_Page_MineLedgerRow']['data'];
export type RequestResult<T> = { data: T | null; error: string | null };
